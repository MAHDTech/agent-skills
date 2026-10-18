+++
title = "announcements-elicitation-stabilized"
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

# Elicitation is stabilized

> Announcement that structured elicitation is now part of the stable ACP protocol.

**Published:** July 24, 2026

The [Elicitation RFD](/rfds/elicitation) has moved to Completed, stabilizing
`elicitation/create` and the optional `elicitation/complete` notification.

Agents can now ask users for structured, non-sensitive information through form
mode or direct them to secure out-of-band interactions through URL mode.
Clients advertise each supported mode explicitly during initialization.
The protocol also defines explicit session, tool-call, and request
scopes.

For the protocol documentation, see [Elicitation](/protocol/v1/elicitation).


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.
