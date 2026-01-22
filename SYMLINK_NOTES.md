# Symbolic Link and Shortcut Support

## ✅ Now Supports Both!

XFast Manager now supports **both** Windows shortcuts (.lnk files) and symbolic links!

### Windows Shortcuts (.lnk) - ✅ SUPPORTED

The easiest way to link scenery on Windows:

1. Right-click on the scenery folder you want to link
2. Select "Create shortcut"
3. Move the shortcut to `Custom Scenery` folder
4. XFast Manager will automatically resolve the shortcut and scan the target folder

**Example:**
- Original scenery: `D:\MyScenery\KSEA_Seattle`
- Create shortcut: Right-click → "Create shortcut"
- Move to: `F:\X-Plane 12\Custom Scenery\KSEA_Seattle.lnk`
- XFast Manager will automatically find and classify the scenery at `D:\MyScenery\KSEA_Seattle`

### Symbolic Links - ✅ ALSO SUPPORTED

For advanced users who prefer symbolic links:

1. Open Command Prompt **as Administrator**
2. Navigate to Custom Scenery folder:
   ```cmd
   cd "F:\SteamLibrary\steamapps\common\X-Plane 12\Custom Scenery"
   ```
3. Create symbolic link:
   ```cmd
   mklink /D "LinkName" "C:\Path\To\Actual\Scenery"
   ```

**Example:**
```cmd
mklink /D "KSEA_Airport" "D:\MyScenery\KSEA_Seattle"
```

## Which Should I Use?

### Use Windows Shortcuts (.lnk) if:
- ✅ You want the easiest method (no admin rights needed)
- ✅ You're not familiar with command line
- ✅ You want to quickly link a few scenery packages

### Use Symbolic Links if:
- ✅ You prefer command-line tools
- ✅ You want the link to appear as a real directory in File Explorer
- ✅ You're managing many links with scripts

## Verification

After creating a shortcut or symlink, XFast Manager will:
- Automatically detect and resolve the link
- Log the resolution in info logs: `"Resolved shortcut XPME_Africa.lnk -> D:\Scenery\XPME_Africa"`
- Scan the target directory and classify the scenery
- Add it to scenery_packs.ini with proper sorting

## Troubleshooting

### Shortcuts Not Working?

1. **Check the log file** (`%LOCALAPPDATA%\XFast Manager\logs\xfastmanager.log`):
   - Look for "Resolved shortcut" messages
   - Look for "Failed to resolve shortcut" errors

2. **Verify the shortcut target**:
   - Right-click the .lnk file → Properties
   - Check the "Target" field points to a valid directory
   - Make sure the target directory still exists

3. **Common issues**:
   - Broken shortcut (target moved/deleted) → Recreate the shortcut
   - Shortcut points to a file instead of folder → Must point to a directory
   - Network path not accessible → Ensure network drive is connected

### Symbolic Links Not Working?

1. **Check if it's a real symlink**:
   - Open Command Prompt and run: `dir /AL "Custom Scenery"`
   - Symlinks will show `<SYMLINK>` or `<SYMLINKD>` in the listing

2. **Check debug logs**:
   - Enable debug mode in XFast Manager
   - Look for "This is a symlink pointing to:" messages
   - Verify the target path is correct

3. **Common issues**:
   - Insufficient permissions → Run Command Prompt as Administrator
   - Broken symlink (target doesn't exist) → Recreate with correct target path

## Technical Details

- **Shortcuts (.lnk)**: Resolved using the `lnk` crate, which parses the .lnk file format
- **Symbolic Links**: Followed automatically by Rust's `metadata()` function
- **Both types**: Fully supported on Windows, work seamlessly with scenery classification
