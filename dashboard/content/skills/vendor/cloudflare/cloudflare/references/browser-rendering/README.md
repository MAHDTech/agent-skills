+++
title = "README"
[extra]
skill = false
category = "vendor/cloudflare"
mermaid = false
skill_name = "cloudflare"
+++

# Browser Run (formerly Browser Rendering)

Use Browser Run for screenshots, PDFs, rendered content extraction, and browser automation. Read the relevant current documentation before implementing; use the [documentation index](https://developers.cloudflare.com/browser-run/llms.txt) to discover additional guides.

Choose the integration by the work and runtime:

- For a self-contained screenshot, PDF, or extraction, start with Quick Actions. They are available through REST and Workers bindings; check the chosen action's supported interface.
- For multi-step interactions or persistent state, use browser sessions. In Workers, use Cloudflare's Puppeteer or Playwright package; from external scripts or CI, use the CDP integration.
- When adapting existing automation, preserve its library where supported and check installed versions against the corresponding guide.

Read only the reference needed for the task:

| Task | Reference |
|------|-----------|
| Set up bindings, dependencies, or development | [configuration.md](@/skills/vendor/cloudflare/cloudflare/references/browser-rendering/configuration.md) |
| Select an endpoint or browser client API | [api.md](@/skills/vendor/cloudflare/cloudflare/references/browser-rendering/api.md) |
| Implement a workflow or manage reusable sessions | [patterns.md](@/skills/vendor/cloudflare/cloudflare/references/browser-rendering/patterns.md) |
| Diagnose failures or plan capacity and cost | [gotchas.md](@/skills/vendor/cloudflare/cloudflare/references/browser-rendering/gotchas.md) |

