import {intro, outro, log} from "@clack/prompts"
import path from "node:path"
import fs from "fs-extra"
import {
    getSkills,
    CATEGORIES,
    NAME_RE,
    logTask,
    AGENT_SKILLS_HOME,
} from "./lib.ts"
import type {Skill} from "./lib.ts"

export function checkDuplicateSkills(skills: Skill[]): {
    errors: string[]
    warnings: string[]
} {
    const errors: string[] = []
    const warnings: string[] = []
    const dirNameToSkills = new Map<string, Skill[]>()

    for (const skill of skills) {
        if (skill.dirName) {
            const list = dirNameToSkills.get(skill.dirName) || []
            list.push(skill)
            dirNameToSkills.set(skill.dirName, list)
        }
    }

    for (const [dirName, list] of dirNameToSkills.entries()) {
        if (list.length > 1) {
            const promotedList = list.filter((s) => s.promoted)
            const paths = list.map((s) => s.path).sort()
            if (promotedList.length > 1) {
                errors.push(
                    `Duplicate skill name '${dirName}' detected in multiple promoted categories: ${paths.join(", ")}.`
                )
            } else {
                warnings.push(
                    `Duplicate skill name '${dirName}' detected in lifecycle/promoted categories: ${paths.join(", ")}.`
                )
            }
        }
    }

    return {errors, warnings}
}

async function checkNoIndexFiles(dirAbs: string): Promise<string[]> {
    const errors: string[] = []
    if (!(await fs.pathExists(dirAbs))) return errors
    const entries = await fs.readdir(dirAbs, {withFileTypes: true})
    for (const entry of entries) {
        const fullPath = path.join(dirAbs, entry.name)
        if (entry.isDirectory()) {
            errors.push(...(await checkNoIndexFiles(fullPath)))
        } else if (entry.isFile() && entry.name.toLowerCase() === "index.md") {
            errors.push(
                `file '${entry.name}' at ${fullPath} is disallowed inside resources/; rename it to prevent Zola section index (_index.md) collisions.`
            )
        }
    }
    return errors
}

// resources/ may contain only auto/ (downloader-owned) and manual/
// (hand-authored) subdirectories — no bare files, no other subdirectories.
export async function checkResourcesLayout(
    resourcesDirAbs: string
): Promise<string[]> {
    const errors: string[] = []
    if (!(await fs.pathExists(resourcesDirAbs))) return errors
    const entries = await fs.readdir(resourcesDirAbs, {withFileTypes: true})
    for (const entry of entries) {
        if (entry.isDirectory()) {
            if (entry.name !== "auto" && entry.name !== "manual") {
                errors.push(
                    `resources/ may only contain 'auto/' and 'manual/' subdirectories; found '${entry.name}/'.`
                )
            } else {
                errors.push(
                    ...(await checkNoIndexFiles(
                        path.join(resourcesDirAbs, entry.name)
                    ))
                )
            }
        } else {
            errors.push(
                `resources/ must not contain files directly; move '${entry.name}' into resources/auto/ (downloaded) or resources/manual/ (authored).`
            )
        }
    }
    return errors
}

export async function lint() {
    intro("Linting Skills")
    const skills = await getSkills()
    let errors = 0

    for (const skill of skills) {
        const file = skill.path
        const relPath = file.startsWith("skills/") ? file.substring(7) : file
        const pathParts = relPath.split(/[\\/]/)
        if (pathParts.length !== 3) {
            log.error(
                `❌ ${file}: skill must reside exactly at skills/<category>/<name>/SKILL.md.`
            )
            errors++
            continue
        }

        if (skill.yamlError) {
            log.error(
                `❌ ${file}: YAML frontmatter syntax error: ${skill.yamlError}`
            )
            errors++
            continue
        }
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
            log.error(
                `❌ ${file}: unknown category '${skill.category}' (not declared in CATEGORIES).`
            )
            errors++
        }

        const cleanContent = skill.raw
            .replace(/```[\s\S]*?```/g, "")
            .replace(/`[^`]*?`/g, "")

        if (/\bfile:(?:[/\\]+|[a-zA-Z]:)/i.test(cleanContent)) {
            log.error(
                `❌ ${file}: contains absolute file:/// URL. CRITICAL LINKING RULE violation: Never use absolute file:/// URLs referencing local paths.`
            )
            errors++
        }

        const resourcesDirAbs = path.join(
            AGENT_SKILLS_HOME,
            path.dirname(skill.path),
            "resources"
        )
        for (const msg of await checkResourcesLayout(resourcesDirAbs)) {
            log.error(`❌ ${file}: ${msg}`)
            errors++
        }
    }

    const dupResults = checkDuplicateSkills(skills)
    for (const warn of dupResults.warnings) {
        log.warn(`⚠️ ${warn}`)
    }
    for (const err of dupResults.errors) {
        log.error(`❌ ${err}`)
        errors++
    }

    if (errors > 0) {
        log.error(`\nFound ${errors} error(s).`)
        process.exit(1)
    }
    logTask(`All ${skills.length} skills passed linting!`)
    outro("Done!")
}
