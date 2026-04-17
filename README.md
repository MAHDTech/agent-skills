# 🥒 Agent Skills

My personal and public agent skills.

## Table of Contents

- [install](docs/install.md)
- [usage](docs/usage.md)
- [Skills](#available-skills)

## Available Skills

### [cmd-email-md](skills/cmd-email-md/SKILL.md)

**Description**: Convert markdown to email-safe HTML with inline styles and cross-client compatibility. Use when writing newsletters, transactional emails, or any HTML email from markdown source.

**Triggers**: -

### [cmd-follow-up](skills/cmd-follow-up/SKILL.md)

**Description**: Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements

**Triggers**: -

### [cmd-gh-issue](skills/cmd-gh-issue/SKILL.md)

**Description**: Create structured GitHub issues from conversation context using gh CLI

**Triggers**: -

### [cmd-idiot-proof-docs](skills/cmd-idiot-proof-docs/SKILL.md)

**Description**: Simplify documentation for clarity and readability with approval-gated edits

**Triggers**: -

### [cmd-latest-msg](skills/cmd-latest-msg/SKILL.md)

**Description**: Store or retrieve the latest agent message to /tmp/agents/{agent}/

**Triggers**: -

### [cmd-local-repo-skills](skills/cmd-local-repo-skills/SKILL.md)

**Description**: Scaffold cross-tool repo-local skills and agent instructions with canonical source in .agents/ and symlinks for Claude, Codex, Gemini, and Codex-home

**Triggers**: -

### [cmd-persona](skills/cmd-persona/SKILL.md)

**Description**: Prime the agent with a behavioral persona for the conversation

**Triggers**: -

### [cmd-pr-build-context](skills/cmd-pr-build-context/SKILL.md)

**Description**: Build high-signal PR context for review with diff analysis, risk assessment, and discussion questions

**Triggers**: -

### [cmd-pr-conflict-resolver](skills/cmd-pr-conflict-resolver/SKILL.md)

**Description**: Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol

**Triggers**: -

### [cmd-pr-description](skills/cmd-pr-description/SKILL.md)

**Description**: Generate a PR title and description, then commit, create/update the PR on approval

**Triggers**: -

### [cmd-pr-edgecase](skills/cmd-pr-edgecase/SKILL.md)

**Description**: Review branch changes for test gaps, logic edge cases, failure modes, and integration risks

**Triggers**: -

### [cmd-pr-gh-comments](skills/cmd-pr-gh-comments/SKILL.md)

**Description**: Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution

**Triggers**: -

### [cmd-pr-review-prepare](skills/cmd-pr-review-prepare/SKILL.md)

**Description**: Prepare branch for code review by building context, identifying issues, and suggesting improvements

**Triggers**: -

### [cmd-pr-test-plan](skills/cmd-pr-test-plan/SKILL.md)

**Description**: Generate manual test plans for PR changes — focused on hands-on verification a developer would do, not unit-test edge cases

**Triggers**: -

### [cmd-productionize](skills/cmd-productionize/SKILL.md)

**Description**: Transform applications into production-ready deployments with systematic analysis, improvement, and framework-specific optimization

**Triggers**: -

### [cmd-proofread](skills/cmd-proofread/SKILL.md)

**Description**: Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability

**Triggers**: -

### [cmd-rfc-review](skills/cmd-rfc-review/SKILL.md)

**Description**: Review RFCs for problem clarity, compliance, security, and performance using SCQA framework

**Triggers**: -

### [cmd-scope-sweep](skills/cmd-scope-sweep/SKILL.md)

**Description**: Final pass to identify missed items, edge cases, and risks before considering a scope done

**Triggers**: -

### [cmd-sculpt-code](skills/cmd-sculpt-code/SKILL.md)

**Description**: Reshape code for readability, naming, structure, TODOs, and reduced surface area across any language

**Triggers**: -

### [cmd-store-plan](skills/cmd-store-plan/SKILL.md)

**Description**: Capture the current conversation's plan, decisions, and action items into a structured markdown file in the project's plans/ directory. Triggers on "store this plan", "save this plan for later", "document this for later", "write up what we discussed", "create a plan file", or "/cmd-store-plan".

**Triggers**: -

### [mermaid-render](skills/mermaid-render/SKILL.md)

**Description**: Render and display Mermaid diagrams inline in iTerm2 or Ghostty. Use when creating, editing, or iterating on mermaid diagrams. Triggers on mermaid diagram work — flowcharts, sequence, state, class, ER, and XY charts.

**Triggers**: -

### [session-commit](skills/session-commit/SKILL.md)

**Description**: Capture learnings from the current coding session and update AGENTS.md. Use when the user asks to close the loop, run session-commit, record best practices, or update agent instructions based on recent work.

**Triggers**: -

### [skill-creator](skills/skill-creator/SKILL.md)

**Description**: Guide for creating effective skills that follow the Agent Skills standards. Use this when the user wants to create a new skill or update an existing one.

**Triggers**: create a new skill, add a skill, how do I write a skill
