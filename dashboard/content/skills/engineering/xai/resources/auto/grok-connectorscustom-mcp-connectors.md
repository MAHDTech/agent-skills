+++
title = "grok-connectorscustom-mcp-connectors"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Grok

# Connectors

Connectors are available to all Grok users and let Grok access your external tools and data sources directly within a conversation. Search your email, browse files in cloud storage, check your calendar, and more without leaving the chat.

For Grok Business and Enterprise users, a team admin must first provision a connector in the [cloud console](https://docs.x.ai/grok/connector-management) before it is available to members of the organization.

There are three kinds of connectors:

## Built-in connectors

Built-in connectors are maintained by xAI and integrate natively with Grok. Each one authenticates via OAuth, so you connect once and Grok can access your data on demand. No configuration beyond the initial sign-in is required.

The following built in connectors are available:

| Connector | What it connects | |
|---|---|---|
| **Gmail & Google Calendar** | Gmail messages and Google Calendar events |  |
| **Google Drive** | Google Drive files, Docs, Sheets, and Slides |  |
| **OneDrive** | Microsoft OneDrive personal storage |  |
| **Outlook Mail & Calendar** | Outlook email and calendar events |  |
| **Microsoft Teams** | Microsoft Teams messages, channels, and chats |  |
| **SharePoint** | Microsoft SharePoint sites and document libraries |  |
| **Salesforce** | Salesforce CRM - explore objects, query records, create and update |  |

To add a builtin connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select the service you want to connect.
3. Complete the OAuth sign-in flow. Grok will request only the permissions it needs.

Once connected, Grok can use the connector's tools automatically whenever your questions relate to that service.

## Connector catalog

In addition to the built-in connectors, Grok provides a catalog of pre-configured OAuth connectors for many popular third-party services. These require no extra setup beyond signing in.

Browse the full catalog at [grok.com/connectors](https://grok.com/connectors).

## Custom MCP connectors

If you need to connect Grok to a service not available in the catalog, you can bring your own [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server. MCP is an open standard that lets AI assistants interact with external tools and data sources through a unified protocol.

With a custom MCP connector you can:

* Expose any internal API, database, or SaaS tool to Grok.
* Define your own tools with custom schemas and logic.
* Control authentication and access on your own infrastructure.

To add a custom MCP connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector**, then select **Custom**.
3. Enter the MCP server URL and complete any required authentication.

Grok will discover the tools your MCP server exposes and make them available in conversations, just like the built-in and catalog connectors.

Your MCP server must be reachable over the public internet. If it is running on your local machine, you will need a tunneling service to make it accessible. See [Custom MCP Server Tunneling](https://docs.x.ai/grok/connectors/custom-mcp-tunneling) for setup instructions.
