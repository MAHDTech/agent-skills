+++
title = "docs-guides-util-path-to-file-url"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "install-skills"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://bun.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Convert an absolute path to a file URL

Use `Bun.pathToFileURL()` to convert an absolute path to a `file://` URL.

```ts theme={"theme":{"light":"github-light","dark":"dracula"}}
Bun.pathToFileURL("/path/to/file.txt").href;
// => "file:///path/to/file.txt"
```

***

See [Utils](https://bun.com/runtime/utils).
