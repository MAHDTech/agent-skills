import { expect, test } from "bun:test"
import { mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { Ledger } from "./ledger"

test("durable receipts preserve identity, timestamps and positions across reopen", () => {
    const directory = mkdtempSync(join(tmpdir(), "factory-ledger-"))
    const path = join(directory, "ledger.ndjson")
    try {
        const first = new Ledger(path, { TARS_GITHUB_TOKEN: "secret-value" })
        first.append("stdout", { error: "Bearer secret-value", api_key: "hidden", response: "password=hunter2" }, 78, "conversation")
        first.append("exit", { code: 1, signal: null })
        first.append("stdout", '{"api_key":"unlisted-credential","nested":{"password":"another-secret"}}')
        first.close()
        const disk = readFileSync(path, "utf8")
        expect(disk).not.toContain("secret-value")
        expect(disk).not.toContain("hidden")
        expect(disk).not.toContain("hunter2")
        expect(disk).not.toContain("unlisted-credential")
        expect(disk).not.toContain("another-secret")
        expect(statSync(path).mode & 0o777).toBe(0o600)
        const second = new Ledger(path, {})
        expect(second.receipts[0]?.conversation).toBe("conversation")
        expect(second.receipts[0]?.position).toBe(78)
        expect(Date.parse(second.receipts[0]!.received)).toBeGreaterThan(0)
        expect(second.append("handled", { turn: 1 }).seq).toBe(4)
        second.close()
    } finally {
        rmSync(directory, { recursive: true, force: true })
    }
})

test("incomplete ledger cannot silently discard the last action", () => {
    const directory = mkdtempSync(join(tmpdir(), "factory-ledger-"))
    const path = join(directory, "ledger.ndjson")
    try {
        writeFileSync(path, '{"seq":1', { mode: 0o600 })
        expect(() => new Ledger(path)).toThrow("truncated ledger")
        expect(readFileSync(path, "utf8")).toBe('{"seq":1')
    } finally {
        rmSync(directory, { recursive: true, force: true })
    }
})
