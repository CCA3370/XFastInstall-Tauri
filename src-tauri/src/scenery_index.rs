//! Scenery index management module
//!
//! This module manages a persistent JSON index of scenery classifications
//! with cache invalidation based on directory modification times.

use crate::logger;
use crate::models::{SceneryIndex, SceneryIndexStats, SceneryPackageInfo};
use crate::scenery_classifier::classify_scenery;
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Resolve Windows shortcut (.lnk) to actual path using Windows COM API
#[cfg(windows)]
fn resolve_shortcut(lnk_path: &Path) -> Option<PathBuf> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use winapi::shared::guiddef::GUID;
    use winapi::shared::minwindef::MAX_PATH;
    use winapi::shared::winerror::{S_OK, S_FALSE, RPC_E_CHANGED_MODE};
    use winapi::um::combaseapi::{CoCreateInstance, CoInitializeEx, CoUninitialize};
    use winapi::um::objbase::COINIT_APARTMENTTHREADED;
    use winapi::um::objidl::IPersistFile;
    use winapi::um::shobjidl_core::IShellLinkW;
    use winapi::Interface;

    // CLSID_ShellLink: 00021401-0000-0000-C000-000000000046
    const CLSID_SHELL_LINK: GUID = GUID {
        Data1: 0x00021401,
        Data2: 0x0000,
        Data3: 0x0000,
        Data4: [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    };

    unsafe {
        // Initialize COM - try both threading models
        let hr = CoInitializeEx(ptr::null_mut(), COINIT_APARTMENTTHREADED);
        let need_uninit = if hr == S_OK || hr == S_FALSE {
            true
        } else if hr == RPC_E_CHANGED_MODE {
            // COM already initialized with different threading model, try to continue anyway
            logger::log_info(
                &format!("  COM already initialized with different threading model"),
                Some("scenery_index")
            );
            false
        } else {
            logger::log_info(
                &format!("  Failed to initialize COM, HRESULT: 0x{:08X}", hr),
                Some("scenery_index")
            );
            return None;
        };

        let mut result = None;

        // Create IShellLink instance
        let mut shell_link: *mut IShellLinkW = ptr::null_mut();
        let hr = CoCreateInstance(
            &CLSID_SHELL_LINK,
            ptr::null_mut(),
            1, // CLSCTX_INPROC_SERVER
            &IShellLinkW::uuidof(),
            &mut shell_link as *mut *mut _ as *mut *mut _,
        );

        if hr == S_OK && !shell_link.is_null() {
            // Query IPersistFile interface
            let mut persist_file: *mut IPersistFile = ptr::null_mut();
            let hr = (*shell_link).QueryInterface(
                &IPersistFile::uuidof(),
                &mut persist_file as *mut *mut _ as *mut *mut _,
            );

            if hr == S_OK && !persist_file.is_null() {
                // Convert path to wide string
                let wide_path: Vec<u16> = lnk_path
                    .as_os_str()
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();

                // Load the shortcut file
                let hr = (*persist_file).Load(wide_path.as_ptr(), 0);

                if hr == S_OK {
                    // Get the target path
                    let mut target_path = vec![0u16; MAX_PATH];
                    let hr = (*shell_link).GetPath(
                        target_path.as_mut_ptr(),
                        MAX_PATH as i32,
                        ptr::null_mut(),
                        0,
                    );

                    if hr == S_OK {
                        // Find the null terminator
                        let len = target_path.iter().position(|&c| c == 0).unwrap_or(MAX_PATH);
                        let target_str = String::from_utf16_lossy(&target_path[..len]);

                        logger::log_info(
                            &format!("  Shortcut target (COM API): {:?}", target_str),
                            Some("scenery_index")
                        );

                        let path = PathBuf::from(target_str);
                        if path.exists() && path.is_dir() {
                            result = Some(path);
                        }
                    } else {
                        logger::log_info(
                            &format!("  GetPath failed with HRESULT: 0x{:08X}", hr),
                            Some("scenery_index")
                        );
                    }
                } else {
                    logger::log_info(
                        &format!("  Failed to load shortcut file, HRESULT: 0x{:08X}", hr),
                        Some("scenery_index")
                    );
                }

                (*persist_file).Release();
            } else {
                logger::log_info(
                    &format!("  Failed to query IPersistFile, HRESULT: 0x{:08X}", hr),
                    Some("scenery_index")
                );
            }

            (*shell_link).Release();
        } else {
            logger::log_info(
                &format!("  Failed to create IShellLink, HRESULT: 0x{:08X}", hr),
                Some("scenery_index")
            );
        }

        if need_uninit {
            CoUninitialize();
        }

        result
    }
}

