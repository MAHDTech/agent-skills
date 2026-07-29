+++
title = "master-index"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-api-v4"
+++

{% raw %}
# Nutanix v4 API Documentation Index

This directory contains the complete reference documentation for the Nutanix v4 API family, converted into structured, AI-agent readable Markdown reference guides.

## Documentation Modules

1. **[overview-and-versioning.md](@/skills/engineering/nutanix-api-v4/resources/manual/overview-and-versioning.md)**
   - Nutanix v4 API overview, developer experience features (OData, SDKs, open API standards), legacy API deprecation timeline (v0.8, v1, v2, v3 deprecation in Q4-CY2026), software compatibility matrix (AOS 7.0+, Prism Central pc.2024.3+), and language-specific SDKs (Python, Go, Java, JavaScript).
2. **[namespaces-reference.md](@/skills/engineering/nutanix-api-v4/resources/manual/namespaces-reference.md)**
   - Complete reference matrix for all 18 Nutanix v4 API namespaces: AIOps, Cluster Management, Data Policies, Data Protection, Files, Flow Management, Identity and Access Management (IAM), Licensing, Life Cycle Management (LCM), Monitoring, Multi Domain Management, NCM Operation Base Platform, Networking, Object Storage Management, Prism, Security, Virtual Machine Management (VMM), and Volumes.
3. **[release-features.md](@/skills/engineering/nutanix-api-v4/resources/manual/release-features.md)**
   - Comprehensive release-by-release updates and new v4 API features across pc.7.5.1, pc.7.5, pc.7.3, pc.2024.3.1, and pc.2024.3, including VM startup policies, custom attributes, ETag removal, capacity planning, storage containers sharing, CVM management endpoints, Recovery Plans v4 APIs, Flow security policy updates, Object Store APIs, Cloud KMS, and BMC out-of-band management APIs.
4. **[troubleshooting-and-known-issues.md](@/skills/engineering/nutanix-api-v4/resources/manual/troubleshooting-and-known-issues.md)**
   - Resolved issues, known issues, error workarounds, OData parameter requirements (`$filter`, `$limit`, `$orderby`, `$page`, `$select`, `$statType`, `$samplingInterval`), response schema changes, RBAC VG restore owner permissions workaround, service account API key VM creation workaround, category value deletion orphaned key cleanup, and discontinued endpoints migration (Layer2 stretches replacement endpoints).

{% endraw %}
