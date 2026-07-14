import {intro, outro} from "@clack/prompts"
import {build} from "./lib.ts"

export async function buildAction() {
    intro("Dashboard Build")
    build()
    outro("Done! Site built to dashboard/public.")
}
