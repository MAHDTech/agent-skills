import {parseArgs} from "util"
import {getSkills, AGENT_SKILLS_HOME} from "./lib.ts"

let options: {
    action?: string | boolean
    skill?: string
    category?: string
    timeout?: string
    concurrency?: string
} = {}
let positionals: string[] = []

let timeoutVal: number | undefined
let concurrencyVal: number | undefined

try {
    const parsed = parseArgs({
        args: process.argv.slice(2),
        options: {
            action: {type: "string", short: "a"},
            skill: {type: "string"},
            category: {type: "string"},
            timeout: {type: "string"},
            concurrency: {type: "string"},
        },
        strict: true,
        allowPositionals: true,
    })
    options = parsed.values
    positionals = parsed.positionals

    if (options.timeout !== undefined) {
        if (
            !/^\d+$/.test(options.timeout) ||
            parseInt(options.timeout, 10) <= 0
        ) {
            console.error("Error: --timeout must be a positive integer.")
            process.exit(1)
        }
        timeoutVal = parseInt(options.timeout, 10)
    }

    if (options.concurrency !== undefined) {
        if (
            !/^\d+$/.test(options.concurrency) ||
            parseInt(options.concurrency, 10) <= 0
        ) {
            console.error("Error: --concurrency must be a positive integer.")
            process.exit(1)
        }
        concurrencyVal = parseInt(options.concurrency, 10)
    }
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
    import("./sync.ts")
        .then((mod) =>
            mod.syncAction({
                category: options.category,
                skill: options.skill,
            })
        )
        .catch(handleException)
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
            await downloadAction(skills, AGENT_SKILLS_HOME, {
                skill: options.skill,
                category: options.category,
                timeout: timeoutVal,
                concurrency: concurrencyVal,
            })
        })
        .catch(handleException)
} else if (action === "clean-resources") {
    getSkills()
        .then(async (skills) => {
            const {cleanAction} = await import("./downloader.ts")
            await cleanAction(skills, AGENT_SKILLS_HOME, {
                skill: options.skill,
                category: options.category,
            })
        })
        .catch(handleException)
} else {
    console.error(
        "Usage: bun run bin/skills/index.ts --action <lint|sync|install|uninstall|download-resources|clean-resources|test>"
    )
    process.exit(1)
}
