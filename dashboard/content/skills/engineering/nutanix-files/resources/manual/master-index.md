+++
title = "master-index"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

{% raw %}
# Nutanix Files Documentation Master Index

This master index provides a complete structural map to the Nutanix Files (v5.0) reference guides located in `resources/manual/`.

## Manual Reference Guides

| Module / Topic | Reference Guide | Key Topics Covered |
| :--- | :--- | :--- |
| **01. Architecture & Overview** | [`overview-and-architecture.md`](@/skills/engineering/nutanix-files/resources/manual/overview-and-architecture.md) | Nutanix Files architecture, FSVMs, CVMs, Volume Groups, Prism Console, High Availability. |
| **02. Prerequisites & Networking** | [`prerequisites-and-network.md`](@/skills/engineering/nutanix-files/resources/manual/prerequisites-and-network.md) | AHV/ESXi sizing, network segmentation, port requirements, firewall rules, IPAM. |
| **03. Deployment & Scaling** | [`deployment-and-scaling.md`](@/skills/engineering/nutanix-files/resources/manual/deployment-and-scaling.md) | File server creation, non-disruptive upgrades (NDU), scale-up, scale-out, storage expansion. |
| **04. Shares & Exports** | [`file-server-and-share-management.md`](@/skills/engineering/nutanix-files/resources/manual/file-server-and-share-management.md) | Standard vs Distributed shares, SMB 2.1/3.0, NFS v3/v4, ABE, symlinks, file blocking, quotas. |
| **05. Directory Services & Users** | [`directory-services-and-user-management.md`](@/skills/engineering/nutanix-files/resources/manual/directory-services-and-user-management.md) | Active Directory, LDAP, Kerberos, disjoint domains, user mapping, RBAC, API roles. |
| **06. Security & Antivirus** | [`security-antivirus-and-compliance.md`](@/skills/engineering/nutanix-files/resources/manual/security-antivirus-and-compliance.md) | ICAP antivirus scanning, FIPS 140-2 encryption, SMB encryption, audit logging, file blocking. |
| **07. Protection & Recovery** | [`data-protection-and-disaster-recovery.md`](@/skills/engineering/nutanix-files/resources/manual/data-protection-and-disaster-recovery.md) | DR replication, Metro Availability, Self-Service Restore (SSR), share migration, backups. |
| **08. Smart Tiering & Cloud** | [`smart-tiering-and-cloud-storage.md`](@/skills/engineering/nutanix-files/resources/manual/smart-tiering-and-cloud-storage.md) | Smart Tiering to Nutanix Objects/S3/Azure/Wasabi, tiering policies, capacity thresholds, recall. |
| **09. Performance Optimization** | [`performance-and-workload-optimization.md`](@/skills/engineering/nutanix-files/resources/manual/performance-and-workload-optimization.md) | Load balancing, workload profiles (VDI, General, Enterprise), file compression, tuning. |
| **10. Analytics & Monitoring** | [`file-analytics-and-monitoring.md`](@/skills/engineering/nutanix-files/resources/manual/file-analytics-and-monitoring.md) | File Analytics dashboard, anomaly detection, ransomware alerts, Prometheus exporter, Syslog. |
| **11. REST APIs** | [`files-rest-apis.md`](@/skills/engineering/nutanix-files/resources/manual/files-rest-apis.md) | Files REST APIs v4, OData filtering, ETag concurrency, API Explorer, Python/Curl automation. |
| **12. Operations & Debugging** | [`troubleshooting-and-operations.md`](@/skills/engineering/nutanix-files/resources/manual/troubleshooting-and-operations.md) | FSVM CLI tools (`ncli`, `afsi`, `minerva`, `allssh`), log paths, logbay support bundles, fixes. |

{% endraw %}
