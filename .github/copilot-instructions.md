<!-- Short, focused instructions for coding agents working on XFastInstall-Tauri -->
# XFastInstall - Copilot instructions

Purpose: Help an AI coding assistant quickly become productive in this repository (Vue 3 + TypeScript frontend, Rust/Tauri backend).

- Quick summary
  - Frontend: `src/` — Vue 3 + TypeScript + Pinia. Main views: `src/views/Home.vue`, `src/views/Settings.vue`.
  - Backend: `src-tauri/src/` — Rust modules: `scanner.rs`, `analyzer.rs`, `installer.rs`, `registry.rs`, `models.rs`, and `lib.rs` (Tauri command wiring).
  - Types: shared shapes in `src/types/index.ts` mirror Rust models (keep them in sync when adding fields/types).

- High-level data flow and important APIs
  - User drops files in `Home.vue` → frontend calls `invoke('analyze_addons', { paths, xplanePath })` (see `Home.vue`).
  - `analyze_addons` (Rust, `lib.rs`) → `Analyzer` (in `analyzer.rs`) returns an `AnalysisResult` (map to `src/types/index.ts`).
  - Frontend shows `InstallTask`s and on user confirm calls `invoke('install_addons', { tasks })` → `Installer` (`installer.rs`) performs extraction/copy.
  - Windows context menu commands are registered via `register_context_menu` / `unregister_context_menu` (see `registry.rs`, only active on Windows and invoked from `Settings.vue`).

- Project-specific conventions & patterns (do not invent these; follow existing code)
  - Addon detection patterns live in `scanner.rs`. Examples: `Aircraft` = presence of `*.acf` (install parent), `Scenery` = `library.txt` or `*.dsf` (go up 1–2 levels), `Plugin` = `*.xpl` with platform dir check, `Navdata` = `cycle.json` parsing. Follow these exact heuristics when extending detection.
  - Deduplication logic is centralized in `analyzer.rs` — if a detected addon is a subdir of another, prefer the parent.
  - Types are mirrored: when adding a new addon type or task field, update `src-tauri/src/models.rs` and `src/types/index.ts` together.
  - CLI args: `lib.rs` collects args on startup and emits a `cli-args` event to the frontend — useful for implementing drag-to-app or context menu behavior.

- Dev workflow & useful commands
  - Install deps: `npm install`
  - Dev: `npm run tauri:dev` (starts Vite + Tauri dev server)
  - Build frontend: `npm run build`; Build bundles: `npm run tauri:build` (or `npm run tauri:build --silent` in CI)
  - Rust checks: `cd src-tauri && cargo check`; tests: `cd src-tauri && cargo test`.
  - Icon generation: `npm run generate:icon` (creates `src-tauri/icons/icon.ico` from PNG)

- CI notes
  - Workflow: `.github/workflows/build.yml` builds Linux, Windows and macOS bundles; it caches `node_modules` and Cargo registry/target.
  - Linux CI installs `libwebkit2gtk-4.1-dev` and other native deps before building — mimic that locally if encountering build errors on Linux.

- Where to add tests and how to validate changes
  - Add unit tests for parsing/detection/deduplication inside `src-tauri/src/` (use Rust tests near `scanner.rs` / `analyzer.rs`).
  - For frontend sanity, the project has no unit tests; use `npm run build` to validate the Vite build quickly.

- Small gotchas & implementation hints
  - Windows-only code is guarded with `#[cfg(target_os = "windows")]` — include those guards when adding OS-specific logic.
  - Archive support currently includes ZIP and 7z; see `installer.rs` and `Cargo.toml` if adding new archive formats (e.g., RAR) — add crates and tests.
  - Keep public Tauri commands stable: add new commands to `lib.rs` and include them in `tauri::generate_handler![ ... ]` and then call from frontend with `invoke`.

If anything here is unclear or you want more detail (examples, tests, or a short onboarding task), tell me which area to expand and I will add it.
