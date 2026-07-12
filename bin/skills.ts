import fs from "fs-extra"
import path from "path"
import os from "os"
import * as yaml from "js-yaml"
import {glob} from "glob"
import {execFileSync} from "child_process"
import {intro, outro, log} from "@clack/prompts"

/**
 * Skills manager (personal dev tool).
 *
 * Skills live at `skills/<category>/<name>/SKILL.md`. Categories are encoded by
 * the directory tree, not by name prefixes. Consumers install via the skills.sh
 * CLI (`npx skills add MAHDTech/agent-skills`); this script is used by the repo
 * owner to lint, regenerate docs/dashboard, and symlink the working tree into
 * the local agent tools for live iteration.
 */

const GITHUB_SOURCE = "MAHDTech/agent-skills"

// Category metadata. Order here is the display order. `lifecycle: true` buckets
// (drafts / retired) are excluded from all generated, published output.
interface CategoryInfo {
    title: string
    description: string
    lifecycle?: boolean
}

const CATEGORIES: Record<string, CategoryInfo> = {
    engineering: {
        title: "Engineering",
        description: "The core build, debug, and delivery loop.",
    },
    "game-development": {
        title: "Game Development",
        description: "Game engines and game-development workflows.",
    },
    planning: {
        title: "Planning",
        description: "Turn ideas into specs, tickets, and multi-session plans.",
    },
    review: {
        title: "Review",
        description: "Review diffs, pull requests, and test plans.",
    },
    github: {
        title: "GitHub",
        description: "GitHub and git workflows via the gh CLI.",
    },
    reflection: {
        title: "Reflection",
        description: "Self-critique and review of your own work.",
    },
    writing: {
        title: "Writing",
        description: "Proofreading and documentation polish.",
    },
    authoring: {
        title: "Authoring",
        description: "Create and maintain the skills themselves.",
    },
    tooling: {
        title: "Tooling",
        description: "Environments, CLIs, and agent conventions.",
    },
    "in-progress": {
        title: "In Progress",
        description: "Drafts not yet promoted.",
        lifecycle: true,
    },
    deprecated: {
        title: "Deprecated",
        description: "Retired skills kept for reference.",
        lifecycle: true,
    },
}

const NAME_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/

const AGENT_SKILLS_HOME =
    process.env.AGENT_SKILLS_HOME || path.resolve(import.meta.dir, "..")
const SKILLS_DIR = path.join(AGENT_SKILLS_HOME, "skills")
const AGENTS_FILE = path.join(AGENT_SKILLS_HOME, "agents", "AGENTS.md")
const README_FILE = path.join(AGENT_SKILLS_HOME, "README.md")
const SKILLS_SH_FILE = path.join(AGENT_SKILLS_HOME, "skills.sh.json")
const DOCS_DIR = path.join(AGENT_SKILLS_HOME, "docs")
const DASHBOARD_CONTENT_DIR = path.join(
    AGENT_SKILLS_HOME,
    "dashboard",
    "content",
    "skills"
)

interface SkillMetadata {
    name?: string
    description?: string
    "disable-model-invocation"?: boolean
    "argument-hint"?: string
    context?: string
    agent?: string
    metadata?: Record<string, string>
    [key: string]: any
}

interface Skill {
    /** Repo-relative path to the SKILL.md file. */
    path: string
    /** Directory basename (the canonical skill name). */
    dirName: string
    /** Top-level bucket the skill lives in. */
    category: string
    /** False for lifecycle buckets (in-progress / deprecated). */
    promoted: boolean
    metadata: SkillMetadata
    content: string
}

function logTask(msg: string) {
    log.step(`✅ ${msg}`)
}

function printSummary(
    title: string,
    metrics: {label: string; value: number | string}[]
) {
    log.message(`  ═══ 📊  ${title}  ═══`)
    metrics.forEach((m) => {
        log.message(`      ✨ ${m.label}: ${m.value}`)
    })
}

/**
 * Rewrite relative markdown links to sibling `.md` files into Zola internal
 * (`@/…`) links so they resolve under Zola's pretty URLs on the dashboard.
 * `SKILL.md` targets map to the skill section's `_index.md`. A link whose target
 * doesn't exist in the source is left untouched, so the Zola build never breaks
 * on an unknown `@/` path.
 */
