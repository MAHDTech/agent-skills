---
name: model-context-protocol
description: Build or debug MCP servers with official SDKs against the stateless 2026-07-28 spec. Use for tool schemas, resources, prompts, server/discover, stdio or Streamable HTTP transports, and MCP Inspector testing.
resources:
  - https://modelcontextprotocol.io/llms.txt
---

<!-- cspell:ignore modelcontextprotocol MCPServer MRTR httpx traceparent -->

# Model Context Protocol server development

Target protocol revision **`2026-07-28`**. It is stateless: there is no `initialize` handshake and no session. Every request carries its own protocol version and client capabilities, and the server answers each request independently.

The vendored spec and docs in `resources/auto/` are the source of truth. Files prefixed `specification-2026-07-28-` and `docs-2026-07-28-` describe the current revision; files for `2024-11-05` through `2025-11-25` describe legacy revisions, and `-draft-` files describe unreleased work. Read a current file before answering from memory, because most pre-2026 MCP knowledge (handshake, sessions, GET streams, server-initiated requests) is now wrong.

## Lookup protocol

1. Start with `resources/auto/specification-2026-07-28-changelog.md` and `resources/auto/specification-2026-07-28-deprecated.md` whenever a question touches lifecycle, transports, notifications, or a feature you remember from an older revision.
2. Open the page for the area in question:

| Topic                                                                 | File in `resources/auto/`                                                                                                                                          |
| --------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Messages, `_meta` keys, error codes, statelessness, JSON Schema rules | `specification-2026-07-28-basic-index.md`                                                                                                                          |
| Version negotiation, extensions, legacy compatibility matrix          | `specification-2026-07-28-basic-versioning.md`                                                                                                                     |
| `server/discover`                                                     | `specification-2026-07-28-server-discover.md`                                                                                                                      |
| stdio framing, cancellation, shutdown, legacy probe                   | `specification-2026-07-28-basic-transports-stdio.md`                                                                                                               |
| Streamable HTTP headers, validation, status codes                     | `specification-2026-07-28-basic-transports-streamable-http.md`                                                                                                     |
| Multi round-trip requests (elicitation, sampling, roots)              | `specification-2026-07-28-basic-patterns-mrtr.md`                                                                                                                  |
| `subscriptions/listen` change notifications                           | `specification-2026-07-28-basic-patterns-subscriptions.md`                                                                                                         |
| Progress and cancellation                                             | `specification-2026-07-28-basic-patterns-progress.md`, `specification-2026-07-28-basic-patterns-cancellation.md`                                                   |
| Tools, results, errors, `x-mcp-header`, stateful handles              | `specification-2026-07-28-server-tools.md`                                                                                                                         |
| Resources, templates, URI schemes                                     | `specification-2026-07-28-server-resources.md`                                                                                                                     |
| Prompts, completion, pagination                                       | `specification-2026-07-28-server-prompts.md`, `specification-2026-07-28-server-utilities-completion.md`, `specification-2026-07-28-server-utilities-pagination.md` |
| `ttlMs` and `cacheScope`                                              | `specification-2026-07-28-server-utilities-caching.md`                                                                                                             |
| Elicitation                                                           | `specification-2026-07-28-client-elicitation.md`                                                                                                                   |
| Authorization (HTTP only)                                             | `specification-2026-07-28-basic-authorization-*.md`, `docs-2026-07-28-tutorials-security-authorization.md`                                                         |
| Exact type definitions                                                | `specification-2026-07-28-schema.md`                                                                                                                               |
| SDK quickstarts (TypeScript, Python, Java, Kotlin, C#)                | `docs-2026-07-28-develop-build-server.md`                                                                                                                          |
| Inspector usage                                                       | `docs-2026-07-28-tools-inspector*.md`                                                                                                                              |
| Extensions (Tasks, Apps, Skills)                                      | `extensions-*.md`, `seps-2663-tasks-extension.md`                                                                                                                  |

- Search with grep across `resources/auto/specification-2026-07-28-*` for a method name, `_meta` key, or error name rather than guessing. Use `resources/auto/llms.txt` to find a page not listed above.
- When a vendored page and an SDK disagree, trust the page for wire behavior and the SDK's own README or changelog for its API, and tell the user which one you followed.

## Protocol rules

- **Per-request metadata.** Every client request carries `_meta` with `io.modelcontextprotocol/protocolVersion` (required), `io.modelcontextprotocol/clientCapabilities` (required), and optionally `io.modelcontextprotocol/clientInfo` and `io.modelcontextprotocol/logLevel`. A request missing a required field is rejected with `-32602` (HTTP `400`).
- **Server identity.** Include `io.modelcontextprotocol/serverInfo` in every result's `_meta`. Treat `clientInfo` as display data only, never as a security or behavior switch.
- **Discovery.** Implement `server/discover`, returning `supportedVersions`, `capabilities`, optional `instructions`, and caching hints. Clients may call it first or skip it.
- **Version mismatch.** Answer an unsupported version with `UnsupportedProtocolVersionError` (`-32022`) whose `data.supported` lists the versions you serve.
- **Capabilities per request.** Read client capabilities from the request being handled. If a request needs one the client did not declare (for example elicitation), return `MissingRequiredClientCapabilityError` (`-32021`) listing `data.requiredCapabilities`.
- **Results.** Every result carries `resultType`: `"complete"` for final results, `"input_required"` for MRTR interim results.
- **Server-to-client input uses MRTR.** To elicit, sample, or list roots, return an `InputRequiredResult` with `inputRequests` (and an opaque `requestState` if you need to correlate). The client retries the original request with a new JSON-RPC `id` plus `inputResponses`. Servers never send JSON-RPC requests to the client.
- **Cross-call state uses handles.** Return an explicit, opaque, high-entropy handle from a creation tool and accept it as an argument on later calls. Authorize the caller against the handle on every call, state its lifetime in the tool description, and return a tool execution error for expired handles. Connection or process identity is never a session.
- **Change notifications.** List changes and resource updates go only to clients that opened `subscriptions/listen` with a matching filter. Send `notifications/subscriptions/acknowledged` first, and tag every notification with `io.modelcontextprotocol/subscriptionId`. Progress and log notifications stay on the response stream of the request they belong to.
- **Caching hints.** `server/discover`, `tools/list`, `prompts/list`, `resources/list`, `resources/templates/list`, and `resources/read` results include `ttlMs` (integer, `>= 0`) and `cacheScope` (`"public"` or `"private"`). Return `tools/list` in a deterministic order, and do not vary list results per connection.
- **Error codes.** Use `-32602` for unknown tools, malformed params, and resource not found (not `-32002`). Use `-32603` for internal errors. New application codes go outside `-32768` to `-32000`; `-32020` to `-32099` is reserved for the spec.
- **Observability.** Propagate OpenTelemetry context through the `traceparent`, `tracestate`, and `baggage` `_meta` keys.

## Server design rules

- **Tools.** Name tools with 1 to 128 ASCII letters, digits, `_`, `-`, or `.`. Write `inputSchema` as JSON Schema 2020-12; for no parameters use `{ "type": "object", "additionalProperties": false }`. Add `outputSchema` when you return `structuredContent`, and also return the serialized JSON as a text block.
- **Two error channels.** Report API failures, input validation failures, and business-rule failures as a tool result with `isError: true` and a message the model can act on. Reserve JSON-RPC errors for unknown tools, malformed requests, and server faults.
- **Inputs and outputs.** Validate every argument, resolve file paths against an allowlist, rate limit calls, bound output size, and keep secrets out of errors and content. Never auto-dereference a network `$ref` in a schema.
- **Resources.** Expose stable URIs (`https://`, `file://`, `git://`, or a custom scheme) and templates for parameterized data. Prefer resource links in tool results over embedding large payloads.
- **Annotations.** Use tool annotations (read-only, destructive, idempotent hints) to describe behavior, knowing clients treat them as untrusted unless the server is trusted.

## Transports

**stdio.** One newline-delimited JSON-RPC message per line on `stdout`, nothing else. Log to `stderr` only (`console.error` in Node, the `logging` module in Python). Cancellation arrives as `notifications/cancelled`. Exit promptly when `stdin` closes.

**Streamable HTTP.** One endpoint (for example `/mcp`) that accepts POST only. Each request is its own POST answered with `application/json` or a request-scoped `text/event-stream`. Then:

- Require `MCP-Protocol-Version`, `Mcp-Method`, and `Mcp-Name` (for `tools/call`, `resources/read`, `prompts/get`). Reject header and body mismatches with `400` and `HeaderMismatch` (`-32020`).
- Answer an unknown method with `404` and `-32601`; answer a notification POST with `202`.
- Treat a closed response stream as cancellation. Streams are not resumable; `Last-Event-ID` is ignored.
- Answer GET or DELETE with `405`, and ignore any `Mcp-Session-Id` header.
- Validate `Origin` (`403` when invalid), bind to `127.0.0.1` when local, and send `X-Accel-Buffering: no` on SSE responses.
- Follow the authorization spec for remote servers; stdio servers read credentials from the environment instead.

## Deprecations

Deprecated features still work during the deprecation window, but new servers adopt the replacement.

| Deprecated                                               | Replacement                                                                           |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| Roots                                                    | Pass directories or files via tool parameters, resource URIs, or server configuration |
| Sampling                                                 | Call LLM provider APIs directly                                                       |
| Logging (`notifications/message`)                        | `stderr` for stdio; OpenTelemetry for observability                                   |
| Sampling `includeContext: "thisServer"` / `"allServers"` | Omit the field or use `"none"`                                                        |
| HTTP+SSE transport (`GET /sse` plus `POST /messages`)    | Streamable HTTP                                                                       |
| OAuth Dynamic Client Registration                        | Client ID Metadata Documents                                                          |

Removed outright in `2026-07-28`: `initialize` / `notifications/initialized`, `ping`, `logging/setLevel` (log level is now per request via `io.modelcontextprotocol/logLevel`), `resources/subscribe` / `resources/unsubscribe` (now `subscriptions/listen`), `notifications/roots/list_changed`, `notifications/elicitation/complete`, the `Mcp-Session-Id` header, the HTTP GET stream, and core-protocol tasks (now the `io.modelcontextprotocol/tasks` extension).

**Legacy compatibility.** A server that must also serve `2025-11-25` or earlier clients can be dual-era: answer `initialize` with legacy semantics scoped to that process or session, and serve requests carrying modern `_meta` statelessly. A modern-only server names its supported versions in the error it returns to `initialize`. See the compatibility matrix in `resources/auto/specification-2026-07-28-basic-versioning.md`.

## Minimal servers

Both examples follow `resources/auto/docs-2026-07-28-develop-build-server.md`. Check the SDK's own documentation for resource and prompt registration and for HTTP hosting, which that guide does not cover.

TypeScript (`npm install @modelcontextprotocol/server zod`, `"type": "module"`):

```typescript
import {McpServer} from "@modelcontextprotocol/server"
import {serveStdio} from "@modelcontextprotocol/server/stdio"
import {z} from "zod"

// serveStdio calls this for each instance it needs, so return a fresh server.
function createServer() {
  const server = new McpServer({name: "weather", version: "1.0.0"})

  server.registerTool(
    "get_alerts",
    {
      description: "Get weather alerts for a US state",
      inputSchema: z.object({
        state: z.string().length(2).describe("Two-letter state code, e.g. CA")
      })
    },
    async ({state}) => ({
      content: [{type: "text", text: `No active alerts for ${state}`}]
    })
  )

  return server
}

serveStdio(createServer, {
  onerror: (error) => console.error("Server error:", error)
})
```

`serveStdio` answers both `2026-07-28` clients and legacy `initialize` clients. Wiring `server.connect(new StdioServerTransport())` by hand serves only legacy clients.

Python (MCP SDK 2.0.0 or newer, `uv add "mcp[cli]"`):

```python
import logging

from mcp.server import MCPServer

logger = logging.getLogger(__name__)
mcp = MCPServer("weather")


@mcp.tool()
async def get_alerts(state: str) -> str:
    """Get weather alerts for a US state.

    Args:
        state: Two-letter US state code (e.g. CA, NY)
    """
    logger.info("get_alerts %s", state)  # stderr, never print()
    return f"No active alerts for {state}"


if __name__ == "__main__":
    mcp.run(transport="stdio")
```

`MCPServer` builds the tool schema from type hints and the docstring. It replaces the 1.x `FastMCP` import.

## Testing with MCP Inspector

The Inspector (`@modelcontextprotocol/inspector`, Node 22.19.0 or newer) has web, `--cli`, and `--tui` clients. It defaults to the `legacy` protocol era, so pass `--protocol-era modern` to exercise `2026-07-28` behavior, and repeat with `--protocol-era legacy` if the server is dual-era.

```bash
# Modern era over stdio
npx @modelcontextprotocol/inspector --cli node build/index.js --protocol-era modern --method tools/list
npx @modelcontextprotocol/inspector --cli node build/index.js --protocol-era modern \
  --method tools/call --tool-name get_alerts --tool-arg state=CA

# Remote Streamable HTTP server
npx @modelcontextprotocol/inspector --cli https://example.com/mcp --transport http \
  --protocol-era modern --method tools/list --strict

# Interactive web UI
npx @modelcontextprotocol/inspector node build/index.js
```

CLI exit codes: `0` success, `3` auth required, `4` unreachable, `5` tool error or tool not found, `6` `--strict` schema portability failure. On failure a single JSON error line goes to `stderr`. Full flag and method reference: `resources/auto/docs-2026-07-28-tools-inspector-cli.md`; era-specific behavior (logging, subscriptions, tasks): `resources/auto/docs-2026-07-28-tools-inspector-protocol-eras.md`.

## Completion criteria

The server work is done when all of these hold:

- `--protocol-era modern --method tools/list` exits `0` and lists every tool with its schema; `--strict` also exits `0`.
- Each tool called with valid arguments exits `0`, and with invalid arguments returns `isError: true` with an actionable message (exit `5`), not a JSON-RPC error.
- A stdio server writes only JSON-RPC to `stdout`; all diagnostics appear on `stderr`.
- No code path depends on `initialize`, `ping`, `logging/setLevel`, `resources/subscribe`, session IDs, a GET stream, or server-sent JSON-RPC requests, except an explicit legacy branch of a dual-era server.
- Cross-call state flows through explicit handles, and user input or LLM calls flow through MRTR `InputRequiredResult`.
- List and read results carry `ttlMs` and `cacheScope`, and results carry `resultType` and `serverInfo` (verify in the Inspector's raw JSON-RPC view if the SDK sets them for you).
- A dual-era server also passes `tools/list` with `--protocol-era legacy`.
- Every spec claim made to the user cites the `resources/auto/specification-2026-07-28-*` page it came from.
