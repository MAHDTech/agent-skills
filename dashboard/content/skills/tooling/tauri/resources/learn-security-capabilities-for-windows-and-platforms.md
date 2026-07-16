+++
title = "learn-security-capabilities-for-windows-and-platforms"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Capabilities for Different Windows and Platforms

This guide will help you customize the capabilities of your Tauri app.

## Content of this guide

- Create multiple windows in a Tauri app
- Use different capabilities for different windows
- Use platform-specific capabilities

## Prerequisites

This exercise is meant to be read after completing
[`Using Plugin Permissions`](/learn/security/using-plugin-permissions/).

## Guide

1.  ### Create Multiple Windows in a Tauri Application

    Here we create an app with two windows labelled `first` and
    `second`. There are multiple ways to create windows in your Tauri
    application.

    #### Create Windows with the Tauri Configuration File

    In the Tauri configuration file, usually named `tauri.conf.json`:

    Show solution

    ```
      "productName": "multiwindow",  ...  "app": {    "windows": [      {        "label": "first",        "title": "First",        "width": 800,        "height": 600      },      {        "label": "second",        "title": "Second",        "width": 800,        "height": 600      }    ],  },  ...}
    ```

    #### Create Windows Programmatically

    In the Rust code to create a Tauri app:

    Show solution

    ```
    tauri::Builder::default()    .invoke_handler(tauri::generate_handler![greet])    .setup(|app| {        let webview_url = tauri::WebviewUrl::App("index.html".into());        // First window        tauri::WebviewWindowBuilder::new(app, "first", webview_url.clone())            .title("First")            .build()?;        // Second window        tauri::WebviewWindowBuilder::new(app, "second", webview_url)            .title("Second")            .build()?;        Ok(())    })    .run(context)    .expect("error while running tauri application");
    ```

2.  ### Apply Different Capabilities to Different Windows

    The windows of a Tauri app can use different features or plugins of
    the Tauri backend. For better security it is recommended to only
    give the necessary capabilities to each window. We simulate a
    scenario where the `first` windows uses filesystem and dialog
    functionalities and `second` only needs dialog functionalities.

    #### Separate capability files per category

    It is recommended to separate the capability files per category of
    actions they enable.

    Show solution

    JSON files in the `src-tauri/capabilities` will be taken into
    account for the capability system. Here we separate capabilities
    related to the filesystem and dialog window into `filesystem.json`
    and `dialog.json`.

    *filetree of the Tauri project:*

    ```
    /src/src-tauri  /capabilities    filesystem.json    dialog.json  tauri.conf.jsonpackage.jsonREADME.md
    ```

    #### Give filesystem capabilities to the `first` window

    We give the `first` window the capability to have read access to the
    content of the `$HOME` directory.

    Show solution

    Use the `windows` field in a capability file with one or multiple
    window labels.

    ```
    {  "identifier": "fs-read-home",  "description": "Allow access file access to home directory",  "local": true,  "windows": ["first"],  "permissions": [    "fs:allow-home-read",  ]}
    ```

    filesystem.json

    #### Give dialog capabilities to the `first` and `second` window

    We give to `first` and `second` windows the capability to create a
    “Yes/No” dialog

    Show solution

    Use the `windows` field in a capability file with one or multiple
    window labels.

    ```
    {  "identifier": "dialog",  "description": "Allow to open a dialog",  "local": true,  "windows": ["first", "second"],  "permissions": ["dialog:allow-ask"]}
    ```

    dialog.json

3.  ### Make Capabilities Platform Dependent

    We now want to customize the capabilities to be active only on
    certain platforms. We make our filesystem capabilities only active
    on `linux` and `windows`.

    Show solution

    Use the `platforms` field in a capability file to make it
    platform-specific.

    ```
    {  "identifier": "fs-read-home",  "description": "Allow access file access to home directory",  "local": true,  "windows": ["first"],  "permissions": [    "fs:allow-home-read",  ],  "platforms": ["linux", "windows"]}
    ```

    filesystem.json

    The currently available platforms are `linux`, `windows`, `macos`,
    `android`, and `ios`.

## Conclusion and Resources

We have learned how to create multiple windows in a Tauri app and give
them specific capabilities. Furthermore these capabilities can also be
targeted to certain platforms.

An example application that used window capabilities can be found in the
[`api`
example](https://github.com/tauri-apps/tauri/tree/dev/examples/api) of
the [Tauri Github repository](https://github.com/tauri-apps/tauri). The
fields that can be used in a capability file are listed in the
[Capability](/reference/acl/capability/) reference.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

