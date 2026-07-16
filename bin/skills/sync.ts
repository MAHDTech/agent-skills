import fs from "fs-extra"
import path from "path"
import {glob} from "glob"
import {execFileSync} from "child_process"
import {intro, outro, log} from "@clack/prompts"
import {
    getSkills,
    groupByCategory,
    CATEGORIES,
    rewriteSkillLinks,
    logTask,
    printSummary,
    unlinkSkills,
    linkSkills,
    toolList,
    AGENTS_FILE,
    README_FILE,
    SKILLS_SH_FILE,
    DASHBOARD_CONTENT_DIR,
    DOCS_DIR,
    GITHUB_SOURCE,
    SKILLS_DIR,
} from "./lib.ts"

function readmeBadge(): string {
    return `[![skills.sh](https://skills.sh/b/${GITHUB_SOURCE})](https://skills.sh/${GITHUB_SOURCE})`
}

export function skillIssuesImage(): string {
    return `![skill issues](./docs/images/skill-issues.png)`
}

async function syncResources(
    src: string,
    dest: string,
    contentBase: string,
    key: string,
    skillName: string
) {
    if (!(await fs.pathExists(src))) return
    await fs.ensureDir(dest)
    await fs.writeFile(
        path.join(dest, "_index.md"),
        `+++\nrender = false\n+++\n`
    )
    const entries = await fs.readdir(src, {withFileTypes: true})
    for (const entry of entries) {
        const srcPath = path.join(src, entry.name)
        const destPath = path.join(dest, entry.name)
        if (entry.isDirectory()) {
            await syncResources(
                srcPath,
                destPath,
                path.posix.join(contentBase, entry.name),
                key,
                skillName
            )
        } else if (entry.isFile()) {
            if (entry.name.endsWith(".md")) {
                const raw = (await fs.readFile(srcPath, "utf-8")).replace(
                    /\r\n/g,
                    "\n"
                )
                const body = rewriteSkillLinks(raw, contentBase, src)
                const sibMermaid = body.includes("```mermaid")
                await fs.writeFile(
                    destPath,
                    `+++
title = ${JSON.stringify(entry.name.replace(/\.md$/, ""))}
[extra]
skill = false
category = ${JSON.stringify(key)}
mermaid = ${sibMermaid}
skill_name = ${JSON.stringify(skillName)}
+++

${body}
`
                )
            } else {
                await fs.copy(srcPath, destPath)
            }
        }
    }
}

