+++
title = "file-server-and-share-management"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

{% raw %}
# Nutanix Files Manual: File Server and Share Management

## Share and Export Architecture

Nutanix Files supports two distinct share architectures depending on scale and performance requirements:

### 1. Standard Shares (Non-Distributed)

- **Owner**: Hosted on a single FSVM node.
- **Use Case**: General departmental shares, small project folders, low-concurrency data.
- **Characteristics**: Simple path structure, single Volume Group backing the share.

### 2. Distributed Shares (Scale-Out Shares)

- **Owner**: Data and Top-Level Directories (TLDs) are split and served concurrently across all FSVM nodes in the file server instance.
- **Use Case**: VDI User Profile Disks (FSLogix / VMware DEM), Home Directories, Large Analytics datasets.
- **Characteristics**: Scale-out throughput; automated load balancing across FSVMs.

---

## SMB Share Configuration

Nutanix Files supports SMB 2.1 and SMB 3.0 (including SMB 3.02 / 3.1.1).

### Key SMB Features & Flags

- **Continuous Availability (CA)**: Enables persistent file handles for mission-critical applications (e.g., SQL Server over SMB or Hyper-V). Prevents application failures during FSVM failover.
- **Access-Based Enumeration (ABE)**: Filters directory listings based on client permissions. Users only see files/folders they have read access to.
- **SMB Symlinks**: Supports creation and resolution of symbolic links across SMB shares.
- **Durable Handles**: Re-establishes broken TCP sessions transparently after short network interruptions.

```bash
# Example CLI view of SMB shares via ncli
ncli file-server list-shares name=<file-server-name>
```

---

## NFS Export Configuration

Nutanix Files supports NFS v3 and NFS v4.1.

### NFS Export Controls & Rules

- **Client Access Rules**: Restrict exports by IP address, CIDR block, or hostname.
- **Access Privilege Levels**: Read-Only (RO), Read-Write (RW), or No Access.
- **Squash Options**:
  - `none`: Root privileges preserved (`no_root_squash`).
  - `root_squash`: Root user mapped to `nobody` (UID 65534).
  - `all_squash`: All connected users mapped to anonymous UID/GID.

---

## Directory-Level Quotas and File Blocking

### Quotas

- **Default User Quota**: Sets baseline storage limit for all users on a share.
- **Individual User / Group Quota**: Specific limits override default quotas.
- **Hard Quota**: Blocks further writes immediately when limit is reached.
- **Soft Quota**: Triggers email/Prism alerts while allowing a grace period before blocking.

### File Blocking

- Block unwanted file extensions (e.g., `.exe`, `.mp3`, `.iso`, `.mp4`) at the file server or share level.
- Rejects file creation or rename operations matching blocked extensions.

{% endraw %}
