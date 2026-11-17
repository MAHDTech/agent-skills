---
name: tailwind
description: Style components or build CSS with Tailwind CSS v4. Use for utility classes, CSS-first theme variables, imports, and CLI stylesheet compilation.
resources:
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/styling-with-utility-classes.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/hover-focus-and-other-states.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/responsive-design.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/dark-mode.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/theme.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/adding-custom-styles.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/detecting-classes-in-source-files.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/functions-and-directives.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/preflight.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/compatibility.mdx
  - https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/upgrade-guide.mdx
---

# Tailwind CSS v4

The vendored docs in `resources/auto/` are the Markdown (MDX) sources of the official Tailwind CSS docs, one file per page (for example `resources/auto/tailwindlabs-tailwindcss.com-main-src-docs-theme.mdx.md`). Search them for directives, theme variable namespaces, and variants before writing CSS. For a single utility's class list, fetch its page source from `https://raw.githubusercontent.com/tailwindlabs/tailwindcss.com/main/src/docs/<utility>.mdx` (for example `padding.mdx`).

## v4 rules

1. Configure in CSS. Import Tailwind and declare design tokens as theme variables in `@theme`; each variable creates matching utilities (`--color-primary` gives `bg-primary`, `text-primary`, and so on):

   ```css
   @import "tailwindcss";

   @theme {
     --color-primary: #1e3a8a;
     --font-sans: "Inter", sans-serif;
   }
   ```

2. Use `@import "tailwindcss";` in place of the v3 `@tailwind base; @tailwind components; @tailwind utilities;` directives.
3. A legacy JavaScript config still works when loaded explicitly with `@config "./tailwind.config.js";`; v4 does not detect it automatically. Prefer `@theme` for new tokens.
4. Content detection is automatic and treats source files as plain text, skipping `.gitignore`d files. Write complete class names (map a prop to `"bg-red-600"`, never build `bg-${color}-600`), and register extra paths with `@source "../path";`.

## Build commands

The v4 CLI ships in the `@tailwindcss/cli` package:

```bash
npx @tailwindcss/cli -i ./src/input.css -o ./dist/output.css            # build once
npx @tailwindcss/cli -i ./src/input.css -o ./dist/output.css --watch    # rebuild on change
npx @tailwindcss/cli -i ./src/input.css -o ./dist/output.css --minify   # minified output
```

Use the project's existing entry point instead (a `tailwindcss` binary on `PATH`, a package script, or a Vite or PostCSS plugin) when it has one.

## Completion criteria

- The build command exits 0 and writes the output file.
- Each class used by the change appears in the compiled CSS (search the output for it), including classes generated from new `@theme` variables.
