+++
title = "announcements-session-usage-stabilized"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Session Usage Updates are stabilized

> Announcement that session context and cumulative cost updates are now part of the stable ACP protocol.

**Published:** June 5, 2026

The [Session Context Size and Cost RFD](https://agentclientprotocol.com/rfds/session-usage) has moved to Completed and `usage_update` session notifications are stabilized.

Agents can report the current context token count and context-window size, along with optional cumulative session cost. This lets Clients keep context and cost displays current throughout the session lifecycle.

For the protocol documentation, see [Session Usage Updates](https://agentclientprotocol.com/protocol/v1/prompt-turn#session-usage-updates).
