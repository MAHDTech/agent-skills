+++
title = "nutanix-api-v4"
description = "Expert guidance, API endpoints, SDK usage, OData filtering and sorting, namespace specifications, release updates, and troubleshooting for Nutanix v4 APIs (AOS 7.0+, Prism Central pc.2024.3+). Use when developing, integrating, or troubleshooting with Nutanix v4 REST APIs, Python/Go/Java/JavaScript v4 SDKs, OData parameters, or migrating off deprecated legacy Nutanix APIs (v1, v2, v3)."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Nutanix v4 API

Nutanix v4 APIs deliver a modern, standardized, and scalable RESTful interface for automating and operating the Nutanix Cloud Platform. Built on Open API specifications, v4 APIs introduce OData query semantics, language-specific SDKs (Python, Go, Java, JavaScript), granular versioning, and modular namespace architecture spanning AOS 7.0+ and Prism Central pc.2024.3+.

> [!IMPORTANT]
> **Legacy API Deprecation Notice**: Nutanix legacy API versions (v0.8, v1, v2, v3) are scheduled for deprecation and support termination starting with the AOS/PC release in Q4-CY2026. All new integrations and custom tooling must target v4 APIs.

---

## Core Nutanix v4 API Features

### 1. OData Query Semantics

Nutanix v4 APIs standardize list and query operations using OData conventions:

```http
GET /api/vmm/v4.0/ahv/config/vms?$filter=name eq 'my-vm'&$select=extId,name,powerState&$orderby=name asc&$limit=10&$page=0
```

Supported OData parameters include:

- `$filter`: Filter resources using criteria (e.g., `name eq 'dev-vm'`, `clusterReference/extId eq '...'`).
- `$select`: Project specific resource fields to reduce payload size.
- `$orderby`: Sort results by entity fields (e.g., `name asc`, `operationType desc`).
- `$limit` & `$page`: Handle pagination deterministically.
- `$expand`: Retrieve inline detailed associations (e.g., `$expand=detailedAssociations`).

### 2. Multi-Language SDKs

Official v4 SDKs are available for Python, Go, Java, and JavaScript/TypeScript. Solutions built using v4 SDKs starting with PC/AOS 7.5+ feature backward-compatible server support, allowing SDK upgrades without requiring immediate server upgrades.

### 3. Service Account Authentication

Authenticate v4 API requests using API keys attached to Service Accounts:

```bash
# Include the API key in request headers
curl -k -H "Authorization: Bearer <api_key>" \
     -H "x-ntnx-api-key: <api_key>" \
     -H "Content-Type: application/json" \
     https://<prism-central-ip>:9440/api/vmm/v4.0/ahv/config/vms
```

---

## Progressive Disclosure Reference Index

When executing specific API integrations, SDK upgrades, or troubleshooting v4 API requests, refer to the dedicated manual reference guides in `resources/manual/`:

| Topic / Requirement                | Reference File Pointer                                                                                         | Key Content Covered                                                                                                                                                                                                              |
| :--------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Overview, SDKs & Deprecation**   | [`resources/manual/overview-and-versioning.md`](@/skills/engineering/nutanix-api-v4/resources/manual/overview-and-versioning.md)                   | Nutanix v4 API architecture, developer features, legacy API deprecation timeline (v0.8-v3 Q4-CY2026), SDK compatibility, and versioning scheme.                                                                                  |
| **Namespaces Reference**           | [`resources/manual/namespaces-reference.md`](@/skills/engineering/nutanix-api-v4/resources/manual/namespaces-reference.md)                         | Complete matrix of all 18 v4 namespaces (AIOps, VMM, Flow, IAM, Volumes, Monitoring, Security, Objects, Multi Domain, LCM, Files, etc.), GA status, and version requirements.                                                    |
| **Release Features & Updates**     | [`resources/manual/release-features.md`](@/skills/engineering/nutanix-api-v4/resources/manual/release-features.md)                                 | Release-by-release API updates (pc.7.5.1, pc.7.5, pc.7.3, pc.2024.3.1, pc.2024.3), VM startup policies, custom attributes, ETag removal, Recovery Plans v4 APIs, Cloud KMS, and BMC APIs.                                        |
| **Troubleshooting & Known Issues** | [`resources/manual/troubleshooting-and-known-issues.md`](@/skills/engineering/nutanix-api-v4/resources/manual/troubleshooting-and-known-issues.md) | Resolved issues, mandatory OData parameter workarounds (`$filter`, `$limit`, `$select`, etc.), RBAC owner permission fixes, service account VM creation workaround, category value deletion cleanup, and discontinued endpoints. |
| **Master Documentation Index**     | [`resources/manual/master-index.md`](@/skills/engineering/nutanix-api-v4/resources/manual/master-index.md)                                         | Master overview map of all v4 API reference documentation modules.                                                                                                                                                               |

