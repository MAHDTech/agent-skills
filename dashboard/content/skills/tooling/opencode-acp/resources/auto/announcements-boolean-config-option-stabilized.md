+++
title = "announcements-boolean-config-option-stabilized"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "opencode-acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Boolean Config Options are stabilized

> Announcement that boolean session configuration options are now part of the stable ACP protocol.

**Published:** July 6, 2026

The [Boolean Config Option Type RFD](/rfds/boolean-config-option) has moved to Completed and boolean session configuration options are stabilized.

Agents can expose native on/off controls using `type: "boolean"`. In v1, Clients explicitly opt in through the `session.configOptions.boolean` capability, allowing Agents to preserve compatibility with Clients that only understand select options.

For the protocol documentation, see [Boolean Config Options](/protocol/v1/session-config-options#boolean-config-options).


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.
