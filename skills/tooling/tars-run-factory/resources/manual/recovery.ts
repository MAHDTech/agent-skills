import { Ledger } from "./ledger"
import { assertProcessGone } from "./ownership"
import { object, type Stop } from "./stream"

export interface Reconciliation {
    disposition: "resume" | "completed" | "parked"
    progress: string
    evidence: unknown
}

export interface Correction {
    refusal: string
    tool: string
    target: string
    permittedBrief: string
    permissionEvidence: string
}

export interface RecoveryRequest {
    pid: number
    conversation: string
    leg: string
    stop?: Stop
    correction?: Correction
    limit: number
}

export async function reconcileRecovery(
    request: RecoveryRequest,
    ledger: Ledger,
    inspect: () => Promise<Reconciliation>,
): Promise<{ conversation: string; prompt: string } | undefined> {
    assertProcessGone(request.pid)
    if (!request.conversation || !request.leg) throw new Error("recovery requires recorded conversation and authorized leg")
    const launch = ledger.receipts.findLast((receipt) => receipt.channel === "launch")
    const identity = ledger.receipts.findLast((receipt) => receipt.conversation)
    const leg = ledger.receipts.findLast((receipt) => receipt.channel === "leg")
    if (object(launch?.payload).pid !== request.pid || identity?.conversation !== request.conversation
        || object(leg?.payload).prompt !== request.leg) throw new Error("recovery does not match recorded host, identity or authorized leg")
    if (request.stop && request.stop.kind !== "refusal") throw new Error(`parked: ${request.stop.reason}`)
    const recoveries = ledger.receipts.filter((receipt) => receipt.channel === "recovery")
    if (!Number.isInteger(request.limit) || request.limit < 1 || recoveries.length >= request.limit) throw new Error("recovery limit reached")
    let correction: Correction | undefined
    if (request.stop) {
        correction = request.correction
        if (!correction || correction.refusal !== request.stop.reason || correction.tool !== request.stop.tool
            || correction.target !== request.stop.target || !correction.permittedBrief.trim() || !correction.permissionEvidence.trim()) {
            throw new Error("unresolved refusal: independently permitted correction is missing or uncertain")
        }
        if (recoveries.some((receipt) => {
            const previousCorrection = object(object(receipt.payload).correction)
            return previousCorrection.tool === correction!.tool && previousCorrection.target === correction!.target
        })) throw new Error("corrected refusal retry already spent")
    }
    const state = await inspect()
    if (!state.progress || state.evidence === undefined) throw new Error("recovery requires fresh stored-state evidence")
    ledger.append("reconciliation", { leg: request.leg, ...state }, undefined, request.conversation)
    if (state.disposition === "completed") return undefined
    if (state.disposition !== "resume") throw new Error("stored workflow is parked; recovery stopped")
    const previous = object(recoveries.at(-1)?.payload)
    if (previous.progress === state.progress) throw new Error("no progress since previous terminal recovery")
    const prompt = correction
        ? `${request.leg}\n${correction.permittedBrief}\nKeep the refused operation and target off limits. Forward this correction to every affected spoke. Resume existing work and preserve completed deliveries and ownership.`
        : `${request.leg}\nResume this recorded conversation and authorized leg from stored state. Preserve completed work and ownership. Do not reset the run or repeat completed work.`
    ledger.append("recovery", {
        leg: request.leg, progress: state.progress, corrected: Boolean(correction),
        correction, prompt,
    }, undefined, request.conversation)
    return { conversation: request.conversation, prompt }
}

export function hostArguments(workspace: string, conversation?: string): string[] {
    const argv = ["agy", "--input-format", "stream-json", "--output-format", "stream-json", "--add-dir", workspace, "--print-timeout", "90m"]
    if (conversation) argv.push("--conversation", conversation)
    return argv
}
