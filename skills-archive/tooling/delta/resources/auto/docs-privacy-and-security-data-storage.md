---
title: "Data Storage & Deletion"
url: "https://delta.dev/docs/privacy-and-security/data-storage"
description: "What DeltaDB stores on your behalf and where it lives."
---

1.  Privacy & Security
2.  Data Storage & Deletion

# Data Storage & Deletion

What DeltaDB stores on your behalf and where it lives.

On this page

1.  [What Delta stores](#what-delta-stores)
2.  [Where your data lives](#where-your-data-lives)
    1.  [Encryption](#encryption)
    2.  [Subprocessors](#subprocessors)
3.  [Data retention and deletion](#data-retention-and-deletion)
    1.  [Delete your account and data](#delete-your-account-and-data)
    2.  [Delete individual threads](#delete-individual-threads)
    3.  [What deletion does not cover](#what-deletion-does-not-cover)
    4.  [Retention](#retention)

When you add a repository or create a thread, Delta uses [DeltaDB](/docs/concepts/delta-and-git#what-is-deltadb) to store its contents and history on your machine and on our servers. This keeps your work available across machines and makes it possible to share a thread with teammates. Storing a thread on our servers does not share it. Threads stay private to you unless you share them or set **Default Visibility** to **Your organization** in **Settings > Thread > Sharing**.

Using the agent also sends your conversation, including any code it quotes, through Zed Cloud to your chosen model provider. Learn more about [what is sent to model providers](/docs/privacy-and-security/privacy#ai-requests).

## [What Delta stores](#what-delta-stores)

Delta stores these categories of data on its servers:

*   **Repository contents and history.** The Git objects for repositories attached to your threads, including commits and file contents.
*   **The thread’s deltas.** A delta is a recorded change to a thread or [Delta worktree](/docs/concepts/worktrees), such as a file edit, a change to the file tree, a message, or a comment. Delta stores these changes in sequence so it can reconstruct each thread and keep copies synchronized across machines.
*   **Metadata.** Delta stores:
    *   The shareable thread record, including the key needed to join the thread and its display name.
    *   Your account’s thread catalog, including which uploaded or joined threads are pinned, archived, or active.
    *   The Git remote URL you configured, the derived repository name, and a normalized lookup key. Avoid remotes that embed credentials.
    *   Each collaborator’s GitHub login when a thread is shared.
    *   Git commit author and committer identities and commit messages.
*   **Account settings.** Your application settings, including model provider API keys and subagent profiles, sync between devices. Each device keeps an encrypted local cache. Environment variables you add for Cloud Runners are stored here.
*   **Cloud runner sandboxes.** If you use [Cloud Runners](/docs/agents/cloud-runners) (private beta), each runner’s sandbox holds checkouts of the thread’s Delta worktrees and anything the agent or your setup script creates there. Archiving or deleting the thread deletes your sandbox for it.
*   **Subscription credentials.** Zed Cloud stores the OAuth grants for ChatGPT and Grok subscriptions and provides Delta with short-lived access tokens. GitHub Copilot credentials remain in your operating system keychain.

> **TODO:** Cloud runner sandbox provider, region, and retention for paused sandboxes.

## [Where your data lives](#where-your-data-lives)

Delta’s backend runs entirely on Cloudflare.

Data

Storage

File contents, Git commits, history packs, and checkpoints

Cloudflare R2

Thread and Delta worktree deltas used for active synchronization

Cloudflare Durable Objects backed by SQLite

Account settings

Cloudflare Durable Objects backed by SQLite

Thread, repository, collaborator, and account metadata

Cloudflare KV and D1

Each thread’s history is stored under its own unique ID. Delta uses the Git remote URL to identify a repository within your organization. Organization members working from the same remote use the same stored repository data. Access to stored repository data comes from direct repository membership, organization membership when the repository is visible to the organization, or access to a thread with an attached Delta worktree from the repository.

Sharing a thread grants access to its attached repositories. Delta worktree history access is repository-wide, so this also grants access to every Delta worktree history for each attached repository, including histories that are not attached to the shared thread. Removing every Delta worktree for a repository from the thread removes the access granted by that thread.

### [Encryption](#encryption)

Data stored by Delta is encrypted at rest. See [Encryption](/docs/privacy-and-security/security#encryption) for details.

### [Subprocessors](#subprocessors)

Delta uses Cloudflare as an additional subprocessor to store and synchronize repository and thread data. Cloudflare processes this data globally. Delta also uses the services on [Zed’s subprocessor list](https://zed.dev/subprocessors).

## [Data retention and deletion](#data-retention-and-deletion)

You can request deletion of your Delta account and personal data. Account deletion is not available directly in Delta; follow the email instructions below.

### [Delete your account and data](#delete-your-account-and-data)

To request deletion, email [privacy@zed.dev](mailto:privacy@zed.dev?subject=Privacy%20Request) from the email address associated with your account. We verify your identity, delete your data, and confirm when deletion is complete.

Account deletion removes:

*   Your account, credentials, and active sessions.
*   Your personal threads.
*   Delta server data controlled solely by you.

Deleting your Zed account also deletes your Delta account and associated data.

### [Delete individual threads](#delete-individual-threads)

You can delete a thread with **Delete Locally** in the thread list’s right-click menu, or with the `delta cli thread delete` and `delta cli thread purge` commands (`purge` only prints what would be lost until you pass `--force`; both require Delta to be closed). **Delete Locally** removes the thread’s local copy on this computer. It does not reliably remove copies on your other devices or delete the thread’s history from Delta’s servers. Account deletion removes all your personal threads together.

If you have a concern about the contents of a specific thread, email [privacy@zed.dev](mailto:privacy@zed.dev?subject=Privacy%20Request) and we will review it.

### [What deletion does not cover](#what-deletion-does-not-cover)

Delta controls its server storage, but it does not control every location your data may reach:

*   **Upstream repositories.** Deleting Delta data does not affect the canonical repository on GitHub or another host.
*   **Local copies.** Delta cannot delete clones, checkouts, or thread copies on other people’s machines. On your own machine, Delta removes the local history of an archived, synced thread three days after you archive or last open it; the server copy remains and downloads again when you open the thread.
*   **Already-shared content.** Deleting Delta’s server-side copy cannot retract content someone else has already downloaded or exported.
*   **Shared content.** Content you contributed to shared threads may be retained. We do not rewrite thread or git history, so your name may remain in retained shared content, such as commit history or thread state. If you have a privacy request about identifying information in retained content, email [privacy@zed.dev](mailto:privacy@zed.dev?subject=Privacy%20Request) and we will review it separately.

### [Retention](#retention)

We may retain some data where legally required. Deleted data may also remain temporarily in backups before it is fully purged.

Locally, Delta keeps the full history of your active threads. Archived threads that are synced to our servers drop their local history after three days to free disk space; opening one downloads its history again. Threads that were never synced keep their local history until you delete them.