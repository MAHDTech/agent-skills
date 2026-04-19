import fs from "fs-extra"
import path from "path"
import os from "os"
import yaml from "js-yaml"
import {glob} from "glob"
import {execFileSync} from "child_process"
import {intro, outro, confirm, spinner, note, cancel} from "@clack/prompts"

const AGENT_SKILLS_HOME =
    process.env.AGENT_SKILLS_HOME || path.resolve(import.meta.dir, "..")
const SKILLS_DIR = path.join(AGENT_SKILLS_HOME, "skills")
const COMMANDS_DIR = path.join(AGENT_SKILLS_HOME, "commands")
const AGENTS_FILE = path.join(AGENT_SKILLS_HOME, "agents", "AGENTS.md")
const README_FILE = path.join(AGENT_SKILLS_HOME, "README.md")
const DOCS_DIR = path.join(AGENT_SKILLS_HOME, "docs")
const DASHBOARD_CONTENT_DIR = path.join(
    AGENT_SKILLS_HOME,
    "dashboard",
    "content",
    "skills"
)

interface SkillMetadata {
    name: string
    description: string
    triggers?: string[]
    category?: string
    type?: string
    [key: string]: any
}

interface Skill {
    path: string
    metadata: SkillMetadata
    content: string
    isCommand: boolean
}

async function getItems(): Promise<Skill[]> {
    const items: Skill[] = []

    // Process skills
    if (await fs.pathExists(SKILLS_DIR)) {
        const skillFiles = await glob("**/SKILL.md", {cwd: SKILLS_DIR})
        for (const file of skillFiles) {
            const fullPath = path.join(SKILLS_DIR, file)
            const rawContent = await fs.readFile(fullPath, "utf-8")
            const match = rawContent.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/)
            if (match) {
                const metadata = yaml.load(match[1]) as SkillMetadata
                items.push({
                    path: `skills/${file}`,
                    metadata,
                    content: match[2],
                    isCommand: false,
                })
            } else {
                console.warn(`Skill at ${file} is missing YAML frontmatter.`)
            }
        }
    }

    // Process commands
    if (await fs.pathExists(COMMANDS_DIR)) {
        const commandFiles = await glob("**/COMMAND.md", {cwd: COMMANDS_DIR})
        for (const file of commandFiles) {
            const fullPath = path.join(COMMANDS_DIR, file)
            const rawContent = await fs.readFile(fullPath, "utf-8")
            const match = rawContent.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/)
            if (match) {
                const metadata = yaml.load(match[1]) as SkillMetadata
                items.push({
                    path: `commands/${file}`,
                    metadata,
                    content: match[2],
                    isCommand: true,
                })
            } else {
                console.warn(`Command at ${file} is missing YAML frontmatter.`)
            }
        }
    }

    return items
}

async function lint() {
    console.log("Linting skills and commands...")
    const items = await getItems()
    let errors = 0

    for (const item of items) {
        const file = item.path
        try {
            const required = ["name", "description"]
            for (const field of required) {
                if (!item.metadata[field]) {
                    console.error(
                        `❌ ${file}: Missing mandatory field '${field}' in frontmatter.`
                    )
                    errors++
                }
            }

            // Check if folder name matches metadata name
            const folderName = path.dirname(file).split(path.sep).pop()
            if (folderName && folderName !== item.metadata.name) {
                console.warn(
                    `⚠️ ${file}: Folder base name '${folderName}' does not match skill name '${item.metadata.name}'.`
                )
            }
        } catch (e) {
            console.error(`❌ ${file}: Error validating frontmatter.`, e)
            errors++
        }
    }

    if (errors > 0) {
        console.error(`\nFound ${errors} errors.`)
        process.exit(1)
    } else {
        console.log("✅ All skills and commands passed linting!")
    }
}