#[cfg(not(windows))]
fn resolve_shortcut(_lnk_path: &Path) -> Option<PathBuf> {
    None
}


const INDEX_VERSION: u32 = 1;

/// Manager for scenery index operations
pub struct SceneryIndexManager {
    xplane_path: PathBuf,
    index_path: PathBuf,
}

impl SceneryIndexManager {
    /// Create a new index manager
    pub fn new(xplane_path: &Path) -> Self {
        let index_path = get_index_file_path();
        Self {
            xplane_path: xplane_path.to_path_buf(),
            index_path,
        }
    }

    /// Load index from disk or create new empty index
    pub fn load_index(&self) -> Result<SceneryIndex> {
        if self.index_path.exists() {
            let content = fs::read_to_string(&self.index_path)?;
            let index: SceneryIndex = serde_json::from_str(&content)?;

            // Check version compatibility
            if index.version != INDEX_VERSION {
                logger::log_info("Index version mismatch, rebuilding index", Some("scenery_index"));
                return Ok(self.create_empty_index());
            }

            Ok(index)
        } else {
            Ok(self.create_empty_index())
        }
    }

    /// Save index to disk
    pub fn save_index(&self, index: &SceneryIndex) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.index_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write to temp file first, then rename for atomic write
        let temp_path = self.index_path.with_extension("tmp");
        let content = serde_json::to_string_pretty(index)?;
        fs::write(&temp_path, &content)?;
        fs::rename(&temp_path, &self.index_path)?;

