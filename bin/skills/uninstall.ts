import {intro, outro} from "@clack/prompts"
import {unlinkSkills, printSummary} from "./lib.ts"

export async function uninstallAction() {
    intro("Agent Skills Uninstaller")
    const m = await unlinkSkills()
    printSummary("Uninstallation Summary", [
        {label: "Symlinks removed", value: m.removed},
    ])
    outro("Done! Skills unlinked.")
}