async function sync() {
    console.log("Syncing skills and commands...")
    const items = await getItems()

    // 1. Update agents/AGENTS.md
    if (await fs.pathExists(AGENTS_FILE)) {
        const agentsContent = await fs.readFile(AGENTS_FILE, "utf-8")
        const agentsFrontmatterMatch = agentsContent.match(
            /^---\n([\s\S]*?)\n---\n/
        )
        const agentsFrontmatter = agentsFrontmatterMatch
            ? agentsFrontmatterMatch[0]
            : ""

        let newAgentsBody = `\n# Available Skills & Commands\n\n`
        items
            .sort((a, b) => a.metadata.name.localeCompare(b.metadata.name))
            .forEach((item) => {
                newAgentsBody += `- **${item.metadata.name}**: ${item.metadata.description}\n`
            })
        await fs.writeFile(AGENTS_FILE, agentsFrontmatter + newAgentsBody)
    }

    // 2. Update README.md
    if (await fs.pathExists(DOCS_DIR)) {
        const docsFiles = await glob("*.md", {cwd: DOCS_DIR})
        let readmeContent = `# 🥒 Agent Skills\n\n`
        readmeContent += `My personal and public agent skills.\n\n`

        readmeContent += `## Table of Contents\n\n`
        docsFiles.sort().forEach((doc) => {
            const name = doc.replace(".md", "")
            readmeContent += `- [${name}](docs/${doc})\n`
        })
        readmeContent += `- [Skills](#available-skills)\n\n`

        readmeContent += `## Available Skills\n\n`
        items.forEach((item) => {
            const triggers = item.metadata.triggers
                ? item.metadata.triggers.join(", ")
                : "-"
            readmeContent += `### [${item.metadata.name}](${item.path})\n\n`
            const desc = item.metadata.description
                ? item.metadata.description.trim()
                : ""
            readmeContent += `**Description**: ${desc}\n\n`
            if (!item.isCommand)
                readmeContent += `**Triggers**: ${triggers}\n\n`
        })

        await fs.writeFile(README_FILE, readmeContent.trim() + "\n")
    }

    // 3. Sync Zola content
    await fs.ensureDir(DASHBOARD_CONTENT_DIR)
    await fs.emptyDir(DASHBOARD_CONTENT_DIR)

    for (const item of items) {
        const hasMermaid = item.content.includes("```mermaid")
        const zolaPath = path.join(
            DASHBOARD_CONTENT_DIR,
            `${item.metadata.name}.md`
        )
        const zolaContent = `+++
title = ${JSON.stringify(item.metadata.name)}
description = ${JSON.stringify(item.metadata.description)}
date = ${new Date().toISOString().split("T")[0]}
[extra]
triggers = ${JSON.stringify(item.metadata.triggers || [])}
mermaid = ${hasMermaid}
is_command = ${item.isCommand}
+++

${item.content}
`
        await fs.writeFile(zolaPath, zolaContent)
    }

    const sectionIndexContent = `+++
title = "Skills Catalog"
sort_by = "title"
template = "section.html"
weight = 1
+++

Welcome to the agent skills catalog.
`
    await fs.writeFile(
        path.join(DASHBOARD_CONTENT_DIR, "_index.md"),
        sectionIndexContent
    )

    // 4. Git Add
    try {
        execFileSync(
            "git",
            ["add", AGENTS_FILE, README_FILE, DASHBOARD_CONTENT_DIR],
            {stdio: "inherit"}
        )
        console.log("Changes staged to git.")
    } catch (e) {
        console.warn(
            "Could not stage changes to git (maybe not a git repo or no changes)."
        )
    }

    console.log("Done!")
}

