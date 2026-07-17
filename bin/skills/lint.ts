import {intro, outro, log} from "@clack/prompts"
import {getSkills, CATEGORIES, NAME_RE, logTask} from "./lib.ts"

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
            log.warn(
                `⚠️ ${file}: unknown category '${skill.category}' (not declared in CATEGORIES).`
            )
        }

        if (/\]\(file:\/\/\/[^)]*\)/.test(skill.raw)) {
            log.error(
                `❌ ${file}: contains absolute file:/// URL. CRITICAL LINKING RULE violation: Never use absolute file:/// URLs referencing local paths.`
            )
            errors++
        }
    }

    if (errors > 0) {
        log.error(`\nFound ${errors} error(s).`)
        process.exit(1)
    }
    logTask(`All ${skills.length} skills passed linting!`)
    outro("Done!")
}
