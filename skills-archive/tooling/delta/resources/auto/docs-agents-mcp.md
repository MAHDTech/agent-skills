---
title: "MCP Servers"
url: "https://delta.dev/docs/agents/mcp"
description: "Connect remote MCP servers to give Delta's agents access to external tools."
---

1.  Using Delta
2.  MCP Servers

# MCP Servers

Connect remote MCP servers to give Delta's agents access to external tools.

On this page

1.  [Connect an MCP server](#connect-an-mcp-server)
    1.  [Authentication](#authentication)
2.  [Manage connections](#manage-connections)
3.  [Troubleshooting](#troubleshooting)
    1.  [A server needs authorization](#a-server-needs-authorization)
    2.  [A custom server cannot connect](#a-custom-server-cannot-connect)
4.  [What’s next](#whats-next)

Connect an MCP (Model Context Protocol) server to let Delta’s agents use tools from external services.

## [Connect an MCP server](#connect-an-mcp-server)

Delta connects to remote HTTPS MCP servers with [OAuth authentication or no authentication](#authentication).

1.  Sign in to Delta and open **Settings > MCP** in the desktop app.
2.  Make sure the switch at the top of the MCP settings page is on.
3.  Click on **Search providers or enter an MCP URL**. Select a provider, or paste your server’s MCP endpoint URL and choose **Use custom MCP server**.
4.  If the server requires authorization, choose **Sign in** beside it and complete authorization in your browser.
5.  Wait for the server’s status to show **Connected**. Its tools are available to the agent on your next turn.

### [Authentication](#authentication)

Delta supports MCP servers with these authentication options:

*   No authentication.
*   OAuth 2.1 for all servers that support dynamic client registration.
*   OAuth through apps manually registered by Delta, including Slack, GitHub, and Figma.

Local MCP servers and remote servers that require manually supplied bearer tokens or API keys (such as `x-api-token` headers) are not supported.

## [Manage connections](#manage-connections)

*   Choose **Reauthorize** to authorize an OAuth connection again.
*   Choose **Remove** to remove a configured server from Delta.
*   Turn off the switch at the top of the MCP settings page to stop offering configured MCP tools to new agent turns without removing the servers.

## [Troubleshooting](#troubleshooting)

### [A server needs authorization](#a-server-needs-authorization)

For **Authorization required**, choose **Sign in**. For **Reconnect required**, choose **Reauthorize** and complete the browser flow.

### [A custom server cannot connect](#a-custom-server-cannot-connect)

Check that you entered the MCP endpoint, that it is publicly reachable over HTTPS, and that it uses one of the supported authentication options listed above.

## [What’s next](#whats-next)

*   [Skills](/docs/agents/skills) teach agents reusable workflows, including how to use connected tools.
*   [Agentic Safety](/docs/privacy-and-security/agentic-safety) covers the risks of giving agents access to tools.