async function linkSkills(quiet = false) {
    const opencodeTarget =
        process.env.AGENT_SKILLS_HOME_OPENCODE ||
        path.join(os.homedir(), ".config", "opencode", "skills")

    // Support cleaning up legacy ~/.agents/skills folder during sync as well
    const legacyTarget = path.join(os.homedir(), ".agents", "skills")

    await fs.ensureDir(opencodeTarget)

    let removed = 0
    let added = 0

    // Cleanup ALL symlinks in the opencode skills directory and legacy .agents directory that point to ANY folder inside AGENT_SKILLS_HOME
    for (const target of [opencodeTarget, legacyTarget]) {
        if (!(await fs.pathExists(target))) continue

        const existingEntries = await fs.readdir(target)
        for (const entry of existingEntries) {
            const entryPath = path.join(target, entry)
            try {
                const stat = await fs.lstat(entryPath)
                if (stat.isSymbolicLink()) {
                    const linkTarget = await fs.readlink(entryPath)
                    const absoluteTarget = path.resolve(
                        path.dirname(entryPath),
                        linkTarget
                    )
                    // If the symlink points anywhere inside AGENT_SKILLS_HOME, remove it
                    if (absoluteTarget.startsWith(AGENT_SKILLS_HOME)) {
                        await fs.remove(entryPath)
                        removed++
                    }
                }
            } catch (err) {}
        }
    }

    // Link Skills
    if (await fs.pathExists(SKILLS_DIR)) {
        const skillDirs = await fs.readdir(SKILLS_DIR)
        for (const dir of skillDirs) {
            const skillPath = path.join(SKILLS_DIR, dir)
            const stat = await fs.stat(skillPath)
            if (stat.isDirectory()) {
                const targetPath = path.join(opencodeTarget, dir)
                await fs.remove(targetPath)
                await fs.ensureSymlink(skillPath, targetPath)
                added++
            }
        }
    }

    // We do NOT link commands here. OpenCode commands are meant to be injected via _config.command in a plugin.

    if (!quiet) {
        console.log(
            `✅ Synced skills to OpenCode (${added} linked, ${removed} old links removed).`
        )
    }
}

async function unlinkSkills() {
    // Opencode target
    const opencodeTarget =
        process.env.AGENT_SKILLS_HOME_OPENCODE ||
        path.join(os.homedir(), ".config", "opencode", "skills")

    // Legacy agents target
    const agentsTarget = path.join(os.homedir(), ".agents", "skills")

    const targets = [opencodeTarget, agentsTarget]
    let removed = 0

    for (const target of targets) {
        if (!(await fs.pathExists(target))) {
            continue
        }

        const existingEntries = await fs.readdir(target)
        for (const entry of existingEntries) {
            const entryPath = path.join(target, entry)
            try {
                const stat = await fs.lstat(entryPath)
                if (stat.isSymbolicLink()) {
                    const linkTarget = await fs.readlink(entryPath)
                    const absoluteTarget = path.resolve(
                        path.dirname(entryPath),
                        linkTarget
                    )
                    if (absoluteTarget.startsWith(AGENT_SKILLS_HOME)) {
                        await fs.remove(entryPath)
                        removed++
                    }
                }
            } catch (err) {}
        }
    }

    console.log(
        `✅ Uninstalled ${removed} skills from OpenCode and legacy agent targets.`
    )
}

async function syncAction() {
    console.log("Syncing skills documentation and configurations...")
    await sync() // Call the existing doc sync
    await linkSkills(false) // Link symmetrically
}

async function uninstallAction() {
    intro("🥒 OpenCode Agent Skills Uninstaller")
    await unlinkSkills()
    outro("🥒 Done! Skills have been unlinked from OpenCode.")
}

async function install() {
    intro("🥒 OpenCode Agent Skills Installer")

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

    const s = spinner()
    s.start("Linking skills to OpenCode...")

    await linkSkills(true)

    s.stop("Skills linked successfully to OpenCode.")
    outro(
        "🥒 Done! Restart your terminal or run `source " +
            rcPath +
            "` to finish."
    )
}

import {parseArgs} from "util"

const {values: options} = parseArgs({
    args: process.argv.slice(2),
    options: {
        action: {
            type: "string",
            short: "a",
        },
    },
    strict: false,
})

const action = options.action || process.argv[2]

if (action === "lint") {
    lint().catch(console.error)
} else if (action === "sync") {
    syncAction().catch(console.error)
} else if (action === "install") {
    install().catch(console.error)
} else if (action === "uninstall") {
    uninstallAction().catch(console.error)
} else {
    console.log(
        "Usage: bun run bin/skills.ts --action <lint|sync|install|uninstall>"
    )
    process.exit(1)
}
