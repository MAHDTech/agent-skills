> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Session Usage Updates are stabilized

> Announcement that session context and cumulative cost updates are now part of the stable ACP protocol.

**Published:** June 5, 2026

The [Session Context Size and Cost RFD](/rfds/session-usage) has moved to Completed and `usage_update` session notifications are stabilized.

Agents can report the current context token count and context-window size, along with optional cumulative session cost. This lets Clients keep context and cost displays current throughout the session lifecycle.

For the protocol documentation, see [Session Usage Updates](/protocol/v1/prompt-turn#session-usage-updates).


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.