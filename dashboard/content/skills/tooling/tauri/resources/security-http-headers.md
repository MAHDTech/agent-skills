+++
title = "security-http-headers"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# HTTP Headers

[Since `2.1.0`](https://v2.tauri.app/release/tauri/v2.1.0)

A header defined in the configuration gets sent along the responses to
the webview. This doesn’t include IPC messages and error responses. To
be more specific, every response sent via the `get_response` function in
[crates/tauri/src/protocol/tauri.rs
↗](https://github.com/tauri-apps/tauri/blob/8e8312bb8201ccc609e4bbc1a990bdc314daa00f/crates/tauri/src/protocol/tauri.rs#L103)
will include those headers.

### Header Names

The header names are limited to:

- [Access-Control-Allow-Credentials
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Credentials)
- [Access-Control-Allow-Headers
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Headers)
- [Access-Control-Allow-Methods
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Allow-Methods)
- [Access-Control-Expose-Headers
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Expose-Headers)
- [Access-Control-Max-Age
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Access-Control-Max-Age)
- [Cross-Origin-Embedder-Policy
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy)
- [Cross-Origin-Opener-Policy
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Opener-Policy)
- [Cross-Origin-Resource-Policy
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Resource-Policy)
- [Permissions-Policy
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Permissions-Policy)
- [Service-Worker-Allowed
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Service-Worker-Allowed)
- [Timing-Allow-Origin
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Timing-Allow-Origin)
- [X-Content-Type-Options
  ↗](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Content-Type-Options)
- Tauri-Custom-Header

### How to Configure Headers

- with a string
- with an array of strings
- with an object/key-value, where the values must be strings
- with null

The header values are always converted to strings for the actual
response. Depending on how the configuration file looks, some header
values need to be composed. Those are the rules on how a composite gets
created:

- `string`: stays the same for the resulting header value
- `array`: items are joined by `, ` for the resulting header value
- `key-value`: items are composed from: key + space + value. Items are
  then joined by `; ` for the resulting header value
- `null`: header will be ignored

### Example

```
{ //...  "app":{    //...    "security": {      //...      "headers": {        "Cross-Origin-Opener-Policy": "same-origin",        "Cross-Origin-Embedder-Policy": "require-corp",        "Timing-Allow-Origin": [          "https://developer.mozilla.org",          "https://example.com",        ],        "X-Content-Type-Options": null, // gets ignored        "Access-Control-Expose-Headers": "Tauri-Custom-Header",        "Tauri-Custom-Header": {          "key1": "'value1' 'value2'",          "key2": "'value3'"        }      },      // notice how the CSP is not defined under headers      "csp": "default-src 'self'; connect-src ipc: http://ipc.localhost",    }  }}
```

src-tauri/tauri.conf.json

In this example `Cross-Origin-Opener-Policy` and
`Cross-Origin-Embedder-Policy` are set to allow for the use of
[`SharedArrayBuffer ↗`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer).
`Timing-Allow-Origin` grants scripts loaded from the listed websites to
access detailed network timing data via the [Resource Timing API
↗](https://developer.mozilla.org/en-US/docs/Web/API/Performance_API/Resource_timing).

For the helloworld example, this config results in:

```
access-control-allow-origin:  http://tauri.localhostaccess-control-expose-headers: Tauri-Custom-Headercontent-security-policy: default-src 'self'; connect-src ipc: http://ipc.localhost; script-src 'self' 'sha256-Wjjrs6qinmnr+tOry8x8PPwI77eGpUFR3EEGZktjJNs='content-type: text/htmlcross-origin-embedder-policy: require-corpcross-origin-opener-policy: same-origintauri-custom-header: key1 'value1' 'value2'; key2 'value3'timing-allow-origin: https://developer.mozilla.org, https://example.com
```

### Frameworks

Some development environments require extra settings, to emulate the
production environment.

#### JavaScript/TypeScript

For setups running the build tool **Vite** (those include **Qwik, React,
Solid, Svelte, and Vue**) add the wanted headers to `vite.config.ts`.

```
import { defineConfig } from 'vite';
export default defineConfig({  // ...  server: {      // ...      headers: {        'Cross-Origin-Opener-Policy': 'same-origin',        'Cross-Origin-Embedder-Policy': 'require-corp',        'Timing-Allow-Origin': 'https://developer.mozilla.org, https://example.com',        'Access-Control-Expose-Headers': 'Tauri-Custom-Header',        'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"      },    },})
```

vite.config.ts

Sometimes the `vite.config.ts` is integrated into the frameworks
configuration file, but the setup stays the same. In case of **Angular**
add them to `angular.json`.

```
{  //...  "projects":{    //...    "insert-project-name":{      //...      "architect":{        //...        "serve":{          //...          "options":{            //...            "headers":{              "Cross-Origin-Opener-Policy": "same-origin",              "Cross-Origin-Embedder-Policy": "require-corp",              "Timing-Allow-Origin": "https://developer.mozilla.org, https://example.com",              "Access-Control-Expose-Headers": "Tauri-Custom-Header",              "Tauri-Custom-Header": "key1 'value1' 'value2'; key2 'value3'"            }          }        }      }    }  }}
```

angular.json

And in case of **Nuxt** to `nuxt.config.ts`.

```
export default defineNuxtConfig({  //...  vite: {    //...    server: {      //...      headers:{        'Cross-Origin-Opener-Policy': 'same-origin',        'Cross-Origin-Embedder-Policy': 'require-corp',        'Timing-Allow-Origin': 'https://developer.mozilla.org, https://example.com',        'Access-Control-Expose-Headers': 'Tauri-Custom-Header',        'Tauri-Custom-Header': "key1 'value1' 'value2'; key2 'value3'"      }    },  },});
```

nuxt.config.ts

**Next.js** doesn’t rely on **Vite**, so the approach is different. Read
more about it [here
↗](https://nextjs.org/docs/pages/api-reference/next-config-js/headers).
The headers are defined in `next.config.js`.

```
module.exports = {  //...  async headers() {    return [      {        source: '/*',        headers: [          {            key: 'Cross-Origin-Opener-Policy',            value: 'same-origin',          },          {            key: 'Cross-Origin-Embedder-Policy',            value: 'require-corp',          },          {            key: 'Timing-Allow-Origin',            value: 'https://developer.mozilla.org, https://example.com',          },          {            key: 'Access-Control-Expose-Headers',            value: 'Tauri-Custom-Header',          },          {            key: 'Tauri-Custom-Header',            value: "key1 'value1' 'value2'; key2 'value3'",          },        ],      },    ]  },}
```

next.config.js

#### Rust

For **Yew** and **Leptos** add the headers to `Trunk.toml`

```
#...[serve]#...headers = {  "Cross-Origin-Opener-Policy" = "same-origin",  "Cross-Origin-Embedder-Policy" = "require-corp",  "Timing-Allow-Origin" = "https://developer.mozilla.org, https://example.com",  "Access-Control-Expose-Headers" = "Tauri-Custom-Header",  "Tauri-Custom-Header" = "key1 'value1' 'value2'; key2 'value3'"}
```

Trunk.toml

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

