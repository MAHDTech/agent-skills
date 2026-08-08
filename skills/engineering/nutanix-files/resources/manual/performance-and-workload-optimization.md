# Nutanix Files Manual: Performance and Workload Optimization

## Workload Profiles

Nutanix Files allows tuning file server profiles for specific application workloads:

| Workload Type | Optimization Characteristics | Typical Applications |
| :--- | :--- | :--- |
| **General Purpose** | Balanced read/write cache allocation and standard metadata handling. | Departmental shares, home directories |
| **VDI (Virtual Desktops)** | Optimized for high concurrent burst IOPS, small file reads, and FSLogix profile disks. | VMware Horizon, Citrix Virtual Apps |
| **Enterprise Apps** | High throughput sequential read/write tuning, large block size I/O. | PACS Imaging, Video Editing, Big Data |

---

## Auto-Balancing and Rebalancing Guardrails

Nutanix Files continuously monitors CPU, Memory, and IOPS load across all FSVMs:

- **Auto-Rebalancing**: Automatically shifts Volume Groups or Top-Level Directories (TLDs) from overloaded FSVMs to underutilized FSVMs.
- **Guardrails**: Prevents aggressive rebalancing during peak business hours; sets thresholds for maximum concurrent migrations.

---

## Storage Compression

- **Inline Compression**: Compresses data blocks in memory before writing to disk, reducing storage footprint without performance impact.
- **Post-Process Compression**: Scans cold data blocks during background maintenance cycles to maximize space savings.
