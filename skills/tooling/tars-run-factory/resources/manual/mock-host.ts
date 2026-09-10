import { createInterface } from "node:readline"

const mode = process.argv[2]
let turns = 0
const emit = (event: unknown) => process.stdout.write(JSON.stringify(event) + "\n")
emit({ event: "init", conversation_id: "mock-conversation" })
const lines = createInterface({ input: process.stdin })
lines.on("line", (line) => {
    turns++
    if (mode === "held-output") {
        Bun.spawn(["sleep", "1.5"], { stdin: "ignore", stdout: "inherit", stderr: "inherit" }).unref()
        process.exit(1)
    }
    if (mode === "eof") process.exit(1)
    if (mode === "timeout") return
    if (mode === "content-filter") {
        emit({ event: "result", result: { status: "ERROR", response: "", error: "response blocked by content safety filters" } })
        return
    }
    if (["partial-timeout", "split-timeout", "late-timeout", "fatal-stderr"].includes(mode ?? "")) {
        const notice = mode === "fatal-stderr" ? "error: host stream failed\n"
            : "[agy] print timeout after 5s with turn in progress; returning partial output\n"
        if (mode === "late-timeout") {
            emit({ event: "result", result: { status: "SUCCESS", response: "partial" } })
            setTimeout(() => process.stderr.write(notice), 20)
        } else if (mode === "split-timeout") {
            process.stderr.write(notice.slice(0, 40))
            setTimeout(() => {
                process.stderr.write(notice.slice(40))
                emit({ event: "result", result: { status: "SUCCESS", response: "partial" } })
            }, 5)
        } else {
            process.stderr.write(notice)
            emit({ event: "result", result: { status: "SUCCESS", response: "partial" } })
        }
        return
    }
    if (mode === "escalating-stop") {
        process.stdout.write([
            { event: "step_update", step_update: { tool_info: { error: "permission denied" } } },
            { event: "step_update", step_update: { error: "GitHub 401: authentication required" } },
        ].map((event) => JSON.stringify(event) + "\n").join(""))
        return
    }
    if (mode === "malformed") {
        process.stdout.write("not-json\n")
        return
    }
    if (mode === "truncated") {
        process.stdout.write('{"event":')
        process.exit(1)
    }
    if (mode === "denial") {
        emit({ event: "step_update", step_update: { tool_info: {
            name: "view_file", parameters: { path: "/private" },
            error: { message: "tool call denied by pre-tool hook" },
        } } })
        setTimeout(() => emit({ event: "result", result: { status: "SUCCESS" } }), 100)
        return
    }
    if (mode === "stderr") {
        process.stderr.write("tool call de")
        setTimeout(() => process.stderr.write("nied by pre-tool hook\n"), 5)
        return
    }
    const prompt = JSON.parse(line).message.content as string
    if (mode === "tool-error") {
        process.stderr.write("tool error: command exited 1; continuing\n")
        emit({ event: "step_update", step_update: { tool_info: { error: "error: command exited 1" } } })
    }
    const response = mode === "missing-server" ? "No MCP server was loaded."
        : prompt.startsWith("List the tools") ? "start_session advance_wave" : `turn ${turns}`
    const result = { event: "result", result: { status: "SUCCESS", response } }
    if (mode === "duplicate") process.stdout.write([result, result].map((event) => JSON.stringify(event) + "\n").join(""))
    else emit(result)
    if (mode === "late-denial") setTimeout(() => emit({ event: "step_update", step_update: { tool_info: { name: "view_file", parameters: { path: "/private" }, error: { message: "tool call denied by pre-tool hook" } } } }), 20)
})
lines.on("close", () => {
    if (mode === "slow-close") setTimeout(() => process.exit(0), 1500)
    else process.exit(0)
})
