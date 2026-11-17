#### Connectors

# OneDrive

**Available on Grok Business and Enterprise plans only.**

The OneDrive connector gives Grok access to your personal Microsoft OneDrive storage. Browse your files, read documents, and upload artifacts Grok generates directly to your OneDrive.

## Prerequisites

Before team members can use OneDrive in Grok, a **team admin** must add the connector through the [console](https://console.x.ai?utm_source=docs\&utm_medium=referral\&utm_campaign=grok-connectors-onedrive\&utm_content=console-home). The setup requires:

1. **Provide the Azure AD Tenant ID** -- The organization's Microsoft Entra (Azure AD) tenant identifier (e.g., `xai.onmicrosoft.com` or a GUID), found in the Azure portal under **Azure Active Directory > Overview**.
2. **Grant admin consent** -- A Microsoft 365 administrator must approve Grok's access to OneDrive. This is a one-time authorization. The admin can approve directly, copy the consent link for their IT team, or skip and complete it later.

After admin consent is complete, individual team members can connect their own Microsoft accounts on [grok.com](https://grok.com/connectors).

## Capabilities

* **Browse files and folders** in your OneDrive, including nested directory structures.
* **Upload files** that Grok generates (spreadsheets, PDFs, reports, etc.) straight to your OneDrive.

For full-text search across your OneDrive files, connect the [SharePoint connector](/grok/connectors/sharepoint) as well. OneDrive for Business files are stored on SharePoint infrastructure and are searchable through the SharePoint document search.

## Required permissions

The OneDrive connector uses Microsoft Graph API and requests the following OAuth scopes during sign-in:

| Scope | Purpose |
|---|---|
| `Files.ReadWrite` | Read and write files in the user's OneDrive |
| `User.Read` | Read the signed-in user's profile (used to identify the account) |
| `offline_access` | Maintain access without repeated sign-in prompts |

These permissions are delegated and scoped to the signed-in user's own OneDrive. Grok cannot access files belonging to other users.

## How to connect (team members)

Once your admin has completed the prerequisites above, each team member connects their own account:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find **OneDrive** and click **Connect**.
3. Sign in with your Microsoft work or school account.
4. Review the requested permissions and click **Accept**.

## Privacy and security

**Your data stays yours.** Grok only accesses OneDrive content when needed to answer your questions. SpaceXSI does not use your OneDrive data for model training.

**Scoped to your account.** OneDrive permissions are delegated to the signed-in user. Grok can only see files in your own OneDrive, not files belonging to other users.

**Data removal on disconnect.** When you disconnect your account, any indexed data associated with it is deleted. If an admin removes the OneDrive connector, all indexed data for the organization is deleted.

## Disconnecting

To disconnect the OneDrive connector:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find OneDrive in your connected list and click **Disconnect**.

You can also revoke the app's access from your Microsoft account at [myapps.microsoft.com](https://myapps.microsoft.com).

