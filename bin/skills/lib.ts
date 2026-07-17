import fs from "fs-extra"
import path from "path"
import os from "os"
import * as yaml from "js-yaml"
import {glob} from "glob"
import {log} from "@clack/prompts"

export const GITHUB_SOURCE = "MAHDTech/agent-skills"

export interface CategoryInfo {
    title: string
    description: string
    lifecycle?: boolean
}

export const CATEGORIES: Record<string, CategoryInfo> = {
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

export const NAME_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/

export const AGENT_SKILLS_HOME =
    process.env.AGENT_SKILLS_HOME || path.resolve(import.meta.dir, "..", "..")
export const SKILLS_DIR = path.join(AGENT_SKILLS_HOME, "skills")
export const AGENTS_FILE = path.join(AGENT_SKILLS_HOME, "agents", "AGENTS.md")
export const README_FILE = path.join(AGENT_SKILLS_HOME, "README.md")
export const SKILLS_SH_FILE = path.join(AGENT_SKILLS_HOME, "skills.sh.json")
export const DOCS_DIR = path.join(AGENT_SKILLS_HOME, "docs")
export const DASHBOARD_CONTENT_DIR = path.join(
    AGENT_SKILLS_HOME,
    "dashboard",
    "content",
    "skills"
)

export interface SkillMetadata {
    name?: string
    description?: string
    resources?: string[]
    "disable-model-invocation"?: boolean
    "argument-hint"?: string
    context?: string
    agent?: string
    metadata?: Record<string, string>
    [key: string]: any
}

export interface Skill {
    path: string
    dirName: string
    category: string
    promoted: boolean
    metadata: SkillMetadata
    content: string
    yamlError?: string
    raw: string
}

export function logTask(msg: string) {
    log.step(`✅ ${msg}`)
}

export function printSummary(
    title: string,
    metrics: {label: string; value: number | string}[]
) {
    log.message(`  ═══ 📊  ${title}  ═══`)
    metrics.forEach((m) => {
        log.message(`      ✨ ${m.label}: ${m.value}`)
    })
}

export function rewriteSkillLinks(
    content: string,
    contentBase: string,
    srcDir: string
): string {
    const placeholders: string[] = []
    const placeholderPrefix = "\x00_CODE_SPAN_PLACEHOLDER_"
    let protectedContent = content.replace(
        /(```[\s\S]*?```|``[\s\S]*?``|`[^`\r\n]*?`)/g,
        (match) => {
            const id = placeholders.length
            placeholders.push(match)
            return `${placeholderPrefix}${id}\x00`
        }
    )

    protectedContent = protectedContent.replace(
        /\]\((?!https?:\/\/|@\/|#|mailto:|\/)([^)#\s]+\.md)(#[^)\s]*)?\)/g,
        (match, relPath: string, anchor?: string) => {
            const targetFullPath = path.join(srcDir, relPath)
            if (!fs.existsSync(targetFullPath)) return match

            const relToSkills = path.relative(SKILLS_DIR, targetFullPath)
            const parts = relToSkills.split(/[\\/]/)
            const category = parts.length >= 2 ? parts[0]! : ""
            if (CATEGORIES[category]?.lifecycle) {
                return match
            }

            const rel = path.posix
                .normalize(path.posix.join(contentBase, relPath))
                .replace(/\/SKILL\.md$/, "/_index.md")
            return `](@/${rel}${anchor || ""})`
        }
    )

    return protectedContent.replace(
        new RegExp(`${placeholderPrefix}(\\d+)\\x00`, "g"),
        (_, id) => placeholders[parseInt(id, 10)]!
    )
}

export async function getSkills(): Promise<Skill[]> {
    const skills: Skill[] = []
    if (!(await fs.pathExists(SKILLS_DIR))) return skills

    const files = await glob("**/SKILL.md", {cwd: SKILLS_DIR})
    for (const file of files.sort()) {
        const fullPath = path.join(SKILLS_DIR, file)
        const raw = (await fs.readFile(fullPath, "utf-8")).replace(
            /\r\n/g,
            "\n"
        )
        const match = raw.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/)
        if (!match || !match[1] || !match[2]) {
            console.warn(`Skill at ${file} is missing YAML frontmatter.`)
            continue
        }
        let metadata: SkillMetadata = {}
        let yamlError: string | undefined
        try {
            metadata = (yaml.load(match[1]) as SkillMetadata) || {}
        } catch (e: any) {
            yamlError = e.message || String(e)
        }
        const parts = file.split("/")
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
            yamlError,
            raw,
        })
    }
    return skills
}

export function groupByCategory(skills: Skill[]): [string, Skill[]][] {
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

export const HOME = process.env.HOME || os.homedir()
export const AGENTS_HUB =
    process.env.AGENT_SKILLS_AGENTS_DIR || path.join(HOME, ".agents", "skills")
export const CLAUDE_SKILLS = path.join(HOME, ".claude", "skills")
export const ANTIGRAVITY_CONFIG_DIR = path.join(HOME, ".gemini", "config")
export const ANTIGRAVITY_SKILLS_JSON = path.join(
    ANTIGRAVITY_CONFIG_DIR,
    "skills.json"
)

export const MANAGED_SKILL_DIRS = [
    AGENTS_HUB,
    CLAUDE_SKILLS,
    path.join(HOME, ".config", "opencode", "skills"),
    path.join(HOME, ".config", "opencode", "commands"),
    path.join(HOME, ".config", "goose", "skills"),
    path.join(HOME, ".gemini", "antigravity-cli", "skills"),
    path.join(HOME, ".gemini", "antigravity", "skills"),
    path.join(HOME, ".gemini", "skills"),
]

export interface Detected {
    claude: boolean
    opencode: boolean
    goose: boolean
    antigravity: boolean
}

export function detectTools(): Detected {
    return {
        claude: fs.existsSync(path.join(HOME, ".claude")),
        opencode: fs.existsSync(path.join(HOME, ".config", "opencode")),
        goose: fs.existsSync(path.join(HOME, ".config", "goose")),
        antigravity: fs.existsSync(path.join(HOME, ".gemini")),
    }
}

export async function cleanStaleLinks(dir: string): Promise<number> {
    if (!(await fs.pathExists(dir))) return 0
    let removed = 0
    let entries: string[]
    try {
        entries = await fs.readdir(dir)
    } catch {
        return 0
    }
    for (const entry of entries) {
        const entryPath = path.join(dir, entry)
        try {
            const stat = await fs.lstat(entryPath)
            if (!stat.isSymbolicLink()) continue
            const target = path.resolve(
                path.dirname(entryPath),
                await fs.readlink(entryPath)
            )

            const normalizedTarget = path.normalize(target)
            const skillsMarker = `${path.sep}skills${path.sep}`
            const skillsIdx = normalizedTarget.lastIndexOf(skillsMarker)
            if (skillsIdx === -1) continue

            const repoRoot = normalizedTarget.slice(0, skillsIdx)
            const relativePath = normalizedTarget.slice(
                skillsIdx + skillsMarker.length
            )

            const parts = relativePath.split(/[\\/]/)
            if (parts.length !== 2) continue
            const [category, name] = parts
            if (
                !category ||
                !name ||
                !CATEGORIES[category] ||
                !NAME_RE.test(name)
            )
                continue

            const pkgPath = path.join(repoRoot, "package.json")
            if (await fs.pathExists(pkgPath)) {
                try {
                    const pkg = await fs.readJson(pkgPath)
                    if (pkg && pkg.name === "agent-skills") {
                        await fs.remove(entryPath)
                        removed++
                    }
                } catch {
                    // Ignore parsing errors
                }
            } else {
                // If package.json is absent, the target has been moved or deleted.
                // We remove the symlink because the already-passing heuristic (conforming category/name structure) holds.
                // Note: The fallback drops the pkg.name === "agent-skills" guarantee,
                // but the deletion risk is bounded by the CATEGORIES and NAME_RE heuristic checks above.
                await fs.remove(entryPath)
                removed++
            }
        } catch {
            // Ignore unreadable entries
        }
    }
    return removed
}

export const ANTIGRAVITY_LOCK_FILE = `${ANTIGRAVITY_SKILLS_JSON}.lock`

export async function acquireLock() {
    const timeout = 10000
    const start = Date.now()
    while (Date.now() - start < timeout) {
        try {
            await fs.writeFile(ANTIGRAVITY_LOCK_FILE, String(process.pid), {
                flag: "wx",
            })
            return
        } catch (err: any) {
            if (err.code === "EEXIST") {
                try {
                    const stat = await fs.stat(ANTIGRAVITY_LOCK_FILE)
                    if (Date.now() - stat.mtimeMs > 10000) {
                        await fs.remove(ANTIGRAVITY_LOCK_FILE)
                    }
                } catch {
                    // Ignore concurrent stat/remove failures
                }
            } else {
                throw err
            }
            await new Promise((resolve) => setTimeout(resolve, 100))
        }
    }
    throw new Error("Timeout acquiring skills.json.lock file lock.")
}

export async function releaseLock() {
    try {
        await fs.remove(ANTIGRAVITY_LOCK_FILE)
    } catch {
        // Ignore
    }
}

export async function registerAntigravity() {
    await fs.ensureDir(ANTIGRAVITY_CONFIG_DIR)
    await acquireLock()
    try {
        let config: {entries?: {path: string}[]; [k: string]: any} = {}
        if (await fs.pathExists(ANTIGRAVITY_SKILLS_JSON)) {
            const rawContent = await fs.readFile(
                ANTIGRAVITY_SKILLS_JSON,
                "utf-8"
            )
            if (rawContent.trim() !== "") {
                let parsedConfig: any = null
                let isCorrupted = false
                let corruptionReason = ""
                try {
                    parsedConfig = JSON.parse(rawContent)
                    if (
                        parsedConfig === null ||
                        typeof parsedConfig !== "object" ||
                        Array.isArray(parsedConfig)
                    ) {
                        isCorrupted = true
                        corruptionReason = "parsed to a non-object value"
                    } else if (
                        parsedConfig.entries !== undefined &&
                        !Array.isArray(parsedConfig.entries)
                    ) {
                        isCorrupted = true
                        corruptionReason = "'entries' is not an array"
                    }
                } catch (e: any) {
                    isCorrupted = true
                    corruptionReason = `contains invalid JSON: ${e.message}`
                }

                if (isCorrupted) {
                    const backupPath = `${ANTIGRAVITY_SKILLS_JSON}.bak`
                    log.warn(
                        `⚠️ Corrupted configuration: ${ANTIGRAVITY_SKILLS_JSON} ${corruptionReason}. Backing up to ${backupPath} and resetting config.`
                    )
                    try {
                        await fs.writeFile(backupPath, rawContent, "utf-8")
                    } catch (err) {
                        log.error(`❌ Failed to write backup file: ${err}`)
                    }
                    config = {}
                } else {
                    config = parsedConfig
                }
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
    } finally {
        await releaseLock()
    }
}

export async function linkSkills() {
    const tools = detectTools()
    const allSkills = await getSkills()
    if (allSkills.some((s) => s.yamlError)) {
        throw new Error(
            "Cannot install: One or more skills contain YAML frontmatter errors."
        )
    }
    const skills = allSkills.filter((s) => s.promoted)

    const linkTargets: string[] = []
    if (tools.claude) linkTargets.push(CLAUDE_SKILLS)
    if (tools.opencode || tools.goose || tools.antigravity)
        linkTargets.push(AGENTS_HUB)

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

export async function unlinkSkills() {
    let removed = 0
    for (const t of MANAGED_SKILL_DIRS) removed += await cleanStaleLinks(t)

    if (await fs.pathExists(ANTIGRAVITY_SKILLS_JSON)) {
        await acquireLock()
        try {
            const rawContent = await fs.readFile(
                ANTIGRAVITY_SKILLS_JSON,
                "utf-8"
            )
            if (rawContent.trim() !== "") {
                let config: any
                try {
                    config = JSON.parse(rawContent)
                    if (config && Array.isArray(config.entries)) {
                        config.entries = config.entries.filter(
                            (e: {path: string}) =>
                                path.resolve(e.path) !== AGENTS_HUB
                        )
                        await fs.writeJson(ANTIGRAVITY_SKILLS_JSON, config, {
                            spaces: 2,
                        })
                    }
                } catch (e: any) {
                    log.warn(
                        `⚠️ Warning: ${ANTIGRAVITY_SKILLS_JSON} contains invalid JSON: ${e.message}`
                    )
                }
            }
        } finally {
            await releaseLock()
        }
    }
    return {removed}
}

export function toolList(tools: Detected): string {
    return (
        Object.entries(tools)
            .filter(([, v]) => v)
            .map(([k]) => k)
            .join(", ") || "none"
    )
}
