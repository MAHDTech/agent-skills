import path from "path"
import {execFileSync} from "child_process"
import {intro, outro, log} from "@clack/prompts"

/**
 * Dashboard manager (personal dev tool).
 *
 * Builds and serves the Zola dashboard. Mirrors the `skills` CLI surface so the
 * whole repo shares one `--action <verb>` convention:
 *
 *   dashboard --action build   sync content, build CSS, build the static site
 *   dashboard --action serve   build once, then serve with live reload
 *   dashboard --action css     build the Tailwind CSS bundle only (escape hatch)
 *
 * `build` is the all-in-one: it regenerates the skill content, compiles the CSS,
 * renders the Zola site, and indexes it with Pagefind. `css` exists for the rare
 * case where you only want to recompile styles.
 */

const ROOT =
    process.env.AGENT_SKILLS_HOME || path.resolve(import.meta.dir, "..")

function run(
    cmd: string,
    args: string[],
    extraEnv: Record<string, string> = {}
) {
    execFileSync(cmd, args, {
        cwd: ROOT,
        stdio: "inherit",
        env: {...process.env, ...extraEnv},
    })
}

/** Compile the Tailwind CSS bundle. */
function buildCss() {
    log.step("🎨 Building CSS…")
    run("tailwindcss", [
        "-i",
        "dashboard/css/input.css",
        "-o",
        "dashboard/static/build/css/generated.css",
    ])
}

/** Regenerate skill content, build CSS, render the site, index it. */
function build() {
    log.step("🔄 Syncing generated content…")
    run("bun", ["run", "skills", "--action", "sync"], {SKILLS_REPO_ONLY: "1"})
    buildCss()
    log.step("🏗️  Building Zola site…")
    run("zola", ["--root", "dashboard", "build"])
    log.step("🔍 Indexing with Pagefind…")
    run("pagefind", ["--site", "dashboard/public"])
}

async function buildAction() {
    intro("Dashboard Build")
    build()
    outro("Done! Site built to dashboard/public.")
}

async function serveAction() {
    intro("Dashboard Serve")
    build()
    log.step("🚀 Serving with live reload…")
    run("zola", ["--root", "dashboard", "serve"])
    outro("Done!")
}

async function cssAction() {
    intro("Dashboard CSS")
    buildCss()
    outro("Done! CSS written to dashboard/static/build/css/generated.css.")
}

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
    buildAction().catch(fail)
} else if (action === "serve") {
    serveAction().catch(fail)
} else if (action === "css") {
    cssAction().catch(fail)
} else {
    console.log("Usage: bun run bin/dashboard.ts --action <build|serve|css>")
    process.exit(1)
}
