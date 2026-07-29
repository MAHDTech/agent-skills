+++
title = "security-runtime-authority-0efcdc07"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Runtime Authority

The runtime authority is part of the Tauri Core. It holds all
permissions, capabilities and scopes at runtime to enforce which window
can access which command and passes scopes to commands.

Whenever a Tauri command is invoked from the webview the runtime
authority receives the invoke request, makes sure that the origin is
allowed to actually use the requested command, checks if the origin is
part of capabilities and if scopes are defined for the command and
applicable then they are injected into the invoke request, which is then
passed to the proper Tauri command.

If the origin is not allowed to call the command, the runtime authority
will deny the request and the Tauri command is never invoked.

![IPC Diagram](https://v2.tauri.app/_astro/runtime-authority.97JqQbdT_Z1M2l65.svg)

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
