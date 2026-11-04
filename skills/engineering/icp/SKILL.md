---
name: icp
description: Build, secure, test, and deploy Internet Computer (ICP) canisters and full-stack dapps in Motoko, Rust, and TypeScript. Use for canister architecture, stable memory, ICRC and chain-key tokens (ckBTC, ckETH, ckSOL), Internet Identity, vetKeys, HTTPS outcalls, or icp CLI workflows.
resources:
  - https://skills.internetcomputer.org/.well-known/skills/index.json
  - https://skills.internetcomputer.org/llms.txt
---

<!-- cspell:words autosync autoupgrade Candid candid canhelp ckerc ckeurc cklink ckusdc ckusdt ckwbtc CyclePay lintoko oisy OpenCloud reentrancy subaccount subaccounts timelock tokenomics utxo vetkd vetkey vetkeys -->

# Internet Computer (ICP) Development

The Internet Computer (ICP) executes smart contracts as **canisters**: WebAssembly actor units written in Motoko or Rust that bundle code and state, expose strongly typed Candid interfaces, persist state across upgrades via stable memory, pay for compute and storage under a reverse-gas cycles model, and serve browser or agent clients directly through HTTP gateways.

## Resolve the Right Sub-Skill First

Upstream DFINITY maintains 29 task-specific ICP sub-skills at `https://skills.internetcomputer.org/`. Always load the matching sub-skill and its companion reference files before writing canister, client, or deployment configuration code:

1. **Check the local index first:** Read `[resources/auto/llms.txt](resources/auto/llms.txt)` to match the user's task to the exact sub-skill name and trigger description.
2. **Read the local sub-skill and its references:** Open the downloaded skill in `resources/auto/well-known-skills-<skill-name>-skill.md` (for example, `[resources/auto/well-known-skills-writing-motoko-skill.md](resources/auto/well-known-skills-writing-motoko-skill.md)` or `[resources/auto/well-known-skills-icp-cli-skill.md](resources/auto/well-known-skills-icp-cli-skill.md)`) along with any companion `resources/auto/well-known-skills-<skill-name>-references-<topic>.md` files listed in that skill.
3. **On-demand `curl` fallback (zero Node.js / zero `npx`):** When local `resources/auto/` files are absent or you need to verify the live upstream catalog (`[index.json](https://skills.internetcomputer.org/.well-known/skills/index.json)`), fetch the discovery index and the target sub-skill directly with `curl -fsSL`:

   ```bash
   curl -fsSL https://skills.internetcomputer.org/.well-known/skills/index.json
   curl -fsSL https://skills.internetcomputer.org/.well-known/skills/<skill-name>/SKILL.md
   ```

   If the entry in `index.json` lists additional paths in its `files` array (such as `references/*.md`), fetch those files from `https://skills.internetcomputer.org/.well-known/skills/<skill-name>/<file>` before implementing.

4. **Optional project-local install:** Use `npx skills add dfinity/icskills` only when a user explicitly asks to install pinned ICP skills into their own project repository and has Node.js available.

## Sub-Skill Catalog by Domain

### 1. Motoko Language and State Persistence

| Sub-Skill                           | Local Resource File                                                                                                                           | Primary Scope and Companion References                                                                                                                       |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `writing-motoko`                    | `[well-known-skills-writing-motoko-skill.md](resources/auto/well-known-skills-writing-motoko-skill.md)`                                       | Modern Motoko syntax, `persistent actor`, `mo:core` standard library, mixins, and compiler error fixes (`well-known-skills-writing-motoko-references-*.md`). |
| `reviewing-motoko`                  | `[well-known-skills-reviewing-motoko-skill.md](resources/auto/well-known-skills-reviewing-motoko-skill.md)`                                   | Design review for `.mo` backends, type invariants, and stable vs transient state discipline (`state-and-persistence`, `type-encoding`).                      |
| `mops-cli`                          | `[well-known-skills-mops-cli-skill.md](resources/auto/well-known-skills-mops-cli-skill.md)`                                                   | Package management, toolchain pinning (`moc`, `lintoko`), building, and testing via `mops.toml`.                                                             |
| `migrating-motoko-actors`           | `[well-known-skills-migrating-motoko-actors-skill.md](resources/auto/well-known-skills-migrating-motoko-actors-skill.md)`                     | Actor state migration and schema evolution with `mops`-managed `migrations/` chains (`well-known-skills-migrating-motoko-actors-references-examples.md`).    |
| `troubleshooting-motoko-migrations` | `[well-known-skills-troubleshooting-motoko-migrations-skill.md](resources/auto/well-known-skills-troubleshooting-motoko-migrations-skill.md)` | Diagnostics and repair for broken migration chains and upgrade compatibility errors.                                                                         |
| `stable-memory`                     | `[well-known-skills-stable-memory-skill.md](resources/auto/well-known-skills-stable-memory-skill.md)`                                         | Upgrade-safe state persistence using `StableBTreeMap` and `MemoryManager` in Rust or persistent actors in Motoko.                                            |

