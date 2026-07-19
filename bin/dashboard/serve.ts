import {intro, outro, log} from "@clack/prompts"
import {
    build,
    watchCss,
    runZolaServe,
    startWatcher,
    rebuildSearchIndexes,
    killActiveProcesses,
} from "./lib.ts"

export async function serveAction(options?: {
    skill?: string
    category?: string
}) {
    intro("Dashboard Serve")
    build({serve: true, ...options})
    log.step("🚀 Serving with live reload…")

    let finished = false
    let watchers: any[] = []

    const cleanup = () => {
        if (finished) return
        finished = true

        for (const w of watchers) {
            try {
                w.close()
            } catch {
                // Ignore
            }
        }

        killActiveProcesses()
        outro("Done!")
        process.exit(0)
    }

    process.on("SIGINT", cleanup)
    process.on("SIGTERM", cleanup)

    try {
        const tailwindChild = watchCss()
        tailwindChild.on("exit", (code) => {
            if (!finished && code !== null && code !== 0) {
                log.warn(`⚠️ Tailwind watch exited with code ${code}`)
            }
        })

        const zolaChild = runZolaServe()
        zolaChild.on("exit", (code) => {
            if (!finished) {
                log.warn(`⚠️ Zola server exited (code: ${code}). Exiting…`)
                cleanup()
            }
        })

        watchers = startWatcher(() => {
            rebuildSearchIndexes(options)
        })
    } catch (err: any) {
        cleanup()
        throw err
    }
}
