+++
title = "troubleshooting-and-known-issues"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-api-v4"
+++

# Troubleshooting, known issues, and workarounds

Resolved issues, known issues, workarounds, and discontinued endpoints across Nutanix v4 API releases. For new and updated API features by release, see [release-features.md](@/skills/engineering/nutanix-api-v4/resources/manual/release-features.md).

## Known issues and workarounds

### Data Protection

- **ENG-697374 (pc.2024.3 through pc.7.5.1):** RBAC users who restore a Volume Group via `POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore` are not set as the owner of the restored Volume Group and cannot access it.
  - **Workaround:** An admin user must grant permission to the RBAC user after the restore.
- **ENG-787595 (pc.7.5, pc.7.5.1):** During asynchronous and near-synchronous planned or unplanned DR failover, VM or VM disk custom attributes might not be retained.

### Networking

- **ENG-855586 (pc.7.5, pc.7.5.1):** Pagination metadata (page number and quantity per page) is omitted in responses from the List reserved IPs of a managed subnet API, though pagination behavior itself works normally.

### Prism

- **ENG-819762 (pc.7.5, pc.7.5.1):** Deleting category values via v4.0 or v4.1 APIs no longer automatically removes parent keys that have no remaining values, leaving orphaned keys in the database.
  - **Workaround:** Delete orphaned category keys manually through the Prism UI after API-based value deletions.
- **ENG-685815 (pc.2024.3 and later):** `PUT api/categories/{extId}` introduced a non-backward-compatible response schema change between beta and GA, returning a list of application messages starting in pc.2024.3.
  - **Workaround:** Update client code after upgrading beta Prism v4 SDKs to pc.2024.3 or later.
- **ENG-628634 (pc.2024.3 through pc.7.5.1):** `GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations` may fail with HTTP `500` when the category is associated with more than 500 entities or policies.

### Virtual Machine Management (VMM)

- **ENG-664770 (pc.2024.3 through pc.7.5.1):** Several mandatory query parameters are unlabeled in the v4 VM Stats and VM ESXi API docs, causing `400 Bad Request` errors that report missing parameters one at a time.
  - **Workaround:** Pass all seven required parameters together: `$filter`, `$limit`, `$orderby`, `$page`, `$select`, `$statType`, and `$samplingInterval`.
- **ENG-763643 (pc.7.5, pc.7.5.1):** Triggering a VM revert from the recovery points page in Prism Central may drop v4 API custom attributes.
  - **Workaround:** Read `custom_attributes` via v4 VM GET before reverting and reapply them after the revert completes.
- **ENG-809786 (pc.7.5, pc.7.5.1):** Deploying a VM from a v4 VM template does not retain custom attributes from the source VM or disk.
- **ENG-665011 (pc.2024.3, pc.2024.3.1; resolved in pc.7.3):** VM creation fails when using a service account user with an attached API key and an Access Control Policy (ACP) granting the Prism Admin role.
  - **Workaround on affected versions:** Call `/users/me` with `x-ntnx-api-key` set to the service account's API key before creating VMs.

## Resolved issues by release

### pc.7.5

- **ENG-673617 (Cluster Management):** Resolved multi-PC Storage Container v4 API failures caused by oversized certificate payloads when communicating with Prism Element; certificate-based authentication was replaced with Mercury fanout.

### pc.7.3

- **ENG-665011 (VMM):** Resolved VM creation failure for service account users with an attached API key and Prism Admin ACP.
- **ENG-719499 (IAM):** Resolved an issue where `PUT /api/iam/v4.0.b3/authn/users/<extId>` prevented Active Directory and other user types from updating profile details.

### pc.2024.3.1

- **ENG-718469 (IAM):** Resolved an issue where `POST api/iam/v4.0/authn/users/{userExtId}/keys` failed to generate an object key for users originally created with the v1 API.

## Discontinued endpoints

### pc.2024.3 (Networking)

The release-candidate endpoint `GET api/networking/v4.0.b1/config/clusters/{extId}/layer2-stretches/related-entities` is discontinued and replaced by six GA endpoints:

- List remote subnets: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-subnets`
- Get remote subnet: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-subnets/{extId}`
- List remote VPN connections: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vpn-connections`
- Get remote VPN connection: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vpn-connections/{extId}`
- List remote VTEP gateways: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vtep-gateways`
- Get remote VTEP gateway: `GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vtep-gateways/{extId}`

