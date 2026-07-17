import {intro, outro, log} from "@clack/prompts"
import {build, run, cleanOutputDir} from "./lib.ts"

export async function serveAction() {
    intro("Dashboard Serve")
    build()
    cleanOutputDir()
    log.step("🚀 Serving with live reload…")
    run("zola", ["--root", "dashboard", "serve"])
    outro("Done!")
}
