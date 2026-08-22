+++
title = "concept-inter-process-communication-brownfield-c890931b"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Brownfield Pattern

***This is the default pattern.***

This is the simplest and most straightforward pattern to use Tauri with,
because it tries to be as compatible as possible with existing frontend
projects. In short, it tries to require nothing additional to what an
existing web frontend might use inside a browser. Not ***everything***
that works in existing browser applications will work out-of-the-box.

If you are unfamiliar with Brownfield software development in general,
the [Brownfield Wikipedia
article](https://en.wikipedia.org/wiki/Brownfield_(software_development))
provides a nice summary. For Tauri, the existing software is current
browser support and behavior, instead of legacy systems.

## Configuration

Because the Brownfield pattern is the default pattern, it doesn’t
require a configuration option to be set. To explicitly set it, you can
use the `app > security > pattern` object in the `tauri.conf.json`
configuration file.

```
{  "app": {    "security": {      "pattern": {        "use": "brownfield"      }    }  }}
```

***There are no additional configuration options for the brownfield
pattern.***

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

