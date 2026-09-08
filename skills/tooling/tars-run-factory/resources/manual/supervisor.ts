import { writeFileSync } from "node:fs"
import { join } from "node:path"
import { Ledger, redactor } from "./ledger"
import { assertProcessGone } from "./ownership"
import { hostArguments, reconcileRecovery, type Correction, type Reconciliation } from "./recovery"
import { object, type Stop } from "./stream"
import { Host } from "./transport"

export interface Options {
    workspace: string
    conversation?: string
    epic?: number
    merge: boolean
    cycles: number
    runtimeMs: number
    turnMs: number
    recoveryLimit: number
    audit: boolean
    triage: boolean
    smoke: boolean
}

interface Position {
    phase: "ready" | "running" | "result" | "failed" | "stopped"
    started: number
    cycles: number
    failures: number
    noProgress: number
    progress?: string
    conversation?: string
    pid?: number
    leg?: string
    action?: string
    resultSeq?: number
    transport?: string
    response?: string
    stop?: Stop
    reason?: string
    nextAt: number
    completed: string[]
    children?: Array<{ conversation: string; role: string }>
}

export interface Control {
    action: string
    resultSeq?: number
    directive?: string
    progress?: string
    evidence?: unknown
    produced?: unknown
    reason?: string
    reconciliation?: Reconciliation
    correction?: Correction
}

export class Supervisor {
    readonly position: Position
    private host?: Host
    private busy = false
    private stopping?: Promise<void>
    private readonly deadline: ReturnType<typeof setTimeout>
    readonly finished: Promise<void>
    private finish!: () => void
    private readonly redact = redactor(process.env)

    constructor(
        readonly options: Options,
        private readonly ledger: Ledger,
        private readonly outputDirectory: string,
        private readonly assertOwner: () => void,
        private readonly launch = (conversation?: string) => new Host(hostArguments(options.workspace, conversation), options.workspace, ledger, conversation),
    ) {
        for (const value of [options.cycles, options.runtimeMs, options.turnMs, options.recoveryLimit]) {
            if (!Number.isSafeInteger(value) || value < 1) throw new Error("factory limits must be positive integers")
        }
        if (options.epic !== undefined && (!Number.isSafeInteger(options.epic) || options.epic < 1)) throw new Error("invalid epic scope")
        if (options.epic && (options.audit || options.triage)) throw new Error("epic scope excludes repository-wide audit and triage")
        const configuration = ledger.receipts.find((receipt) => receipt.channel === "configuration")
        if (configuration && JSON.stringify(configuration.payload) !== JSON.stringify(options)) throw new Error("invocation differs from recorded scope or limits")
        if (!configuration) ledger.append("configuration", options)
        const checkpoint = ledger.receipts.findLast((receipt) => receipt.channel === "checkpoint")
        this.position = checkpoint ? structuredClone(checkpoint.payload) as Position : {
            phase: "ready", started: Date.now(), cycles: 0, failures: 0, noProgress: 0, nextAt: 0, completed: [], conversation: options.conversation,
        }
        const launches = ledger.receipts.filter((receipt) => receipt.channel === "launch")
        const latestLaunch = launches.at(-1)
        if (latestLaunch) {
            this.position.pid = Number(object(latestLaunch.payload).pid)
            assertProcessGone(this.position.pid)
            this.position.conversation = ledger.receipts.findLast((receipt) => receipt.conversation)?.conversation
            const unfinished = ledger.receipts.some((receipt) => receipt.seq > (checkpoint?.seq ?? 0)
                && ["launch", "stdin", "stdout", "failure"].includes(receipt.channel))
            if (unfinished || this.position.phase === "running") {
                this.position.phase = "failed"
                this.position.reason = "foreman interrupted; reconcile recorded conversation before recovery"
                const failure = object(ledger.receipts.findLast((receipt) => receipt.channel === "failure")?.payload)
                this.position.stop = failure.stop as Stop | undefined
                this.position.leg = String(object(ledger.receipts.findLast((receipt) => receipt.channel === "leg")?.payload).prompt ?? "")
            }
        }
        this.checkpoint()
        this.finished = new Promise((resolve) => { this.finish = resolve })
        this.deadline = setTimeout(() => { void this.stop("stopped", "runtime limit reached") }, Math.max(1, options.runtimeMs - (Date.now() - this.position.started)))
        if (this.position.phase === "stopped") { clearTimeout(this.deadline); this.finish() }
    }

    snapshot() {
        this.inventory()
        return this.redact(structuredClone(this.position))
    }
    private isStopped() { return this.position.phase === "stopped" }

    private inventory() {
        if (this.host) this.position.children = Array.from(this.host.turn.children, ([conversation, child]) => ({ conversation, role: child.role }))
    }

    private checkpoint() { this.inventory(); this.ledger.append("checkpoint", this.position, undefined, this.position.conversation) }

