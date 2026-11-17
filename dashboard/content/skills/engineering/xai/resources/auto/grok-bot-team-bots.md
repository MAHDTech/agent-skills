+++
title = "grok-bot-team-bots"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Use Grok Bot

# Team Bots

A Bot is either personal or a Team Bot. Share a Bot the way you'd share a doc:
with your Cursor team, your whole organization, or only certain people and
groups. Sharing doesn't make a copy, and each person's chat with a Team Bot is
private.

Team Bots always run in the cloud. A Bot that runs on your own computer can't
be shared. A few screens, such as **Browse shared Bots**, still say shared
Bots. They're the same thing.

## Who can use Team Bots?

Team Bots are available on Team and Enterprise plans, for members who have Grok
Bot. Choose **+** in
the sidebar to see **Create a Bot** and **Browse shared Bots**. If **Browse
shared Bots** isn't there, you aren't on a Cursor team, or your Grok Bot app
needs an update.

Your team also needs at least one other member. If you're the only person on
it, for example on a team plan nobody else has joined yet, you can't create or
share a Team Bot until a teammate joins. Your personal Bots aren't affected.

In Slack, members of your workspace who aren't on the team can sometimes use a
Team Bot too. See [Who can talk to a Team Bot in
Slack?](#who-can-talk-to-a-team-bot-in-slack)

## How do I create a Bot for my team?

1. Choose **+** in the sidebar, then **Create a Bot**, and give it a name.
2. Tell the Bot what it should help the team with. A sentence is enough.
3. Follow its setup steps for apps, secrets, skills, and files. Only add keys
   and files you're happy for everyone you share it with to use. Until you
   publish it, it's a draft only you can see.
4. When the Bot says it's ready to share, choose **Publish**. Teammates can
   then find it. To change who has access, see [How do I share a Bot with my
   team?](#how-do-i-share-a-bot-with-my-team)

You can also share a Bot you already have. Sharing doesn't duplicate it. Before
you share, check its apps, secrets, skills, and files, since they work in every
teammate's chat.

## How do I share a Bot with my team?

Choose **Share** at the top right of the Bot's chat. In **Share this Bot**:

* Under **Who has access**, choose your team to share the Bot with everyone on
  the team. If your team is part of an organization, choose the organization
  to share it with everyone in it. Choose **People you invite** to share it only with the people
  and groups you add. Limiting it asks you to confirm with **Limit access**.
  The owner and Managers can always use the Bot.

* Search under **Add people or groups** to add someone. People you add can chat
  with the Bot as a **Collaborator**. In the list of people who have access,
  open a person's role and choose **Manager** to let them also change how the
  Bot works and share it with others.

* Choose **Copy link** to send teammates a link to the Bot.

To stop sharing, the owner can choose **Only you** under **Who has access**.
This removes everyone else after you confirm with **Remove**. To remove one
person or group, open their role in the list and choose **Remove**.

## Who can share Bots?

Anyone on the team, unless an admin changes it. On the Grok Bot page of the team
dashboard, **Bot sharing** under **Sharing** is set to **Everyone** or **Admins
only**. With **Admins only**, **Share** is disabled for members and shows **Only
team admins can share Bots**. Bots that were already shared stay shared, and
their owners and Managers can still use them. Ask an admin to share a Bot for
you, or to change the setting. See [How do admins manage Team
Bots?](#how-do-admins-manage-team-bots)

## How do teammates find a Team Bot?

* Choose **+** in the sidebar, then **Browse shared Bots**. The list shows Bots
  shared with you, and each one shows who made it. Choose **Add** to keep a Bot
  in your sidebar.
* In the desktop app, press `Cmd/Ctrl+K` and choose **Browse shared Bots**.
  The list starts with **Popular** Bots, and you can search it. You can also
  type a Team Bot's name and choose it to add it.
* Ask one of your Bots in your own chat, for example "Is there a Team Bot
  for on-call triage?" It searches the Team Bots you can use by what they do,
  and tells you which ones you've already added.
* On your phone, tap **+**, then **Browse shared Bots**.
* Open a link a teammate sent you, then choose **Start a chat**.

A Team Bot's link opens the same Bot for everyone. It's different from
[sharing a Bot template](/grok-bot/bots#share-a-bot), which gives each person
their own copy.

## Can the owner see my chats with a Team Bot?

No. Each person's chat with a Team Bot is private. The owner and Managers
can't read your chats, and you can't read the owner's or anyone else's.

The Bot can learn things for the whole team. What it learns from a teammate
waits until the owner or a Manager approves it under **Review learnings**. It
doesn't save anything about you personally for the team. What it remembers
about working with you stays between you and the Bot. See [What does a Team Bot
learn for the team?](#what-does-a-team-bot-learn-for-the-team)

## What does a Team Bot learn for the team?

A Team Bot keeps team memory and skills that everyone who uses it benefits
from. When a teammate teaches it something for the team, such as a better way
to do a task, the Bot suggests the change instead of saving it straight away.

* Suggested memory and skill changes wait until the owner or a Manager approves
  them. Owners and Managers see **Review learnings**, with the number waiting,
  under **Suggested** in the Bot's details. Each one says what it does, such as
  **Updates the skill** or **Forgets a team memory**, with **Approve** and
  **Reject**.
* Changes the owner or a Manager makes in the Bot's main chat apply right away.

What the Bot learns about you personally stays between you and the Bot. It
isn't saved for the team.

## Whose apps and secrets does a Team Bot use?

A Team Bot uses the apps and secrets its owner added. Those work the same for
everyone who chats with it.

In your own chat with a Team Bot, it can also use your apps. It asks before it
uses one of your personal accounts. The card says **Allow**, then the app name:

* **Allow once** lets it use that account this time.
* **Always allow** remembers your answer. Its menu has **Always allow for your
  messages to this Bot** and **Always allow for your messages to every Team
  Bot**.
* **Skip** means it doesn't use that account this time.

In Slack channels, threads, and group chats, a Team Bot uses only its owner's
apps, and asks before it uses your accounts.

A connected account acts as the person the Bot is answering. One teammate's
connection is never used for someone else, unless a Manager shares it as a
shared connector.

To make every Team Bot ask again, open **Settings** > **General** > **Team
Bots** and choose **Clear** next to **Clear connector preferences**.

## How do shared connectors work?

A shared connector lets everyone use an app through one account, so teammates
don't each have to connect it. The owner or a Manager sets it up: open the
Bot's details and, under **Setup**, choose **Service accounts**. Pick an app
under **Shared connectors**, then choose how the Bot signs in to it:

* **Use individual accounts**: each teammate connects their own.
* **Share your account**: everyone who uses the Bot acts through your account,
  including to make changes. Connect the app for yourself first, and only share
  an account that holds nothing sensitive or private.
* The Bot's own account: a separate account the Bot uses for the team. Some
  apps don't support this yet.

Sharing your account needs a team admin to turn on **Allow Shared Logins**
first. Until then, the Bot says **Your admin has turned off sharing logins with
Bots**. See [How do admins manage Team Bots?](#how-do-admins-manage-team-bots)

The app's row shows whose account the Bot uses. To stop, choose **Stop
sharing**, or remove the app. Teammates then connect their own. If a shared app
needs reconnecting, only the Manager whose account it is can reconnect it,
under the Bot's **Shared connectors**.

## Which computer does a Team Bot use?

In your own chat with a teammate's Team Bot, the Bot usually works on your
Grok Bot computer. That means it can use the files there and the sites you're
signed in to there. Sometimes it uses a separate computer just for that
conversation, for example when your privacy mode is stricter than the owner's.

When you DM a Team Bot in Slack, it also works on your own computer. In Slack
channels, threads, group DMs, and group chats, a Team Bot uses one shared
computer of its own.

## Whose usage pays for a Team Bot chat?

A Team Bot reply uses the Grok Bot usage of the person it's answering. That's
true in your own chat, in Slack, and in group chats. Your own limits apply. The
owner's usage pays for the owner's own messages, and for Slack posts from apps
or workflows that aren't linked to a Cursor account. A reply to someone in
Slack who hasn't linked an account or isn't on the team uses the team's usage.
See [How much does Grok Bot
cost?](/grok-bot/faq#how-much-does-grok-bot-cost)

## How do approvals work on a Team Bot?

A Team Bot asks the person it's answering before risky actions.

* In your own chat with a Team Bot, you answer its approval cards.
* In a Slack thread, the Bot posts a **Review an action** card with **Allow
  once**, **Deny**, and **View details**. Only the person who asked can answer
  it.
* In a group chat, only the person whose message the Bot is answering can
  answer.

It also asks before it uses your personal accounts. See [Whose apps and secrets
does a Team Bot use?](#whose-apps-and-secrets-does-a-team-bot-use) For the
rest of the approval model, see [Approvals, security, and
privacy](/grok-bot/approvals-security-and-privacy).

## How do I add a Team Bot to Slack?

The owner or a Manager adds it, after sharing:

1. Open the Bot's details, or choose **Manage** and look under **Channels**.
2. Find **Bring to your team's Slack** or **Slack**, and choose **Connect**.
3. If it says **Awaiting admin approval**, a Slack workspace admin needs to
   approve the app.

Once it says **Connected**, teammates can message the Bot directly or mention it
in channels.

In a channel thread or a DM, the Team Bot acts for the teammate who mentioned
it. It can hand work to that person's own Bots, and their replies are posted
back into the same thread. A personal Bot doesn't hand work to other Bots from a
channel thread. See [Let Bots hand work
off](/grok-bot/chat-and-collaboration#let-bots-hand-work-off).

Bots record who asked for each message they post in Slack, so anyone in the
thread can ask the Bot who requested it. When a Team Bot's routine replies in a
Slack thread, the Bot follows that thread and answers later replies without an
@mention.

Link your Slack account so the Bot knows who you are and can use your apps. If
you message a Bot before you link, it asks you to connect:

1. Choose **Connect**. Your browser opens cursor.com.
2. Sign in with the Cursor account on the Bot's team, then choose **Link
   Account**. If you're already signed in, you go straight to that page.
3. Choose **Open Slack** to return to Slack. The Bot answers the message you
   sent.

You link once per Slack workspace. The link covers every Bot in that workspace,
and Cloud Agents you start from Slack. To link before a Bot asks, open
**Settings** > **General** > **Team Bots** and choose **Link**, or choose Slack
under [Integrations](https://cursor.com/dashboard/integrations?view=slack-management)
in the Cursor dashboard. There, a workspace marked **Connection expired** needs
**Reconnect**, and a workspace that isn't listed needs **Add to another
workspace**, or **Install Cursor in Slack** if none are linked.

## Who can talk to a Team Bot in Slack?

It depends on who the Bot is shared with.

* **Your organization**: any full member of your Slack workspace can mention it
  in channels and threads, even without a linked account or a seat on your
  team. Without a linked account, they're told once per thread that **some
  features will be limited**. In a DM, they still need to link first. The same
  goes for a Bot shared with your team when the team isn't in an organization,
  or is the only team in it.
* **Your team, or certain people**: only people it's shared with. A linked
  teammate without access is told the Bot **hasn't been shared with you yet**.
  Add them under **Share this Bot**. Someone without a linked account is asked
  to link one.
* **A personal Bot** only chats with its owner.

Slack guests, Slack Connect channels, and other Slack workspaces are never
included. If your Slack email matches an account that already has access, the
Bot asks you to link it instead.

A team admin can turn this off for workspace members who aren't on the team.
See [How do admins manage Team Bots?](#how-do-admins-manage-team-bots)

## How do I add a Team Bot to Microsoft Teams?

A team admin turns on Microsoft Teams for the team first. Then the owner or a
Manager connects the Bot from its details under **Bring to your team's
Microsoft Teams**, and a Microsoft Teams admin approves it. The full steps,
what the Bot can do there, and what to do when it isn't in Teams yet are on
[Use Grok Bot in Microsoft Teams](/grok-bot/microsoft-teams).

## How do I edit or delete a Team Bot?

The owner and the Bot's Managers can change a Team Bot. Its details show in
the panel on the right of the chat:

* **Plugins** (the Bot's connected apps), **Secrets**, **Skills**, and
  **Files** hold what the Bot uses for everyone.
* Click the name to rename the Bot. Edit the description under **About**.
* **Share** controls who can use the Bot and who manages it.

Choose the wrench, **Manage**, for the rest:

* **General** has the Bot's settings, such as whether Collaborators can set up
  routines, its channels such as Slack and Microsoft Teams, and the **Bot's
  accounts** it signs in to apps with.
* **Knowledge** shows what the Bot learned for everyone, and its skills. New
  learnings wait under **To review** until you choose **Approve** or
  **Reject**.
* **Analytics**, when shown, has the Bot's messages, hours saved, and top users.

The owner can also rename the Bot by right-clicking it in the sidebar and
choosing **Rename Bot**.

Only the owner can delete a Team Bot. Right-click the Bot in the sidebar and
choose **Delete**. A team admin can also delete it from **Manage Bots** in the
dashboard. This deletes the Bot for your whole team. Teammates lose it,
along with their chats and routines on it. It can't be undone.

## How do I remove a Team Bot from my sidebar?

Right-click the Bot and choose **Remove from sidebar**. To add it back, choose
**+** in the sidebar, then **Browse shared Bots**, and choose **Add** next to
it.

If your admin added the Bot to your sidebar for you, **Remove from sidebar**
isn't in its menu, and you can't remove it.

## Can I use Team Bots on my phone?

Yes. Team Bots you've added appear in the [phone app](/grok-bot/mobile). To
find another, tap **+**, then **Browse shared Bots**. Each Bot in the list
shows how many people use it, and **Add** keeps it in your list. You can chat
with Team Bots and answer their **Allow** cards. A Team Bot can't be your
[primary Bot](/grok-bot/bots#your-primary-bot).

Sharing and browsing on the phone need Grok Bot 1.19.0 or later on iPhone and
Android. Earlier versions still open the Team Bots already in your list.

To remove a teammate's Bot from your list, touch and hold it and choose
**Remove from sidebar**.

The owner and Managers can also manage a Team Bot from its profile on the
phone:

* **Share** opens **Share this Bot**, where you choose who has access and add
  people. If you're on more than one team, choose a team before you share.
* **Tools** has the Bot's service accounts, and **Slack** adds the Bot to Slack
  with your own Slack connection. See [How do I add a Team Bot to
  Slack?](#how-do-i-add-a-team-bot-to-slack)
* **Skills** lists the Bot's skills. Tap **+** to choose **Add Custom Skill**
  or **Import Existing Skill**, and open a skill to edit it and choose **Save
  changes**.
* **Learnings** shows how many learnings are waiting, for example "3 to
  review". Choose **Approve** or **Reject** on each one.
* **Routines** lists the Bot's routines, with **Add routine** and the
  **Collaborators can create routines** switch.
* **Analytics**, when shown, has weekly and daily active users, messages, hours
  saved, and top users, over **7 days**, **30 days**, or **90 days**.

When you ask one of your own Bots on the phone to work with your team, it shows
a **Team access** card. **Share** moves that Bot to the team with its chat and
memories, or **Copy** makes a team copy and keeps the original private. **Start
fresh** creates a new Team Bot without this chat or its memories.

## Can I voice chat with a teammate's Team Bot?

Yes, on desktop. In your own chat with the Bot, choose **Start voice chat**. See
[Start a voice chat](/grok-bot/chat-and-collaboration#start-a-voice-chat).

## Can a Team Bot join a group chat?

Yes. You can add a Bot a teammate shared with you to your own group chat. It
can't share a group with Bots that run on your own computer. See [Start a group
chat](/grok-bot/chat-and-collaboration#start-a-group-chat).

## How do admins manage Team Bots?

On the Grok Bot page of the [Cursor dashboard](https://cursor.com/dashboard/bot),
open **Manage Bots** under **Shared Bots**.

* **Access** shows who can use each Team Bot. Click it to change who can use
  the Bot.
* **Auto-add** puts the Bot in members' sidebars automatically. Pick **All
  team** or one or more groups. Those members can't hide it. Auto-add only
  applies to groups that have access.
* Admins can also delete a Team Bot here.

Under **Sharing** on the same page:

* **Bot sharing** decides who may share Bots: **Everyone** or **Admins only**.
  Bots already shared stay shared.
* **Allow Shared Logins** lets Managers share their own app logins as [shared
  connectors](#how-do-shared-connectors-work). Everyone who uses the Bot then
  acts through that Manager's account. It's off by default.
* **Slack Workspace Access** lets members of your Slack workspace who aren't on
  the team use Bots shared with your organization. It's on by default, and in
  an organization it applies to every team. The setting appears as it reaches
  your team.

See [Grok Bot for teams and
enterprises](/grok-bot/teams-and-enterprises#manage-bots).

## Why can't I see or use a Team Bot?

| What you see | What it means |
| --- | --- |
| **Browse shared Bots** isn't under **+** | You aren't on a Cursor team, or your Grok Bot app needs an update. |
| **Team Bots Not Available** | Your account doesn't have access to Team Bots. |
| **Only team admins can share Bots** | Your team admin limited Bot sharing to admins. **Share** is disabled, and the message appears when you hover it. Ask an admin to share the Bot for you, or to change **Bot sharing** on the team dashboard. |
| **Bot Not Found** | The Bot may have been removed, or it isn't shared with you. |
| **No Access to This Bot** | The Bot is shared only with certain people. Ask its owner or a Manager for access. |
| A teammate's Bot isn't in **Browse shared Bots** | The owner hasn't shared it with you yet. |
| **Your usage limit has been reached** | Your Grok Bot usage ran out. Your chats with a teammate's Team Bot use your usage. See [How much does Grok Bot cost?](/grok-bot/faq#how-much-does-grok-bot-cost) |
| **This Bot's owner has reached their usage limit** | This reply was billed to the owner's account, and the owner's usage ran out. Ask the owner to check their plan. |
| **This conversation can't run on your computer because your privacy mode is stricter than the bot owner's.** | Your privacy mode setting is stricter than the owner's, so this chat can't use your computer. |
| In Slack, the Bot asks you to link a Cursor account | Link your Slack account under **Settings** > **General** > **Team Bots**, using the Cursor account on the Bot's team. |
| In Slack, the Bot still asks you to connect after you linked | You signed in with a different Cursor account, or linked a different Slack workspace. Unlink under **Settings** > **General** > **Team Bots**, then choose **Connect** again with the account on the Bot's team. |
| In Slack, **some features will be limited** | The Bot answered you without a linked account. Link one so it knows who you are and can use your apps. |
| In Slack, **I haven't been shared with you yet** | Ask the Bot's owner or a Manager to add you under **Share this Bot**. |
| In Slack, the Bot says it's someone's personal Bot | Only its owner can talk to it. |
| **Your admin has turned off sharing logins with Bots** | A team admin needs to turn on **Allow Shared Logins** before a Manager can share an account as a shared connector. |

If none of these match, [contact support](/grok-bot/troubleshooting) with the
Bot's name, the exact message, and your Grok Bot version.

## Related pages

* [Use Grok Bot in Microsoft Teams](/grok-bot/microsoft-teams)
* [Add a Bot to your meetings](/grok-bot/meetings)
* [Create and manage Bots](/grok-bot/bots)
* [Message and collaborate](/grok-bot/chat-and-collaboration)
* [Use the computer and apps](/grok-bot/computer-and-apps)
* [Vault and secure sign-in](/grok-bot/vault)
* [Approvals, security, and privacy](/grok-bot/approvals-security-and-privacy)
* [Grok Bot for teams and enterprises](/grok-bot/teams-and-enterprises)
* [Troubleshooting](/grok-bot/troubleshooting)


