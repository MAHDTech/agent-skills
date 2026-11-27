+++
title = "rig"
description = "Build Rust LLM applications, agents, and Bevy harness plugins with Rig. Use for rig, rig-core, rig-agent, providers, tools, RAG, memory, cassettes, MCP, or extending the rig coding agent."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Rig development

Build applications with the Rig Rust LLM framework (`rig`, `rig-core`, `rig-agent`, and companion crates) and operate or extend the `rig` terminal coding agent (`rig-cli`, `rig-harness`, `rig-ecs`, `rig-tools`).

Rig is pre-1.0 and breaks public APIs frequently across `0.x` releases. Never rely on memorized signatures or older blog snippets: always confirm the project's pinned version and read live docs from the matching release before writing code.

## 1. Verify the pinned version and live APIs first

Run these checks in order before implementing or upgrading Rig code:

1. **Read the pinned version** from the workspace's `Cargo.lock` or `Cargo.toml` (`rig`, `rig-core`, `rig-agent`, `rig-harness`, `rig-ecs`). For coding-agent plugins, check `RIG_HOME/project/Cargo.toml` (default `RIG_HOME` is `~/.rig`), which records either a crates.io version or a local checkout path.

2. **Query exact Rust signatures** for that version:

   ```bash
   # In an existing Cargo project, build docs only for the Rig crates you touch:
   cargo doc -p rig -p rig-core -p rig-agent --no-deps

   # Search the generated item index (hyphens become underscores in target/doc/):
   grep -i "AgentBuilder" target/doc/rig_agent/all.html

   # Before adding a dependency, inspect docs.rs for latest or a pinned <ver>:
   curl -sL "https://docs.rs/rig/latest/rig/all.html" | grep -i "AgentBuilder"
   ```

   - Feature list for the root facade: `https://docs.rs/crate/rig/latest/features`
   - Machine-readable rustdoc JSON (zstd-compressed): `https://docs.rs/crate/rig/latest/json` (replace `latest` with `<ver>` for a pinned release)

3. **Fetch version-tagged Markdown from `0xPlaygrounds/rig`** when you need architecture guides, provider options, or breaking-change diffs. Substitute `<ref>` with `v<version>` (for a pinned release) or `main` (for HEAD) in `https://raw.githubusercontent.com/0xPlaygrounds/rig/<ref>/<path>`:

   | Topic                                       | Repository path (`<path>`)         |
   | ------------------------------------------- | ---------------------------------- |
   | Facade overview, quickstart, crate map      | `README.md`                        |
   | Release-by-release API migration diffs      | `MIGRATING.md`                     |
   | Core contracts, `Catalog`, providers, WASM  | `crates/rig-core/README.md`        |
   | Provider-specific typed request options     | `crates/rig-core/TYPED_OPTIONS.md` |
   | Agent runtime, `AgentRun`, routing, hooks   | `crates/rig-agent/README.md`       |
   | Persisted sessions and compaction           | `crates/rig-memory/README.md`      |
   | Effect logs, HTTP replay/record testing     | `crates/rig-cassette/README.md`    |
   | MCP client tools and dynamic resources      | `crates/rig-rmcp/README.md`        |
   | Coding agent CLI usage and config           | `docs/agent.md`                    |
   | Bevy plugin authoring guide                 | `crates/rig-harness/PLUGINS.md`    |
   | ECS kernel (`AgentPass`, stages, entities)  | `crates/rig-ecs/README.md`         |
   | Built-in coding tools and remote workspaces | `crates/rig-tools/README.md`       |

   Working code examples live under `examples/` (library) and `corpus/` (CI-tested harness plugins) at the same tag. Conceptual guides live at `https://rig.rs/docs`, and real-world community projects and templates are cataloged at `https://github.com/0xPlaygrounds/awesome-rig` (verify their `Cargo.toml` Rig version before copying patterns, as community projects may trail `main`). Treat `docs.rs` and the release tag as the source of truth when guides or examples differ.

## 2. Crate and feature architecture

Rig splits into a root facade, a portable LLM library stack, opt-in integration crates, and a Bevy-based coding agent harness:

