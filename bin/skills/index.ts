import {parseArgs} from "util"
import {getSkills, AGENT_SKILLS_HOME} from "./lib.ts"

const {values: options} = parseArgs({
    args: process.argv.slice(2),
    options: {action: {type: "string", short: "a"}},
    strict: false,
})

const action = (options.action as string) || process.argv[2]

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
} else if (action === "download") {
    getSkills()
        .then(async (skills) => {
            const {downloadAction} = await import("./downloader.ts")
            await downloadAction(skills, AGENT_SKILLS_HOME)
        })
        .catch(handleException)
} else if (action === "clean") {
    getSkills()
        .then(async (skills) => {
            const {cleanAction} = await import("./downloader.ts")
            await cleanAction(skills, AGENT_SKILLS_HOME)
        })
        .catch(handleException)
} else {
    console.log(
        "Usage: bun run bin/skills/index.ts --action <lint|sync|install|uninstall|download|clean|test>"
    )
    process.exit(1)
}
