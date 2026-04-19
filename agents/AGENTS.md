---
name: agents
description: Single source of truth for agent instructions
---

# Available Skills

- **cmd-devenv**: Strict guidelines for using devenv for shell and dependency management
- **cmd-follow-up**: Self-review after implementation — surface missed work, simplification opportunities, and idiomatic improvements
- **cmd-gh-issue**: Create structured GitHub issues from conversation context using gh CLI
- **cmd-idiot-proof-docs**: Simplify documentation for clarity and readability with approval-gated edits
- **cmd-latest-msg**: Store or retrieve the latest agent message to /tmp/agents/{agent}/
- **cmd-local-repo-skills**: Scaffold cross-tool repo-local skills and agent instructions with canonical source in .agents/ and symlinks for Claude, Codex, Gemini, and Codex-home
- **cmd-persona**: Prime the agent with a behavioral persona for the conversation
- **cmd-pr-build-context**: Build high-signal PR context for review with diff analysis, risk assessment, and discussion questions
- **cmd-pr-conflict-resolver**: Resolve merge conflicts systematically with context-aware 3-tier classification and escalation protocol
- **cmd-pr-description**: Generate a PR title and description, then commit, create/update the PR on approval
- **cmd-pr-edgecase**: Review branch changes for test gaps, logic edge cases, failure modes, and integration risks
- **cmd-pr-gh-comments**: Triage and resolve GitHub PR review comments with categorized action plans and approval-gated execution
- **cmd-pr-review-prepare**: Prepare branch for code review by building context, identifying issues, and suggesting improvements
- **cmd-pr-test-plan**: Generate manual test plans for PR changes — focused on hands-on verification a developer would do, not unit-test edge cases
- **cmd-productionize**: Transform applications into production-ready deployments with systematic analysis, improvement, and framework-specific optimization
- **cmd-proofread**: Proofread posts before publishing for spelling, grammar, repetition, logic, weak arguments, broken links, and optionally reformat for skimmability
- **cmd-rfc-review**: Review RFCs for problem clarity, compliance, security, and performance using SCQA framework
- **cmd-scope-sweep**: Final pass to identify missed items, edge cases, and risks before considering a scope done
- **cmd-scratchpad**: Enforces the use of a "scratch/" directory for all temporary or experimental AI agent scripts. Ensures the directory is .gitignored and instructs agents to clean up after completion.
- **cmd-sculpt-code**: Reshape code for readability, naming, structure, TODOs, and reduced surface area across any language
- **cmd-store-plan**: Capture the current conversation's plan, decisions, and action items into a structured markdown file in the project's plans/ directory. Triggers on "store this plan", "save this plan for later", "document this for later", "write up what we discussed", "create a plan file", or "/cmd-store-plan".
- **mermaid-render**: Render and display Mermaid diagrams inline in iTerm2 or Ghostty. Use when creating, editing, or iterating on mermaid diagrams. Triggers on mermaid diagram work — flowcharts, sequence, state, class, ER, and XY charts.

- **session-commit**: Capture learnings from the current coding session and update AGENTS.md. Use when the user asks to close the loop, run session-commit, record best practices, or update agent instructions based on recent work.
- **skill-creator**: Guide for creating effective skills that follow the Agent Skills standards. Use this when the user wants to create a new skill or update an existing one.
