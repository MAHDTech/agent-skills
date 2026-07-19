import {parseArgs} from "util"
import {getSkills, AGENT_SKILLS_HOME} from "./lib.ts"

let options: {action?: string | boolean} = {}
let positionals: string[] = []

try {
    const parsed = parseArgs({
        args: process.argv.slice(2),
        options: {action: {type: "string", short: "a"}},
        strict: true,
        allowPositionals: true,
    })
    options = parsed.values
    positionals = parsed.positionals
} catch (e) {
    const message = e instanceof Error ? e.message : String(e)
    console.error(`Error: ${message}`)
    console.error(
        "Usage: bun run bin/skills/index.ts --action <lint|sync|install|uninstall|download-resources|clean-resources|test>"
    )
    process.exit(1)
}

const action =
    (typeof options.action === "string" ? options.action : undefined) ||
    positionals[0]

const handleException = (err: any) => {
    console.error(err)
    process.exit(1)
}

if (action === "lint") {
    import("./lint.ts").then((mod) => mod.lint()).catch(handleException)
} else if (action === "sync") {
    import("./sync.ts").then((mod) => mod.syncAction()).catch(handleException)
} else if (action === "install") {
    import("./install.ts").then((mod) => mod.install()).catch(handleException)
} else if (action === "uninstall") {
    import("./uninstall.ts")
        .then((mod) => mod.uninstallAction())
        .catch(handleException)
} else if (action === "test") {
    import("child_process")
        .then((cp) => {
            try {
                cp.execSync("bun test bin/skills", {stdio: "inherit"})
            } catch (e) {
                handleException(e)
            }
        })
        .catch(handleException)
} else if (action === "download-resources") {
    getSkills()
        .then(async (skills) => {
            const {downloadAction} = await import("./downloader.ts")
            await downloadAction(skills, AGENT_SKILLS_HOME)
        })
        .catch(handleException)
} else if (action === "clean-resources") {
    getSkills()
        .then(async (skills) => {
            const {cleanAction} = await import("./downloader.ts")
            await cleanAction(skills, AGENT_SKILLS_HOME)
        })
        .catch(handleException)
} else {
    console.error(
        "Usage: bun run bin/skills/index.ts --action <lint|sync|install|uninstall|download-resources|clean-resources|test>"
    )
    process.exit(1)
}
