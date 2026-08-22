+++
title = "troubleshooting-and-known-issues"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-api-v4"
+++

# Troubleshooting, Known Issues & Workarounds

## RELEASE NOTES

### Release Notes | pc.7.5.1

This document describes release-specific new or updated features, resolved issues, and known issues forrelease pc.7.5.1.

### New Features or Enhancements

This release includes the following new and updated features: Multi Domain Management FEAT-16939 The Nutanix Multi Domain Management namespace now includes the following newendpoints integrated with Nutanix Central:

| • | Registered Domains |
| --- | --- |
| • | Local Domain |
| • | Locations |

Note: This release does not include Task APIs for retrieving operation status.

### Known Issues

You might encounter the following known issues in this or recent v4 API releases: Data Protection

| • | ENG-697374 RBAC users who restore a Volume Group POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore are not set as the owner of the restored Volume Group and areunable to access it. |
| --- | --- |

Workaround: Admin users can provide permission to RBAC users.

| • | ENG-787595 During disaster recovery (DR), asynchronous and near-synchronous planned and unplannedfail over might not retain VM or VM Disk custom attributes. |
| --- | --- |

Networking

| • | ENG-855586 The pagination metadata for the List reserved IPs of a managed subnet API is notdisplayed in pagination requests, which includes the page number and quantity per page. However, thepagination functionality of this API is working correctly. |
| --- | --- |

Prism

| • | ENG-819762 When deleting category values through the API (v4.0 or v4.1), the parent keys with noassociated values will no longer be automatically removed. This can result in orphaned keys remaining inthe database. |
| --- | --- |

Workaround: To maintain database consistency and ensure proper cleanup, users must manually deleteany orphaned keys through the UI after performing API based deletions.

| • | ENG-685815 The PUT api/categories/{extId} response includes a non-backward compatible schemachange between the beta version and the GA version of v4 APIs. Starting with pc.2024.3, the PUT api/categories/{extId} returns a list of application messages. |
| --- | --- |

Workaround: After upgrading the Prism v4 SDKs to use the pc.2024.3 version, beta SDK users arerequired to update their code to accommodate the response schema change.

| • | ENG-628634 The GetCategoryById endpoint GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations might fail with the 500 server error if the category represented by theexternal ID is associated with more than 500 entities or policies. This applies only when the ODataparameter $expand is applied to detailedAssociations. |
| --- | --- |

VMM

| • | ENG-664770 Most of the mandatory parameters are not labelled in the v4 VM Stats and VM ESXi APIdocumentation. As a result, submitting a request to retrieve the statistics for a specific AHV or ESXiVM might lead to a 400 Bad Request error, indicating that a mandatory parameter is missing. Also, thesemandatory parameters are listed as missing one by one, requiring you to add these parameters whenindicated in the 400 bad request error. |
| --- | --- |

Workaround: Ensure that you add the following mandatory parameters:

| • | $filter |
| --- | --- |
| • | $limit |
| • | $orderby |
| • | $page |
| • | $select |
| • | $statType |
| • | $samplingInterval |
| • | ENG-763643When a VM revert is triggered from the recovery points page in the Prism Central webconsole, the v4 API custom attributes might not be retained. |

Workaround: Copy the custom_attributes field data before performing the VM revert (v4 VM GET) andreapply the attributes after the operation.

| • | ENG-809786When you deploy a VM from the v4 VM template, the VM does not retain the customattributes from the original disk or VM from which the template was created. |
| --- | --- |

### Release Notes | pc.7.5

This document describes release-specific new or updated features, resolved issues, and known issues forrelease pc.7.5.

### New Features or Enhancements

This release includes the following new and updated features: Java 21 and Spring Framework Upgrade FEAT-17426 Java 21 and the Spring Framework is upgraded across multiple services andcomponents within the API Infra service in Prism Central and Prism Element. The latest springframework offers better security and performance. SDK ENG-425490 Previously, solutions built using SDKs were compatible only with Prism Central andAOS versions that match or exceed the version bundled with the SDK. With this enhancement, you can upgrade SDK versions in your existing solutions on older server versions,provided they do not rely on newer APIs that the older servers do not support. This enables customers torebuild their existing solutions immediately while scheduling server upgrades for a later phase.

