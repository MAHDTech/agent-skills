import {intro, outro, log} from "@clack/prompts"
import {build, run} from "./lib.ts"

export async function serveAction() {
    intro("Dashboard Serve")
    build({serve: true})
    log.step("🚀 Serving with live reload…")
    run("zola", ["--root", "dashboard", "serve"])
    outro("Done!")
}
