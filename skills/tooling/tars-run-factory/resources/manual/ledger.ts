import { closeSync, constants, fstatSync, fsyncSync, openSync, readFileSync, writeSync } from "node:fs"

export interface Receipt {
    seq: number
    received: string
    channel: string
    position?: number
    conversation?: string
    payload: unknown
}

export function redactor(environment: Record<string, string | undefined>) {
    const secrets = Object.entries(environment)
        .filter(([key, value]) => /token|secret|password|credential|api.?key/i.test(key) && value)
        .map(([, value]) => value!)
        .sort((left, right) => right.length - left.length)
    const text = (input: string) => {
        for (const secret of secrets) input = input.split(secret).join("[REDACTED]")
        return input
            .replace(/\b(?:gh[pousr]_[A-Za-z0-9_]+|github_pat_[A-Za-z0-9_]+)\b/g, "[REDACTED]")
            .replace(/(Bearer\s+)[^\s"\\]+/gi, "$1[REDACTED]")
            .replace(/("(?:token|password|secret|api[_-]?key|authorization)"\s*:\s*")[^"\\]*/gi, "$1[REDACTED]")
            .replace(/((?:token|password|secret|api[_-]?key)\s*[=:]\s*)[^\s,;"'\\]+/gi, "$1[REDACTED]")
    }
    const redact = (value: unknown): unknown => {
        if (typeof value === "string") {
            if (/^\s*[\[{]/.test(value)) {
                try { return JSON.stringify(redact(JSON.parse(value))) } catch { /* Malformed output still needs a redacted receipt. */ }
            }
            return text(value)
        }
        if (Array.isArray(value)) return value.map(redact)
        if (value !== null && typeof value === "object") {
            return Object.fromEntries(Object.entries(value).map(([key, item]) => [
                key, /token|secret|password|credential|api.?key/i.test(key) ? "[REDACTED]" : redact(item),
            ]))
        }
        return value
    }
    return redact
}

export class Ledger {
    readonly receipts: Receipt[]
    private readonly fd: number
    private readonly redact: (value: unknown) => unknown

    constructor(path: string, environment = process.env) {
        this.redact = redactor(environment)
        this.fd = openSync(path, constants.O_RDWR | constants.O_APPEND | constants.O_CREAT | constants.O_NOFOLLOW, 0o600)
        try {
            const stat = fstatSync(this.fd)
            if (!stat.isFile() || stat.nlink !== 1 || (stat.mode & 0o077)) throw new Error("ledger must be a private regular file with one link")
            const content = readFileSync(this.fd, "utf8")
            if (content && !content.endsWith("\n")) throw new Error("truncated ledger requires reconciliation")
            this.receipts = content.split("\n").filter(Boolean).map((line, index) => {
                const receipt = JSON.parse(line) as Receipt
                if (receipt.seq !== index + 1 || typeof receipt.channel !== "string" || !Number.isFinite(Date.parse(receipt.received))) throw new Error("invalid ledger sequence or timestamp")
                return receipt
            })
        } catch (error) {
            closeSync(this.fd)
            throw error
        }
    }

    append(channel: string, payload: unknown, position?: number, conversation?: string) {
        const receipt: Receipt = {
            seq: this.receipts.length + 1,
            received: new Date().toISOString(),
            channel,
            position,
            conversation,
            payload: this.redact(payload),
        }
        const bytes = Buffer.from(JSON.stringify(receipt) + "\n")
        let offset = 0
        while (offset < bytes.length) offset += writeSync(this.fd, bytes, offset)
        fsyncSync(this.fd)
        this.receipts.push(receipt)
        return receipt
    }

    close() {
        closeSync(this.fd)
    }
}
