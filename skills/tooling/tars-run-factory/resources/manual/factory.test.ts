import { expect, test } from "bun:test"
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { argumentsFor } from "./factory"
import { object } from "./stream"

test("CLI validates scope flags and supports a bounded epic invocation", () => {
    const parsed = argumentsFor([process.cwd(), "--epic", "42", "--cycles", "2", "--merge"])
    expect(parsed?.options.epic).toBe(42)
    expect(parsed?.options.cycles).toBe(2)
    expect(parsed?.options.merge).toBe(true)
    expect(() => argumentsFor([process.cwd(), "--dangerously-skip-permissions"])).toThrow()
})

test("runnable CLI supervises two mock turns and leaves a clean durable handover", async () => {
    const root = mkdtempSync(join(tmpdir(), "factory-cli-"))
    const workspace = join(root, "workspace")
    const state = join(root, "state")
    const bin = join(root, "bin")
    mkdirSync(workspace)
    mkdirSync(bin)
    const quote = (value: string) => "'" + value.replaceAll("'", "'\\''") + "'"
    writeFileSync(join(bin, "agy"), `#!/bin/sh\nexec ${quote(process.execPath)} ${quote(join(import.meta.dir, "mock-host.ts"))} normal\n`, { mode: 0o700 })
    const child = Bun.spawn([process.execPath, join(import.meta.dir, "factory.ts"), workspace, "--smoke", "--cycles", "2", "--state-dir", state], {
        stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, PATH: `${bin}:${process.env.PATH}` },
    })
    const stderr = new Response(child.stderr).text()
    const reader = child.stdout.getReader()
    const decoder = new TextDecoder()
    let pending = ""
    const receive = async () => {
        while (!pending.includes("\n")) {
            const part = await reader.read()
            if (part.done) throw new Error(`supervisor ended early: ${await stderr}`)
            pending += decoder.decode(part.value, { stream: true })
        }
        const newline = pending.indexOf("\n")
        const event = JSON.parse(pending.slice(0, newline))
        pending = pending.slice(newline + 1)
        return event
    }
    const send = async (control: unknown) => { child.stdin.write(JSON.stringify(control) + "\n"); await child.stdin.flush(); return receive() }
    try {
        expect((await receive()).event).toBe("ready")
        for (let index = 0; index < 2; index++) {
            const result = await send({ action: "status" })
            expect(result.state.phase).toBe("result")
            const handled = await send({ action: "handle", resultSeq: result.state.resultSeq, directive: "continue", progress: `turn-${index}`, evidence: { transportOnly: true } })
            expect(handled.state.phase).toBe(index === 0 ? "ready" : "stopped")
        }
        await child.stdin.end()
        expect(await child.exited).toBe(0)
        expect(await stderr).toBe("")
        const receipts = readFileSync(join(state, "FACTORY_LEDGER.ndjson"), "utf8").trim().split("\n").map((line) => object(JSON.parse(line)))
        expect(receipts.filter((receipt) => receipt.channel === "launch")).toHaveLength(1)
        expect(receipts.filter((receipt) => receipt.channel === "handled")).toHaveLength(2)
        expect(receipts.filter((receipt) => receipt.channel === "exit")).toHaveLength(1)
        expect(readFileSync(join(state, "FACTORY_REPORT.md"), "utf8")).toContain("cycle limit reached")
    } finally {
        child.kill()
        await child.exited
        reader.releaseLock()
        rmSync(root, { recursive: true, force: true })
    }
}, 10_000)
