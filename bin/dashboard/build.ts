import {intro, outro} from "@clack/prompts"
import {build} from "./lib.ts"

export async function buildAction(options?: {
    skill?: string
    category?: string
}) {
    intro("Dashboard Build")
    build(options)
    outro("Done! Site built to dashboard/public.")
}
