#### Connectors

# Google Drive

The Google Drive connector lets Grok search, read, and manage files in your Google Drive. Find documents by content or title, read their contents inline, create new files, organize folders, and upload artifacts Grok generates.

## Capabilities

* **Search files** by content keywords or title across Docs, Sheets, Slides, and other file types.
* **Read file contents** to summarize or analyze documents directly in the conversation.
* **Create and write files** including new Google Docs.
* **Manage folders** by creating new folders, listing folder contents, and trashing files.
* **Upload artifacts** that Grok generates (spreadsheets, reports, etc.) to any folder in your Drive.
* **Filter by attributes** such as starred files, shared files, files modified after a date, or files within a specific folder.

## Required permissions

The Google Drive connector uses Google OAuth and requests the following scopes during sign-in:

| Scope | Purpose |
|---|---|
| `drive.metadata.readonly` | View metadata for files in your Drive (titles, dates, folder structure) |
| `drive.readonly` | Read the content of files in your Drive |
| `drive` | Create and modify files in your Drive (write operations, optional) |
| `userinfo.email` | Identify your Google account |

Google will show a consent screen listing these permissions. Grok can only access files that the signed-in Google account has access to.

## How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Google Drive**.
3. Sign in with your Google account.
4. Review the requested permissions and click **Allow**.

## Privacy and security

**We do not train on your data.** SpaceXSI does not use your Google Drive data for model training.

**Nothing is stored.** Conversations that use your Google Drive connector do not result in any of your files or documents being stored on SpaceXSI servers. Grok accesses your data in real time when you ask a question, and does not retain it afterward.

**You control access.** Disconnect at any time to immediately revoke Grok's access to your Google account.

## Disconnecting

To disconnect the Google Drive connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find Google Drive in your connected list and click **Disconnect**.

You can also revoke the app's access from your Google account at [myaccount.google.com/permissions](https://myaccount.google.com/permissions).

