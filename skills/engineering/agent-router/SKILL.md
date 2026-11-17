---
name: agent-router
description: Configure, route, or troubleshoot AgentRouter AI gateways, unified LLM endpoints, and MCP or A2A servers, including the gateway powering Nutanix Enterprise AI (NAI).
metadata:
  group: nutanix
resources:
  - https://theagentrouter.ai/llms-full.txt
---

<!-- cspell:words theagentrouter agentrouter a2a guardrails aigw aigatewayroute aiservicebackend backendsecuritypolicy mcproute aigateway envoyproxy vertexai inferencepool ratelimiting accesslogs aigwinstall aigwrun openinference -->

# AgentRouter

AgentRouter (formerly Envoy AI Gateway) is an open-source AI gateway and MCP/A2A routing platform built on Envoy and Envoy Gateway (`aigateway.envoyproxy.io`).
It provides application teams a single OpenAI-compatible and Anthropic-compatible API for Large Language Models (LLMs), Model Context Protocol (MCP) tools, and Agent-to-Agent (A2A) traffic, while giving platform teams centralized control over credentials, routing, token quotas, failover, and OpenTelemetry observability.
AgentRouter also powers the underlying Agent Gateway and unified endpoint router in Nutanix Enterprise AI (NAI); see [nutanix-enterprise-ai](../nutanix-enterprise-ai/SKILL.md) for NAI platform deployment, Kubernetes Helm charts, and cluster administration.

---

## Documentation lookup

Before authoring gateway configurations, routing policies, MCP server definitions, or client integrations, search the vendored documentation bundle `resources/auto/llms-full.txt`. It concatenates every upstream docs page as Markdown, each section starting with a `Source: https://theagentrouter.ai/docs/...` line, so search for the page path to jump to it:

- Core CRDs and API reference: `docs/concepts/resources` and `docs/api/` (includes the v1alpha1 to v1beta1 migration notes)
- Provider connections and LLM routing: `docs/getting-started/connect-providers/` and `docs/capabilities/llm-integrations/`
- MCP gateway routing: `docs/capabilities/mcp/`
- A2A agent traffic: `docs/capabilities/a2a/`
- Fallback and token rate limiting: `docs/capabilities/traffic/provider-fallback`, `docs/capabilities/traffic/usage-based-ratelimiting`, and `docs/capabilities/traffic/quota-policy`
- Security and observability: `docs/capabilities/security/upstream-auth`, `docs/capabilities/observability/metrics`, and `docs/capabilities/observability/tracing`
- Standalone CLI (`aigw`): `docs/cli/` and `docs/cli/aigwrun`

When the local bundle looks stale, fetch the current index at `https://theagentrouter.ai/llms.txt`.

---

## Core workflows

### 1. Unified LLM gateway and OpenAI-compatible routing

AgentRouter manages LLM traffic on Kubernetes through three primary custom resources (`aigateway.envoyproxy.io/v1beta1`; `v1alpha1` is deprecated):

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

### 2. MCP server registration and tool routing

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
- Search the `docs/capabilities/mcp/` section of `resources/auto/llms-full.txt` for OAuth authorization, JWT scope enforcement, and per-user header forwarding.

### 3. Agent-to-Agent (A2A) routing and guardrails

AgentRouter routes Agent2Agent (A2A) traffic using Envoy Gateway `HTTPRoute` resources paired with `EnvoyPatchPolicy` and the `envoy.filters.http.a2a` HTTP filter:

- Expose public agent discovery on `GET /.well-known/agent-card.json` and JSON-RPC task execution on `POST /a2a` with extended streaming timeouts (for example, `30m` for Server-Sent Events).
- Isolate A2A listeners onto a dedicated `Gateway` resource or per-route filter scope so JSON-RPC envelope validation does not buffer unrelated LLM routes.
- Attach `SecurityPolicy`, token rate limits (`BackendTrafficPolicy`), and `QuotaPolicy` rules to enforce authentication and usage budgets across agent calls.
- A2A support is a preview built on Envoy's alpha A2A filter; search the `docs/capabilities/a2a/` section of `resources/auto/llms-full.txt` for complete listener and route manifests.

### 4. Nutanix Enterprise AI (NAI) gateway integration

In a Nutanix Enterprise AI environment, this skill covers the AgentRouter routing layer: `AIGatewayRoute`, `AIServiceBackend`, `BackendSecurityPolicy`, `MCPRoute`, provider fallback, token rate limiting, MCP tool multiplexing, and A2A routing. For deploying NAI Helm charts (`nai-operators`, `nai-core`), sizing GPU/CPU nodes, NAI RBAC and platform API keys, or operating vLLM and NVIDIA NIM inference endpoints, use [nutanix-enterprise-ai](../nutanix-enterprise-ai/SKILL.md).

---

## Completion criteria

- Gateway routes, provider credentials, and MCP/A2A schemas are checked against the matching section of `resources/auto/llms-full.txt`.
- Kubernetes manifests (`AIGatewayRoute`, `AIServiceBackend`, `BackendSecurityPolicy`, `MCPRoute`, `HTTPRoute`) and client requests specify explicit API versions, authentication headers, timeouts, and model or tool selectors.
- Every applied resource reports `Accepted` in its status (`kubectl get <kind> <name> -o jsonpath='{.status.conditions}'`), and a test request through the gateway succeeds: a `curl` to `$GATEWAY_URL/v1/chat/completions` for LLM routes, or an MCP client connection to the `MCPRoute` path for MCP routes.
