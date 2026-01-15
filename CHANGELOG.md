# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- Initial features

## [0.1.0] - 2026-01-16

### Added

- Smart addon detection (Aircraft, Scenery, Plugins, Navdata, Library)
- Drag & drop interface for easy installation
- Archive support (.zip, .7z, .rar formats)
- Password-protected archive handling
- Intelligent duplicate detection and deduplication
- Multi-task batch installation with progress tracking
- Install modes: Clean Install and Direct Overwrite
- Aircraft livery and configuration backup during clean install
- File integrity verification (MD5, SHA-1, SHA-256, CRC32)
- Atomic installation mode for safer operations
- Windows context menu integration ("Install to X-Plane")
- Multi-language support (English, Chinese)
- Dark/Light theme support
- Comprehensive logging system
- Performance optimizations (parallel processing, memory-mapped I/O)

### Security
- Path traversal protection
- Compression bomb detection
- Size and compression ratio validation

[unreleased]: https://github.com/CCA3370/XFastInstall-Tauri/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/CCA3370/XFastInstall-Tauri/releases/tag/v0.1.0
