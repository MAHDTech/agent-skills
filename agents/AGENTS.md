---
name: agents
description: Single source of truth for agent instructions
---

# Available Skills & Commands

- **brain-critical-thinking**: Analyzes the agent's own previous response with rigorous critical thinking, looking for flaws, biases, and unstated assumptions.
- **brain-follow-up**: Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements
- **brain-idiot-proof-docs**: Simplify documentation for clarity and readability with approval-gated edits
- **brain-magic-words**: Intercepts requests to ensure the AI agent and human work back and forth to create an implementation plan before diving into code.
- **brain-pr-edgecase**: Review branch changes for test gaps, logic edge cases, failure modes, and integration risks
- **brain-proofread**: Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability
- **brain-rfc-review**: Review RFCs for problem clarity, compliance, security, and performance using SCQA framework
- **brain-scope-sweep**: Final pass to identify missed items, edge cases, and risks before considering a scope done
- **cmd-devenv**: Strict guidelines for using devenv for shell and dependency management
- **cmd-opencode**: This skill provides comprehensive guidance for using OpenCode, the open-source AI coding agent. Use this skill when working with OpenCode CLI commands, keyboard shortcuts, agents (build/plan), slash commands, tools, skills, MCP servers, or configuration. Automatically triggered when OpenCode-specific questions or tasks are detected.
- **cmd-opencode-acp**: Control OpenCode directly via the Agent Client Protocol (ACP). Start sessions, send prompts, resume conversations, and manage OpenCode updates.
- **cmd-scratchpad**: Enforces the use of a "scratch/" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion.
- **code-productionize**: Transform applications into production-ready deployments with systematic analysis, improvement, and framework-specific optimization
- **code-sculpt**: Reshape code for readability, naming, structure, TODOs, and reduced surface area across any language
- **gh-create-issue**: Create structured GitHub issues from conversation context using gh CLI
- **gh-create-pr**: Generate a PR title and description, then commit, create/update the PR on approval
- **gh-reply-pr-comments**: Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution
- **git-resolve-conflicts**: Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol
- **guidelines**: Behaviour Guidelines for AI Agents.
- **plan-store**: Capture the current conversation's plan, decisions, and action items into a structured markdown file in the project's plans/ directory. Triggers on "store this plan", "save this plan for later", "document this for later", "write up what we discussed", "create a plan file", or "/cmd-store-plan".
- **pr-build-context**: Build high-signal PR context for review with diff analysis, risk assessment, and discussion questions
- **pr-create-test-plan**: Generate manual test plans for PR changes — focused on hands-on verification a developer would do, not unit-test edge cases
- **pr-prepare-review**: Prepare branch for code review by building context, identifying issues, and suggesting improvements
- **sys-command-creator**: Guide for creating effective commands that follow the Agent Commands standards. Use this when the user wants to create a new command or update an existing one.
- **sys-skill-creator**: Guide for creating effective skills that follow the Agent Skills standards. Use this when the user wants to create a new skill or update an existing one.
