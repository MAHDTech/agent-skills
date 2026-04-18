import fs from "fs-extra"
import path from "path"
import yaml from "js-yaml"
import {glob} from "glob"

const SKILLS_DIR = path.join(
    process.env.AGENT_SKILLS_HOME || path.resolve(__dirname, ".."),
    "skills"
)

async function lint() {
    console.log("Linting skills...")
    const skillFiles = await glob("**/SKILL.md", {cwd: SKILLS_DIR})
    let errors = 0

    for (const file of skillFiles) {
        const fullPath = path.join(SKILLS_DIR, file)
        const content = await fs.readFile(fullPath, "utf-8")
        const match = content.match(/^---\n([\s\S]*?)\n---\n/)

        if (!match) {
            console.error(`❌ ${file}: Missing YAML frontmatter.`)
            errors++
            continue
        }

        try {
            const metadata = yaml.load(match[1]) as any
            const required = ["name", "description"]
            for (const field of required) {
                if (!metadata[field]) {
                    console.error(
                        `❌ ${file}: Missing mandatory field '${field}' in frontmatter.`
                    )
                    errors++
                }
            }

            // Check if folder name matches metadata name
            const folderName = path.dirname(file)
            if (folderName !== "." && folderName !== metadata.name) {
                // If it's in a subdirectory, the folder name should match the skill name
                // For nested: dirname could be "category/name" so we need basename!
                const baseName = path.basename(folderName)
                if (baseName !== metadata.name) {
                    console.warn(
                        `⚠️ ${file}: Folder base name '${baseName}' does not match skill name '${metadata.name}'.`
                    )
                }
            }
        } catch (e) {
            console.error(`❌ ${file}: Invalid YAML in frontmatter.`, e)
            errors++
        }
    }

    if (errors > 0) {
        console.error(`\nFound ${errors} errors.`)
        process.exit(1)
    } else {
        console.log("✅ All skills passed linting!")
    }
}

lint().catch(console.error)
