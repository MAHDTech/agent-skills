+++
title = "grok-connectors-gmail-google-calendar"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Connectors

# Gmail & Google Calendar

Gmail and Google Calendar are connected as separate connectors, each with its own OAuth sign-in and permissions. Together they let Grok manage your email and schedule from within a conversation.

## Gmail

### Capabilities

* **Search emails** using Gmail search operators (`from:`, `to:`, `subject:`, `newer_than:`, `has:attachment`, etc.).
* **Read full messages** including body text, headers, and attachments.
* **Compose and manage drafts** so you can review before sending.
* **Send messages**, reply, and forward when write permissions are enabled.
* **Organize mail** by modifying labels, creating labels, trashing, or deleting messages.

### Required permissions

Gmail uses a tiered permission model. The base connection is read-only; write and send capabilities are enabled progressively by your organization's administrators.

| Scope | Purpose | When requested |
|---|---|---|
| `gmail.readonly` | Search and read emails | Always (base) |
| `gmail.modify` | Drafts, trash, label changes | When write tools are enabled |
| `gmail.send` | Send messages, reply, forward | When send tools are enabled |
| `gmail.labels` | Create and delete labels | When label management tools are enabled |
| `userinfo.email` | Identify your Google account | Always |

Note: **gmail.modify** is a superset of **gmail.readonly**. When write tools are enabled, only the modify scope is requested to avoid duplicate permission prompts.

### How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Gmail**.
3. Sign in with your Google account and approve the requested permissions.

## Google Calendar

### Capabilities

* **Search events** by date range and keyword.
* **View event details** including attendees, location, and description.
* **Check availability** with free/busy lookups.
* **Create, update, and delete events** when write permissions are enabled.
* **RSVP to events** (accept, decline, tentative).
* **List calendars** the account has access to.

### Required permissions

Like Gmail, Google Calendar uses a tiered permission model.

| Scope | Purpose | When requested |
|---|---|---|
| `calendar.readonly` | Search and read calendar events | Always (base) |
| `calendar.events` | Create, update, and delete events | When write tools are enabled |
| `calendar.freebusy` | Check availability / free-busy status | When availability tool is enabled |
| `calendar.calendarlist.readonly` | List accessible calendars | When calendar list tool is enabled |
| `userinfo.email` | Identify your Google account | Always |

### How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Google Calendar**.
3. Sign in with your Google account and approve the requested permissions.

## Privacy and security

**We do not train on your data.** SpaceXSI does not use your Gmail or Google Calendar data for model training.

**Nothing is stored.** Conversations that use your Google connectors do not result in any of your email or calendar data being stored on SpaceXSI servers. Grok accesses your data in real time when you ask a question, and does not retain it afterward.

**You control access.** Disconnect at any time to immediately revoke Grok's access to your Google account.

## Disconnecting

To disconnect either connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find the connector and click **Disconnect**.

You can also revoke the app's access from your Google account at [myaccount.google.com/permissions](https://myaccount.google.com/permissions).


