---
name: nutanix-objects
description: Expert guidance, CLI commands, architecture reference, bucket lifecycle management, S3 API operations, streaming replication, IAM configuration, monitoring, and troubleshooting for Nutanix Objects (v5.3). Use when working with Nutanix Objects, S3 object storage, Nutanix Objects Browser, bucket policies, WORM storage, cloud tiering, or Nutanix Objects REST APIs.
---

# Nutanix Objects

Nutanix Objects is an enterprise-grade, software-defined S3-compatible object storage service built on the Nutanix Cloud Infrastructure (NCI) and Microservices Platform (MSP). It delivers scalable, secure, multi-tenant object storage capable of handling petabytes of unstructured and machine-generated data for backup, long-term retention, big data analytics, and cloud-native applications.

Nutanix Objects combines:

- **S3 & NFS Compatibility**: RESTful S3 API compliance along with NFS v3 read/write access to object buckets.
- **High Availability & Scale-Out**: Microservices architecture running on Nutanix MSP with automated VM scale-out and storage expansion.
- **Data Protection & Compliance**: Write-Once-Read-Many (WORM) storage, object versioning, immutability, legal hold, and encryption (SSE-C).
- **Multi-Site Streaming Replication**: Active-passive and active-active bucket replication across Nutanix Objects instances and public AWS S3 destinations.
- **Federated Namespace**: Single global namespace spanning multiple Nutanix Objects deployments and external S3 object stores.
- **Nutanix Objects Browser**: Native web-based object browser for bucket management, object upload/download, and user key management.

---

## Core Operations & Workflows

### 1. Bucket Operations & S3 API Access

Nutanix Objects endpoints provide standard S3-compatible REST interfaces accessible via AWS CLI, S3 SDKs, or standard HTTP clients:

```bash
# Set S3 endpoint and credentials for Nutanix Objects
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export ENDPOINT_URL="https://objects.domain.com"

# List buckets in Nutanix Objects
aws --endpoint-url $ENDPOINT_URL s3 ls

# Create a new bucket
aws --endpoint-url $ENDPOINT_URL s3 mb s3://my-app-bucket

# Upload object with server-side encryption or tags
aws --endpoint-url $ENDPOINT_URL s3 cp dataset.csv s3://my-app-bucket/data/dataset.csv
```

### 2. Multi-Site Streaming Replication

Replicate buckets between local Nutanix Objects instances or to AWS S3:

- **Nutanix-to-Nutanix**: Configured via Prism Central by adding remote Prism Central instances as Availability Zones and setting up IAM synchronization.
- **Nutanix-to-S3**: Direct streaming replication to external AWS S3 buckets using S3 access keys and endpoints.

### 3. WORM & Compliance Bucket Configuration

Ensure data immutability with WORM policies:

- **Grace Period**: 24-hour grace window during which WORM policies can be altered or removed.
- **Retention Locking**: After the grace period, objects cannot be deleted or modified until the retention period expires.
- **Versioning**: Bucket versioning must be enabled to update objects in a WORM bucket.

### 4. Telemetry & Prometheus Monitoring

Expose object store metrics directly to Prometheus:

```bash
# Scrape Nutanix Objects Prometheus Exporter endpoint
curl -k https://<objects-exporter-ip>:9443/metrics
```

---

## Progressive Disclosure Reference Index

When performing specific administration, deployment, or troubleshooting tasks for Nutanix Objects, consult the detailed manual reference guides in `resources/manual/`:

| Topic / Requirement                 | Reference File Pointer                                                                                               | Key Content Covered                                                                                                                           |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| **Architecture & RBAC Roles**       | [`resources/manual/overview-and-architecture.md`](resources/manual/overview-and-architecture.md)                     | Terminology, architecture components, MSP controllers, LCM upgrades, RBAC permissions, and admin roles.                                       |
| **Prerequisites & Networking**      | [`resources/manual/prerequisites-and-network.md`](resources/manual/prerequisites-and-network.md)                     | AHV/ESXi prerequisites, port requirements, shared vs. single network topologies, network segmentation, and VPC modes.                         |
| **Federated Namespace**             | [`resources/manual/federated-namespace.md`](resources/manual/federated-namespace.md)                                 | Global namespace setup, external S3 object store integration, namespace management, and error handling.                                       |
| **Deployment & Scaling**            | [`resources/manual/deployment-and-scaling.md`](resources/manual/deployment-and-scaling.md)                           | Prism Central deployment, FT1n:2d fault tolerance, storage expansion, scale-out, SSL certificates, NC2, and dark site installs.               |
| **IAM & User Management**           | [`resources/manual/iam-and-user-management.md`](resources/manual/iam-and-user-management.md)                         | User types, Active Directory / LDAP integration, API key generation, IAM High Availability (HA), and user quotas.                             |
| **Bucket Management & Features**    | [`resources/manual/bucket-management-and-features.md`](resources/manual/bucket-management-and-features.md)           | S3 & NFS bucket creation, bucket policies, tag-based conditions, versioning, lifecycle rules, cloud tiering, WORM, CORS, and website hosting. |
| **Streaming Replication**           | [`resources/manual/streaming-replication.md`](resources/manual/streaming-replication.md)                             | Replication topologies (Nutanix & S3 targets), IAM sync, delete markers, rule control (pause/resume/delete), and statistics.                  |
| **Monitoring, Alerts & Events**     | [`resources/manual/monitoring-alerts-and-notifications.md`](resources/manual/monitoring-alerts-and-notifications.md) | Performance metrics, usage tracking, Objects and MSP alerts, event notifications (Syslog, NATS, Kafka), and data event rules.                 |
| **Nutanix Objects Browser**         | [`resources/manual/objects-browser.md`](resources/manual/objects-browser.md)                                         | Browser UI compatibility, admin workflows, bucket and object operations, and key management within the browser interface.                     |
| **APIs, Prometheus & Integrations** | [`resources/manual/apis-and-integrations.md`](resources/manual/apis-and-integrations.md)                             | Prometheus exporter, S3 API operations matrix, S3 Select SQL functions, SSE-C encryption, Kafka schemas, and backup integrations.             |
| **Troubleshooting & Operations**    | [`resources/manual/troubleshooting-and-operations.md`](resources/manual/troubleshooting-and-operations.md)           | VM shutdown/startup procedures, slow connection detection, UI/subnet discovery fixes, and manual DNS server updates.                          |
| **Master Documentation Index**      | [`resources/manual/master-index.md`](resources/manual/master-index.md)                                               | Master overview map of all 11 documentation modules.                                                                                          |
