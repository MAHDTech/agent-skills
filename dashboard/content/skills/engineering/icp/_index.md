+++
title = "icp"
description = "Build, secure, and deploy Internet Computer (ICP) canisters and dapps in Motoko, Rust, or TypeScript. Use for canister state and upgrades, ICRC or chain-key tokens, identity, HTTPS outcalls, or icp CLI work."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


<!-- cspell:words autosync autoupgrade Candid candid canhelp ckerc ckeurc cklink ckusdc ckusdt ckwbtc CyclePay lintoko oisy OpenCloud reentrancy subaccount subaccounts timelock tokenomics utxo vetkd vetkey vetkeys -->

# Internet Computer (ICP) Development

The Internet Computer (ICP) executes smart contracts as **canisters**: WebAssembly actor units written in Motoko or Rust that bundle code and state, expose strongly typed Candid interfaces, persist state across upgrades via stable memory, pay for compute and storage under a reverse-gas cycles model, and serve browser or agent clients directly through HTTP gateways.

## Resolve the right sub-skill first

Upstream DFINITY maintains task-specific ICP sub-skills at `https://skills.internetcomputer.org/`. Load the matching sub-skill and its companion reference files before writing canister, client, or deployment configuration code:

1. **Match the task to a sub-skill:** Use the domain list below, then confirm against the `description` in the sub-skill file's frontmatter. `resources/auto/llms.txt` explains the upstream loading modes; the live trigger descriptions are in `https://skills.internetcomputer.org/.well-known/skills/index.json`.
2. **Read the local sub-skill and its references:** Open the downloaded skill in `resources/auto/well-known-skills-<skill-name>-skill.md` (for example, `resources/auto/well-known-skills-writing-motoko-skill.md` or `resources/auto/well-known-skills-icp-cli-skill.md`) along with any companion `resources/auto/well-known-skills-<skill-name>-references-<topic>.md` files listed in that skill.
3. **On-demand `curl` fallback (zero Node.js / zero `npx`):** When local `resources/auto/` files are absent or you need to verify the live upstream catalog (`https://skills.internetcomputer.org/.well-known/skills/index.json`), fetch the discovery index and the target sub-skill directly with `curl -fsSL`:

   ```bash
   curl -fsSL https://skills.internetcomputer.org/.well-known/skills/index.json
   curl -fsSL https://skills.internetcomputer.org/.well-known/skills/<skill-name>/SKILL.md
   ```

   If the entry in `index.json` lists additional paths in its `files` array (such as `references/*.md`), fetch those files from `https://skills.internetcomputer.org/.well-known/skills/<skill-name>/<file>` before implementing.

4. **Optional project-local install:** Use `npx skills add dfinity/icskills` only when a user explicitly asks to install pinned ICP skills into their own project repository and has Node.js available.

## Sub-skills by domain

Start from the domain:

- Motoko language and state persistence: `writing-motoko`, `reviewing-motoko`, `mops-cli`, `migrating-motoko-actors`, `troubleshooting-motoko-migrations`, `stable-memory`.
- Security, architecture, and cryptography: `canister-security`, `multi-canister`, `certified-variables`, `vetkeys`, `encrypted-maps`.
- Tokens, DeFi, and cross-chain: `icrc-ledger`, `ckbtc`, `cketh`, `cksol`, `evm-rpc`.
- Authentication, wallets, and governance: `internet-identity`, `agent-web-identity`, `wallet-integration`, `sns-launch`.
- Networking, hosting, and external services: `https-outcalls`, `static-site`, `custom-domains`, `smtp-gateway`, `service-discoverability`, `ic-dashboard`.
- Tooling, deployment, and cycles: `icp-cli`, `cycles-management`, `buy-cycles`, `cloud-engine-canisters`, `deploy-to-cloud-engine`, `caffeine-app`, `canhelp`, `autosync-ic-skills`, `autoupgrade-icp-cli`.

The upstream catalog grows; when `index.json` lists a sub-skill missing from these groups, use it.

## Core architectural rules and guardrails

1. **Prefer `icp` CLI and `mops` over legacy `dfx`:**
   - Default to `icp` (`icp.yaml`, `icp network start`, `icp build`, `icp deploy`) and `mops` (`mops.toml`, `mops test`) for new and modernized projects.
   - Use `dfx` commands only when working inside an existing repository anchored by `dfx.json` that has not migrated to `icp.yaml`.
2. **Enforce Access Control and Anonymous Principal Rejection:**
   - Validate caller identity at the start of every privileged update or query method and explicitly reject the anonymous principal (`2vxsx-fae`) before mutating state.
3. **Guard Against Inter-Canister `await` Reentrancy and Post-Await Traps:**
   - Canister execution is single-threaded per message slice, but state can change across `await` points while an inter-canister call is in flight.
   - Re-validate caller permissions and state invariants after every `await`, or use a per-principal `CallerGuard` lock.
   - Avoid panicking or trapping (`assert`, `unwrap()`, `unreachable!`) after an `await` if state was already mutated prior to the `await`, as post-await traps only roll back the callback execution slice.
4. **Preserve State Safely Across Upgrades:**
   - In Motoko, declare actors with `persistent actor` and manage schema changes through `mops` migration chains (`migrations/`) rather than ad-hoc pre/post-upgrade hooks.
   - In Rust, store upgrade-persistent state in `ic-stable-structures` (`MemoryManager`, `StableBTreeMap`, `StableCell`) rather than heap-only `thread_local!` collections.
5. **Distinguish Certified State from Uncertified Queries:**
   - Standard query calls execute on a single replica without consensus. Use update calls or certified variables (`ic-http-certification` / `CertTree`) whenever clients require cryptographic proof of response integrity.
6. **Configure HTTPS Outcalls and Cycle Pricing Accurately:**
   - For **replicated** HTTPS outcalls, always supply a transform function that strips non-deterministic headers (timestamps, request IDs, rate-limit counters) so all subnet replicas reach consensus on the response byte payload. For **non-replicated** (`is_replicated = false`) and **flexible** (`flexible_http_request`) outcalls, a transform is optional: omitting it reserves zero transform instructions, whereas supplying a transform under pricing v2 reserves the full query instruction limit unless you set `with_expected_transform_instructions`.
   - Match cycle attachment to the pricing version and SDK wrapper: pricing v1 uses `ic0.cost_http_request` (used by Motoko `Call.httpRequest` and legacy Rust `ic-cdk-management-canister` 0.1), while pricing v2 uses `ic0.cost_http_request_v2` (used by Rust `ic-cdk >= 0.20.3` with `ic-cdk-management-canister >= 0.2` via `HttpRequest::send()`).

## Verification checklist

Run the applicable checks for the project stack before declaring canister or client changes complete:

```bash
# 1. Motoko package and unit tests (when mops.toml is present)
mops test

# 2. Rust canister compilation target check (when Cargo.toml is present)
cargo check --target wasm32-unknown-unknown

# 3. Build all canisters configured in icp.yaml
icp build
```

- Verify that Candid interface changes (`.did`) remain backward-compatible with existing clients and upgrade state layouts.
- Confirm that every privileged update call rejects the anonymous principal (`2vxsx-fae`) and handles inter-canister call failures without trapping after state mutation.

