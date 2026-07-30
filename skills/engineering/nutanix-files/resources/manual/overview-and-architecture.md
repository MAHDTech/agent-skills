# Nutanix Files Manual: Overview and Architecture

## Nutanix Files Overview

Nutanix Files (v5.0) is a software-defined, scale-out file storage solution built natively on the Nutanix Cloud Infrastructure (NCI) and Acropolis Distributed Storage Fabric (DSF). It provides enterprise-grade SMB (Server Message Block) and NFS (Network File System) file services, serving workloads such as user profile disks (VDI), enterprise application data, home directories, healthcare PACS imaging, and big data analytics.

Nutanix Files consolidates file storage onto hyperconverged infrastructure (HCI), eliminating traditional standalone NAS arrays and dedicated storage networks.

### Key Architectural Characteristics

- **Software-Defined & Scale-Out**: Runs as virtual machines—File Server Virtual Machines (FSVMs)—across Nutanix cluster nodes. Compute and capacity scale independently or together.
- **Acropolis DSF Integration**: Leverages Nutanix DSF features including data tiering, inline/post-process compression, erasure coding (EC-X), data-at-rest encryption, and snapshot protection.
- **Multiprotocol Storage**: Simultaneous support for SMB 2.1, SMB 3.0, NFS v3, and NFS v4.1 protocols on a single file server instance.
- **High Availability & Self-Healing**: Automated FSVM failure detection, virtual IP failover, and non-disruptive Volume Group reattachment.
- **File Analytics & Intelligent Tiering**: Native metadata engine delivering audit logging, anomaly detection, ransomware alerts, and automated Smart Tiering to S3/Azure cloud endpoints.

---

## Core Components & Microservices

| Component | Function / Role |
| :--- | :--- |
| **FSVM (File Server VM)** | Dedicated virtual machine running the Nutanix Files stack. Each FSVM handles client connections, lock management, protocol negotiation, and file system metadata processing. |
| **CVM (Controller VM)** | Storage controller VM on each Nutanix node providing raw block storage, Volume Groups, and DSF replication to the FSVMs. |
| **Volume Group (VG)** | Nutanix block storage containers mapped to FSVMs via iSCSI. Each FSVM owns a set of VGs for file system data and metadata storage. |
| **Minerva Controller** | Central control plane service running on Prism Central/Element responsible for FSVM lifecycle, deployment, HA coordination, and scale-out workflows. |
| **Zookeeper / Chandy** | Distributed consensus service maintaining cluster topology, FSVM ownership, and file server IP assignments. |
| **Stargate** | DSF service on CVM handling block I/O, cache management, data tiering, and parity calculations for underlying Volume Groups. |

---

## High Availability & Failover Architecture

Nutanix Files delivers high availability across compute, storage, and networking:

1. **FSVM High Availability**:
   - In a multi-node file server deployment (minimum 3 FSVMs), if an FSVM fails, the Minerva service detects the failure within seconds.
   - The virtual IP address associated with the failed FSVM automatically migrates to a surviving FSVM node in the cluster.
   - The Volume Groups owned by the failed FSVM are attached to the target FSVM, resuming file service I/O without client disconnection for SMB 3.0 Continuous Availability or with quick auto-reconnect for SMB 2.1 / NFS.

2. **Storage Resiliency**:
   - Underlying data blocks are protected by Nutanix DSF Redundancy Factor 2 (RF2) or Redundancy Factor 3 (RF3) and optional Erasure Coding (EC-X).
   - Disk or node failures on the Nutanix HCI cluster do not impact FSVM availability.

3. **Prism Central Monitoring**:
   - Monitors FSVM health, CPU/RAM utilization, storage growth, and network latency continuously.