| • | FEAT-16886 The VMs API includes the following new endpoints: |
| --- | --- |
| • | Add to the VM's custom attributes |
| • | Remove from the VM's custom attributes |
| • | Add to the VM disk's custom attributes |
| • | Remove from the VM disk's custom attributes |
| • | FEAT-15248 The Generate VM console token is newly added to VMM namespace. This API generatesa token to launch a VM console. |

### Resolved Issues

This v4 API release resolves the following critical issues: Cluster Management ENG-673617 Resolved an issue in multi-PC setups where the Storage Container v4 APIs failedto process due to communication failures with the respective PE cluster. The failure was causedby oversized payloads from multiple PC certificates. The certificate-based authentication is nowreplaced with Mercury fanout to ensure reliable API processing across multi-PC environments.

### Known Issues

You might encounter the following known issues in this or recent v4 API releases: Data Protection

| • | ENG-697374 RBAC users who restore a Volume Group POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore are not set as the owner of the restored Volume Group and areunable to access it. |
| --- | --- |

Workaround: Admin users can provide permission to RBAC users.

| • | ENG-787595During disaster recovery (DR), asynchronous and near-synchronous planned and unplannedfail over might not retain VM or VM Disk custom attributes. |
| --- | --- |

Networking

| • | ENG-855586 The pagination metadata for the List reserved IPs of a managed subnet API is notdisplayed in pagination requests, which includes the page number and quantity per page. However, thepagination functionality of this API is working correctly. |
| --- | --- |

Prism

| • | ENG-819762 When deleting category values through the API (v4.0 or v4.1), the parent keys with noassociated values will no longer be automatically removed. This can result in orphaned keys remaining inthe database. |
| --- | --- |

Workaround: To maintain database consistency and ensure proper cleanup, users must manually deleteany orphaned keys through the UI after performing API based deletions.

| • | ENG-685815 The PUT api/categories/{extId} response includes a non-backward compatible schemachange between the beta version and the GA version of v4 APIs. Starting with pc.2024.3, the PUT api/categories/{extId} returns a list of application messages. |
| --- | --- |

Workaround: After upgrading the Prism v4 SDKs to use the pc.2024.3 version, beta SDK users arerequired to update their code to accommodate the response schema change.

| • | ENG-628634 The GetCategoryById endpoint GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations might fail with the 500 server error if the category represented by the |
| --- | --- |

external ID is associated with more than 500 entities or policies. This applies only when the ODataparameter $expand is applied to detailedAssociations. VMM

| • | ENG-664770 Most of the mandatory parameters are not labelled in the v4 VM Stats and VM ESXi APIdocumentation. As a result, submitting a request to retrieve the statistics for a specific AHV or ESXiVM might lead to a 400 Bad Request error, indicating that a mandatory parameter is missing. Also, thesemandatory parameters are listed as missing one by one, requiring you to add these parameters whenindicated in the 400 bad request error. |
| --- | --- |

Workaround: Ensure that you add the following mandatory parameters:

| • | $filter |
| --- | --- |
| • | $limit |
| • | $orderby |
| • | $page |
| • | $select |
| • | $statType |
| • | $samplingInterval |
| • | ENG-763643When a VM revert is triggered from the recovery points page in the Prism Central webconsole, the v4 API custom attributes might not be retained. |

Workaround: Copy the custom_attributes field data before performing the VM revert (v4 VM GET) andreapply the attributes after the operation.

| • | ENG-809786When you deploy a VM from the v4 VM template, the VM does not retain the customattributes from the original disk or VM from which the template was created. |
| --- | --- |

### Release Notes | pc.7.3

Release-specific information for release pc.7.3

### New Features and Enhancements

This release includes the following new or updated features: Life Cycle Management

| • | FEAT-16388The Lifecycle APIs - /inventory, /upgrade, and /prechecks now support a dryrun capability. When the dryRun parameter is set to true, these APIs validate the provided credentialsor credential store references on the underlying cluster and return the validation response. This releasealso enhances the existing Lifecycle namespace APIs /inventory, /upgrade, /prechecks, and/computeNotifications to support upgrades for ESXi and Cisco intersight using either direct |
| --- | --- |

credentials or credential store references. Credential store references can be created using the CredentialStore APIs available in the Security namespace. Virtual Machine Management

