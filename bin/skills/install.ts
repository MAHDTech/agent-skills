import {intro, outro} from "@clack/prompts"
import {linkSkills, printSummary, toolList} from "./lib.ts"

export async function install() {
    intro("Agent Skills Installer")
    const m = await linkSkills()
    printSummary("Installation Summary", [
        {label: "Skills", value: m.skills},
        {label: "Symlinks created", value: m.linked},
        {label: "Stale links removed", value: m.removed},
        {label: "Conflicts", value: m.conflicts},
        {label: "Tools detected", value: toolList(m.tools)},
    ])
    outro("Done! Skills linked into your detected agent tools.")
}
