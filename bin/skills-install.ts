import {
    intro,
    outro,
    select,
    multiselect,
    text,
    confirm,
    spinner,
    note,
} from "@clack/prompts"
import fs from "fs-extra"
import path from "path"
import os from "os"
import {execSync} from "child_process"
import {glob} from "glob"

async function install() {
    intro("🥒 Agent Skills Installer")

    const AGENT_SKILLS_HOME =
        process.env.AGENT_SKILLS_HOME || path.resolve(__dirname, "..")
    const shell = process.env.SHELL || "/bin/bash"
    const rcFile = shell.includes("zsh") ? ".zshrc" : ".bashrc"
    const rcPath = path.join(os.homedir(), rcFile)

    note(
        `Installing from: ${AGENT_SKILLS_HOME}\nTarget RC file: ${rcPath}`,
        "Context"
    )

    const setEnv = await confirm({
        message: `Do you want to add AGENT_SKILLS_HOME to your ${rcFile}?`,
        initialValue: true,
    })

    if (setEnv) {
        const s = spinner()
        s.start("Updating shell config...")
        const exportLine = `\nexport AGENT_SKILLS_HOME="${AGENT_SKILLS_HOME}"\n`
        await fs.appendFile(rcPath, exportLine)
        s.stop("Shell config updated.")
    }

    const selectedCLIs = (await multiselect({
        message: "Select the CLIs you want to link these skills to:",
        options: [
            {value: "gemini", label: "Gemini CLI", hint: "~/.agents/skills"},
            {value: "claude", label: "Claude Code", hint: "~/.claude/skills"},
            {
                value: "opencode",
                label: "OpenCode",
                hint: "~/.config/opencode/skills",
            },
        ],
        required: true,
    })) as string[]

    const s = spinner()
    s.start("Linking skills...")

    const skillsDir = path.join(AGENT_SKILLS_HOME, "skills")
    const skillFiles = await glob("**/SKILL.md", {cwd: skillsDir})

    const targets: Record<string, string> = {
        gemini:
            process.env.AGENT_SKILLS_HOME_GEMINI_CLI ||
            path.join(os.homedir(), ".agents", "skills"),
        claude:
            process.env.AGENT_SKILLS_HOME_CLAUDE_CODE ||
            path.join(os.homedir(), ".claude", "skills"),
        opencode:
            process.env.AGENT_SKILLS_HOME_OPENCODE ||
            path.join(os.homedir(), ".config", "opencode", "skills"),
    }

    for (const cli of selectedCLIs) {
        const targetBase = targets[cli]
        if (!targetBase) continue
        await fs.ensureDir(targetBase)

        // Declarative cleanup: remove any symlink in the target folder that points back to our skills directory
        // This ensures that renamed or deleted skills are automatically purged from the target CLI
        const existingEntries = await fs.readdir(targetBase)
        for (const entry of existingEntries) {
            const entryPath = path.join(targetBase, entry)
            try {
                const stat = await fs.lstat(entryPath)
                if (stat.isSymbolicLink()) {
                    const linkTarget = await fs.readlink(entryPath)
                    const absoluteTarget = path.resolve(
                        path.dirname(entryPath),
                        linkTarget
                    )
                    // If it points inside our local skills repository, remove it
                    if (
                        absoluteTarget.startsWith(skillsDir + path.sep) ||
                        absoluteTarget === skillsDir
                    ) {
                        await fs.remove(entryPath)
                    }
                }
            } catch (err) {
                // Ignore errors reading bad links
            }
        }

        for (const skillFile of skillFiles) {
            const skillPath = path.dirname(path.join(skillsDir, skillFile))
            const skillName = path.basename(skillPath)

            const targetPath = path.join(targetBase, skillName)

            // Because of the declarative sweep above, we don't strictly need this, but it's safe to keep
            await fs.remove(targetPath)

            // Create symlink
            await fs.ensureSymlink(skillPath, targetPath)
        }

        // Also link AGENTS.md to the CLI's expected instruction file if needed
        // For Gemini/Claude/Opencode, they often look for AGENTS.md or CLAUDE.md in their config dirs
        if (cli === "claude") {
            await fs.ensureSymlink(
                path.join(AGENT_SKILLS_HOME, "agents", "AGENTS.md"),
                path.join(os.homedir(), ".claude", "CLAUDE.md")
            )
        }
        if (cli === "gemini") {
            // Gemini often uses ~/.agents/AGENTS.md
            await fs.ensureSymlink(
                path.join(AGENT_SKILLS_HOME, "agents", "AGENTS.md"),
                path.join(os.homedir(), ".agents", "AGENTS.md")
            )
            await fs.ensureSymlink(
                path.join(AGENT_SKILLS_HOME, "agents", "MEMORIES.md"),
                path.join(os.homedir(), ".agents", "MEMORIES.md")
            )
        }
    }

    s.stop("Skills linked successfully.")

    outro(
        "🥒 Done! Restart your terminal or run `source " +
            rcPath +
            "` to finish."
    )
}

install().catch((err) => {
    console.error("Installation failed:", err)
    process.exit(1)
})