export async function sync() {
    const skills = await getSkills()
    if (skills.some((s) => s.yamlError)) {
        throw new Error(
            "Cannot sync: One or more skills contain YAML frontmatter errors."
        )
    }
    const grouped = groupByCategory(skills)

    // 1. agents/AGENTS.md — regenerate the skill index, preserve frontmatter.
    if (await fs.pathExists(AGENTS_FILE)) {
        const agentsContent = (await fs.readFile(AGENTS_FILE, "utf-8")).replace(
            /\r\n/g,
            "\n"
        )
        const fm = agentsContent.match(/^---\n[\s\S]*?\n---\n/)
        const frontmatter = fm ? fm[0] : ""

        let body = `\n# Available Skills\n`
        for (const [key, list] of grouped) {
            body += `\n## ${CATEGORIES[key]!.title}\n\n`
            for (const s of list) {
                body += `- **${s.dirName}**: ${s.metadata.description}\n`
            }
        }
        await fs.writeFile(AGENTS_FILE, frontmatter + body)
        logTask("Updated agents/AGENTS.md index.")
    }

    // 2. README.md — badge, install, and a categorised catalog.
    const docsList = (await fs.pathExists(DOCS_DIR))
        ? (await glob("*.md", {cwd: DOCS_DIR})).sort()
        : []
    const docsContent = docsList.length
        ? `## Documentation\n\n${docsList.map((doc) => `- [${doc.replace(".md", "")}](docs/${doc})`).join("\n")}\n\n`
        : ""

    const catalogContent = grouped
        .map(([key, list]) => {
            const title = CATEGORIES[key]!.title
            const description = CATEGORIES[key]!.description
            const skillsList = list
                .map((s) => {
                    const desc = (s.metadata.description || "").trim()
                    return `- **[${s.dirName}](${s.path})** — ${desc}`
                })
                .join("\n")
            return `### ${title}\n\n${description}\n\n${skillsList}`
        })
        .join("\n\n")

    const readme = `# Agent Skills

${readmeBadge()}

Working on my _skill issues_.

${skillIssuesImage()}

These are my personal agent skills and attempt to be cross-compatible with Antigravity, Claude Code, Goose and OpenCode.

## Install

\`\`\`bash
# Using npm
npx skills add ${GITHUB_SOURCE}

# Using Bun
bunx skills add ${GITHUB_SOURCE}
\`\`\`

${docsContent}## Available Skills

${catalogContent}
`

    await fs.writeFile(README_FILE, readme.trimEnd() + "\n")
    logTask("Updated README.md catalog.")

    // 3. skills.sh.json — display groupings for the skills.sh repo page.
    const skillsSh = {
        $schema: "https://skills.sh/schemas/skills.sh.schema.json",
        notGrouped: "bottom",
        groupings: grouped.map(([key, list]) => ({
            title: CATEGORIES[key]!.title,
            description: CATEGORIES[key]!.description,
            skills: list.map((s) => s.dirName),
        })),
    }
    await fs.writeJson(SKILLS_SH_FILE, skillsSh, {spaces: 2})
    logTask("Updated skills.sh.json groupings.")

    // 4. Zola dashboard content — mirror the category tree so the theme groups
    //    skills into collapsible sections automatically.
    await fs.ensureDir(DASHBOARD_CONTENT_DIR)
    await fs.emptyDir(DASHBOARD_CONTENT_DIR)
    await fs.writeFile(
        path.join(DASHBOARD_CONTENT_DIR, "_index.md"),
        `+++\ntitle = "Skills Catalog"\nsort_by = "title"\ntemplate = "section.html"\nweight = 1\n+++\n\nWelcome to the agent skills catalog.\n`
    )
    let weight = 1
    for (const [key, list] of grouped) {
        const catDir = path.join(DASHBOARD_CONTENT_DIR, key)
        await fs.ensureDir(catDir)
        await fs.writeFile(
            path.join(catDir, "_index.md"),
            `+++\ntitle = ${JSON.stringify(CATEGORIES[key]!.title)}\ndescription = ${JSON.stringify(CATEGORIES[key]!.description)}\nsort_by = "title"\ntemplate = "section.html"\nweight = ${weight++}\n+++\n`
        )
        for (const s of list) {
            const skillSrcDir = path.join(SKILLS_DIR, key, s.dirName)
            const contentBase = `skills/${key}/${s.dirName}`
            const outDir = path.join(catDir, s.dirName)
            await fs.ensureDir(outDir)

            const skillBody = rewriteSkillLinks(
                s.content,
                contentBase,
                skillSrcDir
            )
            const skillMermaid = skillBody.includes("```mermaid")
            await fs.writeFile(
                path.join(outDir, "_index.md"),
                `+++
title = ${JSON.stringify(s.dirName)}
description = ${JSON.stringify(s.metadata.description || "")}
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = ${JSON.stringify(key)}
mermaid = ${skillMermaid}
+++

${skillBody}
`
            )

            const siblings = (await fs.readdir(skillSrcDir)).filter(
                (f) => f.endsWith(".md") && f !== "SKILL.md"
            )
            for (const file of siblings.sort()) {
                const raw = (
                    await fs.readFile(path.join(skillSrcDir, file), "utf-8")
                ).replace(/\r\n/g, "\n")
                const body = rewriteSkillLinks(raw, contentBase, skillSrcDir)
                const sibMermaid = body.includes("```mermaid")
                await fs.writeFile(
                    path.join(outDir, file),
                    `+++
title = ${JSON.stringify(file.replace(/\.md$/, ""))}
[extra]
skill = false
category = ${JSON.stringify(key)}
mermaid = ${sibMermaid}
skill_name = ${JSON.stringify(s.dirName)}
+++

${body}
`
                )
            }

            const resourcesSrc = path.join(skillSrcDir, "resources")
            const resourcesDest = path.join(outDir, "resources")
            await syncResources(
                resourcesSrc,
                resourcesDest,
                path.posix.join(contentBase, "resources"),
                key,
                s.dirName
            )
        }
    }
    logTask("Generated dashboard content.")

    // 5. Stage regenerated files.
    try {
        execFileSync(
            "git",
            [
                "add",
                AGENTS_FILE,
                README_FILE,
                SKILLS_SH_FILE,
                DASHBOARD_CONTENT_DIR,
            ],
            {stdio: "inherit"}
        )
        logTask("Staged generated files to git.")
    } catch {
        log.warn("Could not stage changes to git (no repo or no changes).")
    }
}

export async function syncAction() {
    intro("Agent Skills Sync")
    const repoOnly = !!(
        process.env.PRE_COMMIT ||
        process.env.CI ||
        process.env.SKILLS_REPO_ONLY
    )
    if (!repoOnly) {
        const un = await unlinkSkills()
        const m = await linkSkills()
        printSummary("Machine Sync", [
            {label: "Skills linked", value: m.linked},
            {label: "Stale links removed", value: un.removed + m.removed},
            {label: "Tools", value: toolList(m.tools)},
        ])
    }
    await sync()
    outro(
        repoOnly
            ? "Done! Generated files regenerated."
            : "Done! Machine relinked and generated files regenerated."
    )
}