    private prompt(action: string): string {
        if (this.options.smoke) {
            if (action !== "status") throw new Error("smoke permits status turns only")
            return "Reply with exactly FACTORY_SMOKE_OK. Do not call tools or change files."
        }
        if (action === "probe") return "List the tools published by the MCP server named tars. Names only."
        if (!this.position.completed.includes("probe")) throw new Error("complete and verify the MCP probe first")
        if (action === "audit" && this.options.audit) return "/tars-audit-workspace"
        if (action === "triage" && this.options.triage) return "/tars-triage-backlog"
        if (action === "cycle") {
            if (this.options.audit && !this.position.completed.includes("audit")) throw new Error("authorized audit is unfinished")
            if (this.options.triage && !this.position.completed.includes("triage")) throw new Error("authorized triage is unfinished")
            const scope = this.options.epic ? `/tars-run-epic ${this.options.epic}` : "/tars-run-batch all"
            return scope + (this.options.merge ? " --merge" : "")
        }
        if (action === "status") return "/tars-show-status"
        throw new Error("action is outside the authorized factory scope")
    }

    async control(control: Control): Promise<unknown> {
        if (control.action === "stop") {
            await this.stop("stopped", control.reason ?? "foreman stopped the shift")
            return this.snapshot()
        }
        if (this.busy) throw new Error("one control and one host turn may be in flight")
        if (this.position.phase === "stopped") throw new Error("shift has stopped")
        this.assertOwner()
        if (Date.now() - this.position.started >= this.options.runtimeMs) {
            await this.stop("stopped", "runtime limit reached")
            return this.snapshot()
        }
        this.busy = true
        try {
            if (this.host?.failure) await this.observeFailure(this.host, this.host.failure)
            if (control.action === "handle") return await this.handle(control)
            if (control.action === "recover") return await this.recover(control)
            if (control.action === "retry-probe") return await this.retryProbe(control)
            if (this.position.phase !== "ready") throw new Error("preceding result or terminal failure must be handled first")
            if (Date.now() < this.position.nextAt) throw new Error("pending CI pacing interval has not elapsed")
            if ((control.action === "cycle" || this.options.smoke) && this.position.cycles >= this.options.cycles) {
                await this.stop("stopped", "cycle limit reached")
                return this.snapshot()
            }
            if (["probe", "audit", "triage"].includes(control.action) && this.position.completed.includes(control.action)) throw new Error("completed stage cannot be repeated")
            const prompt = this.prompt(control.action)
            this.position.action = control.action
            this.position.leg = prompt
            if (control.action === "cycle" || this.options.smoke) this.position.cycles++
            this.ledger.append("leg", { prompt, action: control.action }, undefined, this.position.conversation)
            return await this.send(prompt)
        } finally { this.busy = false }
    }

    private async send(prompt: string): Promise<unknown> {
        this.assertOwner()
        if (!this.host || this.host.gone) {
            if (this.position.pid) assertProcessGone(this.position.pid)
            this.host = this.launch(this.position.conversation)
            const host = this.host
            void host.failed.then((error) => this.observeFailure(host, error))
            this.position.pid = this.host.child.pid
        }
        this.position.phase = "running"
        this.position.transport = undefined
        this.position.response = undefined
        this.position.stop = undefined
        this.position.reason = undefined
        this.checkpoint()
        try {
            const result = await this.host.send(prompt, Math.min(this.options.turnMs, this.options.runtimeMs - (Date.now() - this.position.started)))
            this.position.conversation = result.conversation
            this.position.transport = result.transport
            this.position.response = result.response
            this.position.stop = result.stop
            if (this.isStopped()) return this.snapshot()
            this.position.phase = "result"
            const receipt = this.ledger.append("turn_result", { transport: result.transport, response: result.response }, undefined, result.conversation)
            this.position.resultSeq = receipt.seq
            this.checkpoint()
            return this.snapshot()
        } catch (error) {
            await this.host.close()
            await this.observeFailure(this.host, error)
            return this.snapshot()
        }
    }

    private async observeFailure(host: Host, error: unknown) {
        await host.exited
        if (this.host !== host || this.isStopped() || this.position.phase === "failed") return
        this.position.conversation = host.turn.conversation
        this.position.stop = host.turn.stop
        this.position.reason = String(error)
        this.position.phase = "failed"
        this.checkpoint()
        this.report("stopped", this.position.reason)
    }

