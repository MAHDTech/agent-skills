+++
title = "develop-tests"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Tests

Tauri offers support for both unit and integration testing utilizing a
mock runtime. Under the mock runtime, native webview libraries are not
executed. [See more about the mock runtime
here](https://v2.tauri.app/develop/tests/mocking/).

Tauri also provides support for end-to-end testing utilizing the
WebDriver protocol. [WebdriverIO Tauri
testing](https://webdriver.io/docs/desktop-testing/tauri) supports
Windows, Linux, and macOS; the WebDriver protocol can also be driven
directly on Windows and Linux, as macOS provides no desktop WebDriver
client. [See more about WebDriver support
here](https://v2.tauri.app/develop/tests/webdriver/).

We offer [tauri-action](https://github.com/tauri-apps/tauri-action) to
help run GitHub actions, but any sort of CI/CD runner can be used with
Tauri as long as each platform has the required libraries installed to
compile against.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

