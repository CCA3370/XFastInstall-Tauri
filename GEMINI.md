# XFast-Manager-Tauri

## Project Overview

**XFast Manager** is a modern, cross-platform installer for X-Plane addons, built with **Tauri 2**, **Rust**, and **Vue 3**. It features a drag-and-drop interface, intelligent addon detection (Aircraft, Scenery, Plugins, Navdata), archive support (.zip, .7z), and Windows context menu integration.

### Tech Stack

*   **Frontend:** Vue 3 (Composition API), TypeScript, Vite, TailwindCSS v4, Pinia (State Management), Vue Router, Vue I18n.
*   **Backend:** Rust, Tauri 2.
*   **Build System:** Vite (Frontend), Cargo (Backend).

## Architecture

The application follows a standard Tauri architecture with a clear separation between the frontend UI and the backend logic.

*   **Frontend (`src/`):** Handles user interaction, file selection (drag & drop), and displays installation progress. It communicates with the backend via Tauri's IPC mechanism (`invoke`).
*   **Backend (`src-tauri/`):**
    *   **`lib.rs`:** Defines the exposed Tauri commands (`analyze_addons`, `install_addons`, `register_context_menu`).
    *   **`scanner.rs`:** Logic for scanning directories and archives to identify addon types based on marker files (e.g., `.acf` for Aircraft, `library.txt` for Scenery).
    *   **`analyzer.rs`:** Processes scanned results to deduplicate nested addons and generate `InstallTask`s.
    *   **`installer.rs`:** Executes the installation, handling file copying and archive extraction.
    *   **`registry.rs`:** Manages Windows registry keys for context menu integration.

## Key Directories and Files

*   **`src/`**
    *   `main.ts`: Application entry point, setup for Vue, Pinia, Router, and i18n.
    *   `App.vue`: Root Vue component.
    *   `views/`: Page components (`Home.vue`, `Settings.vue`).
    *   `stores/`: Pinia stores (`app.ts`, `modal.ts`, etc.).
    *   `types/index.ts`: Shared TypeScript interfaces mirroring Rust structs (e.g., `AddonType`, `InstallTask`).
    *   `i18n/`: Localization files (`en.ts`, `zh.ts`).
*   **`src-tauri/`**
    *   `src/lib.rs`: Main entry point for Tauri commands.
    *   `tauri.conf.json`: Tauri configuration (window settings, bundle info, permissions).
    *   `Cargo.toml`: Rust dependencies (includes `tauri`, `walkdir`, `zip`, `sevenz-rust`, `winreg`).
*   **Documentation**
    *   `DEVELOPER_GUIDE.md`: Detailed documentation on detection logic and architecture.
    *   `USER_GUIDE.md`: End-user instructions.

## Building and Running

### Prerequisites
*   Node.js 20+
*   Rust (latest stable)
*   **Linux:** `libwebkit2gtk-4.1-dev`, `build-essential`, `libssl-dev`
*   **Windows:** Visual Studio Build Tools

### Commands

*   **Install Dependencies:**
    ```bash
    npm install
    ```
*   **Run in Development Mode:**
    ```bash
    npm run tauri:dev
    ```
*   **Build for Production:**
    ```bash
    npm run tauri:build
    ```
*   **Generate Icon:**
    ```bash
    npm run generate:icon
    ```

## Development Conventions

*   **Styling:** TailwindCSS v4 is used for styling.
*   **State Management:** Pinia is used for managing global state.
*   **Internationalization:** Vue I18n is used for multi-language support.
*   **Type Safety:** TypeScript interfaces in `src/types/index.ts` should be kept in sync with Rust structs in `src-tauri/src/models.rs`.
*   **Tauri Commands:** New backend functionality should be exposed via commands in `src-tauri/src/lib.rs` and registered in the `tauri::Builder`.
