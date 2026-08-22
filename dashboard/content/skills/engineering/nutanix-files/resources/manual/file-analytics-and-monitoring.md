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
- **Ransomware & Anomaly Detection**: Detects abnormal file deletion spikes, mass file renames, or known ransomware extension patterns, triggering real-time alerts or automated share locking.

---

## External Monitoring and Telemetry

- **Prometheus Exporter**: Exposes real-time file server metrics at `https://<fsvm-ip>:9443/metrics`.
- **Syslog Forwarding**: Streams file audit logs to SIEM systems (Splunk, Elastic, QRadar).
- **Event Streaming**: Integrates with Kafka or NATS message queues for custom event automation.

