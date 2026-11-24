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

Administrative operations and troubleshooting use `ncli` and `ncc` on a Nutanix CVM, and `afs`, `allssh`, and `smbstatus` on an FSVM:

```bash
# On a CVM: list file servers, list shares, and run Files NCC health checks
ncli file-server list
ncli fs list-shares uuid=<file-server-uuid>
ncli file-server list-all-fs-shares
ncc health_checks fileserver_checks run_all

# On an FSVM: inspect file server info, shares, SMB health, and HA state
afs fs.info
afs share.list
afs smb.health_check
afs ha.minerva_check_ha_state

# On an FSVM: execute a command across all FSVM nodes
allssh "uptime"

# On an FSVM: check active SMB sessions
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