function rewriteSkillLinks(
    content: string,
    contentBase: string,
    srcDir: string
): string {
    return content.replace(
        /\]\((?!https?:\/\/|@\/|#|mailto:|\/)([^)#\s]+\.md)(#[^)\s]*)?\)/g,
        (match, relPath: string, anchor?: string) => {
            if (!fs.existsSync(path.join(srcDir, relPath))) return match
            const rel = path.posix
                .normalize(path.posix.join(contentBase, relPath))
                .replace(/\/SKILL\.md$/, "/_index.md")
            return `](@/${rel}${anchor || ""})`
        }
    )
}

async function getSkills(): Promise<Skill[]> {
    const skills: Skill[] = []
    if (!(await fs.pathExists(SKILLS_DIR))) return skills

    const files = await glob("**/SKILL.md", {cwd: SKILLS_DIR})
    for (const file of files.sort()) {
        const fullPath = path.join(SKILLS_DIR, file)
        const raw = await fs.readFile(fullPath, "utf-8")
        const match = raw.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/)
        if (!match || !match[1] || !match[2]) {
            console.warn(`Skill at ${file} is missing YAML frontmatter.`)
            continue
        }
        const metadata = (yaml.load(match[1]) as SkillMetadata) || {}
        const parts = file.split(path.sep)
        // Expected shape: <category>/<name>/SKILL.md
        const category = parts.length >= 3 ? parts[0]! : "uncategorized"
        const dirName = parts[parts.length - 2]!
        const promoted = !CATEGORIES[category]?.lifecycle
        skills.push({
            path: `skills/${file}`,
            dirName,
            category,
            promoted,
            metadata,
            content: match[2],
        })
    }
    return skills
}

/** Promoted skills grouped by category, in CATEGORIES display order. */
function groupByCategory(skills: Skill[]): [string, Skill[]][] {
    const groups: [string, Skill[]][] = []
    for (const key of Object.keys(CATEGORIES)) {
        if (CATEGORIES[key]!.lifecycle) continue
        const inCat = skills
            .filter((s) => s.promoted && s.category === key)
            .sort((a, b) => a.dirName.localeCompare(b.dirName))
        if (inCat.length) groups.push([key, inCat])
    }
    return groups
}

async function lint() {
    intro("Linting Skills")
    const skills = await getSkills()
    let errors = 0

    for (const skill of skills) {
        const file = skill.path
        const name = skill.metadata.name

        if (!name) {
            log.error(`❌ ${file}: missing mandatory field 'name'.`)
            errors++
        }
        if (!skill.metadata.description) {
            log.error(`❌ ${file}: missing mandatory field 'description'.`)
            errors++
        }
        if (name && name !== skill.dirName) {
            log.error(
                `❌ ${file}: name '${name}' must equal directory '${skill.dirName}'.`
            )
            errors++
        }
        if (name && !NAME_RE.test(name)) {
            log.error(
                `❌ ${file}: name '${name}' must be lowercase kebab-case ([a-z0-9-], no leading/trailing/double hyphens).`
            )
            errors++
        }
        if (name && name.length > 64) {
            log.error(`❌ ${file}: name '${name}' exceeds 64 characters.`)
            errors++
        }
        if (name && /anthropic|claude/i.test(name)) {
            log.error(
                `❌ ${file}: name '${name}' must not contain 'anthropic' or 'claude'.`
            )
            errors++
        }
        if (
            skill.metadata.description &&
            skill.metadata.description.length > 1024
        ) {
            log.error(`❌ ${file}: description exceeds 1024 characters.`)
            errors++
        }
        if (skill.category === "uncategorized") {
            log.error(
                `❌ ${file}: skill must live under skills/<category>/<name>/SKILL.md.`
            )
            errors++
        } else if (!CATEGORIES[skill.category]) {
            log.warn(
                `⚠️ ${file}: unknown category '${skill.category}' (not declared in CATEGORIES).`
            )
        }
    }

    if (errors > 0) {
        log.error(`\nFound ${errors} error(s).`)
        process.exit(1)
    }
    logTask(`All ${skills.length} skills passed linting!`)
    outro("Done!")
}

function readmeBadge(): string {
    return `[![skills.sh](https://skills.sh/b/${GITHUB_SOURCE})](https://skills.sh/${GITHUB_SOURCE})`
}

