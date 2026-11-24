+++
title = "release-features"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-api-v4"
+++

# Release features and API enhancements

New and updated Nutanix v4 API features by Prism Central release. For resolved issues, known issues, workarounds, and discontinued endpoints, see [troubleshooting-and-known-issues.md](@/skills/engineering/nutanix-api-v4/resources/manual/troubleshooting-and-known-issues.md).

## pc.7.5.1

### Multi Domain Management

- **FEAT-16939:** Added new Multi Domain Management endpoints integrated with Nutanix Central: Registered Domains, Local Domain, and Locations.
- Note: This release does not include Task APIs for retrieving operation status.

## pc.7.5

### Platform and SDK

- **FEAT-17426 (Java 21 and Spring Framework upgrade):** Upgraded Java 21 and the Spring Framework across API Infra services in Prism Central and Prism Element for security and performance improvements.
- **ENG-425490 (SDK backward compatibility):** Solutions built with newer v4 SDKs can run against older PC/AOS 7.5+ servers as long as they do not call APIs unsupported by the older server. Upgrade servers to PC/AOS 7.5 before rebuilding with a newer SDK.

### AIOps

- **ENG-779150 / ENG-779151:** Made `vendor` and `dataStoreConfig` optional when creating or updating a capacity planning scenario. `vendor` auto-populates for existing clusters and must still be set to a supported vendor for new clusters.
- **ENG-779154:** Added optional `isDisregarded` under `clusterConfig > nodeConfigs` to exclude an existing cluster node from a capacity planning scenario.

### Cluster Management

- **FEAT-16525:** Added `isShared` to the Storage Containers API to support cross-cluster VM storage sharing.
- **FEAT-16485:** Added `externalStorageExtId` to Storage Container v4 GET responses to identify the hosting external storage.
- **FEAT-16179:** Added Clusters API endpoints to list CVMs in a cluster, get details of a specific CVM, and reconfigure CVMs within a cluster.

### Data Policies and Data Protection

- **FEAT-13057:** Added Nutanix Disaster Recovery v4 APIs for Recovery Plans and Recovery Plan Jobs to initiate and monitor planned and unplanned failovers.

### Flow Management and Networking

- **ENG-765455 / ENG-737608:** Deprecated the self-service admin role and replaced it with the project manager role.
- **ENG-269306:** Added support for address and address exception fields in entity group objects.
- **ENG-769635:** Made the secured group rule optional in the Flow Network Security (FNS) policy API.

### Monitoring

- **ENG-791093:** Added `tagOpts` to the Collect logs API request to pass tag-specific arguments.
- **ENG-770187 (List Audits API):** Added `$filter` support for `clusterReference/extId` and `$orderby` support for `sourceEntity/name`, `sourceEntity/type`, and `userReference/name`.
- **ENG-770182 (List Events API):** Added `$filter` support for `classifications`, `clusterUUID`, and `operationType`, and `$orderby` support for `sourceEntity/name` and `operationType`.
- **FEAT-17361 (List Alerts API):** Added `clusterName`, `kbArticles`, and `isRunnable` to responses and `$filter`, plus `$orderby` support for `sourceEntity` and `severity`.

### Prism

- **ENG-819762:** Tightened category create/update validation to restrict certain special characters in keys and values. Do not modify existing keys with restricted characters; create a new compliant key instead.
- **FEAT-16272:** Updated the product API to support enabling Flow Controller and Flow Network Security.

### Virtual Machine Management (VMM)

- **FEAT-5081 (VM startup policies API):** Added CRUD and list endpoints for VM startup policies, compliance listing, start condition conflicts (list/get and dependent/dependee VMs), and dependency conflicts (list/get and dependent/dependee VMs).
- **FEAT-7265 (VM guest customization profiles API):** Added CRUD and list endpoints for guest customization profiles, and added `guestCustomizationProfileConfig` to Clone a VM (replacing `guestCustomizationProfileSpec`).
- **FEAT-17341 (ETag removal):** Removed ETag enforcement from Delete a VM and VM device endpoints (create/remove network, disk, CD-ROM, GPU, PCIe, and serial port devices).
- **FEAT-14568:** Added optional `storageContainersMapping` to Migrate a VM across clusters (supported for unprotected VMs only).
- **FEAT-16606:** Added optional `shouldRevertVmCategories` and `shouldRevertVmOwner` parameters to Revert AHV VM (by default, categories, owner, and project remain unchanged).
- **FEAT-15116 / FEAT-13852:** Added `guestInfo` (`guestOsFullName`, `lastBootUpTime`, `dnsName`, `installedVirtIoVersion`, `guestOsBuildNumber`, and read-only `isVmMobilityDriversInstalled`) and deprecated `guestOsVersion`.
- **FEAT-16886:** Added VMs API endpoints to add and remove custom attributes on VMs and VM disks.
- **FEAT-15248:** Added the Generate VM console token API to the VMM namespace.

## pc.7.3

### Life Cycle Management (pc.7.3)

- **FEAT-16388:** Added `dryRun: true` credential validation to `/inventory`, `/upgrade`, and `/prechecks`. Extended `/inventory`, `/upgrade`, `/prechecks`, and `/computeNotifications` to support ESXi and Cisco Intersight upgrades using direct credentials or Security `CredentialStore` references.

### Virtual Machine Management (pc.7.3)

- **FEAT-14830:** Made OVA v4 APIs generally available (export, create, download, update, delete, deploy), including sourcing OVA files from Nutanix Objects keys.

### Security (pc.7.3)

- **FEAT-16437:** Made STIG, System User Passwords, Secure Snapshot, and SSL Certificates v4 APIs generally available.
- **FEAT-15964:** Made Cloud KMS v4 APIs generally available for managing Azure Cloud KMS keys with Software Data-At-Rest Encryption.

### Data Policies (pc.7.3)

- **FEAT-14586:** Added a Storage Policy v4 API endpoint.

### Cluster Management (pc.7.3)

- **ENG-700261:** Added disk management v4 APIs on Prism Central and Prism Element to update physical disk LED state.

### Prism (pc.7.3)

- **FEAT-16570:** Added Prism Central product enablement APIs to list portfolio products/status and enable Nutanix Disaster Recovery.
- **FEAT-15709:** Added v4 APIs with idempotency and task management to fetch and update Disaster Recovery enablement state.

### IAM (pc.7.3)

- **FEAT-15606:** Deprecated the Self-Service Admin role and replaced it with the Project Manager role.

## pc.2024.3.1

### Object Storage Management (pc.2024.3.1)

- **ENG-624349:** Made Objects v4 APIs version 5.1.1 generally available (CRUD object stores and SSL certificate management).
- **ENG-691601:** Added `alternateIps` (up to four `publicNetworkIps` included in the CA SAN list) to `POST api/objects/v4.0.b1/config/object-stores/{objectStoreExtId}/certificates` and `$filter` support on `alternateIps` in the List Certificate API.
- **ENG-677902:** Added write-only `shouldGenerate` to the certificate model in `POST api/objects/v4.0.b1/config/object-stores/{objectStoreExtId}/certificates` (when `true`, omit `publicCert`, `privateKey`, and `CA`).

### Cluster Management (pc.2024.3.1)

- **ENG-696888:** Added Baseboard Management Controller (BMC) v4 APIs for CRUD management of out-of-band (OOB) BMC credentials.