- **Root facade (`rig`)**: re-exports `rig-core`. Default features enable `agent` (`rig-agent`), `derive` (`rig-derive`), `reqwest` + `rustls` (`rig-reqwest`), and on native targets `cli` (`rig-cli`). Disable defaults (`default-features = false`) for minimal, custom-transport, or WASM builds. Companion crates map to facade features named without the `rig-` prefix (for example `cargo add rig --features memory,fastembed,qdrant`).
- **Core contracts (`rig-core`)**: messages, models, tools, 20+ built-in providers, `conversation` (stable message IDs, branches, change log), `Catalog` (`vendor/model` selection with `models.dev` and user overrides), `Model::check` dry-run option checks, OpenTelemetry GenAI conventions, and `InMemoryVectorStore`.
- **Classic agent runtime (`rig-agent`)**: `AgentBuilder`, the sans-I/O serializable `AgentRun` state machine (`next_step()` yielding `CallModel`, `CallTools`, or `Done`), `Handlers` dispatch table, `AgentHook::on_model_select` runtime routing, `AgentBuilder::memory`, and `AgentBuilder::record_to`.
- **State, testing, and MCP**:
  - `rig-memory`: JSON-lines conversation logs (`rig-session/4`), compaction, turn-boundary forking, and `AsyncStore`. Pure data (WASM-safe) by default; `fs` adds `DirStore`; SQL stores live in `rig-sqlite` (`SqliteSessionStore`) and `rig-postgres` (`PostgresSessionStore`) behind their `memory` feature.
  - `rig-cassette`: `effect_log` (always enabled), classic-agent replay adapters (`agent`), JSONL log files (`jsonl`), scrubbed traffic recording (`traffic`), and native HTTP replay/record fixtures (`http`, `bedrock`).
  - `rig-rmcp`: native-only MCP client (`client_config`, `McpConnection`, `McpTool`, `McpClientHandler` synced to `ToolServerHandle`, and `resource_context` for live MCP resources).
- **Models and vector stores**: `rig-bedrock`, `rig-vertexai`, `rig-gemini-grpc`, `rig-candle`, `rig-fastembed`, `rig-typesafeai`, and vector-store crates (`rig-lancedb`, `rig-qdrant`, `rig-postgres`, `rig-sqlite`, `rig-mongodb`, `rig-neo4j`, `rig-milvus`, `rig-surrealdb`, `rig-helixdb`, `rig-scylladb`, `rig-s3vectors`, `rig-vectorize`).
- **Coding agent and Bevy harness**:
  - `rig-cli`: the `rig` launcher binary that generates and builds a Cargo project per `RIG_HOME` from the enabled plugin set.
  - `rig-ecs`: Bevy ECS kernel (no features, WASM-safe). Models agents, turns (`TurnOf`), tool calls (`CallOf`), and effects stepped each frame through `AgentPass` stages: `Arrive`, `Review`, `Commit`, `Admit`, `Prepare`, `Gate`, `Start`.
  - `rig-harness`: adds sessions, settings, slash commands, and fronts on `rig-ecs`; stock plugins live in `rig-basics` and `rig-tui`, with optional plugins such as `rig-mcp-client`, `rig-acp`, `rig-inspect`, `rig-traffic`, `rig-fold`, `rig-steel`, `rig-bridge`, and `rig-watch`.
  - `rig-tools`: native coding tools (`read`, `write`, `edit`, `search`, `list`, `shell`, `delete`) over `Workspace` (`Local`, `Remote`, and `Hosts` supporting `HOST:PATH`).

## 3. Library implementation workflow

1. **Pick the right dependency layer**:
   - Standard apps: depend on `rig` (add `tokio` with `macros,rt-multi-thread` and opt into only the companion features you need).
   - Low-level completion/embedding callers without the agent loop: depend on `rig-core` (with `--features reqwest` for the default client, or pass a custom transport to `<Provider>Config::from_env()?.connect(http)`).
   - Custom schedulers or sans-I/O hosts (ECS, job queues): depend on `rig-agent` with `default-features = false` and drive `rig_agent::run::AgentRun` directly.
2. **Validate provider options and routing**:
   - Use `Model::check` to catch options a selected model drops silently, and consult `crates/rig-core/TYPED_OPTIONS.md` for provider-specific option structs.
   - For multi-model routing, register models via `AgentBuilder::named_model` / `model_route`, choose per-run candidates with `using_model`, or route per call boundary with `AgentHook::on_model_select`.
