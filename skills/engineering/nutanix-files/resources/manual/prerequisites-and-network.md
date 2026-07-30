# Nutanix Files Manual: Prerequisites and Network

## Prerequisites and Environmental Requirements

Before deploying Nutanix Files v5.0, ensure the underlying Nutanix cluster and infrastructure meet the following prerequisites.

### Hypervisor and Platform Compatibility

- **Hypervisor**: Nutanix AHV or VMware ESXi.
- **AOS Release**: AOS 5.20 or later (AOS 6.5+ / 7.0+ recommended).
- **Prism Central**: pc.2022.6 or later with Nutanix Files License (Nutanix Unified Storage / NUS).
- **Cluster Sizing**: Minimum 3 Nutanix HCI nodes for production multi-FSVM deployments; single-node deployments supported for ROBO/edge.

### Sizing and Resource Requirements per FSVM

| FSVM Footprint | vCPUs | RAM (GB) | Max Recommended Connections | Typical Workload Use Case |
| :--- | :--- | :--- | :--- | :--- |
| **Small** | 4 | 12 | 500 | Small Office / ROBO / Low IOPS |
| **Medium** | 4 | 16 | 1,000 | General Enterprise Shares |
| **Large** | 8 | 32 | 2,500 | VDI Profiles / High Throughput |
| **Extra Large** | 12 | 64 | 5,000+ | High Performance Analytics / PACS |

---

## Network Architecture and Segmentation

Nutanix Files requires two distinct virtual networks (VLANs) for optimal security and isolation:

1. **Client Network**:
   - Used by end-user clients, application servers, and Active Directory controllers to access SMB shares and NFS exports.
   - Requires dedicated IP addresses for each FSVM plus one Virtual IP (VIP) for the file server instance.
2. **Storage Network**:
   - Dedicated internal network for communication between FSVMs and Nutanix CVMs (iSCSI traffic for Volume Groups).
   - Requires one IP address per FSVM on the CVM network/VLAN.

---

## Port Requirements and Firewall Rules

Ensure the following network ports are open across firewalls:

| Port | Protocol | Source | Destination | Purpose |
| :--- | :--- | :--- | :--- | :--- |
| **445** | TCP | Clients | FSVM Client VIP/IPs | SMB file access |
| **139** | TCP | Clients | FSVM Client VIP/IPs | NetBIOS SMB session |
| **2049** | TCP/UDP | Clients | FSVM Client VIP/IPs | NFS v3 / NFS v4 service |
| **111** | TCP/UDP | Clients | FSVM Client VIP/IPs | RPC Portmapper (NFS) |
| **20048** | TCP/UDP | Clients | FSVM Client VIP/IPs | NFS Mount Daemon |
| **88** | TCP/UDP | FSVMs | Active Directory | Kerberos authentication |
| **389 / 636** | TCP | FSVMs | AD / LDAP | LDAP directory queries / LDAPS |
| **53** | TCP/UDP | FSVMs | DNS Servers | Domain Name Resolution |
| **123** | UDP | FSVMs | NTP Servers | Time Synchronization |
| **1344** | TCP | FSVMs | ICAP AV Servers | Antivirus scanning protocol |
| **9440** | TCP | FSVM / PC | Prism Central | Minerva management & control plane |
