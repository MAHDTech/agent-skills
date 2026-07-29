+++
title = "v1-guides-getting-started-setup"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# Quick Start

Tauri is compatible with **almost every frontend stack**. Select yours
and get started!

`create-tauri-app`

The easiest way to scaffold a new project is the
[`create-tauri-app`](https://github.com/tauri-apps/create-tauri-app)
utility. It provides opinionated templates for vanilla
HTML/CSS/JavaScript and many frontend frameworks like React, Svelte, and
Yew.

- Bash
- PowerShell
- Cargo
- npm
- Yarn
- pnpm
- Bun

``` prism-code
sh <(curl https://create.tauri.app/sh) --tauri-version 1
```

``` prism-code
$Env:CTA_ARGS = "--tauri-version 1"; irm https://create.tauri.app/ps | iex
```

``` prism-code
cargo install create-tauri-app --locked
cargo create-tauri-app --tauri-version 1
```

``` prism-code
npm create tauri-app@latest -- --tauri-version 1
```

``` prism-code
yarn create tauri-app --tauri-version 1
```

``` prism-code
pnpm create tauri-app --tauri-version 1
```

``` prism-code
bunx create-tauri-app  --tauri-version 1
```

Note that you do not need to follow the below guides if you use
`create-tauri-app`, but we still recommend reading one (such as the
[HTML/CSS/JavaScript](https://tauri.app/v1/guides/getting-started/setup/html-css-js)
guide) to understand the setup.

If you're unfamiliar with web development or have no favorite frontend
stack you might find the
[HTML/CSS/JavaScript](https://tauri.app/v1/guides/getting-started/setup/html-css-js)
guide the most helpful. It guides you through getting started with the
most minimal frontend setup possible using either Node or Cargo.

[](https://tauri.app/v1/guides/getting-started/setup/html-css-js)

![](https://tauri.app/img/guides/getting-started/setup/html5-light.svg#gh-light-mode-only)![](https://tauri.app/img/guides/getting-started/setup/html5-dark.svg#gh-dark-mode-only)

## HTML, CSS, and JavaScript

Build a Tauri app using HTML, CSS, and JavaScript using either Node or
Cargo

[](https://tauri.app/v1/guides/getting-started/setup/next-js)

![](https://tauri.app/img/guides/getting-started/setup/nextjs-light.svg#gh-light-mode-only)![](https://tauri.app/img/guides/getting-started/setup/nextjs-dark.svg#gh-dark-mode-only)

## Next.js

Build a Tauri app using Next.js as the frontend build tool

[](https://tauri.app/v1/guides/getting-started/setup/qwik)

![](https://tauri.app/img/guides/getting-started/setup/qwik.svg)

## Qwik

Build a Tauri app using Qwik as the frontend build tool

[](https://tauri.app/v1/guides/getting-started/setup/sveltekit)

![](https://tauri.app/img/guides/getting-started/setup/svelte.svg)

## SvelteKit

Build a Tauri app using SvelteKit as the frontend build tool

[](https://tauri.app/v1/guides/getting-started/setup/vite)

![](https://tauri.app/img/guides/getting-started/setup/vite.svg)

## Vite

Build a Tauri app using Vite as the frontend build tool

[](https://tauri.app/v1/guides/getting-started/setup/integrate)

![](https://tauri.app/img/guides/getting-started/setup/integrate-light.svg#gh-light-mode-only)![](https://tauri.app/img/guides/getting-started/setup/integrate-dark.svg#gh-dark-mode-only)

## Integrate into Existing Project

If you already have an existing web project you can integrate Tauri into
it

Missing your favorite framework?

If you miss your favorite frontend framework or build tool, we're always
looking for Getting Started guides! Read our
[contributing](https://github.com/tauri-apps/tauri/blob/dev/.github/CONTRIBUTING.md)
guidelines and help us out!

{% endraw %}
