import { expect, test } from "bun:test"
import { readFileSync } from "node:fs"
import { Ndjson, object, Turn } from "./stream"

const archive = process.env.FACTORY_REPLAY

test.skipIf(!archive)("frozen spike stops at the original refusal event", () => {
    const turn = new Turn()
    turn.begin()
    let firstStop: { elapsed: unknown; step: unknown; channel: unknown } | undefined
    const parser = new Ndjson((record) => {
        if (firstStop) return
        if (record.channel === "stdout") turn.accept(object(record.payload))
        if (record.channel === "stderr") turn.notice(String(record.payload))
        if (turn.stop) firstStop = {
            elapsed: record.elapsed,
            step: object(object(record.payload).step_update).step_index,
            channel: record.channel,
        }
    })
    const bytes = readFileSync(archive!)
    for (let offset = 0; offset < bytes.length; offset += 37) parser.push(bytes.subarray(offset, offset + 37))
    parser.end()
    expect(firstStop).toEqual({ elapsed: 159.7849583520001, step: 52, channel: "stdout" })
    expect(turn.stop?.kind).toBe("refusal")
    expect(turn.stop?.tool).toBe("view_file")
    expect(turn.conversation).toBe("cec0955e-9b2f-46f1-a703-16130dd85394")
    expect(turn.transport).toBeUndefined()
    expect(parser.position).toBe(bytes.length)
})
