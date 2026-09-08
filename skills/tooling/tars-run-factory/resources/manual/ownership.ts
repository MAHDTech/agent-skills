import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process"
import { createHash } from "node:crypto"
import { mkdirSync, readdirSync, readFileSync, readlinkSync, realpathSync } from "node:fs"
import { basename, join, resolve } from "node:path"

export function assertProcessGone(pid: number) {
    if (!Number.isSafeInteger(pid) || pid < 1) throw new Error("a recorded positive host PID is required")
    try { process.kill(pid, 0) } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ESRCH") return
        throw error
    }
    throw new Error(`previous host ${pid} is still present`)
}

export function workspaceHosts(workspace: string): number[] {
    const canonical = realpathSync(workspace)
    const found: number[] = []
    for (const entry of readdirSync("/proc")) {
        if (!/^\d+$/.test(entry)) continue
        let argv: string[]
        try { argv = readFileSync(`/proc/${entry}/cmdline`, "utf8").split("\0") }
        catch (error) {
            if (["ENOENT", "ESRCH", "EACCES"].includes((error as NodeJS.ErrnoException).code ?? "")) continue
            throw error
        }
        if (basename(argv[0] ?? "") !== "agy") continue
        try {
            const cwd = readlinkSync(`/proc/${entry}/cwd`)
            const paths = [cwd]
            argv.forEach((arg, index) => {
                if (arg === "--add-dir" && argv[index + 1]) paths.push(argv[index + 1]!)
                if (arg.startsWith("--add-dir=")) paths.push(arg.slice(10))
            })
            if (paths.some((path) => realpathSync(resolve(cwd, path)) === canonical)) found.push(Number(entry))
        } catch (error) {
            try { assertProcessGone(Number(entry)); continue } catch {}
            throw new Error(`cannot establish workspace of agy process ${entry}: ${String(error)}`)
        }
    }
    return found
}

export class WorkspaceLease {
    private constructor(
        readonly workspace: string,
        private readonly guard: ChildProcessWithoutNullStreams,
        private readonly closed: Promise<void>,
    ) {}

    static async acquire(workspace: string, lockRoot: string) {
        if (process.platform !== "linux") throw new Error("factory ownership verification currently requires Linux /proc and flock")
        const canonical = realpathSync(workspace)
        mkdirSync(lockRoot, { recursive: true, mode: 0o700 })
        const key = createHash("sha256").update(canonical).digest("hex")
        const guard = spawn("flock", ["--nonblock", "--conflict-exit-code", "75", join(lockRoot, key), process.execPath, join(import.meta.dir, "lease-holder.ts")], { stdio: "pipe" })
        const closed = new Promise<void>((resolve) => guard.on("close", () => resolve()))
        await new Promise<void>((resolve, reject) => {
            guard.once("error", reject)
            guard.once("exit", () => reject(new Error("workspace already supervised or flock unavailable")))
            guard.stdout.once("data", (bytes: Buffer) => {
                if (bytes.toString() === "locked\n") resolve()
                else reject(new Error("invalid workspace lock handshake"))
            })
        }).catch(async (error) => { guard.stdin.end(); await closed; throw error })
        const lease = new WorkspaceLease(canonical, guard, closed)
        try {
            lease.assertHeld()
            const hosts = workspaceHosts(canonical)
            if (hosts.length) throw new Error(`workspace has existing agy processes: ${hosts.join(", ")}`)
        } catch (error) {
            await lease.release()
            throw error
        }
        return lease
    }

    assertHeld() {
        if (this.guard.exitCode !== null || this.guard.signalCode !== null || this.guard.stdin.destroyed) {
            throw new Error("workspace lease is no longer held")
        }
    }

    async release() {
        this.guard.stdin.end()
        await this.closed
    }
}
