import {intro, outro} from "@clack/prompts"
import {buildCss} from "./lib.ts"

export async function cssAction() {
    intro("Dashboard CSS")
    buildCss()
    outro("Done! CSS written to dashboard/static/build/css/generated.css.")
}
