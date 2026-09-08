import { expect, test } from "bun:test"
import { mkdtempSync, rmSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { Ledger } from "./ledger"
import { Host } from "./transport"
import { reconcileRecovery } from "./recovery"

async function fixture(mode: string, run: (host: Host, ledger: Ledger) => Promise<void>) {
    const directory = mkdtempSync(join(tmpdir(), "factory-host-"))
    const ledger = new Ledger(join(directory, "ledger.ndjson"))
    const argv = mode === "brokenpipe"
        ? ["/bin/sh", "-c", 'exec 0<&-; printf \'{"event":"init","conversation_id":"mock-conversation"}\\n\'; exec sleep 30']
        : [process.execPath, join(import.meta.dir, "mock-host.ts"), mode]
    const host = new Host(argv, directory, ledger)
    try {
        await run(host, ledger)
    } finally {
        await host.close()
        ledger.close()
        rmSync(directory, { recursive: true, force: true })
    }
}

test("two turns share a process, each waits for durable handling", async () => {
    await fixture("normal", async (host, ledger) => {
        const first = await host.send("first", 1000)
        expect(first.response).toBe("turn 1")
        await expect(host.send("too soon", 1000)).rejects.toThrow("handled")
        host.handled({ workflow: "continue" })
        const second = await host.send("second", 1000)
        expect(second.response).toBe("turn 2")
        host.handled({ workflow: "stopped" })
        await host.close()
        expect(host.gone).toBe(true)
        expect(ledger.receipts.filter((r) => r.channel === "launch")).toHaveLength(1)
        expect(ledger.receipts.filter((r) => r.channel === "stdin")).toHaveLength(2)
        expect(ledger.receipts.filter((r) => r.channel === "handled")).toHaveLength(2)
        expect(ledger.receipts.at(-1)?.channel).toBe("exit")
    })
})

test.each([
    ["eof", "missing result"], ["timeout", "timeout"], ["malformed", "JSON"],
    ["truncated", "truncated"], ["denial", "denied"], ["stderr", "denied"],
    ["duplicate", "duplicate"],
])("%s is terminal and cannot emit another prompt", async (mode, reason) => {
    await fixture(mode!, async (host, ledger) => {
        await expect(host.send("first", mode === "timeout" ? 100 : 1000)).rejects.toThrow(reason)
        await host.close()
        expect(host.gone).toBe(true)
        await expect(host.send("rival", 1000)).rejects.toThrow()
        expect(ledger.receipts.filter((r) => r.channel === "stdin")).toHaveLength(1)
        expect(ledger.receipts.some((r) => r.channel === "exit")).toBe(true)
    })
})

test("writing after process exit is explicit and never starts a replacement", async () => {
    await fixture("normal", async (host, ledger) => {
        await host.close()
        await expect(host.send("late", 1000)).rejects.toThrow()
        expect(ledger.receipts.filter((r) => r.channel === "launch")).toHaveLength(1)
    })
})

test("broken stdin pipe terminates and records the old host before recovery", async () => {
    await fixture("brokenpipe", async (host, ledger) => {
        await host.initialized
        await expect(host.send("first", 1000)).rejects.toThrow("broken pipe")
        expect(host.gone).toBe(true)
        expect(ledger.receipts.some((receipt) => receipt.channel === "failure")).toBe(true)
        expect(ledger.receipts.at(-1)?.channel).toBe("exit")
    })
})

test("terminal failure resumes the same recorded conversation after old host exit", async () => {
    await fixture("eof", async (host, ledger) => {
        const leg = "/tars-run-epic 42"
        ledger.append("leg", { prompt: leg })
        await expect(host.send(leg, 1000)).rejects.toThrow("missing result")
        const recovered = await reconcileRecovery({
            pid: host.child.pid!, conversation: host.turn.conversation!, leg, limit: 2,
        }, ledger, async () => {
            expect(host.gone).toBe(true)
            return { disposition: "resume", progress: "new-evidence", evidence: { phase: "R.1" } }
        })
        const resumed = new Host([process.execPath, join(import.meta.dir, "mock-host.ts"), "normal", "--conversation", recovered!.conversation], process.cwd(), ledger, recovered!.conversation)
        try {
            const result = await resumed.send(recovered!.prompt, 1000)
            expect(result.conversation).toBe("mock-conversation")
            resumed.handled({ workflow: "continue" })
        } finally {
            await resumed.close()
        }
        const channels = ledger.receipts.map((receipt) => receipt.channel)
        expect(channels.indexOf("exit")).toBeLessThan(channels.indexOf("recovery"))
        expect(ledger.receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(2)
    })
})

test("descendant-held output cannot hang shutdown after the host exits", async () => {
    await fixture("held-output", async (host, ledger) => {
        await expect(host.send("first", 5000)).rejects.toThrow("incomplete output")
        expect(host.gone).toBe(true)
        expect(ledger.receipts.some((receipt) => receipt.channel === "exit")).toBe(true)
    })
})

test("buffered events after a refusal retain a stronger recovery stop", async () => {
    await fixture("escalating-stop", async (host, ledger) => {
        await expect(host.send("first", 1000)).rejects.toThrow("denied")
        expect(host.turn.stop?.kind).toBe("authentication")
        const failure = ledger.receipts.filter((receipt) => receipt.channel === "failure").at(-1)
        expect(failure?.payload).toMatchObject({ stop: { kind: "authentication" } })
        expect(ledger.receipts.filter((receipt) => receipt.channel === "stdout")).toHaveLength(3)
    })
})
