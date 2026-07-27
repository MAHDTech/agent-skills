+++
title = "tauri"
description = "Expert reference and development guidelines for Tauri v2+ cross-platform desktop and mobile apps. Use when the user mentions Tauri, src-tauri, tauri v2, tauri.conf.json, or capabilities.json, or asks to build, modify, or debug Tauri application settings, IPC commands, or capabilities."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Tauri Development

Guidelines and reference for developing Tauri v2+ cross-platform applications. All Tauri v2+ modifications must follow secure-by-default capabilities configurations and correct multi-platform project separation patterns to ensure successful deployment and high-performance desktop and mobile execution.

## Reference Documentation

When deep context, precise API definitions, or config schemas are required, consult the bundled Tauri reference excerpts:

- Start from [concept.md](resources/auto/concept.md), a short index of core-concept topics that links into the granular reference pages vendored under `resources/auto/` from the official v2 docs.

## Constraints

To ensure compatibility, security, and stability:

- **Frontend APIs**: Use `@tauri-apps/api/core` or official v2 plugins exclusively.
- **Async Commands**: Use owned types (such as `String`) for parameters and return values in async Tauri commands.
- **Thread Safety**: Run heavy or I/O operations asynchronously to ensure the main thread remains unblocked.
- **Paths**: Reference dynamic path configurations using Tauri's path APIs (e.g., `app.path()`) or paths relative to the working directory.
- **Capability Declarations**: Declare explicit permissions in `src-tauri/capabilities/default.json` (or other capability files) for all IPC and plugin operations.

## Project Architecture & Setup

1. **Clean passthrough entrypoint**: Keep `src-tauri/src/main.rs` as a thin passthrough:

   ```rust
   #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
   fn main() {
       app_lib::run();
   }
   ```

2. **Core application setup**: Place all setup, state management, and command registration in `src-tauri/src/lib.rs`.
3. **Mobile compatibility**: Configure a single `run` entry point marked with mobile compatibility attributes in `lib.rs`:

   ```rust
   #[cfg_attr(mobile, tauri::mobile_entry_point)]
   pub fn run() {
       tauri::Builder::default()
           .invoke_handler(tauri::generate_handler![/* commands */])
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

## IPC & Command Rules

1. Register every command intended for frontend access inside `tauri::generate_handler![...]`.
2. For commands returning potential errors, return a `Result<T, E>` where the error type `E` implements `serde::Serialize` to ensure error details are safely communicated across the IPC boundary.
3. Manage state shared between multiple commands via Tauri state APIs and wrap in thread-safe containers (e.g., `Mutex<T>` or `RwLock<T>`).
4. To establish stable communication vectors, use high-frequency typed streams or Channels for high-volume backend data transfers.

## Security & Capabilities

1. Define explicit permissions inside `src-tauri/capabilities/default.json` or another schema-compliant JSON file.
2. For every plugin used in the frontend (e.g. `fs`, `dialog`, `shell`, `http`, `store`), declare its core permissions within capability files (e.g. `"fs:default"`, `"dialog:default"`).
3. If an API silently fails or times out, immediately inspect the active capabilities configurations to confirm permissions have been correctly granted.

## Completion Criteria

The Tauri task is successfully complete when:

1. `src-tauri/src/main.rs` contains only a thin passthrough to `app_lib::run()`.
2. All custom backend commands are implemented with owned parameters and registered in `tauri::generate_handler![]`.
3. Every active plugin has corresponding permissions listed in the active capabilities configuration files under `src-tauri/capabilities/`.
4. All frontend components reference exclusively Tauri v2 APIs (e.g., `@tauri-apps/api/core` or official v2 plugins).
5. Compilation and build checks pass successfully (e.g. via `cargo check` or `npm run build`).

