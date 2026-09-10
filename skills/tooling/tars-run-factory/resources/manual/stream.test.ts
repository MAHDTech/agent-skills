import { describe, expect, test } from "bun:test"
import { Ndjson, Turn } from "./stream"

const encode = (event: unknown) => Buffer.from(JSON.stringify(event) + "\n")
const denial = {
    event: "step_update",
    step_update: {
        conversation_id: "parent",
        step_index: 52,
        state: "ERROR",
        tool_info: {
            name: "view_file",
            parameters: { AbsolutePath: "/private/transcript.jsonl" },
            error: { message: "tool call denied by pre-tool hook: outside this session's workspace" },
        },
    },
}

describe("stream framing", () => {
    test("consumes every event across byte splits, including UTF-8", () => {
        const events: unknown[] = []
        const parser = new Ndjson((event) => events.push(event))
        const input = Buffer.concat([encode({ text: "café" }), encode(denial)])
        for (const byte of input) parser.push(Buffer.from([byte]))
        parser.end()
        expect(events).toEqual([{ text: "café" }, denial])
        expect(parser.position).toBe(input.length)
    })

    test("rejects malformed and truncated records without skipping them", () => {
        expect(() => new Ndjson(() => {}).push(Buffer.from("oops\n"))).toThrow()
        const parser = new Ndjson(() => {})
        parser.push(Buffer.from('{"event":'))
        expect(() => parser.end()).toThrow("truncated")
    })
})

describe("turn decisions", () => {
    test.each([
        { event: "result", result: { status: "ERROR", error: "response blocked by content safety filters" } },
        { event: "step_update", step_update: { error: "The response was blocked by content safety filters. Please try rephrasing your request." } },
        { event: "result", result: { status: "ERROR", error: { stop_reason: "STOP_REASON_CONTENT_FILTER" } } },
    ])("content-filter errors stop the turn: %j", (event) => {
        const turn = new Turn()
        turn.begin()
        turn.accept(event)
        expect(turn.stop?.kind).toBe("content_filter")
        expect(() => turn.begin()).toThrow("turn stopped")
    })

    test.each([
        "Review the content safety filter configuration.",
        "The documented error is STOP_REASON_CONTENT_FILTER.",
        "The fixture contains: response blocked by content safety filters",
    ])("successful prose and tool output may quote filter diagnostics: %s", (text) => {
        const turn = new Turn()
        turn.begin()
        turn.accept({ event: "step_update", step_update: { tool_info: { output: text } } })
        turn.accept({ event: "step_update", step_update: { step_type: "agent_response", text_delta: text } })
        turn.accept({ event: "result", result: { status: "SUCCESS", response: text } })
        expect(turn.stop).toBeUndefined()
    })

    test.each([
        { event: "step_update", step_update: { tool_info: { name: "view_file", output: { error: "permission denied" } } } },
        { event: "step_update", step_update: { error: { message: "permission denied" } } },
        { event: "result", result: { status: "SUCCESS", error: { message: "permission denied" } } },
        { event: "init", error: { message: "permission denied" } },
    ])("structured errors cannot hide a refusal: %j", (event) => {
        const turn = new Turn()
        turn.begin()
        turn.accept(event)
        expect(turn.stop?.kind).toBe("refusal")
    })

    test("a later human or authentication gate supersedes a correctable refusal", () => {
        const turn = new Turn()
        turn.begin()
        turn.accept(denial)
        turn.notice("permission denied; approval_needed")
        expect(turn.stop?.kind).toBe("parked")
        turn.notice("GitHub 401: authentication required")
        expect(turn.stop?.kind).toBe("authentication")
    })

    test("denial between display samples stops at the error event", () => {
        const turn = new Turn()
        turn.accept({ event: "init", conversation_id: "parent" })
        turn.begin()
        turn.accept(denial)
        expect(turn.stop?.kind).toBe("refusal")
        expect(turn.stop?.tool).toBe("view_file")
        expect(turn.stop?.target).toContain("/private/transcript.jsonl")
        turn.accept({ event: "result", result: { status: "SUCCESS" } })
        expect(turn.stop?.kind).toBe("refusal")
        expect(turn.transport).toBe("SUCCESS")
        expect(() => turn.begin()).toThrow()
    })

    test.each(["approval_needed", "pending_human_merge", "human_door", "stalled"])(
        "SUCCESS with %s parks without another turn",
        (directive) => {
            const turn = new Turn()
            turn.begin()
            turn.accept({ event: "result", result: { status: "SUCCESS", response: directive } })
            expect(turn.stop?.kind).toBe("parked")
            expect(() => turn.begin()).toThrow()
        },
    )

    test("response fallback catches denial even with absent denied_actions", () => {
        const turn = new Turn()
        turn.begin()
        turn.accept({ event: "result", result: { status: "SUCCESS", response: "Operation refused by hook" } })
        expect(turn.stop?.kind).toBe("refusal")
    })

    test("one in-flight turn, one handled result, stable identity", () => {
        const turn = new Turn()
        turn.accept({ event: "init", conversation_id: "parent" })
        turn.begin()
        expect(() => turn.begin()).toThrow()
        turn.accept({ event: "result", result: { status: "SUCCESS" } })
        expect(() => turn.begin()).toThrow("handled")
        turn.handled()
        turn.begin()
        expect(() => turn.accept({ event: "init", conversation_id: "rival" })).toThrow()
    })

    test("duplicate terminal result is rejected", () => {
        const turn = new Turn()
        turn.begin()
        const result = { event: "result", result: { status: "SUCCESS" } }
        turn.accept(result)
        expect(() => turn.accept(result)).toThrow("duplicate")
    })

    test("spawn DONE inventories children without claiming completion", () => {
        const turn = new Turn()
        turn.begin()
        turn.accept({ event: "step_update", step_update: {
            state: "DONE", subagent_info: { subagents: [
                { conversation_id: "child", role: "DOYLE", log_uri: "file:///private" },
            ] },
        } })
        expect(turn.children.get("child")).toEqual({ role: "DOYLE" })
    })
})
