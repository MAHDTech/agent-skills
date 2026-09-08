import { existsSync, mkdirSync, realpathSync, writeFileSync } from "node:fs"
import { homedir } from "node:os"
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path"
import { createInterface } from "node:readline"
import { parseArgs } from "node:util"
import { Ledger, redactor } from "./ledger"
import { WorkspaceLease } from "./ownership"
import { object } from "./stream"
import { Supervisor, type Control, type Options } from "./supervisor"

export function argumentsFor(args: string[]) {
    const { values, positionals } = parseArgs({ args, allowPositionals: true, options: {
        epic: { type: "string" }, conversation: { type: "string" }, cycles: { type: "string", default: "10" },
        "runtime-minutes": { type: "string", default: "480" }, "turn-minutes": { type: "string", default: "90" },
        "recovery-limit": { type: "string", default: "10" }, "state-dir": { type: "string" },
        merge: { type: "boolean", default: false }, audit: { type: "boolean", default: false },
        triage: { type: "boolean", default: false }, smoke: { type: "boolean", default: false }, help: { type: "boolean", default: false },
    } })
    if (values.help) return undefined
    if (positionals.length !== 1) throw new Error("exactly one authorized workspace is required")
    const workspace = realpathSync(positionals[0]!)
    const options: Options = {
        workspace, ...(values.epic ? { epic: Number(values.epic) } : {}), ...(values.conversation ? { conversation: values.conversation } : {}), merge: values.merge!, cycles: Number(values.cycles),
        runtimeMs: Number(values["runtime-minutes"]) * 60_000, turnMs: Number(values["turn-minutes"]) * 60_000,
        recoveryLimit: Number(values["recovery-limit"]), audit: values.audit!, triage: values.triage!, smoke: values.smoke!,
    }
    const directory = values["state-dir"] ?? join(dirname(workspace), "tars-factory", basename(workspace))
    return { options, directory }
}

export async function main(args: string[]) {
    const parsed = argumentsFor(args)
    if (!parsed) {
        console.log("bun factory.ts WORKSPACE [--conversation ID] [--epic N] [--merge] [--cycles N] [--runtime-minutes N] [--turn-minutes N] [--recovery-limit N] [--audit] [--triage] [--smoke] [--state-dir DIR]\nControls are one JSON object per line: probe/audit/triage/cycle/status, handle, recover, stop. Read supervisor.md before use.")
        return
    }
    let parent = resolve(parsed.directory)
    const suffix: string[] = []
    while (!existsSync(parent)) { suffix.unshift(basename(parent)); parent = dirname(parent) }
    const destination = join(realpathSync(parent), ...suffix)
    const inside = relative(parsed.options.workspace, destination)
    if (inside === "" || (!isAbsolute(inside) && inside !== ".." && !inside.startsWith(".." + sep))) throw new Error("factory state must be outside the customer repository")
    mkdirSync(destination, { recursive: true, mode: 0o700 })
    const directory = realpathSync(destination)
    const lease = await WorkspaceLease.acquire(parsed.options.workspace, join(homedir(), ".local", "state", "tars-factory", "locks"))
    let ledger: Ledger | undefined
    let supervisor: Supervisor | undefined
    let input: ReturnType<typeof createInterface> | undefined
    const redact = redactor(process.env)
    const emit = (value: unknown) => console.log(JSON.stringify(redact(value)))
    const interrupt = () => { void supervisor?.stop("stopped", "foreman interrupted by signal").finally(() => input?.close()) }
    try {
        ledger = new Ledger(join(directory, "FACTORY_LEDGER.ndjson"))
        supervisor = new Supervisor(parsed.options, ledger, directory, () => lease.assertHeld())
        input = createInterface({ input: process.stdin, crlfDelay: Infinity })
        process.once("SIGINT", interrupt)
        process.once("SIGTERM", interrupt)
        void supervisor.finished.then(() => input?.close())
        emit({ event: "ready", state: supervisor.snapshot(), directory })
        for await (const line of input) {
            let control: Control
            try {
                control = object(JSON.parse(line)) as unknown as Control
                if (typeof control.action !== "string") throw new Error("control action is required")
            } catch (error) {
                await supervisor.stop("stopped", `malformed control input: ${String(error)}`)
                emit({ event: "stopped", state: supervisor.snapshot() })
                break
            }
            try { emit({ event: "state", state: await supervisor.control(control) }) }
            catch (error) { emit({ event: "control_error", error: String(error), state: supervisor.snapshot() }) }
            if (supervisor.position.phase === "stopped") break
        }
        if (supervisor.position.phase !== "stopped") await supervisor.stop("stopped", "foreman control input reached EOF")
    } catch (error) {
        const reason = String(redact(String(error)))
        writeFileSync(join(directory, "FACTORY_START_FAILURE.md"), `# Factory start failure\n\n${JSON.stringify(reason)}\n`, { mode: 0o600 })
        throw new Error(reason)
    } finally {
        process.removeListener("SIGINT", interrupt)
        process.removeListener("SIGTERM", interrupt)
        input?.close()
        if (supervisor && supervisor.position.phase !== "stopped") await supervisor.stop("stopped", "supervisor cleanup")
        ledger?.close()
        await lease.release()
    }
}

if (import.meta.main) {
    main(process.argv.slice(2)).catch((error) => { console.error(String(redactor(process.env)(String(error)))); process.exitCode = 1 })
}
