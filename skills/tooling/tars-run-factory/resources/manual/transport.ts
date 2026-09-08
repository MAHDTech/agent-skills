import { StringDecoder } from "node:string_decoder"
import { Ledger } from "./ledger"
import { Ndjson, Turn } from "./stream"

export class Host {
    readonly turn = new Turn()
    readonly child: Bun.Subprocess<"pipe", "pipe", "pipe">
    readonly exited: Promise<void>
    readonly initialized: Promise<void>
    readonly failed: Promise<Error>
    private recordFailure!: (error: Error) => void
    gone = false
    private fatal?: Error
    private recordedStop?: string
    private waiting?: { resolve: (turn: Turn) => void; reject: (error: Error) => void }
    private timer?: ReturnType<typeof setTimeout>
    private killTimer?: ReturnType<typeof setTimeout>
    private closing = false
    private readonly readers = new Set<{ cancel(): Promise<void> }>()

    constructor(argv: string[], cwd: string, private readonly ledger: Ledger, conversation?: string) {
        if (!argv[0]) throw new Error("missing host executable")
        this.turn.conversation = conversation
        ledger.append("launch_intent", { argv, cwd }, undefined, conversation)
        const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !["TARS_GITHUB_TOKEN", "TARS_DOYLE_GITHUB_TOKEN"].includes(key)))
        this.child = Bun.spawn(argv, { cwd, env, stdin: "pipe", stdout: "pipe", stderr: "pipe" })
        this.failed = new Promise((resolve) => { this.recordFailure = resolve })
        let initialized!: () => void
        let initializationFailed!: (error: Error) => void
        this.initialized = new Promise((resolve, reject) => { initialized = resolve; initializationFailed = reject })
        this.initialized.catch(() => {})
        const stdout = new Ndjson(
            (event) => {
                this.turn.accept(event)
                if (event.event === "init") {
                    ledger.append("identity", {}, undefined, this.turn.conversation)
                    initialized()
                }
                if (this.turn.stop) this.fail(new Error(this.turn.stop.reason))
                if (event.event === "result") {
                    setImmediate(() => {
                        if (!this.fatal && this.waiting) {
                            clearTimeout(this.timer)
                            const waiting = this.waiting
                            this.waiting = undefined
                            waiting.resolve(this.turn)
                        }
                    })
                }
            },
            (line, position) => ledger.append("stdout", line, position, this.turn.conversation),
        )
        const stderrDecoder = new StringDecoder("utf8")
        let stderr = ""
        let stderrPosition = 0
        const flushStderr = () => {
            if (stderr) ledger.append("stderr", stderr, stderrPosition, this.turn.conversation)
            stderr = ""
        }
        const readers = [
            this.consume(this.child.stdout, (bytes) => stdout.push(bytes), () => stdout.end()),
            this.consume(this.child.stderr, (bytes) => {
                stderrPosition += bytes.length
                stderr += stderrDecoder.write(bytes)
                this.turn.notice(stderr)
                if (this.turn.stop) this.fail(new Error(this.turn.stop.reason))
                if (stderr.endsWith("\n")) flushStderr()
                if (stderr.length > 1024 * 1024) this.fail(new Error("stderr record exceeds 1 MiB"))
            }, () => { stderr += stderrDecoder.end(); flushStderr() }),
        ]
        const child = this.child
        this.exited = (async () => {
            const code = await child.exited
            const drainTimer = setTimeout(() => {
                this.fail(new Error("incomplete output: descriptors remained open after host exit"))
                for (const reader of this.readers) void reader.cancel().catch(() => {})
            }, 1000)
            await Promise.all(readers)
            clearTimeout(drainTimer)
            this.gone = true
            clearTimeout(this.timer)
            clearTimeout(this.killTimer)
            try { ledger.append("exit", { pid: child.pid, code, signal: child.signalCode }, undefined, this.turn.conversation) }
            catch (error) { this.fatal ??= error instanceof Error ? error : new Error(String(error)) }
            const failure = this.fatal ?? new Error(`EOF with missing result (exit ${code}, signal ${child.signalCode})`)
            initializationFailed(failure)
            this.waiting?.reject(failure)
            this.waiting = undefined
        })()
        try { ledger.append("launch", { pid: this.child.pid, argv, cwd }, undefined, conversation) }
        catch (error) { this.fail(error) }
    }

    private async consume(stream: ReadableStream<Uint8Array>, accept: (bytes: Uint8Array) => void, end: () => void) {
        const reader = stream.getReader()
        this.readers.add(reader)
        try {
            while (true) {
                const item = await reader.read()
                if (item.done) break
                try { accept(item.value) } catch (error) { this.fail(error) }
            }
            end()
        } catch (error) { this.fail(error) }
        finally { this.readers.delete(reader); reader.releaseLock() }
    }

    private fail(error: unknown) {
        const first = !this.fatal
        if (first) {
            this.fatal = error instanceof Error ? error : new Error(String(error))
            this.recordFailure(this.fatal)
        }
        const stop = JSON.stringify(this.turn.stop)
        if (first || stop !== this.recordedStop) {
            this.recordedStop = stop
            try { this.ledger.append("failure", { reason: this.fatal!.message, stop: this.turn.stop }, undefined, this.turn.conversation) }
            catch { /* Process termination must survive a failed evidence write. */ }
        }
        if (!this.gone && !this.killTimer) {
            this.child.kill("SIGTERM")
            this.killTimer = setTimeout(() => this.child.kill("SIGKILL"), 1000)
        }
    }

    get failure() { return this.fatal }

    async send(prompt: string, timeoutMs: number): Promise<Turn> {
        if (this.gone || this.closing || this.fatal) throw this.fatal ?? new Error("host is closed")
        if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) throw new Error("positive timeout required")
        this.turn.begin()
        const input = { event: "user", message: { content: prompt } }
        this.ledger.append("stdin", input, undefined, this.turn.conversation)
        return new Promise((resolve, reject) => {
            this.waiting = { resolve, reject }
            this.timer = setTimeout(() => this.fail(new Error("turn timeout with incomplete result")), timeoutMs)
            try {
                this.child.stdin.write(JSON.stringify(input) + "\n")
                Promise.resolve(this.child.stdin.flush()).catch((error) => this.fail(new Error(`broken pipe: ${String(error)}`)))
            } catch (error) { this.fail(new Error(`broken pipe: ${String(error)}`)) }
        })
    }

    handled(workflow: unknown) {
        if (this.fatal) throw this.fatal
        this.ledger.append("handled", workflow, undefined, this.turn.conversation)
        this.turn.handled()
    }

    async close() {
        if (this.gone) return
        this.closing = true
        if (this.waiting) this.fail(new Error("shutdown during incomplete turn"))
        else {
            try { await this.child.stdin.end() } catch (error) { this.fail(error) }
        }
        const timer = setTimeout(() => this.fail(new Error("clean shutdown timeout")), 10_000)
        try { await this.exited } finally { clearTimeout(timer) }
    }
}
