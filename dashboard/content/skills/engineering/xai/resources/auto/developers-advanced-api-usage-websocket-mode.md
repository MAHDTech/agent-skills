+++
title = "developers-advanced-api-usage-websocket-mode"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Advanced API Usage

# WebSocket Mode

The Responses API can be driven over a single, long-lived WebSocket connection to `/v1/responses`
instead of opening a fresh HTTP request for every turn. After the first response, subsequent turns
only need to send the new input items along with a `previous_response_id` — the server keeps the
prior state in memory on the open socket.

This works with both Zero Data Retention (ZDR) and `store=false`, since nothing about the
continuation needs to touch persistent storage.

## When it helps

WebSocket mode is aimed at agentic workloads with many sequential tool calls — coding agents,
orchestration loops, anything that goes back and forth with the model dozens of times.

Each turn skips the connection setup and re-sends only the new input rather than the full
conversation, which adds up over long rollouts. In our internal benchmarks on agentic workloads
with many tool calls, we have measured up to ~20% lower end-to-end latency compared to repeated
HTTP requests with the same `previous_response_id` chaining.

## Opening a connection and sending the first turn

After the WebSocket upgrade succeeds, every turn is initiated by the client sending a
`response.create` message. The body is the same shape as the
[Responses create body](https://docs.x.ai/developers/rest-api-reference/inference/chat#create-new-response), minus
transport-only fields like `stream` and `background` (responses are always streamed back as
events on the socket).

```pythonWithoutSDK
import json
import os
from websocket import create_connection

ws = create_connection(
    "wss://api.x.ai/v1/responses",
    header=[
        f"Authorization: Bearer {os.environ['XAI_API_KEY']}",
    ],
)

ws.send(
    json.dumps(
        {
            "type": "response.create",
            "model": "grok-4.6",
            "store": False,
            "input": [
                {
                    "type": "message",
                    "role": "user",
                    "content": [{"type": "input_text", "text": "Find fizz_buzz()"}],
                }
            ],
            "tools": [],
        }
    )
)
```

```javascriptWithoutSDK
import WebSocket from "ws";

const ws = new WebSocket("wss://api.x.ai/v1/responses", {
  headers: {
    Authorization: \`Bearer \${process.env.XAI_API_KEY}\`,
  },
});

ws.on("open", () => {
  ws.send(
    JSON.stringify({
      type: "response.create",
      model: "grok-4.6",
      store: false,
      input: [
        {
          type: "message",
          role: "user",
          content: [{ type: "input_text", text: "Find fizz_buzz()" }],
        },
      ],
      tools: [],
    })
  );
});

ws.on("message", (data) => {
  console.log(JSON.parse(data.toString()));
});
```

### Warmups with `generate: false`

If you already know the tools, instructions, or system messages you'll need for an upcoming turn,
you can prime the connection by sending `response.create` with `generate: false`. The server
prepares the request state but does not run the model — no output is returned. The warmup still
emits a response ID, which you can chain from later via `previous_response_id` so the actual
generation turn starts faster.

## Continuing a run

For every follow-up turn, send a new `response.create` and include:

* `previous_response_id` — the ID of the last response on this chain.
* `input` — only the new items for this turn (typically tool outputs plus the next user
  message). Don't resend prior history; the server already has it.

```pythonWithoutSDK
ws.send(
    json.dumps(
        {
            "type": "response.create",
            "model": "grok-4.6",
            "store": False,
            "previous_response_id": "resp_123",
            "input": [
                {
                    "type": "function_call_output",
                    "call_id": "call_123",
                    "output": "tool result",
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [{"type": "input_text", "text": "Now optimize it."}],
                },
            ],
            "tools": [],
        }
    )
)
```

```javascriptWithoutSDK
ws.send(
  JSON.stringify({
    type: "response.create",
    model: "grok-4.6",
    store: false,
    previous_response_id: "resp_123",
    input: [
      {
        type: "function_call_output",
        call_id: "call_123",
        output: "tool result",
      },
      {
        type: "message",
        role: "user",
        content: [{ type: "input_text", text: "Now optimize it." }],
      },
    ],
    tools: [],
  })
);
```

## How chaining works on the socket

`previous_response_id` behaves the same way it does over HTTP, but the WebSocket path has an
additional in-memory shortcut. Each open connection holds the state for its most recent response
in a per-connection cache. Continuing from that response avoids touching storage entirely, which
is what makes WebSocket mode safe to use with `store=false` and ZDR.

If you reference an older `previous_response_id` that is no longer in the connection cache:

* With `store=true`, the server may rehydrate it from persisted state, but you lose the
  in-memory latency win.
* With `store=false` or under ZDR, there is no fallback storage to read from, and the turn fails
  with `previous_response_not_found`.

A turn that fails (`4xx` or `5xx`) evicts its `previous_response_id` from the connection cache
so a retry doesn't continue from broken state.

## Connection limits and behavior

* The event types and ordering are identical to the existing Responses streaming format.
* One connection processes turns serially — sending a second `response.create` while one is
  in-flight will queue, not multiplex.
* Need parallel turns? Open multiple connections.
* A single connection can stay open for up to 25 minutes. After that, the server closes it and
  you'll need to reconnect.

## Reconnecting

When the socket drops (network blip, deploy, hitting the 25-minute cap), open a new connection
and pick whichever recovery path applies:

1. If you used `store=true` and still have a valid response ID, just continue with
   `previous_response_id` and the new input items on the new socket.
2. Otherwise (e.g. `store=false` or you hit `previous_response_not_found`), drop
   `previous_response_id` entirely and start a fresh chain by sending the full input context for
   the next turn.

## Errors

A few error responses are specific to WebSocket mode and worth handling explicitly.

### `previous_response_not_found`

Returned when the requested `previous_response_id` is not in the connection cache and cannot be
hydrated from storage (e.g. ZDR, `store=false`, or it was evicted by a prior failure).

```json
{
  "type": "error",
  "status": 400,
  "error": {
    "code": "previous_response_not_found",
    "message": "Previous response with id 'resp_abc' not found.",
    "param": "previous_response_id"
  }
}
```

### `websocket_connection_limit_reached`

Sent right before the server closes a connection that has been open for the maximum 25 minutes.
Open a fresh WebSocket and reconnect using one of the patterns above.

```json
{
  "type": "error",
  "status": 400,
  "error": {
    "type": "invalid_request_error",
    "code": "websocket_connection_limit_reached",
    "message": "Responses websocket connection limit reached (25 minutes). Create a new websocket connection to continue."
  }
}
```

## Related guides

* [Streaming](https://docs.x.ai/developers/model-capabilities/text/streaming)
* [Function Calling](https://docs.x.ai/developers/tools/function-calling)
* [Responses API Reference](https://docs.x.ai/developers/rest-api-reference/inference/chat#create-new-response)
{% endraw %}
