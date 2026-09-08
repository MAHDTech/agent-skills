import { expect, test } from "bun:test"
import { mkdtempSync, mkdirSync, rmSync, symlinkSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { assertProcessGone, WorkspaceLease, workspaceHosts } from "./ownership"

test("canonical workspace aliases cannot obtain rival ownership", async () => {
    const root = mkdtempSync(join(tmpdir(), "factory-owner-"))
    const workspace = join(root, "workspace")
    mkdirSync(workspace)
    symlinkSync(workspace, join(root, "alias"))
    const lease = await WorkspaceLease.acquire(workspace, join(root, "locks"))
    try {
        await expect(WorkspaceLease.acquire(join(root, "alias"), join(root, "locks"))).rejects.toThrow("already supervised")
        lease.assertHeld()
    } finally {
        await lease.release()
    }
    try {
        const next = await WorkspaceLease.acquire(workspace, join(root, "locks"))
        await next.release()
        expect(() => next.assertHeld()).toThrow()
        expect(() => assertProcessGone(process.pid)).toThrow("still present")
    } finally {
        rmSync(root, { recursive: true, force: true })
    }
})

test("an existing host's relative add-dir prevents a competing supervisor", async () => {
    const root = mkdtempSync(join(tmpdir(), "factory-existing-"))
    mkdirSync(join(root, "workspace"))
    mkdirSync(join(root, "peer"))
    const host = Bun.spawn(["/bin/bash", "-c", "exec -a agy /bin/bash -c 'printf ready; read -r line' -- --add-dir ../workspace"], {
        cwd: join(root, "peer"), stdin: "pipe", stdout: "pipe", stderr: "pipe",
    })
    const reader = host.stdout.getReader()
    try {
        await reader.read()
        expect(workspaceHosts(join(root, "workspace"))).toContain(host.pid)
        await expect(WorkspaceLease.acquire(join(root, "workspace"), join(root, "locks"))).rejects.toThrow("existing agy")
    } finally {
        reader.releaseLock()
        host.kill()
        await host.exited
        rmSync(root, { recursive: true, force: true })
    }
})
