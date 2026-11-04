+++
title = "agent-router"
description = "Configure, route, or troubleshoot AgentRouter AI gateways, unified LLM endpoints, and MCP or A2A servers, including the gateway powering Nutanix Enterprise AI (NAI)."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


<!-- cspell:words theagentrouter agentrouter a2a guardrails aigw aigatewayroute aiservicebackend backendsecuritypolicy mcproute aigateway envoyproxy vertexai inferencepool ratelimiting accesslogs aigwinstall aigwrun openinference -->

# AgentRouter

AgentRouter (formerly Envoy AI Gateway) is an open-source AI gateway and MCP/A2A routing platform built on Envoy and Envoy Gateway (`aigateway.envoyproxy.io`).
It provides application teams a single OpenAI-compatible and Anthropic-compatible API for Large Language Models (LLMs), Model Context Protocol (MCP) tools, and Agent-to-Agent (A2A) traffic, while giving platform teams centralized control over credentials, routing, token quotas, failover, and OpenTelemetry observability.
AgentRouter also powers the underlying Agent Gateway and unified endpoint router in Nutanix Enterprise AI (NAI); see [nutanix-enterprise-ai](@/skills/engineering/nutanix-enterprise-ai/_index.md) for NAI platform deployment, Kubernetes Helm charts, and cluster administration.

---

## Documentation Lookup Protocol

Before authoring gateway configurations, routing policies, MCP server definitions, or client integrations, inspect the downloaded upstream documentation in `resources/auto/`:

1. Read [`resources/auto/llms.txt`](resources/auto/llms.txt) first to locate the relevant getting-started, concept, capability, CLI, or API reference guide.
2. Open the matching reference files in `resources/auto/` to verify exact CRD group versions, field names, CLI flags, and security policies:
   - Core CRDs and architecture: [`resources/auto/docs-concepts-resources.md`](@/skills/engineering/agent-router/resources/auto/docs-concepts-resources.md), [`resources/auto/docs-concepts-architecture-system-architecture.md`](@/skills/engineering/agent-router/resources/auto/docs-concepts-architecture-system-architecture.md), and [`resources/auto/docs-api.md`](@/skills/engineering/agent-router/resources/auto/docs-api.md)
   - Provider connections and LLM routing: [`resources/auto/docs-capabilities-llm-integrations-connect-providers.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-llm-integrations-connect-providers.md), [`resources/auto/docs-capabilities-llm-integrations-supported-providers.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-llm-integrations-supported-providers.md), and [`resources/auto/docs-capabilities-inference-inferencepool-support.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-inference-inferencepool-support.md)
   - MCP gateway routing: [`resources/auto/docs-capabilities-mcp.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-mcp.md)
   - A2A agent traffic: [`resources/auto/docs-capabilities-a2a.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-a2a.md)
   - Traffic policies, fallback, and token rate limiting: [`resources/auto/docs-capabilities-traffic-provider-fallback.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-traffic-provider-fallback.md), [`resources/auto/docs-capabilities-traffic-usage-based-ratelimiting.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-traffic-usage-based-ratelimiting.md), and [`resources/auto/docs-capabilities-traffic-quota-policy.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-traffic-quota-policy.md)
   - Security and observability: [`resources/auto/docs-capabilities-security-upstream-auth.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-security-upstream-auth.md), [`resources/auto/docs-capabilities-observability-metrics.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-observability-metrics.md), and [`resources/auto/docs-capabilities-observability-tracing.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-observability-tracing.md)
   - Standalone CLI (`aigw`): [`resources/auto/docs-cli.md`](@/skills/engineering/agent-router/resources/auto/docs-cli.md) and [`resources/auto/docs-cli-aigwrun.md`](@/skills/engineering/agent-router/resources/auto/docs-cli-aigwrun.md)

---

## Core Workflows

### 1. Unified LLM Gateway and OpenAI-Compatible Routing

AgentRouter manages LLM traffic on Kubernetes through three primary custom resources (`aigateway.envoyproxy.io/v1alpha1`):

- `AIGatewayRoute`: Defines the unified client-facing API schema, model-matching rules, and backend routing weights or fallback chains.
- `AIServiceBackend`: Represents an individual upstream AI service backend (such as OpenAI, Anthropic, AWS Bedrock, Azure OpenAI, GCP Vertex AI, or self-hosted vLLM and NVIDIA NIM endpoints) and declares its expected output schema.
- `BackendSecurityPolicy`: Attaches upstream authentication credentials (API keys, AWS credentials, or workload identity tokens) to an `AIServiceBackend`.

For local development or standalone proxy deployments without Kubernetes, run AgentRouter directly with the `aigw` CLI (`aigw run`).

Clients query the gateway through standard OpenAI-compatible endpoints:

```bash
export AGENT_ROUTER_URL="http://localhost:8080/v1"
export AGENT_ROUTER_API_KEY="your-gateway-api-key"

curl -sS -X POST "$AGENT_ROUTER_URL/chat/completions" \
  -H "Authorization: Bearer $AGENT_ROUTER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "openai/gpt-4o",
    "messages": [
      {"role": "system", "content": "You are a routing assistant."},
      {"role": "user", "content": "Verify unified gateway connectivity."}
    ]
  }'
