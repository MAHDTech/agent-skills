> ## Documentation Index
> Fetch the complete documentation index at: https://agentclientprotocol.com/llms.txt
> Use this file to discover all available pages before exploring further.

# RFD Updates

> Lifecycle updates for ACP Requests for Dialog

This page tracks lifecycle changes for ACP Requests for Dialog. For broader ACP announcements, see [Updates](https://agentclientprotocol.com/updates.md).

**October 12, 2026**

## Client-Provided System Prompt RFD moves to Draft stage

The RFD for letting Clients provide session-scoped system prompt instructions in ACP v1 and v2 has moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/client-system-prompt.md) for more information and provide feedback on the delivery modes and session lifecycle semantics.

**October 9, 2026**

## Session Notices RFD moves to Completed

The Session Notices RFD has been stabilized and is now included in the
protocol schemas and guides for ACP v1 and v2.
See the [v1 guide](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#session-notices),
the [v2 guide](https://agentclientprotocol.com/protocol/v2/prompt-lifecycle.md#session-notices), and the
[RFD](https://agentclientprotocol.com/rfds/session-notices.md) for delivery, presentation, and replay semantics.

**October 6, 2026**

## Session Compaction RFD moves to Completed

The Session Compaction RFD has been stabilized and is now included in the
protocol schemas and guides for ACP v1 and v2. V1 delivery and replay still require the Client's
`session.compaction` capability; v2 does not require a capability.
See the [v1 guide](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#session-compaction),
the [v2 guide](https://agentclientprotocol.com/protocol/v2/prompt-lifecycle.md#session-compaction), and the
[RFD](https://agentclientprotocol.com/rfds/session-compaction.md) for patch, ordering, and replay semantics.

**September 30, 2026**

## Next Edit Suggestions RFD removed

The Next Edit Suggestions (NES) RFD and its unstable implementation have been
removed from ACP v1 and v2. This includes NES methods and capabilities,
associated document-event notifications, and position-encoding negotiation.

**September 30, 2026**

## Next Edit Suggestions RFD moves to To be removed

The Next Edit Suggestions (NES) RFD moved to "To be removed" because the
proposal was no longer being pursued. Its draft implementation was scheduled
for removal. See the [removal update](#next-edit-suggestions-rfd-removed) for
the subsequent cleanup.

**September 24, 2026**

## Session Notices RFD moves to Preview stage

The Session Notices RFD has moved to Preview. Agents can send live user-facing
notices without adding conversation history, with Client-controlled
presentation and dismissal. The protocol implementation covers ACP v1 and v2;
v1 implementations have landed in Zed and the Claude Agent and Codex adapters.
The feature remains unstable. Please review the [RFD](https://agentclientprotocol.com/rfds/session-notices.md)
and provide Agent/Client interoperability feedback before stabilization.

**September 23, 2026**

## Session Compaction RFD moves to Preview stage

The Session Compaction RFD has moved to Preview. Agents can report
ID-addressed context compaction lifecycle updates and optionally stream or
provide retained summaries in ACP v1 and v2. The feature remains unstable.
Please review the [RFD](https://agentclientprotocol.com/rfds/session-compaction.md), especially the
[Preview validation targets](https://agentclientprotocol.com/rfds/session-compaction.md#preview-validation), and
provide Agent/Client implementation feedback before the feature is stabilized.

**September 17, 2026**

## Tool Call Name RFD moves to Completed

The Tool Call Name RFD has been stabilized and is now part of the protocol.
Agents can report an optional programmatic tool `name` alongside each
human-readable tool-call title in ACP v1 and v2. The field is informational
metadata and does not grant capabilities or authorization. Please review the
[v1 tool-call documentation](https://agentclientprotocol.com/protocol/v1/tool-calls.md), the
[v2 tool-call documentation](https://agentclientprotocol.com/protocol/v2/tool-calls.md), and the
[RFD](https://agentclientprotocol.com/rfds/tool-call-name.md) for the version-specific update semantics.

**September 14, 2026**

## Tool Call Name RFD moves to Preview stage

The RFD for adding an optional programmatic `name` to tool calls in ACP v1 and v2 has moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/tool-call-name.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**August 20, 2026**

## Terminal Authentication RFD moves to Completed

The Terminal Authentication RFD has been stabilized and is now part of the
protocol. Clients can advertise terminal authentication support, run an Agent's
configured interactive login flow, and reconnect after it succeeds. The v1 and
v2 wire shapes are now included in the stable schemas and SDK types. Please
review the [authentication documentation](https://agentclientprotocol.com/protocol/v1/authentication.md) and the
[RFD](https://agentclientprotocol.com/rfds/auth-methods.md) for the version-specific capability and lifecycle
semantics.

**August 5, 2026**

## Session Notices RFD moves to Draft stage

The RFD for fire-and-forget, Client-presented session notices in ACP v1 and v2 has moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-notices.md) for more information and provide feedback on the event and presentation semantics.

**August 5, 2026**

## Session Compaction RFD moves to Draft stage

The RFD for reporting ID-addressed context compaction lifecycle and optional retained summaries in ACP v1 and v2 has moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-compaction.md) for more information and provide feedback while the entity and replay semantics are refined.

**July 27, 2026**

## Terminal Authentication RFD moves to Preview

The Terminal Authentication RFD has moved to Preview. Clients can run an
Agent's interactive login flow in a terminal and reconnect after it succeeds.
The proposal now focuses on terminal authentication. The experimental `env_var`
method has been removed as an explicit SDK and schema variant. Please review
the [RFD](https://agentclientprotocol.com/rfds/auth-methods.md) for the version-specific wire shapes and lifecycle
semantics.

**July 24, 2026**

## Elicitation RFD moves to Completed

The RFD for structured user input through `elicitation/create` has been
stabilized and is now part of the protocol. Agents can request form-based
structured input or URL-based out-of-band interactions, and Clients advertise
each mode they support explicitly. Please review the
[documentation](https://agentclientprotocol.com/protocol/v1/elicitation.md) for more information.

**July 21, 2026**

## auth/status RFD moves to Draft stage

The RFD for allowing clients to query an agent's current authentication state has moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/get-auth-state.md) for more information and provide feedback as work on the implementation begins.

**July 21, 2026**

## Tool Call Name RFD moves to Draft stage

The RFD for adding an optional programmatic `name` to tool calls in ACP v1 and v2 has moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/tool-call-name.md) for more information and provide feedback while the draft implementation is exercised.

**July 9, 2026**

## Elicitation RFD moves to Preview stage

The RFD for structured user input through `elicitation/create` has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/elicitation.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**July 6, 2026**

## Boolean Config Option RFD moves to Completed

The RFD for boolean session configuration options has been stabilized and is now part of the protocol. Agents can expose simple on/off toggles with `type: "boolean"` config options, and Clients opt in on v1 by advertising the `session.configOptions.boolean` capability. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-config-options.md#boolean-config-options) for more information.

**July 2, 2026**

## Agent Telemetry Export RFD removed

The Agent Telemetry Export RFD has been removed. Its design relied on clients injecting OpenTelemetry environment variables when launching agent subprocesses, which doesn't translate to the new remote transports where agents aren't launched by the client. Since the approach required no changes to the protocol itself, clients and agents that want this setup can adopt standard [OpenTelemetry SDK environment variable configuration](https://opentelemetry.io/docs/specs/otel/configuration/sdk-environment-variables/) directly.

**July 2, 2026**

## Active RFD stage added

The RFD lifecycle now includes an Active stage between Draft and Preview. Active RFDs are the proposals that core maintainers or a working group are currently spending bandwidth on.

**June 30, 2026**

## Boolean Config Option RFD moves to Preview stage

The RFD for adding boolean session configuration options has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/boolean-config-option.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**June 29, 2026**

## Request Cancellation RFD moves to Completed

The RFD for `$/cancel_request` protocol-level request cancellation has been stabilized and is now part of the protocol. Either side can send this notification to request cancellation of an outstanding JSON-RPC request by ID. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/cancellation.md) for more information.

**June 25, 2026**

## Rust SDK based on SACP RFD moves to Completed

The RFD for basing the Rust SDK on SACP has been completed with the 1.0 SDK releases. The Rust SDK is available as [`agent-client-protocol` v1.0.0](https://crates.io/crates/agent-client-protocol/1.0.0), and the TypeScript SDK reached the same milestone with [`@agentclientprotocol/sdk` v1.0.0](https://www.npmjs.com/package/@agentclientprotocol/sdk/v/1.0.0), showing that the SDK design goals apply beyond Rust.

**June 24, 2026**

## model\_config Category RFD moves to Completed

The RFD for the `model_config` session configuration option category has been stabilized and is now a part of the protocol. Agents can use `model_config` for model-related parameters such as context size or speed/quality trade-offs. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-config-options.md#option-categories) for more information.

**June 24, 2026**

## Request Cancellation RFD moves to Preview stage

The RFD for adding `$/cancel_request` protocol-level request cancellation has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/request-cancellation.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**June 22, 2026**

## model\_config Category RFD moves to Preview stage

The RFD for the `model_config` session configuration option category has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/model-config-category.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**June 5, 2026**

## Message ID RFD moves to Completed

The RFD for adding agent-generated `messageId` fields to streamed message chunks has been stabilized and is now part of the protocol. In v1, message IDs are optional on message chunks for compatibility; in v2, they are required. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#message-ids) for more information.

**June 5, 2026**

## Session Context Size and Cost RFD moves to Completed

The RFD for `usage_update` session notifications has been stabilized and is now a part of the protocol. Agents can report session-level context window size and optional cumulative cost through `session/update`. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/prompt-turn.md#session-usage-updates) for more information.

**June 5, 2026**

## session/delete RFD moves to Completed

The RFD for allowing clients to delete a given session has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-delete.md) for more information.

**June 3, 2026**

## \_meta Propagation RFD moves to Completed

The RFD for documenting `_meta` propagation conventions for correlation metadata has been completed and is now part of the protocol documentation. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/extensibility.md#the-_meta-field) for more information.

**June 3, 2026**

## Message ID RFD moves to Preview stage for v1

The v1 portion of the RFD for adding message IDs to streamed message chunks has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/message-id.md) for more information on the current proposal and provide feedback before the v1 behavior is stabilized.

**June 3, 2026**

## Session Usage RFD moves to Preview stage

The RFD for adding `usage_update` session notifications for context window size and cumulative session cost has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-usage.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**June 2, 2026**

## v2 Plan Variants RFD moves to Draft stage

The RFD for making item-based `plan_update` the default v2 plan shape has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/v2/plan-variants.md) for more information on replacing the old v1 `plan` update in v2 while the broader Plan Operations RFD continues.

**June 2, 2026**

## End-Turn Token Usage RFD moves to Draft stage

The end-turn token usage portions of the original Session Usage RFD have been split into a separate Draft RFD for further design work. Please review the [RFD](https://agentclientprotocol.com/rfds/end-turn-token-usage.md) for more information on the current proposal and provide feedback.

**June 2, 2026**

## session/delete RFD moves to Preview stage

The RFD for allowing clients to delete a given session has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-delete.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**June 1, 2026**

## Unstable Session Model API removed

The never-stabilized `session/set_model` API and related session model response fields have been removed from the protocol artifacts. Agents should continue to expose model selection through [Session Config Options](https://agentclientprotocol.com/rfds/session-config-options.md).

**June 1, 2026**

## Additional Directories RFD moves to Completed

The RFD for allowing clients to specify additional workspace roots for session lifecycle requests has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-setup.md#additional-workspace-roots) for more information.

**May 27, 2026**

## v2 Enum Variant Extension RFD moves to Draft stage

The RFD for reserving `_`-prefixed enum variants for extensions while preserving non-underscore variants for future ACP versions has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/v2/enum-variant-extension.md) for more information on the current proposal and provide feedback as work on v2 continues.

**May 27, 2026**

## Plan Operations Support RFD moves to Draft stage

The RFD for adding `plan_update` and `plan_removed` session update types has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/plan-operations.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**May 22, 2026**

## Logout Method RFD moves to Completed

The RFD for the `logout` method has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/authentication.md#logging-out) for more information.

**May 21, 2026**

## Additional Directories RFD moves to Preview stage

The RFD for allowing clients to specify additional workspace roots for session lifecycle requests has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/additional-directories.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**May 17, 2026**

## Logout Method RFD moves to Preview stage

The RFD for adding a `logout` method to the protocol has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/logout-method.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**May 12, 2026**

## Rust SDK based on SACP RFD moves to Preview stage

The RFD for basing the Rust SDK on SACP has been moved to Preview stage now that the initial crate import has been upstreamed to [agentclientprotocol/rust-sdk](https://github.com/agentclientprotocol/rust-sdk). Please review the [RFD](https://agentclientprotocol.com/rfds/rust-sdk-v1.md) for more information on the current proposal and provide feedback before the SDK is stabilized as 1.0.

**May 7, 2026**

## model\_config Category RFD moves to Draft

The RFD for the `model_config` category been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/model-config-category.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**April 23, 2026**

## v2 Prompting RFD moves to Draft

The RFD for how the prompt lifecycle will work in v2 of the protocol been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/v2/prompt.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**April 23, 2026**

## session/close RFD moves to Completed

The RFD for the `session/close` method has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-setup.md#closing-active-sessions) for more information.

**April 22, 2026**

## session/resume RFD moves to Completed

The RFD for the `session/resume` method has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-setup.md#resuming-sessions) for more information.

**April 22, 2026**

## Streamable HTTP & WebSocket Transport RFD moves to Draft stage

The RFD for standardizing a remote ACP transport based on MCP Streamable HTTP with a WebSocket upgrade has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**April 14, 2026**

## session/resume RFD moves to Preview stage

The RFD for adding a `session/resume` method to the protocol has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-resume.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**April 14, 2026**

## session/close RFD moves to Preview stage

The RFD for allowing agents to close a given session has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-close.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**March 27, 2026**

## Custom LLM Endpoint RFD moves to Draft stage

The RFD for allowing clients to specify custom provider configuration has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/custom-llm-endpoint.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**March 24, 2026**

## Additional Directories RFD moves to Draft stage

The RFD for allowing clients to specify additional directories for the agent to access has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/additional-directories.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**March 9, 2026**

## ACP Registry RFD moves to Completed

The RFD for the initial version of the ACP Registry has been completed. Please review the [documentation](https://agentclientprotocol.com/get-started/registry.md) for more information.

## session\_info\_update Notification RFD moves to Completed

The RFD for the session\_info\_update notification has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-list.md#updating-session-metadata) for more information.

## session/list RFD moves to Completed

The RFD for the session/list method has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-list.md) for more information.

**February 24, 2026**

## Next Edit Suggestions RFD moves to Draft stage

The RFD for allowing agents to provide next edit suggestions moved to Draft stage. It was subsequently removed; see the [removal update](#next-edit-suggestions-rfd-removed).

## session/close RFD moves to Draft stage

The RFD for allowing agents to close a given session has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-close.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## Elicitation RFD moves to Draft stage

The RFD for allowing agents to elicit user input has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/elicitation.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## Boolean Config Option RFD moves to Draft stage

The RFD for adding boolean config options to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/boolean-config-option.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**February 20, 2026**

## Delete in Diff RFD moves to Draft stage

The RFD for indicating whether a diff resulted in a file deletion has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/diff-delete.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**February 18, 2026**

## Message ID RFD moves to Draft stage

The RFD for adding message ids to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/message-id.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**February 18, 2026**

## session/list RFD moves to Preview stage

The RFD for the session/list method has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-list.md) for more information on the current proposal and provide feedback before the feature is stabilized.

## session\_info\_update RFD moves to Preview stage

The RFD for the session\_info\_update notification has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-info-update.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**February 10, 2026**

## ACP Registry RFD moves to Preview stage

The RFD for the ACP Registry has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/acp-agent-registry.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**February 4, 2026**

## Session Config Options RFD moves to Completed

The RFD for adding more generic Session Config Options to the protocol has been stabilized and is now a part of the protocol. Please review the [documentation](https://agentclientprotocol.com/protocol/v1/session-config-options.md) for more information.

**February 4, 2026**

## session/delete moves to Draft stage

The RFD for allowing clients to delete a given session has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-delete.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## Logout method moves to Draft stage

The RFD for allowing clients to logout from an agent connection has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/logout-method.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**January 15, 2026**

## Rust SDK based on SACP RFD moves to Draft stage

The RFD for basing the Rust SDK on SACP has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/rust-sdk-v1.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**January 15, 2026**

## Session Config Options RFD moves to Preview stage

The RFD for adding more generic Session Config Options to the protocol has been moved to Preview stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-config-options.md) for more information on the current proposal and provide feedback before the feature is stabilized.

**January 14, 2026**

## Authentication Methods RFD moves to Draft stage

The RFD for creating additional types of authentication methods has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/auth-methods.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**January 1, 2026**

## Agent Registry RFD moves to Draft stage

The RFD for creating an Agent Registry has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/acp-agent-registry.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## Session Usage RFD moves to Draft stage

The RFD for adding a new usage\_update variant on the session/update notification and usage field on prompt responses in the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-usage.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**December 31, 2025**

## Proxy Chains RFD moves to Draft stage

The RFD for adding proxy chain functionality in the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/proxy-chains.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**December 11, 2025**

## Agent Telemetry Export RFD moves to Draft stage

The RFD for providing more guidance on how agents should export telemetry has been moved to Draft stage. (This RFD was later removed — see the July 2, 2026 update.)

**December 3, 2025**

## session\_info\_update notification RFD moves to Draft stage

The RFD for adding a new session\_info\_update variant on the session/update notification in the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-info-update.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## \_meta Propagation RFD moves to Draft stage

The RFD for providing more guidance on how the \_meta parameter should be used within the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/meta-propagation.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**November 26, 2025**

## session/resume RFD moves to Draft stage

The RFD for adding a session/resume method to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-resume.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**November 20, 2025**

## \$/cancelRequest RFD moves to Draft stage

The RFD for adding a \$/cancelRequest method to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/request-cancellation.md) for more information on the current proposal and provide feedback as work on the implementation begins.

## session/fork RFD moves to Draft stage

The RFD for adding a session/fork method to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-fork.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**November 3, 2025**

## Session Config Options RFD moves to Draft stage

The RFD for adding more generic Session Config Options to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-config-options.md) for more information on the current proposal and provide feedback as work on the implementation begins.

**October 31, 2025**

## session/list RFD moves to Draft stage

The RFD for adding a session/list method to the protocol has been moved to Draft stage. Please review the [RFD](https://agentclientprotocol.com/rfds/session-list.md) for more information on the current proposal and provide feedback as work on the implementation begins.


This documentation is built and hosted on [Mintlify](https://mintlify.com), a developer documentation platform.