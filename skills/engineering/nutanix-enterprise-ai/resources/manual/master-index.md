# Nutanix Enterprise AI v2.8 Documentation Index

This directory contains the complete reference documentation for Nutanix Enterprise AI (NAI) v2.8, converted into structured, AI-agent readable Markdown reference guides.

## Documentation Modules

1. **[overview-and-architecture.md](./overview-and-architecture.md)**
   - Nutanix Enterprise AI (NAI) architecture overview, core components, microservices, end-to-end workflow, deployment models (GPT-in-a-Box 2.0, Bare Metal, Standalone), and NAI dashboard overview.
2. **[requirements-and-sizing.md](./requirements-and-sizing.md)**
   - Prerequisites, supported Kubernetes versions, supported NVIDIA GPUs (L40S, H100, H100-NVL, A100, B300, H200, RTX PRO 6000), RWX/RWO storage classes, MetalLB networking, compute and memory sizing matrices, profile-based deployments (c1k_k200, c5k_k1k), component compute and storage requirements, Agent Gateway sizing, and platform limitations.
3. **[deployment-and-installation.md](./deployment-and-installation.md)**
   - Nutanix Docker Hub access tokens, Helm chart configuration parameters for nai-operators and nai-core, connected deployment on Nutanix Kubernetes Platform (NKP), air-gapped NKP deployment with private registries, deployment on Amazon EKS, Azure AKS, Google GKE, self-managed PostgreSQL configurations, dashboard IP access, and Docker registry credentials rotation.
4. **[tls-and-certificates.md](./tls-and-certificates.md)**
   - TLS encryption setup, self-signed certificates via Helm, cert-manager ClusterIssuer integration (AWS Route 53, Let's Encrypt), custom TLS certificates, certificate rotation procedures, verification, and TLS troubleshooting.
5. **[authentication-and-authorization.md](./authentication-and-authorization.md)**
   - Initial login, password management, language settings, Identity & Access Management (IAM), local accounts, Active Directory, OpenLDAP, SAML SSO integration, user management, built-in and custom roles (entities & operations), and fine-grained authorization policies and permissions.
6. **[licensing-and-settings.md](./licensing-and-settings.md)**
   - NAI licensing management, license key generation, GPU/CPU/Agent Gateway licenses, switching/upgrading licenses, third-party credentials (Hugging Face tokens, NVIDIA NGC keys), Nutanix Pulse telemetry, and remote Syslog server integration.
7. **[generative-ai-models.md](./generative-ai-models.md)**
   - Pre-validated generative AI models catalog, model access control, importing models from Hugging Face (catalog, URL/ID, manual air-gap), importing NVIDIA NIMs (catalog, URL/ID, air-gap), model size calculation, forward proxy configuration, import troubleshooting, model fine-tuning workflows, data sources, training metrics, and importing fine-tuned models.
8. **[local-endpoints.md](./local-endpoints.md)**
   - Local endpoint lifecycle, attributes and performance widgets, deploying endpoints with validated models, non-validated Hugging Face models, non-catalog NVIDIA NIMs, air-gapped NIM deployments, experimental endpoints and runtime parameters, hibernation and resumption, and OpenAI-compatible API invocation.
9. **[batch-inference.md](./batch-inference.md)**
   - Batch inference architecture, data sources (S3/NFS buckets), creating dedicated batch local endpoints, creating, monitoring, pausing, resuming, and deleting batch inference jobs.
10. **[unified-endpoints.md](./unified-endpoints.md)**
   - Unified endpoints architecture, multi-model routing across local endpoints, external model provider integration (Anthropic, AWS Bedrock, Azure OpenAI, Cohere, GCP Vertex AI, Google Gemini, Mistral, Remote NAI, OpenAI), provider credentials, rate limit management, and OpenAI-compatible client access.
11. **[model-and-endpoint-security.md](./model-and-endpoint-security.md)**
   - Inline model scanning architecture, scan settings and credentials, automated and manual scanning of models and endpoints, re-triggering scans for blocked models, security dashboard, and vulnerability remediation.
12. **[api-keys-and-audit.md](./api-keys-and-audit.md)**
   - API client key lifecycle (creation, activation, deactivation, expiration), audit event logging and event filtering, cluster-level and node-level infrastructure metrics, and Kubernetes cluster health monitoring.
13. **[mcp-servers.md](./mcp-servers.md)**
   - Model Context Protocol (MCP) support in NAI, remote MCP servers and credentials, local MCP server deployment (container registry credentials, lifecycle), MCP connectors and client keys, MCP logging, and security considerations.
14. **[observability-and-opentelemetry.md](./observability-and-opentelemetry.md)**
   - OpenTelemetry collector configuration, ClickHouse server and keeper architecture, exporting metrics and logs to external monitoring systems (Prometheus, Grafana, SIEM), observability daemonsets and collectors.
15. **[nai-labs-and-support.md](./nai-labs-and-support.md)**
   - NAI Labs sample applications (Chat, Talk to My Data, Agent application), Milvus vector database integration, application configuration resets, Nutanix Enterprise AI support bundle generation (NKP and non-NKP), online help, and glossary of terms.
