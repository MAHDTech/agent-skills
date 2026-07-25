# Frequently Asked Questions

## [Zed](#zed)

### How can I contribute?

Check out our [contribution
guide](https://github.com/zed-industries/zed/blob/main/CONTRIBUTING.md)
to learn how you can contribute to Zed.

------------------------------------------------------------------------

### How can I download Zed?

You can obtain the release build via the [download page](https://zed.dev/download).

------------------------------------------------------------------------

### How do I log out of GitHub in Zed?

We currently don't offer a convenient way for users to log out of GitHub
from within Zed. If you'd like to log out, follow these steps:

1.  Open the `Keychain Access` application
2.  In the `login` keychain, find and delete the `https://zed.dev` entry
3.  Restart Zed

------------------------------------------------------------------------

### How does private beta testing work?

When you receive an invite to test a feature in private beta, your user
record in our database is registered to a particular feature flag.
During the beta period, you must log in via the `Sign in` button in the
upper right corner of Zed, which authenticates you through GitHub. A
network connection is required when opening Zed during private beta
testing, as Zed sends a request to our server to verify your access. If
you are not associated with the feature flag, UI elements related to the
feature will be hidden.

For features in private beta, we prioritize updates to the preview
channel. During a private beta, we accelerate our preview patch release
schedule to roughly a minimum of one update per day, including all the
latest additions and fixes since the previous preview patch build.

Some users have noticed that, in certain cases, they can access private
beta features on the stable channel. This occurs because private betas
are **not** tied to a particular release channel, but rather scoped to
specific users. Depending on the duration of the beta testing period,
code related to the feature will gradually make its way into stable
builds, as our [minor update release
cycle](https://zed.dev/faq#what-are-the-release-channels) remains the same. While we
recommend testing private beta features on the preview channel to access
the most recent updates and improvements, you can use older preview
patch builds, or potentially stable builds, if you don't have access to
preview—such as when using third-party packages maintained by developers
who don't ship Zed preview. However, be aware that you will be working
with outdated versions of the feature.

Some deep-divers have found they can enable feature flags by changing a
small snippet of code and compiling Zed from source; we'll leave digging
up that code as an exercise for the reader.

------------------------------------------------------------------------

### Is Zed extensible?

Yes! Zed currently supports various types of [extensions](https://zed.dev/extensions),
with support for deeper extension APIs [planned](https://zed.dev/roadmap).

------------------------------------------------------------------------

### Is Zed free?

Yes! Zed (the editor) is free for all to use, with binaries for macOS
and popular Linux distributions readily available for
[download](https://zed.dev/download).

------------------------------------------------------------------------

### Is Zed open source?

Yes! Zed is [open source](https://github.com/zed-industries/zed)!

Check out [the announcement post](https://zed.dev/blog/zed-is-now-open-source) for more
details.

------------------------------------------------------------------------

### What are the release channels?

- `Nightly`: Built daily from the `main` branch with the latest
  features. It has not gone through any testing and may have stability
  issues. *Nightly isn't public.*

- [`Preview`](https://zed.dev/releases/preview): Our weekly release with recent
  features that have undergone initial internal testing. More stable
  than Nightly, but potentially less stable than Stable. Preview may
  contain experimental functionality and might experience reverts of
  recently-added features.

  You should anticipate experiencing a higher frequency of updates if
  you are on Preview. If the frequency of these updates is distracting
  for you, you can disable automatic update checks via
  `"auto_update": false` and run manual update checks via
  `command palette > auto update: check`.

- [`Stable`](https://zed.dev/releases/stable): Our most production-ready version that
  has been tested in Preview for a week. This release is recommended for
  those relying on Zed for mission-critical work.

------------------------------------------------------------------------

On a typical release Wednesday, we follow these steps:

1.  **Promote last week's preview to stable**

    - The preview branch from the previous week (e.g., `v0.180.x`) is
      promoted to stable by taking the most recent preview tag and
      creating a new tag on the latest commit without the `-pre` suffix
      (e.g., `v0.180.1-pre` → `v0.180.1`)

2.  **Create a new preview branch**

    - A new branch is forked off of the `main` branch
    - A tag with a `-pre` suffix is added to the latest commit (e.g.,
      `v0.181.0-pre`) to mark the branch as preview

3.  **Prepare main for the next cycle**

    - The version number on `main` is bumped to the next minor version

During the week between minor releases, we may cherry-pick important
fixes to the preview or stable branches and release new patches. The
frequency of preview patch updates will increase during periods where we
are conducting private beta testing of new features.

------------------------------------------------------------------------

### What can I expect of the beta?

While we are committed to building a stable, professional-grade editor,
you'll likely run into bugs and pain points throughout the beta phase.
Additionally, Zed may not support all languages you use and may not be
available for your preferred operating system. What we can confidently
say is that we are working hard to bring you the features, support, and
stability you need to make Zed your homebase editor for building great
software.

**As diligent as we try to be with every line of code we write, human
error is inevitable, especially during periods of rapid development.
Make sure to properly backup any code you are working on in Zed to
protect yourself from potential catastrophic events. While we believe
Zed to be safe to use, it is always advisable to err on the side of
caution in these early stages.**

------------------------------------------------------------------------

### What is Quality Week?

Zed is a large, complex codebase. While we work to ship a steady stream
of bug fixes in our releases each week, periodically, we conduct what we
refer to as "Quality Week." During this time, we halt work on feature
additions and pivot to working solely on user pain points within Zed.
Prior to Quality Week, the community team spends time curating a project
board of issues. We focus on finding issues that try to balance a small
amount of work with a potentially large impact. We prioritize reducing
cases where Zed can panic. The overall goal of Quality Week is to ship a
more stable, refined Zed.

------------------------------------------------------------------------

### What is "Zednesday"?

We publish new minor patches for Stable and Preview **every** Wednesday
(aside from some holidays), so we internally celebrate it by calling it
"Zednesday!"

*Don't worry, we don't know how to pronounce it either.*

------------------------------------------------------------------------

### What platforms does Zed support?

Zed supports macOS, Linux, and Windows. You can download the [latest
builds here](https://zed.dev/download).

------------------------------------------------------------------------

### Where are language servers stored?

Language servers are stored at
`~/Library/Application Support/Zed/languages`. See [language
servers](https://zed.dev/features#language_servers) for more information.

------------------------------------------------------------------------

### Where are my configuration files located?

The `settings.json` can be found at `~/.config/zed/settings.json`. See
[configuring Zed](https://zed.dev/docs/configuring-zed) for more
information. The `keymap.json` can be found at
`~/.config/zed/keymap.json`. See [key
bindings](https://zed.dev/docs/key-bindings) for more information.

------------------------------------------------------------------------

### Where can I find Zed's log output?

See the `troubleshooting`
[documentation](https://zed.dev/docs/troubleshooting#zed-log).

------------------------------------------------------------------------

### Where is Zed's workspace data stored?

See the `troubleshooting`
[documentation](https://zed.dev/docs/troubleshooting#startup-and-workspace-issues).

------------------------------------------------------------------------

### Why a new editor? Why not contribute to an existing editor?

We stopped working on Atom and started on the foundations of Zed when we
realized that we couldn't shape Atom into our vision for the ultimate
editor. While we respect and appreciate the innovations brought by
Visual Studio Code, we never found ourselves loving it enough to give up
on the dream. Ultimately, we think we're going to add the most value to
the world by creating something new. It's also a lot more fun.

------------------------------------------------------------------------

### Why "Zed?"

We liked the simplicity of the name "Ed", but we didn't want to shadow
[ed](https://en.wikipedia.org/wiki/Ed_(text_editor)), the editor in
which the Unix was originally developed. We liked how adding the letter
"Z" formed the word "Zed", which is also the name for the letter "Z" in
some dialects of English. As the last letter of the alphabet, it seemed
like an appropriate name for the ultimate editor we are building.

## [Zed Pro](#zed-pro)

### Where can I get help with my Zed Pro subscription?

If you have any questions about your Zed Pro subscription, feel free to
send us an email at
[billing-support@zed.dev](mailto:billing-support@zed.dev). Make sure to
write us from the email account associated with your Stripe/Zed Pro
subscription.

------------------------------------------------------------------------

### Where can I learn about Zed Pro?

For information on plans, billing, and models offered through Zed Pro,
see the [documentation](https://zed.dev/docs/ai/subscription), or visit
our [pricing page](https://zed.dev/pricing).

## [GPUI](#gpui)

### Why not use an existing Rust GPU UI library?

When we started on Zed, the Rust UI framework space was much younger. In
absence of a mature solution that met our exact needs, the simplest path
for us was to build it ourselves. We're too far into things now to
change at this point. We also like our framework. It's working well for
us.

## [Data And Privacy](#data-and-privacy)

### Are collaboration sessions hosted on a server?

Yes. Collaboration traffic is proxied through our servers, which is more
reliable than peer-to-peer connections and lets new collaborators join
quickly.

For shared projects, Zed's servers act as a relay. We maintain only the
transient state needed to route the session, and that state is deleted
as collaborators disconnect.

------------------------------------------------------------------------

### What data gets sent to your servers when collaborating?

**user identification** - Collaboration requires signing in. When you're
signed out, Zed doesn't connect to the collaboration service. Once you
sign in, we establish a WebSocket connection to one of our servers
that's associated with your Zed account, but we don't send any other
data.

**project metadata** - When you're in a call and you share one of your
projects, we send to our servers the name of that project and the
relative paths of all of the files in that project. We also send the
names of any language servers that are running for that project, and
their status messages. Note that we only store relative file paths,
language server statuses, and git status codes of the shared project on
our server; all other data stays on the host and is proxied through our
servers when requested by a guest.

**file contents** - When you're sharing a project in a call, your
collaborators can open any file in your project, as well as files that
are returned by your language servers from requests like 'go to
definition' and 'find all references'. When they do open these files, we
send them to our servers, which forward them to the collaborator. File
contents are never persisted on our servers.

In general, collaboration access is limited to what's needed to provide
the session. Shared project contents, audio, video, and screen sharing
are relayed in real time rather than stored. Session state is transient
and deleted as collaborators disconnect and when the room ends.

------------------------------------------------------------------------

### What other data gets sent to your servers?

See [Telemetry in Zed](https://zed.dev/docs/telemetry).

------------------------------------------------------------------------

### Where can I find your EULA?

Our EULA can be [found here](https://zed.dev/eula).

## [Company](#company)

### Are you hiring?

Check our [jobs listings](https://zed.dev/jobs).

------------------------------------------------------------------------

### How do you generate revenue?

We generate revenue through optional [Zed Pro](https://zed.dev/pricing) subscriptions.

------------------------------------------------------------------------

### Is Zed a for-profit company?

Yes. We sold equity in our company to investors to enable ourselves to
give Zed the focus it deserves.
