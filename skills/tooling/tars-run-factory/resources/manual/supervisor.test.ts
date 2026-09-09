import { expect, test } from "bun:test"
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { Ledger } from "./ledger"
import { object } from "./stream"
import { Supervisor, type Options } from "./supervisor"
import { Host } from "./transport"

async function fixture(run: (supervisor: Supervisor, ledger: Ledger, directory: string) => Promise<void>, changes: Partial<Options> = {}, mode = "normal") {
    const directory = mkdtempSync(join(tmpdir(), "factory-supervisor-"))
    const ledger = new Ledger(join(directory, "ledger.ndjson"))
    const options: Options = { workspace: directory, merge: false, cycles: 10, runtimeMs: 10_000, turnMs: 1000, recoveryLimit: 3, audit: false, triage: false, smoke: true, ...changes }
    const supervisor = new Supervisor(options, ledger, directory, () => {}, (conversation) => new Host([process.execPath, join(import.meta.dir, "mock-host.ts"), mode], directory, ledger, conversation))
    try { await run(supervisor, ledger, directory) }
    finally { await supervisor.stop("stopped", "test cleanup"); ledger.close(); rmSync(directory, { recursive: true, force: true }) }
}

const handle = (supervisor: Supervisor, directive = "continue", progress = "fresh") => supervisor.control({
    action: "handle", resultSeq: supervisor.position.resultSeq, directive, progress, evidence: { observed: "test store state" },
})

test("SUCCESS cannot schedule another turn without acknowledgement of its receipt", async () => {
    await fixture(async (supervisor, ledger) => {
        await supervisor.control({ action: "status" })
        expect(supervisor.position.transport).toBe("SUCCESS")
        await expect(supervisor.control({ action: "status" })).rejects.toThrow("handled")
        await expect(supervisor.control({ action: "handle", resultSeq: 1, directive: "continue", progress: "fresh", evidence: {} })).rejects.toThrow("current result")
        await handle(supervisor)
        await supervisor.control({ action: "status" })
        await handle(supervisor, "drained", "next")
        expect(supervisor.position.phase).toBe("stopped")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(1)
    })
})

test.each(["approval_needed", "pending_human_merge", "human_door", "stalled"])("%s parks even with merge enabled", async (directive) => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        await handle(supervisor, directive)
        expect(supervisor.position.phase).toBe("stopped")
        await expect(supervisor.control({ action: "cycle" })).rejects.toThrow("stopped")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "stdin")).toHaveLength(1)
        expect(readFileSync(join(directory, "FACTORY_REPORT.md"), "utf8")).toContain(directive)
    }, { merge: true })
})

test("epic invocation never becomes a repository-wide batch", async () => {
    await fixture(async (supervisor, ledger) => {
        await supervisor.control({ action: "probe" })
        await handle(supervisor)
        await supervisor.control({ action: "cycle" })
        const input = object(ledger.receipts.findLast((receipt) => receipt.channel === "stdin")?.payload)
        expect(object(input.message).content).toBe("/tars-run-epic 42 --merge")
        expect(JSON.stringify(input)).not.toContain("run-batch")
    }, { smoke: false, epic: 42, merge: true })
})

test("partial SUCCESS cannot advance the workflow", async () => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        expect(supervisor.position.phase).toBe("failed")
        await expect(handle(supervisor, "drained")).rejects.toThrow("current result")
        await expect(supervisor.control({ action: "status" })).rejects.toThrow("handled")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "stdin")).toHaveLength(1)
        expect(ledger.receipts.filter((receipt) => receipt.channel === "handled")).toHaveLength(0)
        expect(readFileSync(join(directory, "FACTORY_REPORT.md"), "utf8")).toContain("returning partial output")
    }, {}, "partial-timeout")
})

test("cycle and no-progress budgets stop with handovers", async () => {
    await fixture(async (supervisor, _ledger, directory) => {
        await supervisor.control({ action: "status" })
        await handle(supervisor)
        expect(supervisor.position.reason).toBe("cycle limit reached")
        expect(readFileSync(join(directory, "FACTORY_REPORT.md"), "utf8")).toContain("cycle limit reached")
    }, { cycles: 1 })
    await fixture(async (supervisor) => {
        for (let index = 0; index < 3; index++) {
            await supervisor.control({ action: "status" })
            await handle(supervisor, "continue", "unchanged")
        }
        expect(supervisor.position.reason).toBe("no-progress limit reached")
    })
})

test("terminal failure writes handover and preserves evidence for recovery", async () => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        expect(supervisor.position.phase).toBe("failed")
        expect(readFileSync(join(directory, "FACTORY_REPORT.md"), "utf8")).toContain("missing result")
        expect(ledger.receipts.some((receipt) => receipt.channel === "exit")).toBe(true)
        await supervisor.control({ action: "recover", reconciliation: { disposition: "parked", progress: "gate", evidence: { blocking_human_gate: true } } })
        expect(supervisor.position.phase).toBe("stopped")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(1)
    }, {}, "eof")
})

