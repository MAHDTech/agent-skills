+++
title = "troubleshooting-and-operations"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

# Nutanix Files Manual: Troubleshooting and Operations

## Essential CLI Tools for Nutanix Files

Administrative operations and troubleshooting can be performed from the Nutanix CVM or FSVM command line:

```bash
# Connect to Minerva service shell from CVM
ncli file-server list

# Check FSVM cluster health and status
afsi file-server show-health

# Execute command across all FSVM nodes
allssh "uptime"

# Check SMB session status on FSVM
smbstatus
```

---

## Common Issues & Remediation Guide

| Symptom / Issue | Potential Cause | Remediation Steps |
| :--- | :--- | :--- |
| **SMB Share Access Denied** | AD time skew or domain trust failure. | Verify NTP sync across CVMs, FSVMs, and Domain Controllers (`ntpdate -q`). |
| **NFS Mount Timeout** | Firewall port blocking or subnet mismatch. | Verify TCP/UDP ports 2049, 111, and 20048 are open on client VLAN. |
| **FSVM High Memory Usage** | Heavy metadata caching or large directory query. | Check active sessions via `smbstatus`; scale up FSVM RAM via Prism if sustained. |
| **AV Scanning Delay** | ICAP server pool offline or slow ICAP response. | Check ICAP server health in Prism Central **Files > Security > Antivirus**. |

---

## Log Locations & Log Collection

- **CVM Minerva Log**: `/home/nutanix/data/logs/minerva_cvm.log`
- **FSVM Minerva Controller**: `/home/nutanix/data/logs/minerva_fsvm.log`
- **SMB Logs**: `/var/log/samba/` or `/home/nutanix/data/logs/smb.log`
- **NFS Logs**: `/home/nutanix/data/logs/nfs.log`

Generate a support bundle using `logbay`:

```bash
logbay collect --components=file_server --aggregate=true
```