| • | FEAT-14830 OVA v4 APIs are generally available in this release. These APIs provide full lifecyclesupport for Nutanix OVA operations, including VM export, creation, download, update, deletion, anddeployment. For OVA creation, you can also upload OVA files to Nutanix Objects and use the object keyas the source. |
| --- | --- |

Namespace: Prism

| • | ENG-685815 The PUT api/categories/{extId} response has a non-backward compatible schemachange between the beta version and the GA version of v4 APIs. Starting with pc.2024.3, the PUT api/categories/{extId} returns a list of application messages. |
| --- | --- |

Workaround: After upgrading the Prism v4 SDKs to use the pc.2024.3 version, beta SDK users arerequired to update their code to accommodate the response schema change.

| • | ENG-628634 The GetCategoryById endpoint GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations might fail with the 500 server error, if the category represented by theexternal ID is associated with more than 500 entities or policies. This applies only when the ODataparameter $expand is applied on detailedAssociations. |
| --- | --- |

Namespace: VMM

| • | ENG-664770 Most of the mandatory parameters are not labeled in the v4 VM Stats and VM ESXi APIdocumentation. As a result, submitting a request to retrieve the statistics for a specific AHV or ESXiVM might lead to a 400 Bad Request error, indicating that a mandatory parameter is missing. Also, thesemandatory parameters are listed as missing one by one, requiring you to add these parameters whenindicated in the 400 bad request error. |
| --- | --- |

Workaround: Ensure to add the following mandatory parameters:

| • | $filter |
| --- | --- |
| • | $limit |
| • | $orderby |
| • | $page |
| • | $select |
| • | $statType |
| • | $samplingInterval |

### Release Notes | pc.2024.3.1

Release-specific information for release pc.2024.3.1.

### New Features and Enhancements

This release includes the following new or updated features: Namespace: Object Storage Management

| • | ENG-624349 The Objects v4 APIs version 5.1.1 is generally available in this release. These APIs coverfeatures related to Nutanix Object Storage. With these APIs, you can create, read, update, and deleteobject stores. You can also manage SSL certificates for the object stores. |
| --- | --- |
| • | ENG-691601 The Create Certificate API POST api/objects/v4.0.b1/config/object-stores/%7BobjectStoreExtId%7D/certificates is updated to include a new parameter called alternateIps. The |

alternate IP addresses are added to the certificate authority (CA) Subject Alternate Name (SAN) list duringthe certificate generation process, or they can be part of an uploaded or existing CA. The alternate IP addresses must be one of the Object Store's public IP addresses (publicNetworkIps).Currently, an Object Store can have a maximum of four public IP address, which means that you canprovide up to four alternate IP addresses. Additionally, the alternateIps parameter is supported by the $filter parameter in the List Certificate APIPOST api/objects/v4.0.b1/config/object-stores/{objectStoreExtId}/certificates.

| • | ENG-677902 A new write-only parameter, shouldGenerate, is added to the certificate model in the CreateCertificate API POST api/objects/v4.0.b1/config/object-stores/%7BobjectStoreExtId%7D/certificates.When the shouldGenerate parameter is set to true in the API request, certificates are generated using thealternate fully qualified domain names (FQDNs) and IP addresses provided in the request. Note that whenthe shouldGenerate parameter is set to true, the parameters publicCert, privateKey, and CA must not beincluded. |
| --- | --- |

Namespace: Cluster Management

| • | ENG-696888 The baseboard management controller (BMC) v4 APIs is available under ClusterManagement namespace. These APIs allow you to read, create, update, and delete out-of-band (OOB)BMC credentials. |
| --- | --- |

### Resolved Issues

This v4 API release resolves the following critical issues: Namespace: Identity and Access Management

| • | ENG-718469 Resolved an issue where you were unable to generate an object key using the Create a userkey API POST api/iam/v4.0/authn/users/{userExtId}/keys request if the user were created with the v1API. All users can now generate the object key. |
| --- | --- |

### Known Issues

You might encounter the following known issues in this or recent v4 API releases: Namespace: Data Protection

| • | ENG-697374 RBAC users who restore a Volume Group POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore are not set as the owner of the restored Volume Group and areunable to access it. |
| --- | --- |

Workaround: Admin users can provide permission to RBAC users. Namespace: Prism

