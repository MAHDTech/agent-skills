+++
title = "oak-docs-ci-runners"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "oak"
+++

---
description: "Register your own machines with gitea-runner: labels, Docker or host jobs, running as a service, and security."
title: Self-hosted runners · Docs
image: https://oak.space/social-card-217ab603.png
---

Get started

- [Introduction](https://oak.space/docs)
- [Install](https://oak.space/docs/install)
- [Quickstart](https://oak.space/docs/quickstart)
- [Coming from Git](https://oak.space/docs/coming-from-git)

Everyday work

- [Making changes](https://oak.space/docs/making-changes)
- [Branches](https://oak.space/docs/branches)
- [Merging & conflicts](https://oak.space/docs/merging)
- [Reviewing](https://oak.space/docs/reviewing)
- [History](https://oak.space/docs/history)

Agents

- [Working with agents](https://oak.space/docs/agents)
- [Example session](https://oak.space/docs/example-session)

Large repositories

- [Lazy mounts](https://oak.space/docs/mounts)
- [Agent spaces](https://oak.space/docs/spaces)
- [Sparse clones](https://oak.space/docs/sparse-clones)

CI

- [Overview](https://oak.space/docs/ci)
- [Workflow syntax](https://oak.space/docs/ci-workflows)
- [Runners](https://oak.space/docs/ci-runners)
- [Environments](https://oak.space/docs/ci-environments)

Organizations and access

- [Organizations](https://oak.space/docs/organizations)
- [Path permissions](https://oak.space/docs/path-permissions)
- [API keys](https://oak.space/docs/api-keys)
- [Plans & limits](https://oak.space/docs/limits)

Integrations

- [Git import & export](https://oak.space/docs/git)
- [Webhooks](https://oak.space/docs/webhooks)
- [Sites](https://oak.space/docs/sites)

Reference

- [CLI reference](https://oak.space/docs/cli)
- [Files & env vars](https://oak.space/docs/configuration)
- [HTTP API](https://oak.space/docs/api)
- [How Oak works](https://oak.space/docs/internals)

For agents

- [All docs as one file](https://oak.space/docs/llms-full.txt)

\<details\>

\<summary\>Docs menu ▾</summary>



Get started

- <a href="https://oak.space/docs">Introduction</a>
- <a href="https://oak.space/docs/install">Install</a>
- <a href="https://oak.space/docs/quickstart">Quickstart</a>
- <a href="https://oak.space/docs/coming-from-git">Coming from Git</a>

Everyday work

- <a href="https://oak.space/docs/making-changes">Making changes</a>
- <a href="https://oak.space/docs/branches">Branches</a>
- <a href="https://oak.space/docs/merging">Merging & conflicts</a>
- <a href="https://oak.space/docs/reviewing">Reviewing</a>
- <a href="https://oak.space/docs/history">History</a>

Agents

- <a href="https://oak.space/docs/agents">Working with agents</a>
- <a href="https://oak.space/docs/example-session">Example session</a>

Large repositories

- <a href="https://oak.space/docs/mounts">Lazy mounts</a>
- <a href="https://oak.space/docs/spaces">Agent spaces</a>
- <a href="https://oak.space/docs/sparse-clones">Sparse clones</a>

CI

- <a href="https://oak.space/docs/ci">Overview</a>
- <a href="https://oak.space/docs/ci-workflows">Workflow syntax</a>
- <a href="https://oak.space/docs/ci-runners">Runners</a>
- <a href="https://oak.space/docs/ci-environments">Environments</a>

Organizations and access

- <a href="https://oak.space/docs/organizations">Organizations</a>
- <a href="https://oak.space/docs/path-permissions">Path permissions</a>
- <a href="https://oak.space/docs/api-keys">API keys</a>
- <a href="https://oak.space/docs/limits">Plans & limits</a>

Integrations

- <a href="https://oak.space/docs/git">Git import & export</a>
- <a href="https://oak.space/docs/webhooks">Webhooks</a>
- <a href="https://oak.space/docs/sites">Sites</a>

Reference

- <a href="https://oak.space/docs/cli">CLI reference</a>
- <a href="https://oak.space/docs/configuration">Files & env vars</a>
- <a href="https://oak.space/docs/api">HTTP API</a>
- <a href="https://oak.space/docs/internals">How Oak works</a>

</details>

# Self-hosted runners

Copy page [.md](https://oak.space/docs/ci-runners.md "This page as raw Markdown")

Register your own machines with gitea-runner: labels, Docker or host jobs, running as a service, and security.

A runner is a machine of yours that runs your organization's CI jobs. It's the open-source **Gitea runner** (`gitea-runner`, formerly `act_runner`), unmodified: Oak speaks the same runner protocol Gitea does. A runner *polls* Oak for work over HTTPS, so it can sit behind a firewall, on a laptop, or in a closet — nothing ever connects in to it.

## Add a runner \#

1. **Install** `gitea-runner` on the machine — a single binary from the [releases page](https://gitea.com/gitea/act_runner/releases) ( `gitea-runner-\<version\>-\<os\>-\<arch\>.xz`). It needs `git`; Docker is optional and only needed for container jobs.

   ```bash
   xz -d gitea-runner-*-darwin-arm64.xz && chmod +x gitea-runner-*-darwin-arm64
   sudo mv gitea-runner-*-darwin-arm64 /usr/local/bin/gitea-runner
   ```

2. **Get a registration token** from your organization's **Settings → Runners** (owners and admins). It's shown once; generating a new one replaces it, and runners already registered keep working.
3. **Register** the machine, choosing its labels, then **start** it:

   ```bash
   gitea-runner register --no-interactive --instance https://oak.space \
     --token <registration-token> --name build-box --labels linux:docker://node:20-bookworm
   gitea-runner daemon
   ```

It appears under **Settings → Runners** as *online* within a few seconds. Registration writes the runner's own credentials to a `.runner` file next to where you ran it; keep that file private.

## Labels \#

A workflow job says where it runs with `runs-on`; a runner takes a job only if it carries **every** label listed there. Each label also tells the runner *how* to run that job:

| Label | Runs jobs |
| --- | --- |
| `linux:docker://node:20-bookworm` | In a fresh container from that image, one per job. The isolated choice. |
| `macos:host` | Directly on the machine, in a scratch directory, as the user running `gitea-runner`. |

A runner can carry several, for example `--labels macos:host,arm64:host`. Label names are yours to choose — `linux`, `gpu`, `big-disk`, `oak-ci` — as long as workflows and runners agree.

## Configure it \#

For anything beyond labels, generate a config file and start the daemon with it:

```bash
gitea-runner config generate > config.yaml
gitea-runner daemon --config config.yaml
```

The settings you're most likely to change:

```yaml
runner:
  capacity: 2            # jobs this machine runs at once
  labels:                # replaces --labels once set here
    - "linux:docker://node:20-bookworm"
  timeout: 3h            # hard cap per job, on top of the workflow's timeout-minutes
cache:
  enabled: true          # the built-in server behind actions/cache, local to this machine
container:
  network: bridge
host:
  workdir_parent: /var/lib/gitea-runner/work
```

## Keep it running \#

On Linux, a systemd unit:

```ini
# /etc/systemd/system/gitea-runner.service
[Unit]
Description=Oak CI runner
After=network-online.target docker.service

[Service]
User=runner
WorkingDirectory=/var/lib/gitea-runner
ExecStart=/usr/local/bin/gitea-runner daemon --config /var/lib/gitea-runner/config.yaml
Restart=always

[Install]
WantedBy=multi-user.target
```

On macOS, a launchd agent (`~/Library/LaunchAgents/space.oak.runner.plist`) that runs the same command with `KeepAlive` set, loaded with `launchctl load`. Either way, run the daemon from the directory holding its `.runner` file.

## Docker on a Mac \#

Container labels need a Docker daemon. On macOS, [colima](https://github.com/abiosoft/colima) is the lightest:

```bash
brew install colima docker
colima start --cpu 6 --memory 8 --disk 80
```

The runner itself stays on the Mac, and its jobs run in Linux containers inside colima's VM. Two settings make that work:

1. **Point the runner at colima's socket** with `DOCKER_HOST` when you start it.
2. **Don't mount that socket into jobs.** By default the runner mounts the Docker socket into every job container, and colima's socket path exists on the Mac but not inside the VM, so every job fails with `error while creating mount source path`. Set `docker_host: "-"` under `container:` in `config.yaml`.

```bash
gitea-runner config generate > config.yaml
sed -i '' 's|^  #docker_host: ""|  docker_host: "-"|' config.yaml
DOCKER_HOST=unix://$HOME/.colima/default/docker.sock gitea-runner daemon --config config.yaml
```

Register with the same `--config config.yaml`. (Docker Desktop doesn't need either setting.)

## JavaScript actions \#

`actions/checkout`, `actions/cache`, and most marketplace actions are JavaScript. The runner runs them with `node` from the job's environment:

- **Container jobs** need `node` in the image (the `node:*` images, and Oak's own CI image, have it).
- **Host jobs** need `node` on the machine's `PATH`. [Bun](https://bun.sh) works in its place: `ln -s "$(command -v bun)" /usr/local/bin/node`.

## Oak's own CI image \#

oakspace's CI runs on runners carrying the `oak-ci` label, served by an image built from this repo:

```bash
docker build -t oak-ci:latest infra/ci-runner
gitea-runner register ... --labels oak-ci:docker://oak-ci:latest
```

It has Rust (the toolchain the repo pins, with `rustfmt`, `clippy`, `cargo-nextest`), Bun standing in for Node, PostgreSQL, and the C toolchain. Use it as a starting point for your own.

## Security \#

- **A runner runs code from anyone who can push.** Every repo in the organization with CI on can send jobs to every runner with matching labels, and a job can read the repo's [CI secrets](https://oak.space/docs/ci#secrets). Treat a runner like any machine that builds untrusted pull requests: prefer container labels, don't register a machine whose own files or credentials matter in host mode, and keep separate runners (and labels) for separate trust levels.
- **Runners belong to one organization.** They only ever receive that organization's jobs, and each job only gets a token for its own repo, valid while the job runs.
- **Credentials are stored hashed.** Remove a runner under **Settings → Runners** and its credentials stop working at once; reset the registration token if it leaked.
- **Environment runners.** An [environment](https://oak.space/docs/ci-environments) sends its jobs only to runners bound to it, which the repo's organization owners and admins choose. A bound runner takes no ordinary CI at all, so code from an ordinary branch push never runs on the machine that receives the environment's secrets. Give an environment its own runner (its own machine, ideally) rather than binding the one your everyday CI uses.
- **Network.** Runners make outbound HTTPS requests to oak.space, and to github.com to fetch `uses:` actions ( `runner.github_mirror` in the config points elsewhere).

## Status and failure modes \#

| You see | Meaning |
| --- | --- |
| Runner *online* | It polled within the last minute and is idle. |
| Runner *running a job* | Busy; the job is linked on the settings page. |
| Runner *offline* | Not heard from in a minute — stopped, asleep, or off the network. |
| Job "waiting for a runner with `linux, x64`" | No online runner carries all those labels. If none in the organization does, the run page says so. |
| Job failed: "The runner stopped reporting on this job." | Its runner went silent for 10 minutes mid-job (killed, rebooted, removed). Re-run it. |
| Job cancelled: "No runner picked this job up in time." | Nothing took it for 24 hours. |
| The runner logs "unregistered" and exits | It was removed, or its `.runner` file belongs to another server. Register it again. |

Something here wrong or missing? Run `oak feedback -m "…"` or email [zach@oak.space](mailto:zach@oak.space).

On this page