async function sync() {
    const skills = await getSkills()
    const grouped = groupByCategory(skills)

    // 1. agents/AGENTS.md — regenerate the skill index, preserve frontmatter.
    if (await fs.pathExists(AGENTS_FILE)) {
        const agentsContent = await fs.readFile(AGENTS_FILE, "utf-8")
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
    let readme = `# Agent Skills\n\n`
    readme += `${readmeBadge()}\n\n`
    readme +=
        `My personal and public agent skills — cross-compatible with ` +
        `Claude Code, OpenCode, Goose, and Antigravity CLI.\n\n`
    readme += `## Install\n\n`
    readme += "```bash\n" + `npx skills add ${GITHUB_SOURCE}\n` + "```\n\n"
    if (await fs.pathExists(DOCS_DIR)) {
        const docsFiles = (await glob("*.md", {cwd: DOCS_DIR})).sort()
        if (docsFiles.length) {
            readme += `## Documentation\n\n`
            for (const doc of docsFiles) {
                readme += `- [${doc.replace(".md", "")}](docs/${doc})\n`
            }
            readme += `\n`
        }
    }
    readme += `## Available Skills\n`
    for (const [key, list] of grouped) {
        readme += `\n### ${CATEGORIES[key]!.title}\n\n`
        readme += `${CATEGORIES[key]!.description}\n\n`
        for (const s of list) {
            const desc = (s.metadata.description || "").trim()
            readme += `- **[${s.dirName}](${s.path})** — ${desc}\n`
        }
    }
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

            // Skill (SKILL.md) → section _index.md, rendered by skill.html and
            // shown in the sidebar.
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

            // Sibling reference files (flat *.md) → pages in the skill section.
            // Marked skill = false so the sidebar (skills-only) skips them; they
            // remain reachable via in-content links and search.
            const siblings = (await fs.readdir(skillSrcDir)).filter(
                (f) => f.endsWith(".md") && f !== "SKILL.md"
            )
            for (const file of siblings.sort()) {
                const raw = await fs.readFile(
                    path.join(skillSrcDir, file),
                    "utf-8"
                )
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
+++

${body}
`
                )
            }
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
    } catch (e) {
        log.warn("Could not stage changes to git (no repo or no changes).")
    }
}

// ── Local install (symlink the working tree into the agent tools) ───────────

const HOME = os.homedir()
const AGENTS_HUB =
    process.env.AGENT_SKILLS_AGENTS_DIR || path.join(HOME, ".agents", "skills")
const CLAUDE_SKILLS = path.join(HOME, ".claude", "skills")
const ANTIGRAVITY_CONFIG_DIR = path.join(HOME, ".gemini", "config")
const ANTIGRAVITY_SKILLS_JSON = path.join(ANTIGRAVITY_CONFIG_DIR, "skills.json")

// Every directory an agent tool reads skills from. Cleanup sweeps all of these
// for stale links so renamed/removed skills never leave orphans behind — no
// matter which tool created them.
const MANAGED_SKILL_DIRS = [
    AGENTS_HUB,
    CLAUDE_SKILLS,
    path.join(HOME, ".config", "opencode", "skills"),
    path.join(HOME, ".config", "opencode", "commands"),
    path.join(HOME, ".config", "goose", "skills"),
    path.join(HOME, ".gemini", "antigravity-cli", "skills"),
    path.join(HOME, ".gemini", "antigravity", "skills"),
    path.join(HOME, ".gemini", "skills"),
]

interface Detected {
    claude: boolean
    opencode: boolean
    goose: boolean
    antigravity: boolean
}

function detectTools(): Detected {
    return {
        claude: fs.existsSync(path.join(HOME, ".claude")),
        opencode: fs.existsSync(path.join(HOME, ".config", "opencode")),
        goose: fs.existsSync(path.join(HOME, ".config", "goose")),
        antigravity: fs.existsSync(path.join(HOME, ".gemini")),
    }
}

/**
 * Remove symlinks in `dir` that resolve to somewhere inside this repo.
 * Real directories/files (skills you hand-copy to test) and symlinks pointing
 * anywhere else are left untouched — this only reclaims links we own, so it is
 * safe to run repeatedly.
 */
async function cleanStaleLinks(dir: string): Promise<number> {
    if (!(await fs.pathExists(dir))) return 0
    let removed = 0
    for (const entry of await fs.readdir(dir)) {
        const entryPath = path.join(dir, entry)
        try {
            const stat = await fs.lstat(entryPath)
            if (!stat.isSymbolicLink()) continue
            const target = path.resolve(
                path.dirname(entryPath),
                await fs.readlink(entryPath)
            )
            if (target.startsWith(AGENT_SKILLS_HOME)) {
                await fs.remove(entryPath)
                removed++
            }
        } catch {
            // Ignore unreadable entries.
        }
    }
    return removed
}

async function registerAntigravity() {
    await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
    let config: {entries?: {path: string}[]; [k: string]: any} = {}
    if (await fs.pathExists(ANTIGRAVITY_SKILLS_JSON)) {
        try {
            config = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
        } catch {
            config = {}
        }
    }
    const entries = config.entries || []
    if (!entries.some((e) => path.resolve(e.path) === AGENTS_HUB)) {
        entries.push({path: AGENTS_HUB})
    }
    config.entries = entries
    if (!config.$schema) {
        config.$schema = "https://skills.sh/schemas/skills.sh.schema.json"
    }
    await fs.writeJson(ANTIGRAVITY_SKILLS_JSON, config, {spaces: 2})
}

async function linkSkills() {
    const tools = detectTools()
    // Only promoted skills are installed; lifecycle buckets (in-progress /
    // deprecated) are excluded, mirroring sync's published output so a retired
    // skill stops being loadable once install is re-run.
    const skills = (await getSkills()).filter((s) => s.promoted)

    // Which flattened hubs get real links.
    const linkTargets: string[] = []
    if (tools.claude) linkTargets.push(CLAUDE_SKILLS)
    if (tools.opencode || tools.goose || tools.antigravity)
        linkTargets.push(AGENTS_HUB)

    // Clean stale repo links across every tool dir before relinking.
    let removed = 0
    for (const t of MANAGED_SKILL_DIRS) removed += await cleanStaleLinks(t)

    let linked = 0
    for (const target of linkTargets) {
        await fs.ensureDir(target)
        for (const skill of skills) {
            const src = path.join(AGENT_SKILLS_HOME, skill.path, "..")
            const dest = path.join(target, skill.dirName)
            await fs.remove(dest)
            await fs.ensureSymlink(path.resolve(src), dest)
            linked++
        }
    }

    if (tools.antigravity) await registerAntigravity()

    return {
        removed,
        linked,
        skills: skills.length,
        tools,
        targets: linkTargets,
    }
}

async function unlinkSkills() {
    let removed = 0
    for (const t of MANAGED_SKILL_DIRS) removed += await cleanStaleLinks(t)

    // De-register the Antigravity hub entry.
    if (await fs.pathExists(ANTIGRAVITY_SKILLS_JSON)) {
        try {
            const config = await fs.readJson(ANTIGRAVITY_SKILLS_JSON)
            if (Array.isArray(config.entries)) {
                config.entries = config.entries.filter(
                    (e: {path: string}) => path.resolve(e.path) !== AGENTS_HUB
                )
                await fs.writeJson(ANTIGRAVITY_SKILLS_JSON, config, {spaces: 2})
            }
        } catch {
            // Ignore malformed config.
        }
    }
    return {removed}
}

function toolList(tools: Detected): string {
    return (
        Object.entries(tools)
            .filter(([, v]) => v)
            .map(([k]) => k)
            .join(", ") || "none"
    )
}

/** `install` — link the working tree into detected tools. Idempotent. */
async function install() {
    intro("Agent Skills Installer")
    const m = await linkSkills()
    printSummary("Installation Summary", [
        {label: "Skills", value: m.skills},
        {label: "Symlinks created", value: m.linked},
        {label: "Stale links removed", value: m.removed},
        {label: "Tools detected", value: toolList(m.tools)},
    ])
    outro("Done! Skills linked into your detected agent tools.")
}

/** `uninstall` — remove every symlink we own + deregister Antigravity. */
async function uninstallAction() {
    intro("Agent Skills Uninstaller")
    const m = await unlinkSkills()
    printSummary("Uninstallation Summary", [
        {label: "Symlinks removed", value: m.removed},
    ])
    outro("Done! Skills unlinked.")
}

/**
 * `sync` — make everything current in one shot: refresh the machine symlinks
 * (uninstall + reinstall) and regenerate the repo's generated files.
 *
 * In automated contexts (pre-commit hooks, CI, dashboard builds) it regenerates
 * files ONLY and never touches your machine — signalled by PRE_COMMIT, CI, or
 * SKILLS_REPO_ONLY in the environment.
 */
async function syncAction() {
    intro("Agent Skills Sync")
    const repoOnly = !!(
        process.env.PRE_COMMIT ||
        process.env.CI ||
        process.env.SKILLS_REPO_ONLY
    )
    if (!repoOnly) {
        await unlinkSkills()
        const m = await linkSkills()
        printSummary("Machine Sync", [
            {label: "Skills linked", value: m.linked},
            {label: "Stale links removed", value: m.removed},
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

import {parseArgs} from "util"

const {values: options} = parseArgs({
    args: process.argv.slice(2),
    options: {action: {type: "string", short: "a"}},
    strict: false,
})

const action = (options.action as string) || process.argv[2]

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