### 2. Canister Security, Architecture, and Cryptography

| Sub-Skill             | Local Resource File                                                                                               | Primary Scope and Companion References                                                                                                                          |
| --------------------- | ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `canister-security`   | `[well-known-skills-canister-security-skill.md](resources/auto/well-known-skills-canister-security-skill.md)`     | Access control, anonymous principal rejection, reentrancy guards (`CallerGuard`), async saga safety, callback trap prevention, and cycle drain defenses.        |
| `multi-canister`      | `[well-known-skills-multi-canister-skill.md](resources/auto/well-known-skills-multi-canister-skill.md)`           | Inter-canister calls, canister factory patterns, bounded vs unbounded waits, and 2 MiB message payload limits.                                                  |
| `certified-variables` | `[well-known-skills-certified-variables-skill.md](resources/auto/well-known-skills-certified-variables-skill.md)` | Cryptographically verified query responses using Merkle trees (`RbTree`/`CertTree`), `ic-http-certification`, and `@dfinity/certificate-verification`.          |
| `vetkeys`             | `[well-known-skills-vetkeys-skill.md](resources/auto/well-known-skills-vetkeys-skill.md)`                         | Threshold key derivation (`vetKD`), identity-based encryption (IBE), timelock encryption, and BLS threshold signatures via `ic-vetkeys` and `@icp-sdk/vetkeys`. |
| `encrypted-maps`      | `[well-known-skills-encrypted-maps-skill.md](resources/auto/well-known-skills-encrypted-maps-skill.md)`           | End-to-end encrypted, access-controlled key-value maps built on vetKeys `EncryptedMaps`.                                                                        |

### 3. Tokens, DeFi, and Cross-Chain Integration

| Sub-Skill     | Local Resource File                                                                               | Primary Scope and Companion References                                                                                          |
| ------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `icrc-ledger` | `[well-known-skills-icrc-ledger-skill.md](resources/auto/well-known-skills-icrc-ledger-skill.md)` | Deploying and integrating ICRC-1 and ICRC-2 fungible token ledgers (`icrc1_transfer`, `icrc2_approve`, `icrc2_transfer_from`).  |
| `ckbtc`       | `[well-known-skills-ckbtc-skill.md](resources/auto/well-known-skills-ckbtc-skill.md)`             | Chain-key Bitcoin (`ckBTC`) deposit and withdrawal flows via the minter, subaccount derivation, and UTXO management.            |
| `cketh`       | `[well-known-skills-cketh-skill.md](resources/auto/well-known-skills-cketh-skill.md)`             | Chain-key Ethereum (`ckETH`) and `ckERC20` (`ckUSDC`, `ckUSDT`, `ckEURC`, `ckWBTC`, `ckLINK`) backend and frontend integration. |
| `cksol`       | `[well-known-skills-cksol-skill.md](resources/auto/well-known-skills-cksol-skill.md)`             | Chain-key Solana (`ckSOL`) deposit address derivation, deposits, and withdrawals.                                               |
| `evm-rpc`     | `[well-known-skills-evm-rpc-skill.md](resources/auto/well-known-skills-evm-rpc-skill.md)`         | Calling Ethereum and EVM chains (Arbitrum, Base, Optimism) from Rust canisters via `evm_rpc_client`.                            |

### 4. Authentication, Wallets, and Governance

| Sub-Skill            | Local Resource File                                                                                             | Primary Scope and Companion References                                                                                                              |
| -------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `internet-identity`  | `[well-known-skills-internet-identity-skill.md](resources/auto/well-known-skills-internet-identity-skill.md)`   | Passkey, OpenID, and enterprise SSO authentication via `@icp-sdk/auth`, identity attributes, shared subdomain sessions, and web push notifications. |
| `agent-web-identity` | `[well-known-skills-agent-web-identity-skill.md](resources/auto/well-known-skills-agent-web-identity-skill.md)` | Linking an AI agent or CLI session to a user's app-specific principal via `icp identity link web`.                                                  |
| `wallet-integration` | `[well-known-skills-wallet-integration-skill.md](resources/auto/well-known-skills-wallet-integration-skill.md)` | External wallet integration using `@icp-sdk/signer` and ICRC signer standards (`ICRC-25`, `ICRC-29`, `ICRC-49`, `ICRC-94`, `ICRC-167`, OISY).       |
| `sns-launch`         | `[well-known-skills-sns-launch-skill.md](resources/auto/well-known-skills-sns-launch-skill.md)`                 | Service Nervous System (SNS) DAO configuration, tokenomics, testflight validation, and decentralization swaps.                                      |

