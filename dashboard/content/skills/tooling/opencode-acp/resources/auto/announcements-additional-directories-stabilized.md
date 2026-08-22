+++
title = "announcements-additional-directories-stabilized"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "opencode-acp"
+++

> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Additional Workspace Roots are stabilized

> Announcement that additional workspace roots are now part of the stable ACP protocol.

**Published:** June 1, 2026

The [Additional Workspace Roots RFD](https://agentclientprotocol.com/rfds/additional-directories) has moved to Completed and `additionalDirectories` is stabilized for session lifecycle requests.

Clients can provide an ordered list of absolute additional workspace roots when the Agent advertises support. The existing `cwd` remains the primary working directory, while the additional roots expand the session's filesystem scope.

For the protocol documentation, see [Additional Workspace Roots](https://agentclientprotocol.com/protocol/v1/session-setup#additional-workspace-roots).