    private async handle(control: Control) {
        if (this.position.phase !== "result" || control.resultSeq !== this.position.resultSeq) throw new Error("acknowledgement must name the current result receipt")
        if (!control.progress || control.evidence === undefined || !control.directive) throw new Error("fresh workflow evidence and progress fingerprint are required")
        const directives = ["continue", "pending_ci", "drained", "parked", "approval_needed", "pending_human_merge", "human_door", "stalled"]
        if (!directives.includes(control.directive)) throw new Error("unknown workflow directive; park and report")
        const completedTransport = ["SUCCESS", "RECONCILED"].includes(this.position.transport ?? "")
        if (control.directive === "drained" && !completedTransport) throw new Error("failed transport cannot establish drained workflow")
        if (this.position.action === "probe" && completedTransport && !["start_session", "advance_wave"].every((tool) => this.position.response?.includes(tool))) throw new Error("MCP probe did not establish the required tool surface")
        this.position.failures = completedTransport ? 0 : this.position.failures + 1
        this.position.noProgress = this.position.progress === control.progress ? this.position.noProgress + 1 : 0
        this.position.progress = control.progress
        const evidence = { directive: control.directive, progress: control.progress, evidence: control.evidence, produced: control.produced }
        if (this.host && !this.host.gone) this.host.handled(evidence)
        else this.ledger.append("handled", evidence, undefined, this.position.conversation)
        if (control.directive === "drained") { await this.stop("drained", "foreman verified workflow completion"); return this.snapshot() }
        if (!["continue", "pending_ci"].includes(control.directive)) { await this.stop("parked", control.directive); return this.snapshot() }
        if (this.position.failures >= 2 || this.position.noProgress >= 2) {
            await this.stop("stopped", this.position.failures >= 2 ? "two consecutive failed turns" : "no-progress limit reached")
            return this.snapshot()
        }
        if (completedTransport && this.position.action) this.position.completed.push(this.position.action)
        if (this.position.cycles >= this.options.cycles) { await this.stop("stopped", "cycle limit reached"); return this.snapshot() }
        this.position.phase = "ready"
        this.position.nextAt = control.directive === "pending_ci" ? Date.now() + 600_000 : 0
        this.checkpoint()
        return this.snapshot()
    }

    private async recover(control: Control) {
        if (this.position.phase !== "failed" || !control.reconciliation) throw new Error("failed host and fresh reconciliation required")
        const request = {
            pid: this.position.pid!, conversation: this.position.conversation!, leg: this.position.leg!,
            stop: this.position.stop, correction: control.correction, limit: this.options.recoveryLimit,
        }
        try {
            const recovery = await reconcileRecovery(request, this.ledger, async () => control.reconciliation!)
            if (!recovery) {
                this.position.phase = "result"
                this.position.transport = "RECONCILED"
                const receipt = this.ledger.append("turn_result", { reconciliation: control.reconciliation }, undefined, this.position.conversation)
                this.position.resultSeq = receipt.seq
                this.checkpoint()
                return this.snapshot()
            }
            return await this.send(recovery.prompt)
        } catch (error) {
            await this.stop("parked", String(error))
            return this.snapshot()
        }
    }

    private async retryProbe(control: Control) {
        if (this.position.phase !== "result" || this.position.action !== "probe" || control.resultSeq !== this.position.resultSeq) throw new Error("only the current missing-server probe can be retried")
        if (this.ledger.receipts.some((receipt) => receipt.channel === "probe_retry")) { await this.stop("stopped", "MCP probe retry exhausted"); return this.snapshot() }
        if (this.position.transport === "SUCCESS" && ["start_session", "advance_wave"].every((tool) => this.position.response?.includes(tool))) throw new Error("MCP probe already established the tool surface")
        const evidence = { directive: "missing_server", response: this.position.response }
        if (this.host && !this.host.gone) { this.host.handled(evidence); await this.host.close() }
        else this.ledger.append("handled", evidence, undefined, this.position.conversation)
        this.ledger.append("probe_retry", evidence, undefined, this.position.conversation)
        return this.send(this.position.leg!)
    }

    async stop(outcome: string, reason: string) {
        if (this.stopping) return this.stopping
        clearTimeout(this.deadline)
        if (this.isStopped()) { this.finish(); return }
        this.position.phase = "stopped"
        this.position.reason = reason
        this.stopping = (async () => {
            if (this.host) await this.host.close()
            this.checkpoint()
            this.report(outcome, reason)
            this.finish()
        })()
        return this.stopping
    }

    private report(outcome: string, reason: string) {
        const json = (value: unknown) => "```json\n" + JSON.stringify(this.redact(value), null, 2) + "\n```"
        const produced = this.ledger.receipts.filter((receipt) => receipt.channel === "handled").map((receipt) => ({ seq: receipt.seq, ...object(receipt.payload) }))
        const anomalies = this.ledger.receipts.filter((receipt) => ["failure", "recovery"].includes(receipt.channel))
        const sections = [
            "# Factory handover", "## Outcome", `- ${this.redact(outcome)}: ${JSON.stringify(this.redact(reason))}.`,
            `- Transport: ${this.position.transport ?? "no terminal result"}; workflow, approval, CI and merge evidence are separate.`,
            "## Produced", produced.length ? json(produced) : "- No produced outcomes were verified.",
            "## Needs a human", outcome === "parked" ? `- ${JSON.stringify(this.redact(reason))}.` : "- Review unfinished work and evidence before another shift.",
            "## Anomalies", anomalies.length ? json(anomalies) : "- No recorded failure or recovery events.",
            "## Handover", "- Invocation and durable position:", json({ options: this.options, position: this.position }),
            "- Restart after interruption with the same invocation and state directory; reconcile an unfinished leg before sending input.",
            "- A stopped shift is terminal; obtain the required human decision before preparing a new bounded shift with the recorded conversation.",
        ]
        writeFileSync(join(this.outputDirectory, "FACTORY_REPORT.md"), sections.join("\n\n") + "\n", { mode: 0o600 })
    }
}
