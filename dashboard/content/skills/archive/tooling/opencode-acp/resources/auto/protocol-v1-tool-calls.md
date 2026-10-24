+++
title = "protocol-v1-tool-calls"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "opencode-acp"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/opencode-acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Tool Calls

> How Agents report tool call execution

Tool calls represent actions that language models request Agents to perform during a [prompt turn](https://agentclientprotocol.com/protocol/v1/prompt-turn.md). When an LLM determines it needs to interact with external systems—like reading files, running code, or fetching data—it generates tool calls that the Agent executes on its behalf.

Agents report tool calls through [`session/update`](https://agentclientprotocol.com/protocol/v1/prompt-turn#3-agent-reports-output) notifications, allowing Clients to display real-time progress and results to users.

While Agents handle the actual execution, they may leverage Client capabilities like [permission requests](#requesting-permission) or [file system access](https://agentclientprotocol.com/protocol/v1/file-system.md) to provide a richer, more integrated experience.

## Creating

When the language model requests a tool invocation, the Agent **SHOULD** report it to the Client:

```json theme={null}
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123def456",
    "update": {
      "sessionUpdate": "tool_call",
      "toolCallId": "call_001",
      "name": "read_file",
      "title": "Reading configuration file",
      "kind": "read",
      "status": "pending"
    }
  }
}
```

**toolCallId** `ToolCallId` *required*

A unique identifier for this tool call within the session

**name** `string`

The optional programmatic name of the invoked tool, such as `read_file`.
Agents **SHOULD** include it in the first report when available and **SHOULD
NOT** change it later except to correct metadata. Names are opaque,
informational metadata: they do not advertise a capability or grant
authorization. Omission and `null` both mean that no name is provided.

**title** `string` *required*

A human-readable title describing what the tool is doing

**kind** `ToolKind`

The category of tool being invoked.

**kinds**

* `read` - Reading files or data
* `edit` - Modifying files or content
* `delete` - Removing files or data
* `move` - Moving or renaming files
* `search` - Searching for information
* `execute` - Running commands or code
* `think` - Internal reasoning or planning
* `fetch` - Retrieving external data
* `switch_mode` - Switching the session mode
* `other` - Other tool types (default)

Tool kinds help Clients choose appropriate icons and optimize how they display tool execution progress.

**status** `ToolCallStatus`

The current [execution status](#status) (defaults to `pending`)

**content** `ToolCallContent[]`

[Content produced](#content) by the tool call

**locations** `ToolCallLocation[]`

[File locations](#following-the-agent) affected by this tool call

**rawInput** `unknown`

The raw input sent to the tool. Any JSON value is accepted, including objects,
arrays, strings, numbers, booleans, and `null`.

**rawOutput** `unknown`

The raw output returned by the tool. Any JSON value is accepted, including
objects, arrays, strings, numbers, booleans, and `null`.

Omission and `null` are equivalent for `rawInput` and `rawOutput`.
On an initial tool call, both mean no value is provided. On a tool call
update, both leave the previously reported value unchanged.

## Updating

As tools execute, Agents send updates to report progress and results.

Updates use the `session/update` notification with `tool_call_update`:

```json theme={null}
{
  "jsonrpc": "2.0",
  "method": "session/update",
  "params": {
    "sessionId": "sess_abc123def456",
    "update": {
      "sessionUpdate": "tool_call_update",
      "toolCallId": "call_001",
      "status": "in_progress",
      "content": [
        {
          "type": "content",
          "content": {
            "type": "text",
            "text": "Found 3 configuration files..."
          }
        }
      ]
    }
  }
}
```

All fields except `toolCallId` are optional in updates. Only the fields being
changed need to be included. For `name`, omission and `null` both leave the
existing value unchanged, while a string sets or replaces it. ACP v1 cannot
clear a previously reported name.

## Requesting Permission

The Agent **MAY** request permission from the user before executing a tool call by calling the `session/request_permission` method:

```json theme={null}
{
  "jsonrpc": "2.0",
  "id": 5,
  "method": "session/request_permission",
  "params": {
    "sessionId": "sess_abc123def456",
    "toolCall": {
      "toolCallId": "call_001"
    },
    "options": [
      {
        "optionId": "allow-once",
        "name": "Allow once",
        "kind": "allow_once"
      },
      {
        "optionId": "reject-once",
        "name": "Reject",
        "kind": "reject_once"
      }
    ]
  }
}
```

**sessionId** `SessionId` *required*

The session ID for this request

**toolCall** `ToolCallUpdate` *required*

The tool call update containing details about the operation

**options** `PermissionOption[]` *required*

Available [permission options](#permission-options) for the user to choose
from

The Client responds with the user's decision:

```json theme={null}
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "outcome": {
      "outcome": "selected",
      "optionId": "allow-once"
    }
  }
}
```

Clients **MAY** automatically allow or reject permission requests according to the user settings.

If the current prompt turn gets [cancelled](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#cancellation), the Client **MUST** respond with the `"cancelled"` outcome:

```json theme={null}
{
  "jsonrpc": "2.0",
  "id": 5,
  "result": {
    "outcome": {
      "outcome": "cancelled"
    }
  }
}
```

**outcome** `RequestPermissionOutcome` *required*

The user's decision, either: - `cancelled` - The [prompt turn was
cancelled](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#cancellation) - `selected` with an
`optionId` - The ID of the selected permission option

### Permission Options

Each permission option provided to the Client contains:

**optionId** `string` *required*

Unique identifier for this option

**name** `string` *required*

Human-readable label to display to the user

**kind** `PermissionOptionKind` *required*

A hint to help Clients choose appropriate icons and UI treatment for each option.

* `allow_once` - Allow this operation only this time
* `allow_always` - Allow this operation and remember the choice
* `reject_once` - Reject this operation only this time
* `reject_always` - Reject this operation and remember the choice

## Status

Tool calls progress through different statuses during their lifecycle:

**pending**

The tool call hasn't started running yet because the input is either streaming
or awaiting approval

**in\_progress**

The tool call is currently running

**completed**

The tool call completed successfully

**failed**

The tool call failed with an error

## Content

Tool calls can produce different types of content:

### Regular Content

Standard [content blocks](https://agentclientprotocol.com/protocol/v1/content.md) like text, images, or resources:

```json theme={null}
{
  "type": "content",
  "content": {
    "type": "text",
    "text": "Analysis complete. Found 3 issues."
  }
}
```

### Diffs

File modifications shown as diffs:

```json theme={null}
{
  "type": "diff",
  "path": "/home/user/project/src/config.json",
  "oldText": "{\n  \"debug\": false\n}",
  "newText": "{\n  \"debug\": true\n}"
}
```

**path** `string` *required*

The absolute file path being modified

**oldText** `string`

The original content (null for new files)

**newText** `string` *required*

The new content after modification

### Terminals

Live terminal output from command execution:

```json theme={null}
{
  "type": "terminal",
  "terminalId": "term_xyz789"
}
```

**terminalId** `string` *required*

The ID of a terminal created with `terminal/create`

When a terminal is embedded in a tool call, the Client displays live output as it's generated and continues to display it even after the terminal is released.

> Learn more about Terminals
>
> [Open](https://agentclientprotocol.com/protocol/v1/terminals.md)

## Following the Agent

Tool calls can report file locations they're working with, enabling Clients to implement "follow-along" features that track which files the Agent is accessing or modifying in real-time.

```json theme={null}
{
  "path": "/home/user/project/src/main.py",
  "line": 42
}
```

**path** `string` *required*

The absolute file path being accessed or modified

**line** `integer | null`

Optional line number within the file, represented as an unsigned 32-bit
integer. Omission and `null` are equivalent and mean no specific line was
provided.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.
