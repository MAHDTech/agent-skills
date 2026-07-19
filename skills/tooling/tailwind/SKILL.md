---
name: tailwind
description: Use when writing Tailwind CSS utility classes, compiling Tailwind bundles, or configuring Tailwind CSS v4 CSS-first themes and variables.
resources:
  - https://r.jina.ai/https://tailwindcss.com/docs
---

# Tailwind CSS v4

Guidelines for compiling stylesheet bundles and styling components using Tailwind CSS (utility-first styling framework). Consult the local [Tailwind CSS v4 Docs](resources/auto/docs.md) for full reference.

## Tailwind CSS v4 Key Rules

Tailwind CSS v4 introduces a completely redesigned compilation engine and configuration syntax:

1. **CSS-First Configuration:** Tailwind v4 abandons `tailwind.config.js` in favor of standard CSS syntax. Configure custom theme variables (colors, fonts, screens) inside `@theme` directive blocks directly in your CSS:

   ```css
   @import "tailwindcss";

   @theme {
     --color-primary: #1e3a8a;
     --color-surface: #f3f4f6;
     --font-sans: "Inter", sans-serif;
   }
   ```

2. **Standard Imports:** Use `@import "tailwindcss";` at the top of your stylesheet instead of old directives like `@tailwind base; @tailwind components; @tailwind utilities;`.

## Build & Compilation Commands

Use the Tailwind v4 CLI to compile utility classes:

- **Build once:** `tailwindcss -i <input.css> -o <output.css>`
- **Watch mode:** `tailwindcss -i <input.css> -o <output.css> --watch`
- **Minify output:** `tailwindcss -i <input.css> -o <output.css> --minify`

## Completion Criteria

The styling task is complete when:

1. The output CSS stylesheet compiles successfully without CLI error logs.
2. Custom theme variables are correctly defined under `@theme` and reflect properly on elements.
3. Utility styles are purged correctly and yield a responsive, optimized output bundle.