3. **Test deterministically with `rig-cassette`**:
   - Attach an `EffectLogRecorder` via `AgentBuilder::record_to` (use `keeping_stream_events()` when stream chunk boundaries matter) and stamp logs with `AgentReplayExt::stamp`.
   - For HTTP provider fixtures, `RIG_PROVIDER_TEST_MODE` defaults to `replay` (`record` refreshes scrubbed fixtures), and `RIG_CASSETTE_SNAPSHOTS=check|write` validates or updates request-body snapshots. Always call `finish()` (or `try_finish()` outside tests) so unplayed interactions fail instead of passing silently.

## 4. Coding agent CLI and Bevy plugin workflow

1. **Understand home vs project scope**:
   - `RIG_HOME` (default `~/.rig`) owns `settings.toml` (`[plugins]`, `[model]`), `auth/`, `sessions/`, `target/`, `AGENTS.md`, `skills/`, and `mcp.json`.
   - A repository provides `AGENTS.md`, `.rig/skills`, `.agents/skills`, and `.rig/mcp.json`, never compiled plugins. Give a project a separate `RIG_HOME` outside the repo (for example via `.envrc`) when it needs a distinct plugin set.
2. **Scaffold and verify plugins**:

   ```bash
   rig plugin new my-plugin     # creates RIG_HOME/plugins/my-plugin and registers it
   rig plugin list              # lists crates, areas, and enabled/disabled plugins
   rig plugin disable NAME      # or `rig plugin enable NAME`
   rig plugin check --build     # builds the agent and validates system ordering and settings
   ```

   Run `/reload` inside an interactive session to rebuild and restart with the same session preserved.

3. **Follow harness plugin invariants**:
   - Implement `Plugin + Default` against `rig-harness` (`use rig_harness::prelude::*;`).
   - Load required peer plugins in `build` with `rig_harness::load::<P>` so `/plugins` tracks ownership. Read plugin settings only in `build`, and gate optional integrations with `app.is_plugin_added::<Other>()` in `finish`.
   - Register systems in `AgentPass` under `Stage::Prepare`, `Stage::Gate`, or `Stage::Review`, exposing a public `SystemSet` whenever a system mutates shared stage data. Use `Decision::decide` in gates (first decision wins) and `HoldOf` entities to pause effects.
   - Register tools in `build` with `app.add_tool`, `app.add_tool_per_call` (workspace-aware tools), or `app.add_open_tool` (observer-answered tools).

## 5. Pitfalls and compatibility boundaries

- **`serde_json` features are app-owned**: `rig-core` does not force `preserve_order` or `float_roundtrip`. Add `serde_json = { version = "1", features = ["preserve_order", "float_roundtrip"] }` in the application's `Cargo.toml` if JSON key order or float bit-preservation is required.
- **WASM boundaries**: `rig-core`, minimal `rig-agent`, default `rig-memory`, and `rig-ecs` compile for `wasm32-unknown-unknown` (WASI is unsupported). Enabling `pdf` requires Web Crypto `Crypto.getRandomValues`. `rig-rmcp`, `rig-tools`, and `rig-cassette`'s `http`/`traffic` features are native-only.
- **Coding agent platform and MSRV**: the `rig` launcher runs on Linux and macOS (not Windows) and tracks a recent Rust MSRV; check `rust-version` in the release's root `Cargo.toml`.
- **Optimistic memory concurrency**: `rig-memory` checks log length and appends atomically; handle `SessionError::Changed` whenever concurrent runs can target the same conversation log.
- **Cross-version state and upgrades**: serialized `AgentRun` envelopes and `rig-memory` session logs require compatible Rig versions. Read the relevant section of `MIGRATING.md` before upgrading across `0.x` releases.

## 6. Completion criteria

- The target Rig version is confirmed from `Cargo.lock`, `Cargo.toml`, or `RIG_HOME/project/Cargo.toml`.
- Every Rig type, trait, feature flag, and builder method used in the change is verified against local `cargo doc`, `docs.rs`, or the matching `0xPlaygrounds/rig` release tag.
- `cargo check` and the relevant `cargo test` suite (or `rig plugin check --build` for coding-agent plugins) exit 0.

