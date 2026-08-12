+++
title = "announcements-elicitation-stabilized"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "acp"
+++

{% raw %}
> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Elicitation is stabilized

> Announcement that structured elicitation is now part of the stable ACP protocol.

**Published:** July 24, 2026

The [Elicitation RFD](https://agentclientprotocol.com/rfds/elicitation) has moved to Completed, stabilizing
`elicitation/create` and the optional `elicitation/complete` notification.

Agents can now ask users for structured, non-sensitive information through form
mode or direct them to secure out-of-band interactions through URL mode.
Clients advertise each supported mode explicitly during initialization.
The protocol also defines explicit session, tool-call, and request
scopes.

For the protocol documentation, see [Elicitation](https://agentclientprotocol.com/protocol/v1/elicitation).
{% endraw %}
