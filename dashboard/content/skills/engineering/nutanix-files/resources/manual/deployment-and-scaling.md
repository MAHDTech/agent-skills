+++
title = "deployment-and-scaling"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

# Nutanix Files Manual: Deployment and Scaling

## Deployment Workflows

Nutanix Files deployment is managed entirely through Prism Central or Prism Element using automated workflows.

### File Server Creation Step-by-Step

1. **Prism Central Setup**:
   - Navigate to **Services > Files** or **Storage > File Servers**.
   - Select **Create File Server**.
2. **General Details Configuration**:
   - Specify File Server Name (e.g., `fs-corp-01`), Domain Name, and DNS Servers.
3. **Client Network & Storage Network Selection**:
   - Assign Client Network VLAN, Subnet, Netmask, Gateway, and IP pool range.
   - Assign Storage Network VLAN.
4. **FSVM Sizing & Scale**:
   - Choose FSVM Count (e.g., 3 FSVMs for scale-out HA).
   - Select Resource Profile (vCPUs and Memory per FSVM).
   - Set Initial File Server Storage Capacity (GB/TB).
5. **Directory Services Integration**:
   - Option to join Active Directory immediately during creation by providing AD Domain Admin credentials or Pre-created Machine Account details.

---

## Upgrades and Maintenance (NDU)

Nutanix Files supports Non-Disruptive Upgrades (NDU) via Nutanix Life Cycle Manager (LCM):

- **Rolling Upgrade Sequence**: LCM updates FSVMs one at a time.
- **Client Continuity**:
  - For SMB 3.0 Continuous Availability shares, clients automatically transition to partner FSVMs without I/O interruption.
  - For NFS v3/v4 clients, connections fail over to partner IPs transparently during the brief reboot of each FSVM node.

---

## Scale-Up and Scale-Out Procedures

### 1. Scale-Up (Vertical Scaling)

Scale-up increases CPU and Memory allocations for existing FSVMs:

- Performed from **Prism Central > File Server > Update Configuration**.
- Increase vCPU count (e.g., 4 to 8 vCPUs) or Memory (16 GB to 32 GB).
- Executed rolling-style to maintain continuous file availability.

### 2. Scale-Out (Horizontal Scaling)

Scale-out adds new FSVM nodes to an existing file server instance (e.g., expanding from 3 to 4 or 5 FSVMs):

- Increases total connection capacity, IOPS, and throughput.
- Automatically distributes Volume Groups and shares across the expanded FSVM pool.
- Requires available IP addresses in the Client and Storage subnet pools.

### 3. Storage Capacity Expansion

- Storage capacity of a file server can be expanded dynamically without downtime.
- In Prism Central, click **Expand Storage**, enter the target capacity (TB), and apply.
- Underlying Volume Groups automatically expand in DSF.

