+++
title = "file-analytics-and-monitoring"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-files"
+++

# Nutanix Files Manual: File Analytics and Monitoring

## File Analytics Overview

Nutanix File Analytics is a dedicated microservice that delivers real-time visibility, security auditing, and data intelligence for Nutanix Files deployments.

### Key Capabilities

- **Audit Trails**: Tracks every file operation (create, read, write, delete, permission change) with user, IP, and timestamp attribution.
- **Data Age & Capacity Distribution**: Visualizes storage usage by file age, size, extension, and top active users.
- **Ransomware and anomaly detection**: Detects abnormal file deletion spikes, mass file renames, and known ransomware extension patterns, raising real-time alerts. Automated response actions (such as blocking offending users or clients when anomaly rules are configured to act) are version-dependent; verify the supported actions for your deployed File Analytics release before enabling enforcement.

---

## External monitoring and telemetry

- **Prism Central and File Analytics metrics**: Monitor capacity, latency, IOPS, and audit trends through Prism Central and the File Analytics console. Confirm any external metrics exporter setup against your Nutanix Files release documentation rather than assuming an FSVM metrics port.
- **Syslog forwarding**: Streams file audit logs to SIEM systems (Splunk, Elastic, QRadar).
- **Event streaming**: Integrates with external event/audit notification pipelines (verify supported brokers and schemas for your Files and File Analytics versions).

