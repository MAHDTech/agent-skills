type ObjectValue = Record<string, unknown>

export function object(value: unknown): ObjectValue {
    return value !== null && typeof value === "object" && !Array.isArray(value)
        ? value as ObjectValue : {}
}

export class Ndjson {
    private pending = Buffer.alloc(0)
    position = 0

    constructor(
        private readonly consume: (event: ObjectValue, position: number) => void,
        private readonly record?: (line: string, position: number) => void,
    ) {}

    push(bytes: Uint8Array) {
        this.pending = Buffer.concat([this.pending, bytes])
        let end: number
        while ((end = this.pending.indexOf(10)) !== -1) {
            if (end > 16 * 1024 * 1024) throw new Error("NDJSON record exceeds 16 MiB")
            const line = this.pending.subarray(0, end)
            this.pending = this.pending.subarray(end + 1)
            this.record?.(line.toString("utf8"), this.position + end + 1)
            const event: unknown = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(line))
            if (!event || typeof event !== "object" || Array.isArray(event)) {
                throw new Error("NDJSON event must be an object")
            }
            this.position += end + 1
            this.consume(event as ObjectValue, this.position)
        }
        if (this.pending.length > 16 * 1024 * 1024) throw new Error("NDJSON record exceeds 16 MiB")
    }

    end() {
        if (this.pending.length) {
            this.record?.(this.pending.toString("utf8"), this.position + this.pending.length)
            throw new Error("truncated NDJSON at EOF")
        }
    }
}

export interface Stop {
    kind: "refusal" | "parked" | "authentication"
    reason: string
    tool?: string
    target?: string
}

const refusal = /denied|refused|permission.{0,30}(?:required|reject)|not (?:allowed|permitted)|outside this session's workspace/i
const parked = /\b(?:approval_needed|pending_human_merge|human_door|stalled|breaker.{0,12}trip|REVIEW_REQUIRED)\b/i
const authentication = /authentication required|\b(?:401|403)\b.{0,40}(?:github|unauthorized|forbidden)|github.{0,40}\b(?:401|403)\b/i

export class Turn {
    conversation?: string
    transport?: string
    response = ""
    stop?: Stop
    readonly children = new Map<string, { role: string }>()
    private phase: "idle" | "inflight" | "result" = "idle"
    private responseText = ""

    begin() {
        if (this.stop) throw new Error(`turn stopped: ${this.stop.reason}`)
        if (this.phase !== "idle") throw new Error("preceding result must be handled before another turn")
        this.phase = "inflight"
        this.transport = undefined
        this.response = ""
        this.responseText = ""
    }

    handled() {
        if (this.phase !== "result") throw new Error("no result to handle")
        this.phase = "idle"
    }

    notice(text: string, tool?: string, target?: string) {
        const kind = authentication.test(text) ? "authentication"
            : parked.test(text) ? "parked"
                : refusal.test(text) ? "refusal" : undefined
        const priority = { refusal: 1, parked: 2, authentication: 3 }
        if (kind && (!this.stop || priority[kind] > priority[this.stop.kind])) {
            this.stop = { kind, reason: text, tool, target }
        }
    }

    accept(event: ObjectValue) {
        const step = object(event.step_update)
        const result = object(event.result)
        for (const error of [event.error, step.error, result.error]) {
            if (error) this.notice(JSON.stringify(error))
        }
        const identity = event.conversation_id ?? step.conversation_id ?? result.conversation_id
        if (typeof identity === "string") {
            if (this.conversation && this.conversation !== identity) throw new Error("conversation identity changed")
            this.conversation = identity
        }
        if (event.event === "init") return
        if (event.event === "result") {
            if (this.phase !== "inflight") throw new Error("duplicate or unsolicited terminal result")
            if (typeof result.status !== "string") throw new Error("result missing transport status")
            this.transport = result.status
            this.response = typeof result.response === "string" ? result.response : JSON.stringify(result.response ?? "")
            this.notice(this.response)
            if (Array.isArray(result.denied_actions) && result.denied_actions.length) {
                this.notice(`denied_actions: ${JSON.stringify(result.denied_actions)}`)
            }
            this.phase = "result"
            return
        }
        if (event.event !== "step_update") return
        const tool = object(step.tool_info)
        if (tool.error) this.notice(JSON.stringify(tool.error), String(tool.name ?? step.tool_name ?? ""), JSON.stringify(tool.parameters ?? {}))
        if (tool.output !== undefined) this.notice(typeof tool.output === "string" ? tool.output : JSON.stringify(tool.output), String(tool.name ?? step.tool_name ?? ""), JSON.stringify(tool.parameters ?? {}))
        if (step.step_type === "agent_response" && typeof step.text_delta === "string") {
            this.responseText += step.text_delta
            this.notice(this.responseText)
        }
        const children = object(step.subagent_info).subagents
        if (Array.isArray(children)) {
            for (const child of children) {
                const metadata = object(child)
                if (typeof metadata.conversation_id === "string") {
                    this.children.set(metadata.conversation_id, { role: String(metadata.role ?? metadata.type_name ?? "") })
                }
            }
        }
    }
}
