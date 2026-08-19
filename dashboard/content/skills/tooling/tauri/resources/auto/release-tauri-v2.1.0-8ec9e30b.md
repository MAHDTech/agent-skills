+++
title = "release-tauri-v2.1.0-8ec9e30b"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# tauri@2.1.0

[← tauri releases](https://v2.tauri.app/release/tauri/) [View on
GitHub](https://github.com/tauri-apps/tauri/releases/tag/tauri-v2.1.0)

Nov 9, 2024

##### New Features

- [`fabc2f283`](https://www.github.com/tauri-apps/tauri/commit/fabc2f283e38b62c721326e44645d47138418cbc)
  ([\#11485](https://www.github.com/tauri-apps/tauri/pull/11485) by
  [@39zde](https://www.github.com/tauri-apps/tauri/../../39zde)) Adds a
  new configuration option `app > security > headers` to define headers
  that will be added to every http response from tauri to the web view.
  This doesn't include IPC messages and error responses.

- [`8036c78e0`](https://www.github.com/tauri-apps/tauri/commit/8036c78e08715b1bc6b9fcb0c59a570eec98014f)
  ([\#11455](https://www.github.com/tauri-apps/tauri/pull/11455) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `PathResolver::home_dir()` method on Android.

- [`5c4b83084`](https://www.github.com/tauri-apps/tauri/commit/5c4b830843ab085f8ff9db9e08d832223b027e4e)
  ([\#11191](https://www.github.com/tauri-apps/tauri/pull/11191) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Improved support for `dpi` module types to allow these types to be
  used without manual conversions with `invoke`:

  - Added `SERIALIZE_TO_IPC_FN` const in `core` module which can be used
    to implement custom IPC serialization for types passed to `invoke`.
  - Added `Size` and `Position` classes in `dpi` module.
  - Implementd `SERIALIZE_TO_IPC_FN` method on `PhysicalSize`,
    `PhysicalPosition`, `LogicalSize` and `LogicalPosition` to convert
    it into a valid IPC-compatible value that can be deserialized
    correctly on the Rust side into its equivalent struct.

- [`4d545ab3c`](https://www.github.com/tauri-apps/tauri/commit/4d545ab3ca228c8a21b966b709f84a0da2864479)
  ([\#11486](https://www.github.com/tauri-apps/tauri/pull/11486) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Added `Window::set_background_color` and
  `WindowBuilder::background_color`.

- [`cbc095ec5`](https://www.github.com/tauri-apps/tauri/commit/cbc095ec5fe7de29b5c9265576d4e071ec159c1c)
  ([\#11451](https://www.github.com/tauri-apps/tauri/pull/11451) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `app > windows > devtools` config option and when creating the
  webview from JS, to enable or disable devtools for a specific webview.

- [`f0da0bde8`](https://www.github.com/tauri-apps/tauri/commit/f0da0bde87a80fdca20c588cefcad86e03b9627c)
  ([\#11439](https://www.github.com/tauri-apps/tauri/pull/11439) by
  [@lucasfernog](https://www.github.com/tauri-apps/tauri/../../lucasfernog))
  Added `WebviewWindow::resolve_command_scope` to check a command scope
  at runtime.

- [\`\`](https://www.github.com/tauri-apps/tauri/commit/undefined)
  Detect if `SERIALIZE_TO_IPC_FN`, const from the JS `core` module, is
  implemented on objects when serializing over IPC and use it.

- [`f37e97d41`](https://www.github.com/tauri-apps/tauri/commit/f37e97d410c4a219e99f97692da05ca9d8e0ba3a)
  ([\#11477](https://www.github.com/tauri-apps/tauri/pull/11477) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `app > windows > useHttpsScheme` config option to choose whether
  the custom protocols should use `https://\<scheme\>.localhost` instead
  of the default `http://\<scheme\>.localhost` on Windows and Android

- [`f37e97d41`](https://www.github.com/tauri-apps/tauri/commit/f37e97d410c4a219e99f97692da05ca9d8e0ba3a)
  ([\#11477](https://www.github.com/tauri-apps/tauri/pull/11477) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `WebviewWindowBuilder/WebviewBuilder::use_https_scheme` to choose
  whether the custom protocols should use `https://\<scheme\>.localhost`
  instead of the default `http://\<scheme\>.localhost` on Windows and
  Android

- [`cbc095ec5`](https://www.github.com/tauri-apps/tauri/commit/cbc095ec5fe7de29b5c9265576d4e071ec159c1c)
  ([\#11451](https://www.github.com/tauri-apps/tauri/pull/11451) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `WebviewWindowBuilder::devtools` and `WebviewBuilder::devtools` to
  enable or disable devtools for a specific webview.

- [`129414faa`](https://www.github.com/tauri-apps/tauri/commit/129414faa4e027c9035d56614682cacc0335a6a0)
  ([\#11569](https://www.github.com/tauri-apps/tauri/pull/11569) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Add `WebviewBuilder::focused` method to choose whether to focus
  webview or not on creation.

- [`2a75c64b5`](https://www.github.com/tauri-apps/tauri/commit/2a75c64b5431284e7340e8743d4ea56a62c75466)
  ([\#11469](https://www.github.com/tauri-apps/tauri/pull/11469) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Added `app > windows > windowClassname` config option to specify the
  name of the window class on Windows.

- [`2a75c64b5`](https://www.github.com/tauri-apps/tauri/commit/2a75c64b5431284e7340e8743d4ea56a62c75466)
  ([\#11469](https://www.github.com/tauri-apps/tauri/pull/11469) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Added `WindowBuilder/WebviewWindowBuilder::window_classname` method to
  specify the name of the window class on Windows.

##### Enhancements

- [`17c6952ae`](https://www.github.com/tauri-apps/tauri/commit/17c6952aec965fa41e6695ad68461a218afc20f1)
  ([\#11522](https://www.github.com/tauri-apps/tauri/pull/11522) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Enhance the error message when using `async` commands with a
  reference.
- [`c33bbf457`](https://www.github.com/tauri-apps/tauri/commit/c33bbf45740274b6918ea6c647f366fb6008e459)
  ([\#11575](https://www.github.com/tauri-apps/tauri/pull/11575) by
  [@kornelski](https://www.github.com/tauri-apps/tauri/../../kornelski))
  Include the path in ACL I/O errors.

##### Bug Fixes

- [`229d7f8e2`](https://www.github.com/tauri-apps/tauri/commit/229d7f8e220cc8d5ca06eff1ed85cb7d047c1d6c)
  ([\#11616](https://www.github.com/tauri-apps/tauri/pull/11616) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Fix regression in creating child webviews on macOS and Windows,
  covering the whole window.
- [`8c6d1e8e6`](https://www.github.com/tauri-apps/tauri/commit/8c6d1e8e6c852667bb223b5f4823948868c26d98)
  ([\#11401](https://www.github.com/tauri-apps/tauri/pull/11401) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Fix `App/AppHandle/Window/Webview/WebviewWindow::cursor_position`
  getter method failing on Linux with
  `GDK may only be used from the main thread`.
- [`f8994b214`](https://www.github.com/tauri-apps/tauri/commit/f8994b214e89acc99ab5ce8dcca8485f43a62dbb)
  ([\#11581](https://www.github.com/tauri-apps/tauri/pull/11581) by
  [@Mikkel-T](https://www.github.com/tauri-apps/tauri/../../Mikkel-T))
  Fix listeners created with `EventTarget::AnyLabel` never receiving
  events.
- [`4191a7a53`](https://www.github.com/tauri-apps/tauri/commit/4191a7a53d941b179780a550638f1b4a09d17fd1)
  ([\#11583](https://www.github.com/tauri-apps/tauri/pull/11583) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Fix tray events not fired for tray icons created inside an async
  command.
- [`129414faa`](https://www.github.com/tauri-apps/tauri/commit/129414faa4e027c9035d56614682cacc0335a6a0)
  ([\#11569](https://www.github.com/tauri-apps/tauri/pull/11569) by
  [@amrbashir](https://www.github.com/tauri-apps/tauri/../../amrbashir))
  Fix webview not focused by default.

##### Dependencies

- Upgraded to `tauri-utils@2.1.0`
- Upgraded to `tauri-runtime@2.2.0`
- Upgraded to `tauri-runtime-wry@2.2.0`
- Upgraded to `tauri-macros@2.0.3`
- Upgraded to `tauri-build@2.0.3`

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
