#### Connectors

# Salesforce

The Salesforce connector gives Grok powerful, real-time access to your Salesforce org. Everything Grok does respects your user permissions, field-level security, and sharing rules — exactly as if you were using Salesforce yourself.

## Salesforce

### Capabilities

* **Explore your data model** - See all the objects in your Salesforce org (standard and custom).
* **Find information quickly** - Search across multiple objects at once or instantly pull up your recently viewed and recently modified records (for example, "my recent Opportunities" or "Show top 10 closed deals for this week").
* **Ask detailed questions about your data** - Retrieve exactly the records you need, with filters, sorting, summaries, and related information across parent-child relationships — without writing queries yourself.
* **Create new records** - Add Leads, Opportunities, Accounts, Cases, or records on custom objects directly from chat.
* **Update records** - Change fields on existing records by ID or by context.

### Required permissions

The Salesforce connector uses standard Salesforce OAuth 2.0 scopes to access the API on your behalf.

| Scope | Purpose | When requested |
|---|---|---|
| `mcp_api` | Access the Salesforce REST and SOAP APIs to read and write data | Always |
| `refresh_token` | Maintain access and refresh tokens between sessions | Always |

All actions are further restricted by your Salesforce profile, role, sharing rules, and field-level security. Grok can never access more than your connected Salesforce user is allowed to see or modify.

### How to connect

Step 1: Company Admin Setup (xAI Console)

1. Go to https://console.x.ai/?utm\_source=docs\&utm\_medium=referral\&utm\_campaign=grok-connectors-salesforce\&utm\_content=console-home
2. Navigate to Grok Enterprise → Connectors
3. Add the Salesforce connector and provide the OAuth credentials (Client ID, Client Secret) during the setup flow.

Step 2: Install and Configure the Salesforce DX MCP Server

Reference: [Install and Configure the Salesforce DX MCP Server (Beta)](https://developer.salesforce.com/docs/atlas.en-us.sfdx_dev.meta/sfdx_dev/sfdx_dev_mcp_server.htm)

Step 3: Team Member Connection

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and search for **Salesforce**.
3. Sign in with your Salesforce credentials and approve the requested permissions.

Once connected, just ask Grok anything about your Salesforce data in normal conversation.

### Common examples

* "Show me the top 10 closed deals for this week"
* "What Opportunities are closing this month over $50k?"
* "Create a new Lead for Jane Smith at Acme Corp"
* "Show my recent cases that are still open"
* "Summarize the Account history for Globex Corporation"

## Privacy and security

**We do not train on your data.** SpaceXSI does not use your Salesforce data for model training.

**Your data never leaves Salesforce.** Grok queries Salesforce in real time and does not copy or store your records on SpaceXSI servers.

**Your permissions are always enforced.** If you cannot see a record or field in Salesforce, Grok cannot see it either. The same applies to create and update operations.

**Disconnect anytime.** Revoking access immediately stops Grok from reaching your org.

## Disconnecting

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find Salesforce in your connected list and click **Disconnect**.

## Limitation

Currently only one Salesforce org/instance is supported per team.

