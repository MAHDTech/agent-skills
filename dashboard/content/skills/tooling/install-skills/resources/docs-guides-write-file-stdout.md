+++
title = "docs-guides-write-file-stdout"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "install-skills"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://bun.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Write to stdout

The `console.log` function writes to `stdout` and appends a line break to the printed data.

```ts theme={"theme":{"light":"github-light","dark":"dracula"}}
console.log("Lorem ipsum");
```

***

Bun also exposes `stdout` as a `BunFile` with the `Bun.stdout` property. Pass it as the destination to [`Bun.write()`](https://bun.com/runtime/file-io#writing-files-bun-write).

```ts theme={"theme":{"light":"github-light","dark":"dracula"}}
await Bun.write(Bun.stdout, "Lorem ipsum");
```

***

See [`Bun.write()`](https://bun.com/runtime/file-io#writing-files-bun-write).