| • | ENG-685815 The PUT api/categories/{extId} response has a non-backward compatible schemachange between the beta version and the GA version of v4 APIs. Starting with pc.2024.3, the PUT api/categories/{extId} returns a list of application messages. |
| --- | --- |

Workaround: After upgrading the Prism v4 SDKs to use the pc.2024.3 version, beta SDK users arerequired to update their code to accommodate the response schema change.

| • | ENG-628634 The GetCategoryById endpoint GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations might fail with the 500 server error, if the category represented by theexternal ID is associated with more than 500 entities or policies. This applies only when the ODataparameter $expand is applied on detailedAssociations. |
| --- | --- |

Namespace: VMM

| • | ENG-664770 Most of the mandatory parameters are not labeled in the v4 VM Stats and VM ESXi APIdocumentation. As a result, submitting a request to retrieve the statistics for a specific AHV or ESXiVM might lead to a 400 Bad Request error, indicating that a mandatory parameter is missing. Also, thesemandatory parameters are listed as missing one by one, requiring you to add these parameters whenindicated in the 400 bad request error. |
| --- | --- |

Workaround: Ensure to add the following mandatory parameters:

| • | $filter |
| --- | --- |
| • | $limit |
| • | $orderby |
| • | $page |
| • | $select |
| • | $statType |
| • | $samplingInterval |
| • | ENG-665011 The VM creation fails if you create a service account user, attach an API key to it, and addan access control policy (ACP) with the Prism admin role to it. |

Workaround: Call the /users/me API and set the x-ntnx-api-key header with the API key associated withthe service account.

### Release Notes | pc.2024.3

Release-specific information for release pc.2024.3.

### Known Issues

You might encounter the following known issues in this or recent v4 API releases: Namespace: Data Protection

| • | ENG-697374 RBAC users who restore a Volume Group POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore are not set as the owner of the restored Volume Group and areunable to access it. |
| --- | --- |

Workaround: Admin users can provide permission to RBAC users. Namespace: Prism

| • | ENG-685815 The PUT api/categories/{extId} response has a non-backward compatible schemachange between the beta version and the GA version of v4 APIs. Starting with pc.2024.3, the PUT api/categories/{extId} returns a list of application messages. |
| --- | --- |

Workaround: After upgrading the Prism v4 SDKs to use the pc.2024.3 version, beta SDK users arerequired to update their code to accommodate the response schema change.

| • | ENG-628634 The GetCategoryById endpoint GET api/prism/v4.0/config/categories/{extId}?$expand=detailedAssociations might fail with the 500 server error, if the category represented by theexternal ID is associated with more than 500 entities or policies. This applies only when the ODataparameter $expand is applied on detailedAssociations. |
| --- | --- |

Namespace: VMM

| • | ENG-664770 Most of the mandatory parameters are not labelled in the v4 VM Stats and VM ESXi APIdocumentation. As a result, submitting a request to retrieve the statistics for a specific AHV or ESXiVM might lead to a 400 Bad Request error, indicating that a mandatory parameter is missing. Also, thesemandatory parameters are listed as missing one by one, requiring you to add these parameters whenindicated in the 400 bad request error. |
| --- | --- |

Workaround: Ensure to add the following mandatory parameters:

| • | $filter |
| --- | --- |
| • | $limit |
| • | $orderby |
| • | $page |
| • | $select |
| • | $statType |
| • | $samplingInterval |
| • | ENG-665011 The VM creation fails if you create a service account user, attach an API key to it, and addan access control policy (ACP) with the Prism admin role to it. |

Workaround: Call the /users/me API and set the x-ntnx-api-key header with the API key associated withthe service account.

### Discontinued Endpoints

Namespace: Networking The following endpoint was provided to limited users with the release candidate (RC) version: Get the stretch-related entities from the specified Prism Central cluster GET api/networking/v4.0.b1/config/clusters/{extId}/layer2-stretches/related-entities With this release, the endpoint is discontinued and replaced with the following six endpoints:

| • | List remote subnets: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-subnets |
| --- | --- |
| • | Get remote subnets: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-subnets/{extId} |
| • | List remote VPN connections: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vpn-connections |
| • | Get remote VPN connection: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vpn-connections/{extId} |
| • | List remote VTEP gateways: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vtep-gateways |
| • | Get remote VTEP gateways: GET api/networking/v4.0/config/clusters/{clusterExtId}/remote-vtep-gateways/{extId} |

