+++
title = "grok-connectors-outlook"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Connectors

# Outlook Mail & Calendar

Outlook Mail and Outlook Calendar are connected as separate connectors, each with its own OAuth sign-in and permissions. Together they give Grok full access to your Microsoft email and schedule.

## Outlook Mail

### Capabilities

* **Search emails** across your mailbox with keyword and filter queries.
* **Read full messages** including body, headers, and attachments.
* **Compose and manage drafts** with support for To, Cc, Bcc, and HTML body.
* **Send messages**, reply-all, and forward emails.
* **Organize mail** by moving messages between folders, creating folders, and batch operations.
* **Upload attachments** from Grok-generated artifacts directly to a draft.

### Required permissions

| Scope | Purpose |
|---|---|
| `Mail.ReadWrite` | Read, create, update, and delete mail and drafts |
| `Mail.Send` | Send mail on behalf of the user |
| `User.Read` | Read the signed-in user's profile |
| `offline_access` | Maintain access without repeated sign-in prompts |

These are delegated permissions. Grok can only access the mailbox of the signed-in user.

### How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Outlook**.
3. Sign in with your Microsoft work or school account.
4. Review the requested permissions and click **Accept**.

## Outlook Calendar

### Capabilities

* **Search events** by date range and keyword.
* **View event details** including attendees, location, and body.
* **Check availability** across multiple attendees.
* **Create and update events** with attendees, location, recurrence, and reminders.
* **RSVP to events** (accept, decline, tentative) with optional comments.

### Required permissions

| Scope | Purpose |
|---|---|
| `Calendars.ReadWrite` | Read, create, update, and delete calendar events |
| `User.Read` | Read the signed-in user's profile |
| `offline_access` | Maintain access without repeated sign-in prompts |

### How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Outlook Calendar**.
3. Sign in with your Microsoft work or school account.
4. Review the requested permissions and click **Accept**.

## Admin consent

Some organizations require an Azure AD administrator to approve application permissions before users can sign in. If you see a "need admin approval" error, contact your IT administrator and ask them to grant consent for the xAI Grok application in the Azure AD admin portal under **Enterprise applications**.

## Privacy and security

**We do not train on your data.** SpaceXSI does not use your Outlook email or calendar data for model training.

**Nothing is stored.** Conversations that use your Outlook connectors do not result in any of your email or calendar data being stored on SpaceXSI servers. Grok accesses your data in real time when you ask a question, and does not retain it afterward.

**Scoped to your account.** Outlook permissions are delegated to the signed-in user. Grok can only access your own mailbox and calendar, not those belonging to other users.

**You control access.** Disconnect at any time to immediately revoke Grok's access to your Microsoft account.

## Disconnecting

To disconnect either connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find the connector and click **Disconnect**.

You can also revoke the app's access from your Microsoft account at [myapps.microsoft.com](https://myapps.microsoft.com).


