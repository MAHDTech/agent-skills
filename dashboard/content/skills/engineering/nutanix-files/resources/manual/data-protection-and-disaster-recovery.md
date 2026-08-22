+++
title = "data-protection-and-disaster-recovery"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

# Nutanix Files Manual: Data Protection and Disaster Recovery

## Data Protection & Replication Topologies

Nutanix Files integrates with Nutanix Disaster Recovery engine for flexible data protection.

### Replication Modes

- **Asynchronous Replication**: Periodic RPO (from 1 hour to 24 hours) between local and remote Nutanix clusters.
- **NearSync Replication**: Low RPO (1 minute to 15 minutes) using lightweight continuous snapshot streaming.
- **Synchronous Replication (Metro Availability)**: Zero RPO (RPO = 0) synchronous file server mirroring across sites within metro distance (< 5ms latency).

---

## Disaster Recovery Failover and Failback

1. **Failover Execution**:
   - In Prism Central DR Console, trigger **Failover** for the target Protection Domain or Recovery Plan.
   - The remote file server instance activates Volume Groups, binds IP addresses, and resumes client access.
2. **Failback**:
   - Reverse replication resynchronizes delta changes back to the primary site before reversing roles.

---

## Self-Service Restore (SSR)

Self-Service Restore enables end users to recover accidentally deleted or overwritten files directly from Windows Explorer using Previous Versions.

- **Snapshot Schedules**: Configure hourly, daily, weekly, or monthly snapshot retention schedules.
- **Shadow Copy Integration**: SSR exposes snapshots seamlessly via standard VSS / Previous Versions tabs on Windows clients.