```

### 2. MCP Server Registration and Tool Routing

AgentRouter aggregates multiple Model Context Protocol (MCP) servers behind a single unified Streamable HTTP endpoint using the `MCPRoute` custom resource (`aigateway.envoyproxy.io/v1beta1`):

```yaml
apiVersion: aigateway.envoyproxy.io/v1beta1
kind: MCPRoute
metadata:
  name: mcp-route
  namespace: default
spec:
  parentRefs:
    - name: aigw-run
      kind: Gateway
      group: gateway.networking.k8s.io
  path: "/mcp"
  backendRefs:
    - name: github
      kind: Backend
      group: gateway.envoyproxy.io
      path: "/mcp/x/issues/readonly"
      securityPolicy:
        apiKey:
          secretRef:
            name: github-token
```

- Use `toolSelector` in `MCPRoute` backend references to filter exposed tools by exact name or regular expression.
- Upstream tool names are prefixed with the backend name (for example, `github__issue_read`) so tool invocations route deterministically across multiplexed MCP backends.
- Consult [`resources/auto/docs-capabilities-mcp.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-mcp.md) for OAuth authorization, JWT scope enforcement, and per-user header forwarding.

### 3. Agent-to-Agent (A2A) Routing and Guardrails

AgentRouter routes Agent2Agent (A2A) traffic using Envoy Gateway `HTTPRoute` resources paired with `EnvoyPatchPolicy` and the `envoy.filters.http.a2a` HTTP filter:

- Expose public agent discovery on `GET /.well-known/agent-card.json` and JSON-RPC task execution on `POST /a2a` with extended streaming timeouts (for example, `30m` for Server-Sent Events).
- Isolate A2A listeners onto a dedicated `Gateway` resource or per-route filter scope so JSON-RPC envelope validation does not buffer unrelated LLM routes.
- Attach `SecurityPolicy`, token rate limits (`BackendTrafficPolicy`), and `QuotaPolicy` rules to enforce authentication and usage budgets across agent calls.
- Consult [`resources/auto/docs-capabilities-a2a.md`](@/skills/engineering/agent-router/resources/auto/docs-capabilities-a2a.md) for complete listener and route manifests.

### 4. Nutanix Enterprise AI (NAI) Gateway Integration

When working in a Nutanix Enterprise AI environment, choose the skill that matches the operational layer:

- Use `agent-router` (`/agent-router`) when configuring or troubleshooting AgentRouter gateway routing rules (`AIGatewayRoute`, `AIServiceBackend`, `BackendSecurityPolicy`, `MCPRoute`), unified LLM provider fallback, token rate limiting, MCP tool multiplexing, or A2A agent routing.
- Use [nutanix-enterprise-ai](@/skills/engineering/nutanix-enterprise-ai/_index.md) (`/nutanix-enterprise-ai`) when deploying NAI Helm charts (`nai-operators`, `nai-core`), sizing Kubernetes GPU/CPU nodes, managing NAI RBAC and platform API keys, or operating vLLM and NVIDIA NIM inference endpoints on Nutanix Kubernetes Platform (NKP).

---

## Completion Criteria

- Gateway routes, provider credentials, and MCP/A2A schemas are verified against [`resources/auto/llms.txt`](resources/auto/llms.txt) and the corresponding `resources/auto/*.md` reference guides.
- Kubernetes manifests (`AIGatewayRoute`, `AIServiceBackend`, `BackendSecurityPolicy`, `MCPRoute`, `HTTPRoute`) and client requests specify explicit API versions, authentication headers, timeouts, and model or tool selectors.
- All configuration changes and documentation updates pass repository skill and catalog validation (`ask skills lint`).

