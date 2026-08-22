+++
title = "announcements-session-delete-stabilized"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Session Delete is stabilized

> Announcement that the session/delete method is now part of the stable ACP protocol.

**Published:** June 5, 2026

The [Session Delete RFD](https://agentclientprotocol.com/rfds/session-delete) has moved to Completed and the `session/delete` method is stabilized.

Clients can use this capability-gated method to remove sessions from future `session/list` results. ACP standardizes the user-facing behavior while leaving soft deletion, hard deletion, and retention policy to the Agent.

For the protocol documentation, see [Session Delete](https://agentclientprotocol.com/protocol/v1/session-delete).
