import {intro, outro, log} from "@clack/prompts"
import {execFileSync} from "node:child_process"
import {run} from "./lib.ts"

// Generated files a full sync rewrites. Restored after the check so lint never
// mutates the working tree — it only reports.
const GENERATED_PATHS = [
    "README.md",
    "agents/AGENTS.md",
    "skills.sh.json",
    "dashboard/content",
]

function git(args: string[]): string {
    return execFileSync("git", args, {encoding: "utf-8"})
}

/**
 * Verify the committed dashboard content matches what a fresh build produces.
 * Regenerates content without staging, then fails if that leaves any
 * un-committed change under dashboard/content — i.e. someone edited a skill but
 * did not run `dashboard --action build` and commit the result.
 */
export async function lintAction() {
    intro("Dashboard Lint")

    // Regenerate deterministically without staging, so drift surfaces as an
    // uncommitted working-tree change we can detect.
    run("bun", ["run", "skills", "--action", "sync"], {
        SKILLS_REPO_ONLY: "1",
        SKILLS_NO_STAGE: "1",
    })

    // Drift = working-tree changes under dashboard/content that are NOT already
    // staged. Porcelain column 2 is the worktree status; "??" is untracked.
    // Purely-staged changes (e.g. "M ") are the content about to be committed
    // and are fine, so they are ignored.
    const status = git(["status", "--porcelain", "--", "dashboard/content"])
    const drifted = status
        .split("\n")
        .filter(Boolean)
        .filter((line) => line.startsWith("??") || line[1] !== " ")

    // Restore everything the regeneration touched — lint is read-only.
    try {
        git(["checkout", "--", ...GENERATED_PATHS])
        git(["clean", "-fdq", "dashboard/content"])
    } catch {
        // Best-effort restore; nothing to do if git has no changes to revert.
    }

    if (drifted.length > 0) {
        log.error(
            "dashboard/content is out of date with the skills. Run `dashboard --action build`, then stage and commit the regenerated dashboard files."
        )
        log.error(`Drifted paths:\n${drifted.join("\n")}`)
        process.exit(1)
    }

    outro("Done! Dashboard content is committed and up to date.")
}
