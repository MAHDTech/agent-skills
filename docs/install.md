# Installation Guide

## Consumer

To install these skills, use the following method:

```bash
# NPM users
npx skills add MAHDTech/agent-skills

# Bun users
bunx skills add MAHDTech/agent-skills
```

When you want to update the skills, run the following command:

```bash
bunx skills update
```

## Developer

To install these skills locally, for use during local development, follow these steps:

1. **Clone the repository**:

```bash
git clone https://github.com/MAHDTech/agent-skills.git
cd agent-skills
```

1. **Checkout a branch**:

```bash
git checkout feat/add-my-new-skill
```

1. **Run the TUI Installer**:

```bash
bun run setup
```

Follow the interactive prompts to link the skills to your preferred AI agent CLIs (Gemini, Claude, or OpenCode).

This will give you live running skills in the environment allowing you to test your changes live.

1. **Verify Installation**:

Check your CLI's skills directory (e.g., `~/.agents/skills`) to ensure the symlinks were created.
