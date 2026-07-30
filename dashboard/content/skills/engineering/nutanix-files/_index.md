+++
title = "nutanix-files"
description = "Expert guidance, CLI commands, architecture reference, file server & share management, SMB/NFS protocols, directory services, smart tiering, disaster recovery, analytics, performance tuning, and troubleshooting for Nutanix Files (v5.0). Use when working with Nutanix Files, SMB/NFS file shares, file analytics, Smart Tiering, Metro Availability, Files REST APIs, or file server performance optimization."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Nutanix Files

Nutanix Files (v5.0) is an enterprise-grade, software-defined, scale-out file storage solution built natively on the Nutanix Cloud Infrastructure (NCI) and Acropolis Distributed Storage Fabric (DSF). It provides high-performance, resilient, and multi-tenant file services supporting SMB (Server Message Block) and NFS (Network File System) protocols for enterprise workloads such as user profile disks (VDI), enterprise application data, home directories, healthcare PACS imaging, and big data analytics.

Nutanix Files combines:

- **Multiprotocol Access**: Simultaneous support for SMB 2.1, SMB 3.0, NFS v3, and NFS v4.1 on a single file server instance.
- **Scale-Out Architecture**: File Server Virtual Machines (FSVMs) running on Nutanix HCI nodes with independent compute and storage scaling.
- **Data Protection & Disaster Recovery**: Asynchronous replication, NearSync, Metro Availability (RPO = 0), and Self-Service Restore (SSR) snapshots.
- **Smart Tiering**: Automated offloading of cold data to S3-compatible endpoints (Nutanix Objects, AWS S3, Azure Blob, Wasabi).
- **Security & Compliance**: ICAP antivirus scanning, FIPS 140-2 encryption at rest, SMB wire encryption, Access-Based Enumeration (ABE), and file blocking.
- **File Analytics & Auditing**: Deep data intelligence engine providing audit logging, user activity tracking, anomaly detection, and ransomware alert rules.

---

## Core Operations & Workflows

### 1. File Server & Share Management

Nutanix Files instances are created and managed via Prism Central or REST APIs. Standard shares run on a single FSVM node, while Distributed shares scale out across all FSVM nodes:

```bash
# Connect to CVM to inspect file server status via ncli
ncli file-server list

# List shares on a specific file server instance
ncli file-server list-shares name="fs-corp-01"

# Check FSVM cluster health and status
afsi file-server show-health
```

### 2. SMB & NFS Access Configuration

Client connections access file servers via dedicated Virtual IPs (VIPs) on the Client Network:

```bash
# Example Windows client mounting an SMB share
net use Z: \\fs-corp-01.domain.com\vdi-profiles /persistent:yes

# Example Linux client mounting an NFS export (NFS v4.1)
mount -t nfs -o vers=4.1,hard,timeo=600,retrans=2 fs-corp-01.domain.com:/app-data /mnt/app-data
```

### 3. Smart Tiering to S3 Targets

Automate tiering of cold files to Nutanix Objects or external S3 stores:

- **Tiering Triggers**: Set storage usage capacity thresholds (e.g., 70%) and file inactivity age (e.g., 30+ days).
- **Stub File Access**: Tiered files remain visible in share listings and are automatically recalled upon client access.

### 4. File Analytics & Security Alerts

Deploy Nutanix File Analytics to track file access operations and enforce security policies:

- **Audit Logging**: Logs all create, read, write, delete, and permission change events.
- **Ransomware Prevention**: Monitors mass file modification/deletion patterns and automatically isolates infected shares.

---

## Progressive Disclosure Reference Index

When executing specific administration, deployment, network setup, or troubleshooting tasks for Nutanix Files, consult the detailed manual reference guides in `resources/manual/`:

