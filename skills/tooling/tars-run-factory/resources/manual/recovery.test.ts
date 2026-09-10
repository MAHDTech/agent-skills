import { expect, test } from "bun:test"
import { spawn } from "node:child_process"
import { mkdtempSync, rmSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { Ledger } from "./ledger"
import { hostArguments, reconcileRecovery, type RecoveryRequest } from "./recovery"

async function fixture(run: (ledger: Ledger, request: RecoveryRequest, path: string) => Promise<void>) {
    const root = mkdtempSync(join(tmpdir(), "factory-recovery-"))
    const child = spawn(process.execPath, ["-e", "process.exit(0)"])
    await new Promise<void>((resolve) => child.once("close", () => resolve()))
    const ledger = new Ledger(join(root, "ledger.ndjson"))
    ledger.append("launch", { pid: child.pid }, undefined, "same-conversation")
    ledger.append("leg", { prompt: "/tars-run-epic 42" })
    try {
        await run(ledger, { pid: child.pid!, conversation: "same-conversation", leg: "/tars-run-epic 42", limit: 3 }, join(root, "ledger.ndjson"))
    } finally {
        ledger.close()
        rmSync(root, { recursive: true, force: true })
    }
}

const inspect = async () => ({ disposition: "resume" as const, progress: "state-one", evidence: { phase: "R.1" } })

test("recovery checks process death before inspection and resumes explicit identity and scope", async () => {
    await fixture(async (ledger, request) => {
        let inspected = false
        await expect(reconcileRecovery({ ...request, pid: process.pid }, ledger, async () => { inspected = true; return inspect() })).rejects.toThrow("still present")
        expect(inspected).toBe(false)
        const recovered = await reconcileRecovery(request, ledger, inspect)
        expect(recovered?.conversation).toBe("same-conversation")
        expect(recovered?.prompt).toStartWith("/tars-run-epic 42\n")
        const argv = hostArguments("/workspace", recovered?.conversation)
        expect(argv.slice(-2)).toEqual(["--conversation", "same-conversation"])
        expect(argv).not.toContain("--continue")
        await expect(reconcileRecovery(request, ledger, inspect)).rejects.toThrow("no progress")
    })
})

test("completed work is reconciled without replay; parked state never retries", async () => {
    await fixture(async (ledger, request) => {
        expect(await reconcileRecovery(request, ledger, async () => ({ ...await inspect(), disposition: "completed" }))).toBeUndefined()
        expect(ledger.receipts.filter((r) => r.channel === "recovery")).toHaveLength(0)
        await expect(reconcileRecovery(request, ledger, async () => ({ ...await inspect(), disposition: "parked" }))).rejects.toThrow("parked")
    })
})

test("unknown refusal stops; exact independently permitted correction gets one durable retry", async () => {
    await fixture(async (ledger, request) => {
        request.stop = { kind: "refusal", reason: "read denied", tool: "view_file", target: "/private" }
        await expect(reconcileRecovery(request, ledger, inspect)).rejects.toThrow("uncertain")
        request.correction = {
            refusal: "read denied", tool: "view_file", target: "/private",
            permittedBrief: "Omit discovery; use the operator-provided brief.", permissionEvidence: "Existing authorized task brief",
        }
        const recovered = await reconcileRecovery(request, ledger, inspect)
        expect(recovered?.prompt).toContain("Forward this correction")
        expect(ledger.receipts.some((r) => r.channel === "recovery")).toBe(true)
        await expect(reconcileRecovery(request, ledger, inspect)).rejects.toThrow("already spent")
    })
})

test.each(["parked", "authentication", "content_filter"] as const)("%s never accepts a corrected retry", async (kind) => {
    await fixture(async (ledger, request) => {
        request.stop = { kind, reason: "human decision required" }
        await expect(reconcileRecovery(request, ledger, inspect)).rejects.toThrow("parked")
    })
})

test("reopened evidence preserves spent corrections while allowing a distinct permitted correction", async () => {
    await fixture(async (ledger, request, path) => {
        request.stop = { kind: "refusal", reason: "read denied", tool: "view_file", target: "/private" }
        request.correction = { refusal: "read denied", tool: "view_file", target: "/private", permittedBrief: "Use supplied brief", permissionEvidence: "Authorized brief" }
        await reconcileRecovery(request, ledger, inspect)
        const reopened = new Ledger(path)
        try {
            await expect(reconcileRecovery(request, reopened, inspect)).rejects.toThrow("already spent")
            request.stop = { ...request.stop, target: "/different" }
            request.correction = { ...request.correction, target: "/different" }
            expect(await reconcileRecovery(request, reopened, async () => ({ ...await inspect(), progress: "state-two" }))).toBeDefined()
            expect(reopened.receipts.filter((receipt) => receipt.channel === "recovery")).toHaveLength(2)
        } finally { reopened.close() }
    })
})