### 5. Networking, Web Hosting, and External Services

| Sub-Skill                 | Local Resource File                                                                                                       | Primary Scope and Companion References                                                                                                |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `https-outcalls`          | `[well-known-skills-https-outcalls-skill.md](resources/auto/well-known-skills-https-outcalls-skill.md)`                   | Canister HTTPS outcalls, deterministic transform functions, flexible outcalls, and `ic0.cost_http_request_v2` cycle budgeting.        |
| `static-site`             | `[well-known-skills-static-site-skill.md](resources/auto/well-known-skills-static-site-skill.md)`                         | Hosting frontends and SPAs via `@dfinity/static-site` (`certified-assets` canister) and migrating from `@dfinity/asset-canister`.     |
| `custom-domains`          | `[well-known-skills-custom-domains-skill.md](resources/auto/well-known-skills-custom-domains-skill.md)`                   | Registering custom domains with IC HTTP gateways, DNS records (`CNAME`, `TXT`, ACME), and `.well-known/ic-domains`.                   |
| `smtp-gateway`            | `[well-known-skills-smtp-gateway-skill.md](resources/auto/well-known-skills-smtp-gateway-skill.md)`                       | Receiving inbound email in canisters via `ic-smtp-gateway-protocol` and chunked delivery.                                             |
| `service-discoverability` | `[well-known-skills-service-discoverability-skill.md](resources/auto/well-known-skills-service-discoverability-skill.md)` | Publishing `/.well-known/ic-architecture` manifests, `candid:service` metadata, `getApiDoc`, and `/.well-known/ii-derivation-origin`. |
| `ic-dashboard`            | `[well-known-skills-ic-dashboard-skill.md](resources/auto/well-known-skills-ic-dashboard-skill.md)`                       | Querying public REST APIs at `dashboard.internetcomputer.org` for canister, ledger, SNS, and subnet telemetry.                        |

### 6. Tooling, Deployment, Cycles, and Cloud Engines

| Sub-Skill                | Local Resource File                                                                                                     | Primary Scope and Companion References                                                                                                          |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `icp-cli`                | `[well-known-skills-icp-cli-skill.md](resources/auto/well-known-skills-icp-cli-skill.md)`                               | Project configuration (`icp.yaml`), local networks (`icp network start`), builds (`icp build`), deployment (`icp deploy`), and `dfx` migration. |
| `cycles-management`      | `[well-known-skills-cycles-management-skill.md](resources/auto/well-known-skills-cycles-management-skill.md)`           | Monitoring cycle balances, freezing thresholds, canister top-ups, and converting ICP to cycles via the Cycles Minting Canister (CMC).           |
| `buy-cycles`             | `[well-known-skills-buy-cycles-skill.md](resources/auto/well-known-skills-buy-cycles-skill.md)`                         | Purchasing cycles with a credit card via `icp cycles buy` and CyclePay.                                                                         |
| `cloud-engine-canisters` | `[well-known-skills-cloud-engine-canisters-skill.md](resources/auto/well-known-skills-cloud-engine-canisters-skill.md)` | Authoring cycle-free canisters targeting CloudEngine subnets (OpenCloud).                                                                       |
| `deploy-to-cloud-engine` | `[well-known-skills-deploy-to-cloud-engine-skill.md](resources/auto/well-known-skills-deploy-to-cloud-engine-skill.md)` | Deploying canisters to a CloudEngine subnet via the `icp` CLI.                                                                                  |
| `caffeine-app`           | `[well-known-skills-caffeine-app-skill.md](resources/auto/well-known-skills-caffeine-app-skill.md)`                     | Scaffolding and building Caffeine (`caffeine.ai`) full-stack Motoko and React/Vite applications.                                                |
| `canhelp`                | `[well-known-skills-canhelp-skill.md](resources/auto/well-known-skills-canhelp-skill.md)`                               | Discovering and inspecting mainnet canister Candid interfaces by canister ID or name.                                                           |
| `autosync-ic-skills`     | `[well-known-skills-autosync-ic-skills-skill.md](resources/auto/well-known-skills-autosync-ic-skills-skill.md)`         | Configuring project session hooks to keep local IC skill copies synchronized.                                                                   |
| `autoupgrade-icp-cli`    | `[well-known-skills-autoupgrade-icp-cli-skill.md](resources/auto/well-known-skills-autoupgrade-icp-cli-skill.md)`       | Configuring session hooks to check and upgrade `icp` and `ic-wasm` binaries.                                                                    |

## Core Architectural Rules and Guardrails

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
6. **Make HTTPS Outcalls Deterministic:**
   - Always supply a transform function that strips non-deterministic headers (timestamps, request IDs, rate-limit counters) so all subnet replicas reach consensus on the response byte payload, and compute cycle attachment with `ic0.cost_http_request_v2`.

## Verification Checklist

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