| Topic / Requirement                     | Reference File Pointer                                                                                                     | Key Content Covered                                                                                      |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| **Architecture & High Availability**    | [`resources/manual/overview-and-architecture.md`](@/skills/engineering/nutanix-files/resources/manual/overview-and-architecture.md)                           | Nutanix Files architecture, FSVMs, CVMs, Volume Groups, Minerva controller, and HA failover.             |
| **Prerequisites & Networking**          | [`resources/manual/prerequisites-and-network.md`](@/skills/engineering/nutanix-files/resources/manual/prerequisites-and-network.md)                           | AHV/ESXi prerequisites, FSVM sizing, network segmentation (Client vs Storage VLAN), and port matrix.     |
| **Deployment & Scale-Out**              | [`resources/manual/deployment-and-scaling.md`](@/skills/engineering/nutanix-files/resources/manual/deployment-and-scaling.md)                                 | Prism file server deployment wizard, non-disruptive upgrades (NDU), scale-up, and scale-out.             |
| **Shares, SMB & NFS Protocols**         | [`resources/manual/file-server-and-share-management.md`](@/skills/engineering/nutanix-files/resources/manual/file-server-and-share-management.md)             | Standard vs Distributed shares, SMB 2.1/3.0, NFS v3/v4, ABE, symlinks, quotas, and file blocking.        |
| **Directory Services & Users**          | [`resources/manual/directory-services-and-user-management.md`](@/skills/engineering/nutanix-files/resources/manual/directory-services-and-user-management.md) | Active Directory, LDAP, Kerberos, disjoint domains, NFS-to-SMB user mapping, and RBAC roles.             |
| **Security, ICAP AV & Compliance**      | [`resources/manual/security-antivirus-and-compliance.md`](@/skills/engineering/nutanix-files/resources/manual/security-antivirus-and-compliance.md)           | ICAP antivirus (Symantec, McAfee, Trend Micro), FIPS encryption, SMB encryption, and audit logs.         |
| **Data Protection & Disaster Recovery** | [`resources/manual/data-protection-and-disaster-recovery.md`](@/skills/engineering/nutanix-files/resources/manual/data-protection-and-disaster-recovery.md)   | DR replication, Metro Availability, Self-Service Restore (SSR), snapshot schedules, and share migration. |
| **Smart Tiering & Cloud Storage**       | [`resources/manual/smart-tiering-and-cloud-storage.md`](@/skills/engineering/nutanix-files/resources/manual/smart-tiering-and-cloud-storage.md)               | Smart Tiering to Nutanix Objects/AWS S3/Azure/Wasabi, tiering policies, capacity thresholds, recall.     |
| **Performance & Workload Optimization** | [`resources/manual/performance-and-workload-optimization.md`](@/skills/engineering/nutanix-files/resources/manual/performance-and-workload-optimization.md)   | Workload profiles (VDI, General, Enterprise), auto-rebalancing guardrails, inline/post compression.      |
| **File Analytics & Telemetry**          | [`resources/manual/file-analytics-and-monitoring.md`](@/skills/engineering/nutanix-files/resources/manual/file-analytics-and-monitoring.md)                   | File Analytics dashboard, anomaly detection, ransomware alerts, Prometheus exporter, Syslog.             |
| **Files REST APIs & OData**             | [`resources/manual/files-rest-apis.md`](@/skills/engineering/nutanix-files/resources/manual/files-rest-apis.md)                                               | Files REST APIs v4, OData filtering, ETag concurrency, API Explorer, Python/Curl automation.             |
| **Troubleshooting & Operations**        | [`resources/manual/troubleshooting-and-operations.md`](@/skills/engineering/nutanix-files/resources/manual/troubleshooting-and-operations.md)                 | FSVM CLI tools (`ncli`, `afsi`, `minerva`, `allssh`), log paths, logbay support bundles, fixes.          |
| **Master Documentation Index**          | [`resources/manual/master-index.md`](@/skills/engineering/nutanix-files/resources/manual/master-index.md)                                                     | Master overview map of all 12 documentation modules.                                                     |

