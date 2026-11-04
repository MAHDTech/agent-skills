# Security Policy

This document defines the security policy and vulnerability reporting process for the `MAHDTech/agent-skills` catalog and the `ask` CLI tooling.
See [README.md](./README.md) for the project overview, [CONTRIBUTING.md](./CONTRIBUTING.md) for contributor guidelines, and [AGENTS.md](./AGENTS.md) for repository rules.

## Supported Versions

- Security updates apply to the rolling `trunk` branch and the latest release of `agent-skills` and `ask-cli`.
- Older releases are not supported and do not receive security patches.

| Version | Supported |
| ------- | --------- |
| `trunk` | Yes       |
| Latest  | Yes       |
| Older   | No        |

## Reporting a Vulnerability

- Report suspected security vulnerabilities privately through GitHub Private Vulnerability Reporting at [https://github.com/MAHDTech/agent-skills/security/advisories/new](https://github.com/MAHDTech/agent-skills/security/advisories/new).
- Do not open public GitHub issues, pull requests, or discussions for security vulnerabilities.
- Include the following details in your report:
  - Affected skill path (for example, `skills/<category>/<name>/SKILL.md`) or Rust crate (`ask-cli`, `skills-core`, or `skills-tui`).
  - Branch, release version, or commit SHA where the issue was observed.
  - Clear reproduction steps or a minimal proof of concept.
  - Assessment of the potential security impact.

## Security Scanning and Verification

- Automated Trivy vulnerability scanning and CodeQL static analysis run on pushes, pull requests, and weekly schedules via [.github/workflows/secops.yaml](./.github/workflows/secops.yaml).
- Contributors and maintainers can run the repository lint hooks and test suite locally inside `devenv`:

```bash
devenv --no-tui shell -- prek run -a
devenv --no-tui test
```