        Ok(())
    }

    /// Update or add a single package in the index
    pub fn update_package(&self, package_info: SceneryPackageInfo) -> Result<()> {
        let mut index = self.load_index()?;
        index.packages.insert(package_info.folder_name.clone(), package_info);
        index.last_updated = SystemTime::now();
        self.save_index(&index)?;
        Ok(())
    }

    /// Remove a package from the index
    pub fn remove_package(&self, folder_name: &str) -> Result<()> {
        let mut index = self.load_index()?;
        if index.packages.remove(folder_name).is_some() {
            index.last_updated = SystemTime::now();
            self.save_index(&index)?;
        }
        Ok(())
    }

    /// Rebuild entire index by scanning all scenery packages
    pub fn rebuild_index(&self) -> Result<SceneryIndex> {
        let custom_scenery_path = self.xplane_path.join("Custom Scenery");
        if !custom_scenery_path.exists() {
            return Err(anyhow!("Custom Scenery folder not found"));
        }

        // Collect all scenery folders (including symlinks and .lnk shortcuts)
        let scenery_folders: Vec<PathBuf> = fs::read_dir(&custom_scenery_path)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();

                // Check if it's a .lnk file (Windows shortcut)
                if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("lnk")) {
                    logger::log_info(
                        &format!("Attempting to resolve shortcut: {:?}", path.file_name().unwrap()),
                        Some("scenery_index")
                    );

                    // Try to resolve the shortcut
                    if let Some(target) = resolve_shortcut(&path) {
                        logger::log_info(
                            &format!("✓ Resolved shortcut {:?} -> {:?}", path.file_name().unwrap(), target),
                            Some("scenery_index")
                        );
                        return Some(target);
                    } else {
                        logger::log_info(
                            &format!("✗ Failed to resolve shortcut: {:?}", path),
                            Some("scenery_index")
                        );
                        return None;
                    }
                }

                // Check if it's a directory (including symlinks)
                if path.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                    return Some(path);
                }

                None
            })
            .collect();

        logger::log_info(&format!("Rebuilding scenery index for {} packages", scenery_folders.len()), Some("scenery_index"));

        // Classify all packages
        // Use sequential processing in debug log mode for ordered logs, parallel otherwise
        let packages: HashMap<String, SceneryPackageInfo> = if logger::is_debug_enabled() {
            // Sequential processing for ordered debug logs
            scenery_folders
                .iter()
                .filter_map(|folder| {
                    match classify_scenery(folder, &self.xplane_path) {
                        Ok(info) => Some((info.folder_name.clone(), info)),
                        Err(e) => {
                            logger::log_info(&format!("Failed to classify {:?}: {}", folder, e), Some("scenery_index"));
                            None
                        }
                    }
                })
                .collect()
        } else {
            // Parallel processing for better performance when not in debug mode
            scenery_folders
                .par_iter()
                .filter_map(|folder| {
                    match classify_scenery(folder, &self.xplane_path) {
                        Ok(info) => Some((info.folder_name.clone(), info)),
                        Err(e) => {
                            logger::log_info(&format!("Failed to classify {:?}: {}", folder, e), Some("scenery_index"));
                            None
                        }
                    }
                })
                .collect()
        };

        let index = SceneryIndex {
            version: INDEX_VERSION,
            packages,
            last_updated: SystemTime::now(),
        };

        self.save_index(&index)?;
        logger::log_info(&format!("Scenery index rebuilt with {} packages", index.packages.len()), Some("scenery_index"));

        Ok(index)
    }

    /// Update index incrementally - only re-classify modified packages
    pub fn update_index(&self) -> Result<SceneryIndex> {
        let custom_scenery_path = self.xplane_path.join("Custom Scenery");
        if !custom_scenery_path.exists() {
            return Err(anyhow!("Custom Scenery folder not found"));
        }

        let mut index = self.load_index()?;

        // Get current scenery folders (including symlinks and .lnk shortcuts)
        let current_folders: HashMap<String, PathBuf> = fs::read_dir(&custom_scenery_path)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();

                // Check if it's a .lnk file (Windows shortcut)
                if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("lnk")) {
                    // Try to resolve the shortcut
                    if let Some(target) = resolve_shortcut(&path) {
                        // Use the target folder name (not the .lnk filename)
                        if let Some(name) = target.file_name().and_then(|s| s.to_str()) {
                            return Some((name.to_string(), target));
                        }
                    }
                    return None;
                }

                // Check if it's a directory (including symlinks)
                if path.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                    if let Some(name) = e.file_name().to_str() {
                        return Some((name.to_string(), path));
                    }
                }

                None
            })
            .collect();

        // Remove stale entries (deleted folders)
        let stale_keys: Vec<String> = index
            .packages
            .keys()
            .filter(|name| !current_folders.contains_key(*name))
            .cloned()
            .collect();

        for key in stale_keys {
            index.packages.remove(&key);
            crate::log_debug!(&format!("Removed stale entry: {}", key), "scenery_index");
        }

        // Find packages that need updating
        let packages_to_update: Vec<PathBuf> = current_folders
            .iter()
            .filter(|(name, path)| {
                // Check if package is new or modified
                if let Some(existing) = index.packages.get(*name) {
                    // Compare modification times
                    if let Ok(metadata) = fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            return modified > existing.indexed_at;
                        }
                    }
                    false
                } else {
                    true // New package
                }
            })
            .map(|(_, path)| path.clone())
            .collect();

        if !packages_to_update.is_empty() {
            logger::log_info(&format!("Updating {} scenery packages", packages_to_update.len()), Some("scenery_index"));

            // Classify updated packages
            // Use sequential processing in debug log mode for ordered logs, parallel otherwise
            let updated_packages: Vec<SceneryPackageInfo> = if logger::is_debug_enabled() {
                // Sequential processing for ordered debug logs
                packages_to_update
                    .iter()
                    .filter_map(|folder| {
                        classify_scenery(folder, &self.xplane_path).ok()
                    })
                    .collect()
            } else {
                // Parallel processing for better performance when not in debug mode
                packages_to_update
                    .par_iter()
                    .filter_map(|folder| {
                        classify_scenery(folder, &self.xplane_path).ok()
                    })
                    .collect()
            };

            for info in updated_packages {
                index.packages.insert(info.folder_name.clone(), info);
            }
        }

        index.last_updated = SystemTime::now();
        self.save_index(&index)?;

        Ok(index)
    }

    /// Clean up stale entries (deleted packages)
    pub fn cleanup_stale_entries(&self) -> Result<usize> {
        let custom_scenery_path = self.xplane_path.join("Custom Scenery");
        let mut index = self.load_index()?;
        let initial_count = index.packages.len();

        // Get current folders (including symlinks)
        let current_folders: std::collections::HashSet<String> = fs::read_dir(&custom_scenery_path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                // Use metadata() to follow symlinks
                e.path().metadata().map(|m| m.is_dir()).unwrap_or(false)
            })
            .filter_map(|e| e.file_name().to_str().map(String::from))
            .collect();

        // Remove entries not in current folders
        index.packages.retain(|name, _| current_folders.contains(name));

        let removed_count = initial_count - index.packages.len();
        if removed_count > 0 {
            index.last_updated = SystemTime::now();
            self.save_index(&index)?;
            logger::log_info(&format!("Cleaned up {} stale index entries", removed_count), Some("scenery_index"));
        }

        Ok(removed_count)
    }

    /// Check if a package needs re-classification
    pub fn is_package_stale(&self, folder_name: &str, folder_path: &Path) -> Result<bool> {
        let index = self.load_index()?;

        if let Some(existing) = index.packages.get(folder_name) {
            if let Ok(metadata) = fs::metadata(folder_path) {
                if let Ok(modified) = metadata.modified() {
                    return Ok(modified > existing.indexed_at);
                }
            }
        }

        Ok(true) // Assume stale if we can't determine
    }

    /// Get package info from index
    pub fn get_package(&self, folder_name: &str) -> Result<Option<SceneryPackageInfo>> {
        let index = self.load_index()?;
        Ok(index.packages.get(folder_name).cloned())
    }

    /// Get or classify a package (uses cache if available and not stale)
    pub fn get_or_classify(&self, folder_path: &Path) -> Result<SceneryPackageInfo> {
        let folder_name = folder_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid folder name"))?;

        // Check if we have a valid cached entry
        if !self.is_package_stale(folder_name, folder_path)? {
            if let Some(info) = self.get_package(folder_name)? {
                return Ok(info);
            }
        }

        // Classify and update index
        let info = classify_scenery(folder_path, &self.xplane_path)?;
        self.update_package(info.clone())?;
        Ok(info)
    }

    /// Get index statistics
    pub fn get_stats(&self) -> Result<SceneryIndexStats> {
        let index = self.load_index()?;

        let mut by_category: HashMap<String, usize> = HashMap::new();
        for info in index.packages.values() {
            let category_name = format!("{:?}", info.category);
            *by_category.entry(category_name).or_insert(0) += 1;
        }

        Ok(SceneryIndexStats {
            total_packages: index.packages.len(),
            by_category,
            last_updated: index.last_updated,
        })
    }

    /// Create an empty index
    fn create_empty_index(&self) -> SceneryIndex {
        SceneryIndex {
            version: INDEX_VERSION,
            packages: HashMap::new(),
            last_updated: SystemTime::now(),
        }
    }
}

/// Get the path to the scenery index file
fn get_index_file_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
            return PathBuf::from(local_app_data)
                .join("XFastInstall")
                .join("scenery_index.json");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("XFastInstall")
                .join("scenery_index.json");
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home)
                .join(".config")
                .join("xfastinstall")
                .join("scenery_index.json");
        }
    }

    // Fallback to current directory
    PathBuf::from("scenery_index.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_file_path() {
        let path = get_index_file_path();
        assert!(path.to_string_lossy().contains("scenery_index.json"));
    }

    #[test]
    fn test_empty_index_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = SceneryIndexManager::new(temp_dir.path());
        let index = manager.create_empty_index();

        assert_eq!(index.version, INDEX_VERSION);
        assert!(index.packages.is_empty());
    }
}
