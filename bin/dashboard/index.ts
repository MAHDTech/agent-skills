import {parseArgs} from "util"

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
        "Usage: bun run bin/dashboard/index.ts --action <build|serve|css|test>"
    )
    process.exit(1)
}

const action =
    (typeof options.action === "string" ? options.action : undefined) ||
    positionals[0]

const fail = (e: unknown) => {
    console.error(e)
    process.exit(1)
}

if (action === "build") {
    import("./build.ts").then((mod) => mod.buildAction()).catch(fail)
} else if (action === "serve") {
    import("./serve.ts").then((mod) => mod.serveAction()).catch(fail)
} else if (action === "css") {
    import("./css.ts").then((mod) => mod.cssAction()).catch(fail)
} else if (action === "test") {
    import("./lib.ts")
        .then((lib) => {
            lib.run("bun", ["test", "bin/dashboard/lib.test.ts"])
            lib.run("bun", ["test", "bin/skills/lib.test.ts"])
            lib.run("bun", ["test", "bin/skills/lint.test.ts"])
            lib.run("bun", ["test", "bin/skills/downloader.test.ts"])
            return import("./build.ts")
        })
        .then((mod) => mod.buildAction())
        .catch(fail)
} else {
    console.error(
        "Usage: bun run bin/dashboard/index.ts --action <build|serve|css|test>"
    )
    process.exit(1)
}
