+++
title = "release-tauri-v2.7.0-8b47082f"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# tauri@2.7.0

[← tauri releases](https://v2.tauri.app/release/tauri/) [View on
GitHub](https://github.com/tauri-apps/tauri/releases/tag/tauri-v2.7.0)

Jul 20, 2025

### New Features

- [`7bc77a038`](https://www.github.com/tauri-apps/tauri/commit/7bc77a038af062a02aabeaf9b228577447bad5e5)
  ([\#13609](https://www.github.com/tauri-apps/tauri/pull/13609) by
  [@Legend-Master](https://www.github.com/tauri-apps/tauri/../../Legend-Master))
  Added `tauri::plugin::Builder::js_init_script_on_all_frames` that
  allows plugins to add initialization scripts that runs on all frames

### Enhancements

- [`7f3c98911`](https://www.github.com/tauri-apps/tauri/commit/7f3c989111e007d7eeb5da118421214848e4bfcd)
  ([\#13837](https://www.github.com/tauri-apps/tauri/pull/13837) by
  [@WSH032](https://www.github.com/tauri-apps/tauri/../../WSH032)) Add
  `AppHandle::plugin_boxed` and `Builder::plugin_boxed` methods to allow
  adding plugins in the form of boxed trait objects.
- [`7bc77a038`](https://www.github.com/tauri-apps/tauri/commit/7bc77a038af062a02aabeaf9b228577447bad5e5)
  ([\#13609](https://www.github.com/tauri-apps/tauri/pull/13609) by
  [@Legend-Master](https://www.github.com/tauri-apps/tauri/../../Legend-Master))
  `tauri::plugin::Builder::js_init_script` now takes
  `impl Into&lt;String&gt;` instead of `String`

### Bug Fixes

- [`6a4451bcd`](https://www.github.com/tauri-apps/tauri/commit/6a4451bcd9cf5a2428857d2e47ea25e3d74712ae)
  ([\#13849](https://www.github.com/tauri-apps/tauri/pull/13849) by
  [@Legend-Master](https://www.github.com/tauri-apps/tauri/../../Legend-Master))
  Fix isolation pattern creates iframes within iframes on Windows
- [`4ba871c5d`](https://www.github.com/tauri-apps/tauri/commit/4ba871c5d2eb3fbb8db56c8d8f9916e65d3e34ac)
  ([\#13782](https://www.github.com/tauri-apps/tauri/pull/13782) by
  [@lucasfernog](https://www.github.com/tauri-apps/tauri/../../lucasfernog))
  Fixes loading external URLs in mobile development mode.
- [`1c5df96fe`](https://www.github.com/tauri-apps/tauri/commit/1c5df96fe8542e815cd887e66c29efb268add710)
  ([\#13773](https://www.github.com/tauri-apps/tauri/pull/13773) by
  [@tasgon](https://www.github.com/tauri-apps/tauri/../../tasgon))
  Forward request body on the mobile frontend proxy.

### Dependencies

- Upgraded to `tauri-runtime-wry@2.7.2`
- Upgraded to `tauri-utils@2.6.0`
- Upgraded to `tauri-runtime@2.7.1`
- Upgraded to `tauri-macros@2.3.2`
- Upgraded to `tauri-build@2.3.1`

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
