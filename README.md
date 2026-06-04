# Agent Skills

My personal and public agent skills.

## Table of Contents

- [install](docs/install.md)
- [usage](docs/usage.md)
- [Skills](#available-skills)

## Available Skills

### [bevy-expert](skills/bevy-expert/SKILL.md)

**Description**: Expert developer guidance on Bevy, Rust's data-driven game engine, covering ECS structure, input, assets, events, states, system scheduling, and performance tuning.

**Triggers**: bevy, bevy game engine, bevy-expert, Rust game engine, Rust gamedev

### [brain-critical-thinking](skills/brain-critical-thinking/SKILL.md)

**Description**: Analyzes the agent's own previous response with rigorous critical thinking, looking for flaws, biases, and unstated assumptions.

**Triggers**: /criticalthink, analyze your last response, apply critical thinking, critical thinking, critical-thinking, critique your response

### [brain-follow-up](skills/brain-follow-up/SKILL.md)

**Description**: Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements

**Triggers**: -

### [brain-idiot-proof-docs](skills/brain-idiot-proof-docs/SKILL.md)

**Description**: Simplify documentation for clarity and readability with approval-gated edits

**Triggers**: -

### [brain-magic-words](skills/brain-magic-words/SKILL.md)

**Description**: Intercepts requests to ensure the AI agent and human work back and forth to create an implementation plan before diving into code.

**Triggers**: magic words, magic-words, about iterating, discussing the plan, work back and forth, before jumping to code

### [brain-pr-edgecase](skills/brain-pr-edgecase/SKILL.md)

**Description**: Review branch changes for test gaps, logic edge cases, failure modes, and integration risks

**Triggers**: -

### [brain-proofread](skills/brain-proofread/SKILL.md)

**Description**: Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability

**Triggers**: -

### [brain-rfc-review](skills/brain-rfc-review/SKILL.md)

**Description**: Review RFCs for problem clarity, compliance, security, and performance using SCQA framework

**Triggers**: -

### [brain-scope-sweep](skills/brain-scope-sweep/SKILL.md)

**Description**: Final pass to identify missed items, edge cases, and risks before considering a scope done

**Triggers**: -

### [cmd-devenv](skills/cmd-devenv/SKILL.md)

**Description**: Strict guidelines for using devenv for shell and dependency management

**Triggers**: devenv, developer environment, when using devenv, when running commands in a devenv environment, when you see a devenv.nix or devenv.yaml file

### [cmd-opencode](skills/cmd-opencode/SKILL.md)

**Description**: This skill provides comprehensive guidance for using OpenCode, the open-source AI coding agent. Use this skill when working with OpenCode CLI commands, keyboard shortcuts, agents (build/plan), slash commands, tools, skills, MCP servers, or configuration. Automatically triggered when OpenCode-specific questions or tasks are detected.

**Triggers**: opencode

### [cmd-opencode-acp](skills/cmd-opencode-acp/SKILL.md)

**Description**: Control OpenCode directly via the Agent Client Protocol (ACP). Start sessions, send prompts, resume conversations, and manage OpenCode updates.

**Triggers**: opencode acp, acp, agent control protocol

### [cmd-scratchpad](skills/cmd-scratchpad/SKILL.md)

**Description**: Enforces the use of a "scratch/" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion.

**Triggers**: create a temporary file, run a quick test, make a scratch script, create a test script, experiment with

### [code-productionize](commands/code-productionize.md)

**Description**: Transform applications into production-ready deployments with systematic analysis, improvement, and framework-specific optimization

### [code-sculpt](commands/code-sculpt.md)

**Description**: Reshape code for readability, naming, structure, TODOs, and reduced surface area across any language

### [gh-create-issue](commands/gh-create-issue.md)

**Description**: Create structured GitHub issues from conversation context using gh CLI

### [gh-create-pr](commands/gh-create-pr.md)

**Description**: Generate a PR title and description, then commit, create/update the PR on approval

### [gh-reply-pr-comments](commands/gh-reply-pr-comments.md)

**Description**: Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution

### [git-resolve-conflicts](commands/git-resolve-conflicts.md)

**Description**: Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol

### [guidelines](skills/guidelines/SKILL.md)

**Description**: Behaviour Guidelines for AI Agents.

**Triggers**: /guidelines, follow the guidelines, verify against the guidelines

### [plan-store](commands/plan-store.md)

**Description**: Capture the current conversation's plan, decisions, and action items into a structured markdown file in the project's plans/ directory. Triggers on "store this plan", "save this plan for later", "document this for later", "write up what we discussed", "create a plan file", or "/cmd-store-plan".

### [pr-build-context](commands/pr-build-context.md)

**Description**: Build high-signal PR context for review with diff analysis, risk assessment, and discussion questions

### [pr-create-test-plan](commands/pr-create-test-plan.md)

**Description**: Generate manual test plans for PR changes — focused on hands-on verification a developer would do, not unit-test edge cases

### [pr-prepare-review](commands/pr-prepare-review.md)

**Description**: Prepare branch for code review by building context, identifying issues, and suggesting improvements

### [refactor-codebase](skills/refactor-codebase/SKILL.md)

**Description**: Remove engineering debt, untangle large modules, collapse duplicated logic, and modernize legacy patterns without changing behavior.

**Triggers**: refactor, refactor codebase, refactor code, remove dead code, modernize legacy patterns, clean up codebase

### [sys-command-creator](skills/sys-command-creator/SKILL.md)

**Description**: Guide for creating effective commands that follow the Agent Commands standards. Use this when the user wants to create a new command or update an existing one.

**Triggers**: create a new command, add a command, how do I write a command

### [sys-skill-creator](skills/sys-skill-creator/SKILL.md)

**Description**: Guide for creating effective skills that follow the Agent Skills standards. Use this when the user wants to create a new skill or update an existing one.

**Triggers**: create a new skill, add a skill, how do I write a skill
