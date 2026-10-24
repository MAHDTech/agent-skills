+++
title = "docs-privacy-and-security-security"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Security"
url: "https://delta.dev/docs/privacy-and-security/security"
description: "How Delta protects your code, conversations, and data."
---

1.  Privacy & Security
2.  Security

# Security

How Delta protects your code, conversations, and data.

On this page

1.  [Encryption](#encryption)
2.  [Secret redaction](#secret-redaction)
    1.  [Limitations](#limitations)
3.  [Background terminal output](#background-terminal-output)
4.  [Data retention and deletion](#data-retention-and-deletion)
5.  [Reporting security vulnerabilities](#reporting-security-vulnerabilities)

Delta encrypts data in transit and at rest. It also redacts recognized secrets before thread content leaves your device.

## [Encryption](#encryption)

Data sent between Delta and its backend is encrypted in transit using TLS. This includes the persistent connections used for live synchronization.

Cloudflare automatically encrypts data stored in R2, Durable Objects, KV, and D1 at rest using Cloudflare-managed keys.

## [Secret redaction](#secret-redaction)

Delta discovers secret values locally and replaces them with `[REDACTED]` before thread content is stored, synchronized, or sent to a model provider. This applies when a recognized secret appears in messages, tool output, or captured command output. Only the placeholder and redaction metadata leave your device. Delta does not persist or replicate the plaintext value.

Delta discovers secrets from:

*   Environment variables whose names indicate that they contain credentials, such as variables containing `token`, `secret`, `key`, or `password`.
*   Environment-variable values that look like credentials based on their length and entropy.
*   Dotenv files, including `.env` and its variants.
*   Mise configuration files, including `mise.toml`, `.mise.toml`, and nested mise configuration paths.
*   Credential files in your home folder, including SSH private keys, cloud and container credentials, package-manager configuration, and developer-tool credentials.
*   Locally stored credential caches and supported credential stores. Delta reads existing values without running credential-helper commands or requesting new tokens at startup.
*   Well-known credential formats, such as GitHub, npm, Google, GitLab, Stripe, Microsoft, Heroku, Fly.io, PyPI, OpenAI, Anthropic, and Slack tokens. Delta can recognize these by their format even if it has not discovered the exact value elsewhere.

On [Cloud Runners](/docs/agents/cloud-runners) (private beta), Delta also redacts the values of the environment variables you add in **Settings > Cloud Runner**.

Delta reloads dotenv and mise files when they change; the home-folder credential sources load once at startup. Short values are excluded: the minimum is eight characters, twelve for all-digit values, and sixteen for values recognized by shape alone. Common non-secret environment variables are also excluded.

If you replace or remove a secret in a dotenv or mise source, Delta keeps the previously loaded value in memory until you quit. This lets Delta continue redacting that value and resolve existing redactions.

### [Limitations](#limitations)

Redaction matches secret values that Delta discovers and well-known credential formats that it recognizes. It does not scan arbitrary files for credentials, so a secret stored only in an unsupported file may not be recognized unless its format is known. Short or low-entropy credentials in unusually named environment variables may also be missed.

DeltaDB does not record or sync files that Git ignores. Files such as `.env` stay on the machine where they were created. If a recognized secret appears in a message or command output, Delta redacts it before that content leaves the device. Learn more in [Delta & Git](/docs/concepts/delta-and-git#what-delta-records-that-git-doesnt).

> **Warning:** Secret redaction is a safeguard, not a substitute for keeping credentials out of repositories. Review thread content before sharing it.

## [Background terminal output](#background-terminal-output)

When you put an interactive terminal into background mode, Delta temporarily captures up to 8 MiB of its output on the machine running the process. This lets the agent inspect output produced after the terminal display in the thread stops updating.

The raw capture stays in a temporary store on that machine and is not added to the shared thread. When an agent requests the output, Delta reads the capture on the machine that owns it and applies known-secret redaction before returning the result.

The capture contains raw terminal output until Delta reads it for the agent. Do not print credentials or other sensitive values to a background terminal.

## [Data retention and deletion](#data-retention-and-deletion)

[Data retention and deletion](/docs/privacy-and-security/data-storage#data-retention-and-deletion) explains how to request deletion, what account deletion covers, and where copies of your data may remain.

## [Reporting security vulnerabilities](#reporting-security-vulnerabilities)

To report a security vulnerability, email [security@zed.dev](mailto:security@zed.dev).

Include a description of the vulnerability, proof-of-concept evidence, its estimated impact, and any suggested fix. Zed does not currently offer bug bounty rewards.
