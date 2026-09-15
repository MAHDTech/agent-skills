---
name: nutanix-enterprise-ai
description: Expert guidance, architecture reference, sizing guidelines, Helm deployment, fine-grained RBAC authorization, Hugging Face and NVIDIA NIM model management, local and unified inference endpoints, batch inference, security scanning, and MCP server configuration for Nutanix Enterprise AI (NAI v2.8). Use when configuring or troubleshooting Nutanix Enterprise AI, GPT-in-a-Box 2.0, LLM model serving, AI Gateway, or Model Context Protocol on Kubernetes.
---

# Nutanix Enterprise AI (NAI)

Nutanix Enterprise AI (NAI) is an enterprise-grade inference endpoint management and AI model orchestration platform built for hybrid multi-cloud Kubernetes environments. NAI enables organizations to securely deploy, scale, monitor, and manage text-based large language models (LLMs) and foundation models locally on-premises or across public cloud Kubernetes clusters.

Nutanix Enterprise AI combines:

- **Model Management & Catalogs**: Support for pre-validated open-weight models from Hugging Face and NVIDIA NGC (NIMs), custom model imports via URL or ID, and manual air-gapped imports.
- **Inference Endpoint Orchestration**: One-click deployment of optimized LLM endpoints powered by vLLM and NVIDIA NIM runtime engines, with support for GPU and CPU acceleration.
- **Unified Endpoints & Agent Gateway**: Intelligent routing across local endpoints and external commercial LLM providers (OpenAI, Anthropic, AWS Bedrock, Azure OpenAI, Google Gemini, Mistral, Cohere, GCP Vertex AI) with rate limiting.
- **Model Context Protocol (MCP) Integration**: Native support for connecting external MCP servers and deploying local containerized MCP tools to augment agentic AI applications.
- **Enterprise Security & Compliance**: Built-in inline model security scanning, fine-grained role-based access control (RBAC), API client keys, and comprehensive audit event tracking.
- **Observability & NAI Labs**: Real-time telemetry via OpenTelemetry and ClickHouse, coupled with NAI Labs applications (Chat, Talk to My Data, Agent) for rapid endpoint verification.

---

## Core Operations & Workflows

### 1. Deploying NAI via Helm on Kubernetes

Nutanix Enterprise AI is deployed into the `nai-system` namespace using the `nai-operators` and `nai-core` Helm charts:

```bash
# Add Nutanix Helm repository
helm repo add ntnx-charts https://nutanix.github.io/helm-releases
helm repo update ntnx-charts

# Deploy NAI Operators
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
  -n nai-system --create-namespace --wait --timeout 15m \
  --set "global.storage.storageClassName=nutanix-volume" \
  --set "global.imagePullSecrets[0].name=nai-registry-secret"

# Deploy NAI Core with NFS storage class for model cache
helm upgrade --install nai-core ntnx-charts/nai-core --version 2.8.0 \
  -n nai-system --create-namespace --wait --timeout 15m \
  --set "global.storage.storageClassNameRWX=nai-nfs-storage" \
  --set "global.storage.storageClassName=nutanix-volume" \
  --set "global.imagePullSecrets[0].name=nai-registry-secret"
```

### 2. Invoking Local Inference Endpoints via OpenAI API

NAI endpoints expose an OpenAI-compatible REST API accessible via standard HTTP clients and SDKs:

```bash
# Set endpoint details and API key
export NAI_ENDPOINT_URL="https://nai.example.com/v1"
export NAI_API_KEY="your-nai-api-client-key"

# Query the local LLM endpoint
curl -k -X POST "$NAI_ENDPOINT_URL/chat/completions" \
  -H "Authorization: Bearer $NAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "meta-llama/Llama-3.1-8B-Instruct",
    "messages": [
      {"role": "system", "content": "You are a helpful AI assistant."},
      {"role": "user", "content": "Explain Nutanix Enterprise AI architecture."}
    ],
    "temperature": 0.7,
    "max_tokens": 512
  }'
```

### 3. Unified Endpoints and Commercial Providers

Route inference traffic seamlessly between on-prem models and cloud APIs:

- **Local Endpoints**: Route between multiple local deployments for load balancing and high availability.
- **Third-Party Providers**: Aggregate AWS Bedrock, Azure OpenAI, Google Gemini, Anthropic, or OpenAI behind a unified gateway endpoint with rate limits and unified API keys.

### 4. Model Context Protocol (MCP) Integration

Connect agentic workflows to internal databases, tools, and search APIs:

```bash
# Verify deployed MCP pods in the cluster
kubectl get pods -n nai-system -l app.kubernetes.io/component=mcp-server
```

---

## Progressive Disclosure Reference Index

