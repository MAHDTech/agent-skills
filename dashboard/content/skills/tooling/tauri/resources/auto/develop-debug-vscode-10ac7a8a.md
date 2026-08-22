+++
title = "develop-debug-vscode-10ac7a8a"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Debug in VS Code

This guide will walk you through setting up VS Code for debugging the
[Core Process of your Tauri
app](https://v2.tauri.app/concept/process-model/#the-core-process).

## All platforms with vscode-lldb extension

### Prerequisites

Install the
[`vscode-lldb`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
extension.

### Configure launch.json

Create a `.vscode/launch.json` file and paste the below JSON contents
into it:

```
{  // Use IntelliSense to learn about possible attributes.  // Hover to view descriptions of existing attributes.  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387  "version": "0.2.0",  "configurations": [    {      "type": "lldb",      "request": "launch",      "name": "Tauri Development Debug",      "cargo": {        "args": [          "build",          "--manifest-path=./src-tauri/Cargo.toml",          "--no-default-features"        ]      },      // task for the `beforeDevCommand` if used, must be configured in `.vscode/tasks.json`      "preLaunchTask": "ui:dev"    },    {      "type": "lldb",      "request": "launch",      "name": "Tauri Production Debug",      "cargo": {        "args": ["build", "--release", "--manifest-path=./src-tauri/Cargo.toml"]      },      // task for the `beforeBuildCommand` if used, must be configured in `.vscode/tasks.json`      "preLaunchTask": "ui:build"    }  ]}
```

.vscode/launch.json

This uses `cargo` directly to build the Rust application and load it in
both development and production modes.

Note that it does not use the Tauri CLI, so exclusive CLI features are
not executed. The `beforeDevCommand` and `beforeBuildCommand` scripts
must be executed beforehand or configured as a task in the
`preLaunchTask` field. Below is an example `.vscode/tasks.json` file
that has two tasks, one for a `beforeDevCommand` that spawns a
development server and one for `beforeBuildCommand`:

```
{  // See https://go.microsoft.com/fwlink/?LinkId=733558  // for the documentation about the tasks.json format  "version": "2.0.0",  "tasks": [    {      "label": "ui:dev",      "type": "shell",      // `dev` keeps running in the background      // ideally you should also configure a `problemMatcher`      // see https://code.visualstudio.com/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson      "isBackground": true,      // change this to your `beforeDevCommand`:      "command": "yarn",      "args": ["dev"]    },    {      "label": "ui:build",      "type": "shell",      // change this to your `beforeBuildCommand`:      "command": "yarn",      "args": ["build"]    }  ]}
```

.vscode/tasks.json

Now you can set breakpoints in `src-tauri/src/main.rs` or any other Rust
file and start debugging by pressing `F5`.

## With Visual Studio Windows Debugger on Windows

Visual Studio Windows Debugger is a Windows-only debugger that is
generally faster than
[`vscode-lldb`](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
with better support for some Rust features such as enums.

### Prerequisites

Install the
[C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools)
extension and follow
<https://code.visualstudio.com/docs/cpp/config-msvc#_prerequisites> to
install Visual Studio Windows Debugger.

### Configure launch.json and tasks.json

```
{  // Use IntelliSense to learn about possible attributes.  // Hover to view descriptions of existing attributes.  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387  "version": "0.2.0",  "configurations": [    {      "name": "Launch App Debug",      "type": "cppvsdbg",      "request": "launch",      // change the exe name to your actual exe name      // (to debug release builds, change `target/debug` to `release/debug`)      "program": "${workspaceRoot}/src-tauri/target/debug/your-app-name-here.exe",      "cwd": "${workspaceRoot}",      "preLaunchTask": "ui:dev"    }  ]}
```

.vscode/launch.json

Note that it does not use the Tauri CLI, so exclusive CLI features are
not executed. The `tasks.json` is the same as with `lldb`, except you
need to add a config group and target your `preLaunchTask` from
`launch.json` to it if you want it to always compile before launching.

Here is an example of running a dev server (equivalent of
`beforeDevCommand`) and the compilation (`cargo build`) as a group, to
use it, change the `preLaunchTask` config in `launch.json` to `dev` (or
anything you named your group).

```
{  // See https://go.microsoft.com/fwlink/?LinkId=733558  // for the documentation about the tasks.json format  "version": "2.0.0",  "tasks": [    {      "label": "build:debug",      "type": "cargo",      "command": "build",      "options": {        "cwd": "${workspaceRoot}/src-tauri"      }    },    {      "label": "ui:dev",      "type": "shell",      // `dev` keeps running in the background      // ideally you should also configure a `problemMatcher`      // see https://code.visualstudio.com/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson      "isBackground": true,      // change this to your `beforeDevCommand`:      "command": "yarn",      "args": ["dev"]    },    {      "label": "dev",      "dependsOn": ["build:debug", "ui:dev"],      "group": {        "kind": "build"      }    }  ]}
```

.vscode/tasks.json

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

