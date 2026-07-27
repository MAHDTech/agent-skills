+++
title = "protocol-v2-cancellation"
[extra]
skill = false
category = "tooling"
mermaid = true
skill_name = "opencode-acp"
+++

{% raw %}
> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Cancellation

> Mechanisms for request cancellation

ACP uses JSON-RPC 2.0 for making requests and getting responses.

The JSON-RPC specification doesn't define any standard mechanism for request cancellation and keeps it up to the implementation.

## `$/cancel_request` Notification

In order to provide a consistent approach to cancellation, ACP defines a `$/cancel_request` notification that can be sent to cancel requests.

Cancellation remains optional as it might not be implementable in all clients or servers. For example if the implementation uses a single threaded synchronous programming language then there is little it can do to react to a `$/cancel_request` notification.

When a `$/cancel_request` notification is received by a supporting implementation, the implementation:

* **MAY** cancel the corresponding request activity and all nested activities related to that request
* **MAY** finish sending any pending notifications before responding
* **MUST** send one of these responses for the original request:
  * A valid response with appropriate data (such as partial results or cancellation marker)
  * An error response with code [`-32800` (Request Cancelled)](https://agentclientprotocol.com/protocol/v2/schema#errorcode)

The calling side **MAY** implement graceful cancellation processing by waiting for the response from the remote side.

Cancellation **MAY** also be done explicitly on a per-feature basis within the protocol to cover specific scenarios, such as cancellation of [active session work](https://agentclientprotocol.com/protocol/v2/prompt-lifecycle#cancellation).

## Internal Cancellation

Requests can also be cancelled internally by the executing party without receiving `$/cancel_request`:

* **Client-side examples**: User closes IDE, switches to different project, file becomes unavailable
* **Agent-side examples**: LLM context limit reached, internal timeout, resource constraints

When internal cancellation occurs, the executing party **SHOULD**:

* Send the same `-32800` (Cancelled) error response as if `$/cancel_request` was received
* Ensure consistent behavior regardless of cancellation source

## Example: Cascading Cancellation Flow

```mermaid theme={null}
sequenceDiagram
    participant Client
    participant Agent

    Note over Client,Agent: 1. Session prompt in progress
    Client->>Agent: session/prompt (id=1, "Analyze file X")
    Agent-->>Client: response to id=1 ({})
    Agent->>Client: session/update (state_update: running)

    Note over Client,Agent: 2. Agent makes concurrent permission requests
    Agent->>Client: session/request_permission (id=2, "read sensitive file")
    Agent->>Client: session/request_permission (id=3, "apply workspace changes")

    Note over Client,Agent: 3. Client cancels active session work
    Client->>Agent: session/cancel (sessionId)

    Note over Client,Agent: 4. Agent cascades cancellation internally
    Agent->>Client: $/cancel_request (id=2) [permission request]
    Agent->>Client: $/cancel_request (id=3) [permission request]

    Note over Client,Agent: 5. Client confirms individual cancellations
    Client->>Agent: response to id=2 (error -32800 "Cancelled")
    Client->>Agent: response to id=3 (error -32800 "Cancelled")

    Note over Client,Agent: 6. Agent reports cancellation completion
    Agent->>Client: session/update (state_update: idle, stopReason: "cancelled")
```
{% endraw %}
