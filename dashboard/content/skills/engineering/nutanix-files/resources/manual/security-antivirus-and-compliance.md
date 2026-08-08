+++
title = "security-antivirus-and-compliance"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

{% raw %}
# Nutanix Files Manual: Security, Antivirus, and Compliance

## Security Architecture & Defense in Depth

Nutanix Files enforces enterprise security across storage, transport, and authentication layers:

- **Security Technical Implementation Guides (STIGs)**: Pre-hardened OS image for FSVMs following DoD STIG rules.
- **Access Control Lists (ACLs)**: Native NTFS ACLs for SMB shares and POSIX / NFSv4 ACLs for NFS exports.
- **Access-Based Enumeration (ABE)**: Prevents unauthorized users from seeing directory listings.

---

## Antivirus (AV) Scanning with ICAP

Nutanix Files uses the Internet Content Adaptation Protocol (ICAP) to integrate with third-party antivirus vendors.

### Supported AV Vendors

- Symantec Protection Engine
- McAfee VirusScan Enterprise for Storage / Trellix
- Trend Micro ServerProtect
- Kaspersky Security for Storage

### ICAP Scanning Modes

1. **On-Access Scanning**: Scans files dynamically when opened, written, or renamed by clients.
2. **On-Demand Scanning**: Scans existing shares or directories on a scheduled basis.
3. **Quarantine Action**: Infected files are blocked from access immediately and moved to a quarantine state.

---

## Data Encryption

- **Data-at-Rest Encryption**: FIPS 140-2 validated encryption using AES-256. Managed at cluster level via Nutanix Native Key Manager or external KMS (KMIP).
- **In-Flight Encryption**: SMB 3.0 AES-128-GCM / AES-128-CCMP wire encryption for all client-to-FSVM sessions.

{% endraw %}
