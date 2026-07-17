import {parseArgs} from "util"

const {values: options} = parseArgs({
    args: process.argv.slice(2),
    options: {action: {type: "string", short: "a"}},
    strict: false,
})

const action = (options.action as string) || process.argv[2]

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
            lib.run("bun", ["test"])
            return import("./build.ts")
        })
        .then((mod) => mod.buildAction())
        .catch(fail)
} else {
    console.log(
        "Usage: bun run bin/dashboard/index.ts --action <build|serve|css|test>"
    )
    process.exit(1)
}
