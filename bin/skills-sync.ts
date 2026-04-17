import fs from "fs-extra"
import path from "path"
import yaml from "js-yaml"
import {glob} from "glob"
import {execSync} from "child_process"

const AGENT_SKILLS_HOME = process.env.AGENT_SKILLS_HOME || process.cwd()
const SKILLS_DIR = path.join(AGENT_SKILLS_HOME, "skills")
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
    [key: string]: any
}

interface Skill {
    path: string
    metadata: SkillMetadata
    content: string
}

async function sync() {
    console.log("Syncing skills...")

    const skillFiles = await glob("**/SKILL.md", {cwd: SKILLS_DIR})
    const skills: Skill[] = []

    for (const file of skillFiles) {
        const fullPath = path.join(SKILLS_DIR, file)
        const rawContent = await fs.readFile(fullPath, "utf-8")
        const match = rawContent.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/)

        if (match) {
            const metadata = yaml.load(match[1]) as SkillMetadata
            const content = match[2]
            skills.push({path: file, metadata, content})
        } else {
            console.warn(`Skill at ${file} is missing YAML frontmatter.`)
        }
    }

    // 1. Update agents/AGENTS.md
    let agentsContent = await fs.readFile(AGENTS_FILE, "utf-8")
    const agentsFrontmatterMatch = agentsContent.match(
        /^---\n([\s\S]*?)\n---\n/
    )
    const agentsFrontmatter = agentsFrontmatterMatch
        ? agentsFrontmatterMatch[0]
        : ""

    let newAgentsBody = `\n# Available Skills\n\n`
    skills
        .sort((a, b) => a.metadata.name.localeCompare(b.metadata.name))
        .forEach((skill) => {
            newAgentsBody += `- **${skill.metadata.name}**: ${skill.metadata.description}\n`
        })

    await fs.writeFile(AGENTS_FILE, agentsFrontmatter + newAgentsBody)

    // 2. Update README.md
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
    skills.forEach((skill) => {
        const triggers = skill.metadata.triggers
            ? skill.metadata.triggers.join(", ")
            : "-"
        readmeContent += `### [${skill.metadata.name}](skills/${skill.path})\n\n`
        const desc = skill.metadata.description
            ? skill.metadata.description.trim()
            : ""
        readmeContent += `**Description**: ${desc}\n\n`
        readmeContent += `**Triggers**: ${triggers}\n\n`
    })

    await fs.writeFile(README_FILE, readmeContent.trim() + "\n")

    // 3. Sync Zola content
    await fs.ensureDir(DASHBOARD_CONTENT_DIR)
    // Clear old content
    await fs.emptyDir(DASHBOARD_CONTENT_DIR)

    for (const skill of skills) {
        const hasMermaid = skill.content.includes("```mermaid")
        const zolaPath = path.join(
            DASHBOARD_CONTENT_DIR,
            `${skill.metadata.name}.md`
        )
        const zolaContent = `+++
title = ${JSON.stringify(skill.metadata.name)}
description = ${JSON.stringify(skill.metadata.description)}
date = ${new Date().toISOString().split("T")[0]}
[extra]
triggers = ${JSON.stringify(skill.metadata.triggers || [])}
mermaid = ${hasMermaid}
+++

${skill.content}
`
        await fs.writeFile(zolaPath, zolaContent)
    }

    // Persist a section index so Zola lists the skills and instructions
    const sectionIndexContent = `+++
title = "Skills Catalog"
sort_by = "title"
template = "section.html"
weight = 1
+++

Welcome to the agent skills catalog.

To install any of these skills into your project, run:
\`\`\`bash
npx skills add MAHDTech/agent-skills
\`\`\`
And select the skill from the interactive prompt.
`
    await fs.writeFile(
        path.join(DASHBOARD_CONTENT_DIR, "_index.md"),
        sectionIndexContent
    )

    // 4. Git Add
    try {
        execSync(
            `git add ${AGENTS_FILE} ${README_FILE} ${DASHBOARD_CONTENT_DIR}`,
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

sync().catch(console.error)
