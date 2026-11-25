+++
title = "DEEPENING"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "codebase-design"
+++

# Deepening

How to deepen a cluster of shallow modules safely, given its dependencies. Assumes the vocabulary in [SKILL.md](@/skills/engineering/codebase-design/_index.md) - **module**, **interface**, **seam**, **adapter**.

## Dependency categories

When assessing a candidate for deepening, classify its dependencies. The category determines how the deepened module is tested across its seam.

### 1. In-process

Pure computation, in-memory state, no I/O. Always deepenable - merge the modules and test through the new interface directly. No adapter needed.

### 2. Local-substitutable

Dependencies that have local test stand-ins (PGLite for Postgres, in-memory filesystem). Deepenable if the stand-in exists. The deepened module is tested with the stand-in running in the test suite. The seam is internal; no port at the module's external interface.

### 3. Remote but owned (Ports & Adapters)

Your own services across a network boundary (microservices, internal APIs). Define a **port** (interface) at the seam. The deep module owns the logic; the transport is injected as an **adapter**. Tests use an in-memory adapter. Production uses an HTTP/gRPC/queue adapter.

Recommendation shape: _"Define a port at the seam, implement an HTTP adapter for production and an in-memory adapter for testing, so the logic sits in one deep module even though it is deployed across a network."_

### 4. True external (Mock)

Third-party services (payment, messaging, etc.) you do not control. The deepened module takes the external dependency as an injected port; tests provide a mock adapter.

## Seam discipline

Apply the seam principles in [SKILL.md](@/skills/engineering/codebase-design/_index.md#principles) when collapsing a cluster:

- Only introduce a port when the dependency category justifies at least two adapters (typically production + test); otherwise merge the modules directly instead of adding single-adapter indirection.
- Keep test-only internal seams private to the implementation rather than exposing them through the deepened module's external interface.

## Testing strategy: replace, don't layer

Follow the interface-as-test-surface principle from [SKILL.md](@/skills/engineering/codebase-design/_index.md#principles) as you deepen:

- Delete obsolete unit tests on collapsed shallow modules once tests at the deepened module's interface cover the same behavior.
- Assert on observable outcomes through the interface rather than internal state, so tests survive internal refactors.
- Write the new interface-level tests one at a time, red before green.

