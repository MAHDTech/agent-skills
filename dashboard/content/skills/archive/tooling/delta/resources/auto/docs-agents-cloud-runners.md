+++
title = "docs-agents-cloud-runners"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Cloud Runners"
url: "https://delta.dev/docs/agents/cloud-runners"
description: "Run a thread's agent turns in a cloud sandbox instead of on your computer."
---

1.  Using Delta
2.  Cloud Runners

# Cloud Runners

Run a thread's agent turns in a cloud sandbox instead of on your computer.

On this page

1.  [Run a thread on a cloud runner](#run-a-thread-on-a-cloud-runner)
2.  [Syncing with a cloud runner](#syncing-with-a-cloud-runner)
3.  [The runner environment](#the-runner-environment)
4.  [Set up your repository](#set-up-your-repository)
    1.  [How .agents/setup relates to .agents/prepare](#how-agentssetup-relates-to-agentsprepare)
5.  [Add environment variables and secrets](#add-environment-variables-and-secrets)
6.  [Models and MCP servers](#models-and-mcp-servers)
7.  [Preview a running app](#preview-a-running-app)
    1.  [Who can open a preview URL](#who-can-open-a-preview-url)
    2.  [Save your project’s startup steps in a skill](#save-your-projects-startup-steps-in-a-skill)
    3.  [Connect a frontend to an API](#connect-a-frontend-to-an-api)
    4.  [Sign in to a development app](#sign-in-to-a-development-app)
8.  [Use Git and GitHub from a runner](#use-git-and-github-from-a-runner)
9.  [When a runner stops](#when-a-runner-stops)
10.  [Troubleshooting](#troubleshooting)
     1.  [There is no runner menu](#there-is-no-runner-menu)
     2.  [The runner failed to start](#the-runner-failed-to-start)
     3.  [The setup script failed](#the-setup-script-failed)
     4.  [Delta warns that runners can’t push to GitHub](#delta-warns-that-runners-cant-push-to-github)
     5.  [A push from the runner fails](#a-push-from-the-runner-fails)
     6.  [A command can’t find a credential](#a-command-cant-find-a-credential)
11.  [What’s next](#whats-next)

IMPORTANT

Cloud Runners are in private beta. To request access, fill out the [beta signup form](https://forms.gle/ftN2qeFE2LE2tR4GA).

A cloud runner is a Linux sandbox that runs the agent’s turns for a thread on your behalf. Use one when a task needs a shell but shouldn’t depend on your computer: the agent keeps working while your laptop sleeps, and you can start turns that run commands from Delta on the web.

## [Run a thread on a cloud runner](#run-a-thread-on-a-cloud-runner)

1.  Open the thread and click the runner button in the composer, between the model selector and the send button. Its tooltip reads **Change Where Turns Run**. You can also press `Cmd+Alt+.` (macOS) or `Ctrl+Alt+.` (Linux/Windows).
2.  Choose **Cloud**. Delta starts a runner for the thread.
3.  Send your message.

While the runner starts, the runner button shows **Sending…**, **Waiting…**, and **Preparing…**. The transcript shows the runner’s progress, including your [setup script](#set-up-your-repository) if the repository has one.

Your choice applies to your own turns in this thread. Each participant chooses separately, and each person who chooses Cloud gets a distinct runner that uses their own settings.

To run turns on your computer again, choose **Local Machine**. If the runner is still starting, Delta cancels it.

New threads you start on the same device use the option you chose last, so after you choose **Cloud** once, new threads start on a runner too.

If the runner button shows **Local on another device**, your turns in this thread are set to run on a different computer where you use Delta.

## [Syncing with a cloud runner](#syncing-with-a-cloud-runner)

The runner uses your thread and Delta worktrees synced to Delta’s servers, not your local checkout. Most threads are already uploaded. If yours isn’t, Delta uploads it using **Settings > Thread > Sharing > Default Visibility**: **Private** keeps it accessible only to you, while **Your organization** lets members of your active organization access it. An already uploaded thread keeps its existing access settings. Choosing Cloud doesn’t copy a share link. See [Collaborate in a Thread](/docs/collaboration/collaborate-thread).

The runner checks out each attached repository from Delta’s servers. It can’t reach anything that exists only on your computer, such as ignored files, [`.agents/linked`](/docs/configuration/project-setup#share-local-files-with-agentslinked) files, local services, or credentials. Changes the agent makes sync back to your mounted checkouts through the Delta worktree. See [Review & Sync Changes](/docs/agents/review-and-sync) to bring them into your repository.

## [The runner environment](#the-runner-environment)

Each runner is a Linux x86-64 sandbox running Debian Bookworm. Choose its size in the desktop app under **Settings > Cloud Runner > Preferred Runner Size**:

*   **Small:** 2 CPUs, 4 GB of RAM, and 15 GB of free disk.
*   **Medium** (default): 4 CPUs, 8 GB of RAM, and 30 GB of free disk.
*   **Large:** 8 CPUs, 16 GB of RAM, and 50 GB of free disk.

The size applies to runners started after you change it, including runners you start from the browser. A thread that already has a runner keeps its size.

## [Set up your repository](#set-up-your-repository)

Create an executable `.agents/setup` file at your repository root and commit it. When a runner starts, it runs the script from the checkout root before it runs the first turn. The transcript shows **Running .agents/setup…**, then **Ready**.

Cloud runner machines are snapshotted after running `.agents/setup`, so that your future runners can launch faster.

For example, this script installs a pnpm project’s dependencies.

```
#!/bin/sh
set -eu

corepack enable
pnpm install --frozen-lockfile
```

Make it executable from your repository root:

```
chmod +x .agents/setup
```

Keep the script non-interactive and safe to run more than once. Your [environment variables](#add-environment-variables-and-secrets) are available to it.

If the script exits with an error, the transcript shows **Failed to run .agents/setup**. The agent is told about the failure and can read the script’s output, so you can ask it to debug the script.

To use a different script only in Delta, add `.delta/setup`. It replaces `.agents/setup` when present, the same way other [Delta-specific overrides](/docs/configuration/project-setup#delta-specific-overrides) work.

### [How `.agents/setup` relates to `.agents/prepare`](#how-agentssetup-relates-to-agentsprepare)

If your repository has an [`.agents/prepare`](/docs/configuration/project-setup#run-setup-in-each-checkout) script, the runner also runs it in each checkout it creates, before `.agents/setup`. Use `.agents/prepare` for fast per-checkout work that should happen everywhere, such as configuring secrets. Use `.agents/setup` for slower dependencies that the runner needs, such as installing dependencies and toolchains.

## [Add environment variables and secrets](#add-environment-variables-and-secrets)

In the desktop app, open **Settings** with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows), then select **Cloud Runner**. Add the variables your setup script, tests, or tools need:

1.  Enter a name and value under **Add Environment Variable**.
2.  Click **Add Variable**.

To add several at once, click **Import** under **Import Variables from File** and choose a `.env` file. Adding a name that already exists replaces its value.

Delta stores the values in your encrypted account settings and doesn’t show them again after you save them. Runners read them from your settings, and Delta redacts their values from the transcript and terminal output. See [Secret redaction](/docs/privacy-and-security/security#secret-redaction).

Names must start with a letter or underscore and contain only letters, digits, and underscores.

A `GITHUB_TOKEN` or `GH_TOKEN` variable also changes how runners reach GitHub. See [Use Git and GitHub from a runner](#use-git-and-github-from-a-runner).

## [Models and MCP servers](#models-and-mcp-servers)

A runner uses the providers connected in your account settings: Delta-hosted models, connected subscriptions such as ChatGPT or Grok, and API keys or custom providers added in **Settings > LLM Providers**. See [Models & Providers](/docs/agents/models-and-providers).

Remote [MCP servers](/docs/agents/mcp) you connected in **Settings > MCP** are also available. A runner connects to them when it starts. If discovery is slow, the first turn can start without some MCP tools; they’re available from the next turn.

## [Preview a running app](#preview-a-running-app)

Ask the agent to start your development server and give you a preview URL. Agents on cloud runners know how to do this: the agent starts the server, checks that it responds, and replies with an HTTPS URL you can open in your browser. Any port works except 49983, which the runner reserves.

If the server keeps the terminal busy, the agent asks you to move that terminal to the background, because agents can’t start background terminals themselves. See [Run a terminal in the background](/docs/agents/terminals#run-a-terminal-in-the-background).

### [Who can open a preview URL](#who-can-open-a-preview-url)

Preview URLs aren’t public. To open one, visitors sign in to Delta and need access to the thread, as described in [Choose who can access a thread](/docs/collaboration/collaborate-thread#choose-who-can-access-a-thread). For a thread shared with **Anyone with the link**, the preview URL is enough; visitors don’t also need the thread’s link. If you lose access to the thread, your runner’s previews stop working for everyone.

Access is controlled per-runner, so a collaborator can access any local port of your runner. Only share threads with collaborators you trust.

### [Save your project’s startup steps in a skill](#save-your-projects-startup-steps-in-a-skill)

The built-in instructions don’t know how your project starts. To save the agent from working it out each time, put your startup steps in a [project skill](/docs/agents/skills#where-skills-live). In a skill or script, get a service’s URL with `delta-runner preview url \<port\>`. For example, in a repository that uses pnpm and Astro, create `.agents/skills/start-preview/SKILL.md`:

```
---
name: start-preview
description: Start an Astro site and return its Cloud Runner preview URL.
---

From the repository root on the cloud runner:

1. Run `pnpm exec astro dev --background --port 3000`.
   If the port is occupied, reuse the server only if it belongs to this project.
   Do not stop an unrelated process.
2. Check that `curl -fsS -o /dev/null http://127.0.0.1:3000/`
   succeeds. If it fails, check the server logs and report the error.
3. Run `delta-runner preview url 3000` and give me the exact URL it prints.
```

Send `/start-preview` in a thread that runs on a cloud runner, adjusting the command and port for your project. In this example, `--background` keeps Astro running after the command exits. For other servers, use their equivalent background mode.

If a service needs another service’s preview URL, pass the output of `delta-runner preview url` to it as an environment variable. For example, add this step to `start-preview` to start a second service too:

```
app_url="$(delta-runner preview url 3000)" &&
PUBLIC_APP_URL="$app_url" pnpm --filter docs dev --background
```

### [Connect a frontend to an API](#connect-a-frontend-to-an-api)

Each port has its own preview hostname, so requests from a frontend’s preview to an API’s preview are cross-origin. They can fail CORS preflight or preview sign-in, and cookies aren’t shared between them. Instead, send browser requests to a path on the frontend’s origin, such as `/api`, and have the frontend’s dev server proxy them to the API’s local port.

For example, with Vite, set `server.proxy` in `vite.config.ts` to your API’s local address:

```
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8792",
      },
    },
  },
});
```

### [Sign in to a development app](#sign-in-to-a-development-app)

Signing in to Delta doesn’t sign you in to your app. If your app uses OAuth, a callback registered for `localhost` won’t return to the preview hostname, and preview hostnames differ for each thread and person. Instead, add a development-only sign-in route, for example:

```
/__dev/sign-me-in/dev@example.com?returnTo=/dashboard
```

Open it on your app’s preview URL. The route should sign in a test account with your app’s normal session cookie, then redirect to `returnTo` only if it’s a same-origin path such as `/dashboard`. Disable it outside development.

## [Use Git and GitHub from a runner](#use-git-and-github-from-a-runner)

The runner gets your code from Delta, not from GitHub, so the agent can read, edit, and run your code without GitHub access. You can push its changes from your own checkout as usual.

To let the agent fetch, push, or open pull requests from the runner, connect GitHub in the desktop app or in your browser:

1.  Open **Settings** with `Cmd+,` (macOS) or `Ctrl+,` (Linux/Windows), then select **Integrations**.
2.  Under **GitHub Account Access**, click **Authorize as @your-login**. Use the GitHub account you sign in to Delta with.
3.  If the thread warns that **Cloud Runners can’t push to** a repository, click **Set up GitHub access** and install the Delta GitHub App on the account or organization that owns it. See [Delta warns that runners can’t push to GitHub](#delta-warns-that-runners-cant-push-to-github).

Git commands and `gh` on the runner then work as you, and commits use your name and email.

To use a personal access token instead, add it as a `GITHUB_TOKEN` or `GH_TOKEN` [environment variable](#add-environment-variables-and-secrets). Runners use your token instead of the GitHub App for both `git` and `gh`.

Runners don’t have an SSH key. Each checkout’s `origin` uses the repository’s HTTPS URL.

> **Note:** Don’t run `gh auth setup-git` in `.agents/setup`. It replaces Delta’s Git credential helper, and pushes from the runner stop working.

## [When a runner stops](#when-a-runner-stops)

Cloud Runners are ephemeral resources. They typically are created when the Thread is created on or moved to a Cloud Runner, and deleted when the Thread is archived.

Deleting a thread also deletes your runner for it. Other participants’ runners for the thread aren’t affected.

That said, we may delete and reprovision Cloud Runners at any time - for example if we release a new version of Delta that is incompatible with the version installed on the Runner.

Cloud Runners are automatically paused after 10 minutes of inactivity. If you run a turn on a paused Cloud Runner, it will be resumed automatically.

## [Troubleshooting](#troubleshooting)

### [There is no runner menu](#there-is-no-runner-menu)

Cloud Runners are in private beta. To request access, fill out the [beta signup form](https://forms.gle/ftN2qeFE2LE2tR4GA).

### [The runner failed to start](#the-runner-failed-to-start)

The transcript shows why the runner couldn’t start. Everyone in the thread sees the same explanation, including after reopening the thread.

### [The setup script failed](#the-setup-script-failed)

Open the failed setup step in the transcript to read its output, or ask the agent to debug `.agents/setup`. Check that the file is executable, committed, and doesn’t depend on files or tools that exist only on your computer.

### [Delta warns that runners can’t push to GitHub](#delta-warns-that-runners-cant-push-to-github)

For a thread set to run on a cloud runner, Delta checks whether the runner can push to each of the thread’s GitHub repositories, in the desktop app and in the browser. It checks again when you attach another repository or return to Delta after setting up GitHub. If there’s a problem, the thread shows a warning:

*   **Authorize GitHub so Cloud Runners can push as you:** click **Authorize** and sign in with the GitHub account you use for Delta. If the warning says **Reauthorize**, your authorization expired or was revoked.
*   **Cloud Runners can’t push to** followed by repository names: the Delta GitHub App doesn’t give your account access to those repositories, or the access is read-only. Click **Set up GitHub access** to install the app on the account or organization that owns them. If the warning asks you to contact an owner or admin of your Delta organization instead, you can’t install the app yourself. For read-only access, also check that your GitHub account can push to the repository.
*   **Couldn’t check GitHub access:** click **Try again**.

The warning doesn’t stop the runner. To hide it, click its close button. It appears again when you reopen the thread or the problem changes.

### [A push from the runner fails](#a-push-from-the-runner-fails)

Open **Settings > Integrations**. If **GitHub Account Access** shows **Authorization expired**, click **Reauthorize**. If you added `GITHUB_TOKEN` or `GH_TOKEN` as an environment variable, runners use that token instead of the GitHub App, so check that it is current and can push to the repository.

### [A command can’t find a credential](#a-command-cant-find-a-credential)

Runners don’t have your local environment. Add the variable in **Settings > Cloud Runner**.

## [What’s next](#whats-next)

*   [Prepare your project](/docs/configuration/project-setup) covers `.agents/prepare` and the other project configuration files.
*   [Collaborate in a Thread](/docs/collaboration/collaborate-thread#where-the-agent-runs) explains how execution environments differ between participants.
*   [Review & Sync Changes](/docs/agents/review-and-sync) covers bringing the runner’s changes into your repository.