test("runtime limit terminates an incomplete host and completes handover", async () => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        await supervisor.finished
        expect(supervisor.position.phase).toBe("stopped")
        expect(ledger.receipts.some((receipt) => receipt.channel === "exit")).toBe(true)
        expect(readFileSync(join(directory, "FACTORY_REPORT.md"), "utf8")).toContain("runtime limit")
    }, { runtimeMs: 150, turnMs: 1000 }, "timeout")
})

test("pending CI enforces pacing instead of an automatic retry loop", async () => {
    await fixture(async (supervisor, ledger) => {
        await supervisor.control({ action: "status" })
        await handle(supervisor, "pending_ci")
        await expect(supervisor.control({ action: "status" })).rejects.toThrow("pacing")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "stdin")).toHaveLength(1)
    })
})

test("late refusal after SUCCESS invalidates acknowledgement and prevents the next turn", async () => {
    await fixture(async (supervisor, ledger) => {
        await supervisor.control({ action: "status" })
        expect(supervisor.position.phase).toBe("result")
        const deadline = Date.now() + 1000
        while (supervisor.position.phase !== "failed" && Date.now() < deadline) await Bun.sleep(5)
        expect(supervisor.position.phase).toBe("failed")
        await expect(handle(supervisor)).rejects.toThrow("current result")
        expect(ledger.receipts.filter((receipt) => receipt.channel === "stdin")).toHaveLength(1)
    }, {}, "late-denial")
})

test("a missing MCP server gets one fresh-process probe retry", async () => {
    await fixture(async (supervisor, ledger) => {
        await supervisor.control({ action: "probe" })
        await supervisor.control({ action: "retry-probe", resultSeq: supervisor.position.resultSeq })
        expect(ledger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(2)
        expect(ledger.receipts.filter((receipt) => receipt.channel === "probe_retry")).toHaveLength(1)
        await supervisor.control({ action: "retry-probe", resultSeq: supervisor.position.resultSeq })
        expect(supervisor.position.reason).toBe("MCP probe retry exhausted")
    }, { smoke: false }, "missing-server")
})

test("reopening an interrupted result preserves identity and budgets without replay", async () => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        const prefix = ledger.receipts.map((receipt) => JSON.stringify(receipt)).join("\n") + "\n"
        const saved = structuredClone(supervisor.position)
        await supervisor.stop("stopped", "stop original process after freezing interruption evidence")
        const path = join(directory, "interrupted.ndjson")
        writeFileSync(path, prefix, { mode: 0o600 })
        const restoredLedger = new Ledger(path)
        const restored = new Supervisor(supervisor.options, restoredLedger, directory, () => {}, (conversation) => new Host([process.execPath, join(import.meta.dir, "mock-host.ts"), "normal"], directory, restoredLedger, conversation))
        try {
            expect(restored.position.started).toBe(saved.started)
            expect(restored.position.cycles).toBe(1)
            expect(restored.position.resultSeq).toBe(saved.resultSeq)
            await expect(restored.control({ action: "status" })).rejects.toThrow("handled")
            await handle(restored)
            expect(restoredLedger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(1)
            await restored.control({ action: "status" })
            expect(restored.position.conversation).toBe(saved.conversation)
            expect(restored.position.cycles).toBe(2)
        } finally { await restored.stop("stopped", "test cleanup"); restoredLedger.close() }
    })
})

test("reopening failed transport reconciles completion without creating a host", async () => {
    await fixture(async (supervisor, ledger, directory) => {
        await supervisor.control({ action: "status" })
        const prefix = ledger.receipts.map((receipt) => JSON.stringify(receipt)).join("\n") + "\n"
        await supervisor.stop("stopped", "freeze terminal failure evidence")
        const path = join(directory, "failed.ndjson")
        writeFileSync(path, prefix, { mode: 0o600 })
        const restoredLedger = new Ledger(path)
        const restored = new Supervisor(supervisor.options, restoredLedger, directory, () => {}, () => { throw new Error("completed work must not launch another host") })
        try {
            await restored.control({ action: "recover", reconciliation: { disposition: "completed", progress: "finished", evidence: { finished: true } } })
            expect(restored.position.transport).toBe("RECONCILED")
            await handle(restored, "drained", "finished")
            expect(restored.position.phase).toBe("stopped")
            expect(restoredLedger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(1)
        } finally { await restored.stop("stopped", "test cleanup"); restoredLedger.close() }
    }, {}, "eof")
})
