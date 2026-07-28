# Nutanix Objects v5.3 Documentation Index

This directory contains the complete reference documentation for Nutanix Objects v5.3, converted into structured, AI-agent readable Markdown reference guides.

## Documentation Modules

1. **[overview-and-architecture.md](./overview-and-architecture.md)**
   - Nutanix Objects architecture overview, terminology reference, salient features, Microservices Platform (MSP) controllers, Life Cycle Manager (LCM) upgrades, and Role-Based Access Control (RBAC) permissions and roles.
2. **[prerequisites-and-network.md](./prerequisites-and-network.md)**
   - Deployment prerequisites for AHV and ESXi, port requirements, network topology configurations (shared vs. single network), network segmentation, VPC modes (with and without NAT), and platform limitations.
3. **[federated-namespace.md](./federated-namespace.md)**
   - Federated namespace architecture, prerequisites, namespace creation, adding/removing external object stores, namespace management, and error handling.
4. **[deployment-and-scaling.md](./deployment-and-scaling.md)**
   - Deploying object stores via Prism Central, object store & domain naming rules, FT1n:2d fault tolerance, endpoint access, storage expansion, scale-out workflows, secondary cluster addition/removal, FQDN & SSL certificate configuration, Nutanix Cloud Clusters (NC2) deployment, and dark site (offline) installation.
5. **[iam-and-user-management.md](./iam-and-user-management.md)**
   - Nutanix Objects user types, directory configuration (Active Directory / LDAP), access key generation & management, API user controls, IAM High Availability (HA) configuration, and user quota policies.
6. **[bucket-management-and-features.md](./bucket-management-and-features.md)**
   - Creating S3 and NFS buckets, bucket naming conventions, NFS allowlist management, bucket access policies, tag-based policy conditions, object versioning, lifecycle rules, cloud tiering, legal hold, WORM buckets, static website hosting, CORS, bucket tagging, and bucket deletion.
7. **[streaming-replication.md](./streaming-replication.md)**
   - Streaming replication types, guarantees & topologies (Nutanix Objects and AWS S3 targets), prerequisites, remote Prism Central availability zones, IAM synchronization, creating replication rules, delete marker handling, pausing/resuming rules, and replication statistics.
8. **[monitoring-alerts-and-notifications.md](./monitoring-alerts-and-notifications.md)**
   - Performance monitoring for object stores and buckets, storage usage tracking, user quota policies, viewing and managing Nutanix Objects and MSP alerts, event notifications (Syslog, NATS, Kafka), and data event notification rules.
9. **[objects-browser.md](./objects-browser.md)**
   - Nutanix Objects Browser compatibility, administrator workflow, launching the browser UI, supported bucket and object operations, and user/access key management within the browser UI.
10. **[apis-and-integrations.md](./apis-and-integrations.md)**
    - Nutanix Objects Prometheus exporter, S3 CRUD API operations & authentication, supported/unsupported S3 APIs, tagging APIs, S3 Select API (SQL query support and functions), SSE-C encryption, Kafka notification schemas, REST error codes, and backup application integrations.
11. **[troubleshooting-and-operations.md](./troubleshooting-and-operations.md)**
    - Orderly VM shutdown and startup procedures, slow connection detection, UI/Prism Element cluster discovery fixes, missing replication targets, and manual DNS server updates.