When performing specific administration, deployment, sizing, or operational tasks for Nutanix Enterprise AI, consult the detailed manual reference guides in `resources/manual/`:

| Topic / Requirement                    | Reference File Pointer                                                                                         | Key Content Covered                                                                                                                                                               |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Overview & Architecture**            | [`resources/manual/overview-and-architecture.md`](resources/manual/overview-and-architecture.md)               | Architectural components, microservices, workflow stages, deployment types (GPT-in-a-Box, Bare Metal, Standalone), and dashboard navigation.                                      |
| **Requirements & Sizing**              | [`resources/manual/requirements-and-sizing.md`](resources/manual/requirements-and-sizing.md)                   | Supported Kubernetes versions, validated NVIDIA GPUs, RWX/RWO storage, MetalLB networking, sizing profiles (c1k_k200, c5k_k1k), component resource requirements, and limitations. |
| **Deployment & Installation**          | [`resources/manual/deployment-and-installation.md`](resources/manual/deployment-and-installation.md)           | Docker Hub tokens, Helm chart parameters (nai-operators, nai-core), connected and air-gapped NKP deployment, EKS, AKS, GKE, and self-managed PostgreSQL.                          |
| **TLS & Certificate Management**       | [`resources/manual/tls-and-certificates.md`](resources/manual/tls-and-certificates.md)                         | Ingress TLS prerequisites, Helm self-signed certs, cert-manager ClusterIssuer (Route 53, Let's Encrypt), custom certificates, rotation, and troubleshooting.                      |
| **Authentication & IAM**               | [`resources/manual/authentication-and-authorization.md`](resources/manual/authentication-and-authorization.md) | First-time login, Active Directory, OpenLDAP, SAML SSO IdP configuration, user lifecycle, custom roles (entities & operations), and fine-grained authorization policies.          |
| **Licensing & System Settings**        | [`resources/manual/licensing-and-settings.md`](resources/manual/licensing-and-settings.md)                     | License key generation, GPU/CPU/Agent Gateway licenses, license switching, Hugging Face and NGC tokens, Pulse telemetry, and Syslog integration.                                  |
| **Generative AI Models & Fine-Tuning** | [`resources/manual/generative-ai-models.md`](resources/manual/generative-ai-models.md)                         | Validated model catalog, model access control, importing from Hugging Face and NVIDIA NGC (online and air-gapped), model sizing, and fine-tuning workflows.                       |
| **Local Endpoints & Inference**        | [`resources/manual/local-endpoints.md`](resources/manual/local-endpoints.md)                                   | Creating endpoints for validated models, non-validated models, and NIMs, air-gapped deployments, experimental endpoints, hibernation, and OpenAI compatibility.                   |
| **Batch Inference**                    | [`resources/manual/batch-inference.md`](resources/manual/batch-inference.md)                                   | Batch inference architecture, batch data sources (S3/NFS), creating batch endpoints, running, monitoring, and managing batch inference jobs.                                      |
| **Unified Endpoints & Providers**      | [`resources/manual/unified-endpoints.md`](resources/manual/unified-endpoints.md)                               | Unified endpoint routing, local endpoint groups, external provider integration (OpenAI, Anthropic, Bedrock, Azure, Gemini), credentials, and rate limits.                         |
| **Model & Endpoint Security**          | [`resources/manual/model-and-endpoint-security.md`](resources/manual/model-and-endpoint-security.md)           | Inline model scanning, scan settings, manual scans for models and endpoints, re-triggering scans, security dashboard, and vulnerability remediation.                              |
| **API Keys & Audit Events**            | [`resources/manual/api-keys-and-audit.md`](resources/manual/api-keys-and-audit.md)                             | API client key creation, activation, and revocation, audit event logging and filtering, and cluster-level/node-level health metrics.                                              |
| **MCP Servers & Connectors**           | [`resources/manual/mcp-servers.md`](resources/manual/mcp-servers.md)                                           | Model Context Protocol support, remote MCP servers, local containerized MCP deployments, MCP connectors, client keys, and security best practices.                                |
| **Observability & OpenTelemetry**      | [`resources/manual/observability-and-opentelemetry.md`](resources/manual/observability-and-opentelemetry.md)   | OpenTelemetry collector configuration, ClickHouse server/keeper architecture, and exporting metrics to Prometheus, Grafana, or external security monitoring systems.              |
| **NAI Labs & Support Tools**           | [`resources/manual/nai-labs-and-support.md`](resources/manual/nai-labs-and-support.md)                         | NAI Labs applications (Chat, Talk to My Data, Agent), vector database integration, support bundle generation (NKP and non-NKP), and glossary.                                     |
| **Master Documentation Index**         | [`resources/manual/master-index.md`](resources/manual/master-index.md)                                         | Master overview map linking all 15 documentation modules.                                                                                                                         |
