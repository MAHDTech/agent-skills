#### Connectors

# Microsoft Teams

The Microsoft Teams connector gives Grok full access to your Teams conversations. Search across channels and chats, read messages, send new messages, reply to threads, and create chats — all without leaving Grok.

## Capabilities

* **Search messages** across all your channels and chats by keyword.
* **Read channel messages** including threaded replies, reactions, and @mentions.
* **Read chat messages** from one-on-one and group chats.
* **Send channel messages** and reply to existing message threads.
* **Send chat messages** in one-on-one and group chats.
* **Create chats** to start new one-on-one or group conversations.
* **Browse teams and channels** to discover what conversations are happening.
* **View team and channel members** including roles and membership types.

## Required permissions

| Scope | Purpose |
|---|---|
| `Team.ReadBasic.All` | List the teams the user belongs to |
| `Channel.ReadBasic.All` | List channels within those teams |
| `ChannelMessage.Read.All` | Read messages in channels the user has access to |
| `ChannelMessage.Send` | Send messages and replies in channels |
| `ChannelMember.Read.All` | View channel membership |
| `TeamMember.Read.All` | View team membership |
| `Chat.Read` | Read one-on-one and group chat messages |
| `Chat.Create` | Create new one-on-one and group chats |
| `ChatMessage.Send` | Send messages in chats |
| `User.Read` | Read the signed-in user's profile |
| `offline_access` | Maintain access without repeated sign-in prompts |

These are delegated permissions. Grok can only access teams, channels, and chats that the signed-in user already has access to.

## How to connect

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Click **New Connector** and select **Microsoft Teams**.
3. Sign in with your Microsoft work or school account.
4. Review the requested permissions and click **Accept**.

Once connected, Grok can search, read, and send messages in your Teams conversations whenever your questions or requests relate to Teams.

## Admin consent

Some organizations require an Azure AD administrator to approve application permissions before users can sign in. If you see a "need admin approval" error, contact your IT administrator and ask them to grant consent for the xAI Grok application in the Azure AD admin portal under **Enterprise applications**.

## Privacy and security

**We do not train on your data.** SpaceXSI does not use your Microsoft Teams data for model training.

**Nothing is stored.** Conversations that use your Teams connector do not result in any of your Teams data being stored on SpaceXSI servers. Grok accesses your data in real time when you ask a question, and does not retain it afterward.

**Scoped to your account.** Teams permissions are delegated to the signed-in user. Grok can only access teams, channels, and chats you are already a member of.

**You control access.** Disconnect at any time to immediately revoke Grok's access to your Microsoft account.

## Disconnecting

To disconnect:

1. Go to [grok.com/connectors](https://grok.com/connectors).
2. Find the Microsoft Teams connector and click **Disconnect**.

You can also revoke the app's access from your Microsoft account at [myapps.microsoft.com](https://myapps.microsoft.com).

