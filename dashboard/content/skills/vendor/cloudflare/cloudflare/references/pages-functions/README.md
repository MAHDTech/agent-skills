+++
title = "README"
[extra]
skill = false
category = "vendor/cloudflare"
mermaid = false
skill_name = "cloudflare"
+++

# Cloudflare Pages Functions

Use this reference for server-side behavior in an existing Pages project. For new applications, follow the Workers recommendation in the [Pages framework guidance](https://developers.cloudflare.com/pages/framework-guides/index.md).

| Task | Documentation |
| --- | --- |
| Identify filesystem routes and invocation boundaries | [Routing](https://developers.cloudflare.com/pages/functions/routing/index.md) |
| Implement request handlers | [API reference](https://developers.cloudflare.com/pages/functions/api-reference/index.md) |
| Understand generated Worker output | [Advanced mode](https://developers.cloudflare.com/pages/functions/advanced-mode/index.md) |

Inspect whether the project uses a Functions directory or framework-generated advanced mode before selecting a routing approach. Fetch current documentation for signatures, supported bindings, configuration, and examples.

## In This Reference

- [api.md](@/skills/vendor/cloudflare/cloudflare/references/pages-functions/api.md) — handlers, context, middleware, and assets
- [configuration.md](@/skills/vendor/cloudflare/cloudflare/references/pages-functions/configuration.md) — bindings, environments, types, and local development
- [patterns.md](@/skills/vendor/cloudflare/cloudflare/references/pages-functions/patterns.md) — request ownership and shared logic
- [gotchas.md](@/skills/vendor/cloudflare/cloudflare/references/pages-functions/gotchas.md) — route, binding, and runtime investigation

See [Pages](@/skills/vendor/cloudflare/cloudflare/references/pages/README.md) for builds and deployment decisions.

