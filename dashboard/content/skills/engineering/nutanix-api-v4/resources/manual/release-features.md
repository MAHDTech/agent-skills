+++
title = "release-features"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-api-v4"
+++

{% raw %}
# Release Features and API Enhancements

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

Note: This enhancement is supported with server PC/AOS 7.5 version and later. Therefore, you are advised toupgrade to PC/AOS 7.5 before a newer SDK is used to re-build the solution. AIOps

| • | ENG-779150 ENG-779151 The vendor and dataStoreConfig attributes are now optional while creating acapacity planning scenario and updating a capacity planning scenario. |
| --- | --- |

Note: The vendor attribute is automatically populated by retrieving the cluster details for an existingcluster. However, the vendor attribute must be filled with one of the supported vendors for a new cluster.

| • | ENG-779154 An optional field, isDisregarded, is added to create capacity planning scenarioand update capacity planning scenario APIs. You can find the isDisregarded field underclusterConfig>nodeConfigs parameter. The isDisregarded field indicates whether the added node is anexisting node in the cluster that must be excluded from the scenario. |
| --- | --- |

Cluster Management

| • | FEAT-16525 The Storage Containers API now supports sharing storage containers through the isSharedparameter. The isShared parameter is part of cross-cluster VM storage. |
| --- | --- |
| • | FEAT-16485 The response from the Storage Container V4 GET API now includes a new parameterexternalStorageExtId. The externalStorageExtId indicates the identifier of the external storage where thisstorage container is hosted. |
| • | FEAT-16179 The Clusters API includes the following new endpoints: |
| • | Get a list of CVMs associated with a cluster |
| • | Get the details of a specific CVM associated with a cluster |
| • | Reconfigure CVMs within a cluster |

Data Policies FEAT-13057 Nutanix Disaster Recovery now includes Recovery Plans v4 APIs. The recovery plansAPI allows seamless integration with customer ecosystems and ensures compatibility with Nutanixinternal products, delivering a unified experience across all Nutanix offerings. Data Protection FEAT-13057 Nutanix Disaster Recovery now includes Recovery Plans and Recovery Plan Jobsv4 APIs. The new APIs support enterprise customers who initiate and monitor both planned andunplanned fail-overs through reliable and consistent APIs. The API facilitates seamless integrationwith customer ecosystems and ensures compatibility with existing Nutanix products. Flow Management

| • | ENG-765455 The self-service admin role is now deprecated and replaced by a new project manager role,which provides the same functionality. |
| --- | --- |
| • | ENG-269306 Introduced acceptance of address and address exception in entity group object. |
| • | ENG-769635 You can now choose not to specify the secured group rule in the flow network security(FNS) policy API. |

Monitoring

| • | ENG-791093 The tagOpts parameter is added to the Collect logs API request. This parameter allowsusers to pass additional arguments specific to a particular tag. |
| --- | --- |

| • | ENG-770187 The List Audits API includes the following enhancements: |
| --- | --- |
| • | The $filter parameter now supports filtering by clusterReference/extId criterion. |
| • | The $orderby parameter now supports sorting by sourceEntity/name, sourceEntity/type, anduserReference/name criteria. |
| • | ENG-770182 The List Events API includes the following enhancements: |
| • | The $filter parameter now supports filtering by classifications, clusterUUID, andoperationType criteria. |
| • | The $orderby parameter now supports sorting by sourceEntity/name and operationType criteria. |
| • | FEAT-17361 The List Alerts API includes the following enhancements: |
| • | The API response now includes the following additional parameters: clusterName, kbArticles, andisRunnable. |
| • | The $filter parameter now supports filtering by clusterName, kbArticles, and isRunnablecriteria. |
| • | The $orderby parameter now supports sorting by sourceEntity and severity criteria. |

Networking ENG-737608 The self-service admin role is now deprecated and replaced by a new project managerrole, which provides the same functionality. Prism

| • | ENG-819762 The validation rules for creating or updating a category are enhanced to restrict the use ofcertain special characters in both keys and values. If a category key contains any restricted characters,users cannot add new values or update existing fields. This behavior aligns with the updated validationframework. |
| --- | --- |

Note: To avoid issues, do not modify existing keys that contain restricted characters. Instead, create a newcategory key that meets the necessary restrictions if updates are required. For more information on the validation requirements, see Prism API documentation.

| • | FEAT-16272 The update product API now supports enabling Flow Controller and the Flow NetworkSecurity product. |
| --- | --- |

Virtual Machine Management (VMM)

| • | FEAT-5081 The VM startup policies API is newly added to VMM and includes the following endpoints: |
| --- | --- |
| • | Get the VM startup policy based on the provided external identifier |
| • | Update VM startup policy |
| • | Delete VM startup policy |
| • | List VM startup policies |
| • | Create VM startup policy |
| • | List VM compliances of a VM startup policy |
| • | List start condition conflicts of a VM startup policy |
| • | Get start condition conflict of a VM startup policy |
| • | List dependent VMs of a start condition conflict of a VM startup policy |
| • | List dependee VMs of a Start condition conflict of a VM startup policy |
| • | List dependency conflicts of a VM startup policy |
| • | Get dependency conflict of a VM startup policy |
| • | List dependent VMs of a dependency conflict of a VM startup policy |
| • | List dependee VMs of a dependency conflict of a VM startup policy |
| • | FEAT-7265 The VM guest customization profiles API is newly added to VMM and includes thefollowing endpoints: |
| • | Get VM guest customization profile configuration based on the provided external identifier |
| • | Update VM guest customization profile |
| • | Delete VM guest customization profile |
| • | List VM guest customization profiles |
| • | Create a VM guest customization profile |

| • | FEAT-17341 The ETag enforcement is removed from the following endpoints: |
| --- | --- |
| • | Delete a VM |
| • | Create a network device for a VM |
| • | Remove a network device from a VM |
| • | Create a disk device for a VM |
| • | Remove the specified disk device from a VM |
| • | Create a CD-ROM device |
| • | Remove a CD-ROM device from a VM |
| • | Attach a GPU device to a VM |
| • | Remove a GPU device from a VM |
| • | Create a PCIe device for a VM |
| • | Remove a PCIe device from a VM |
| • | Create a serial port for a VM |
| • | Remove a serial port from a VM |
| • | FEAT-14568 An optional parameter, storageContainersMapping, is added to migrate a VM acrossclusters API request. This parameter specifies the storage container mapping used when migrating a VMfrom the source cluster to the target cluster. |

Note: The storageContainerMapping parameter is supported only for unprotected VMs.

| • | FEAT-16606 The revert the AHV VM API request includes the following new optional parameters: |
| --- | --- |
| • | shouldRevertVmCategories: Reverts the VM categories to the ones captured at the time the VMrecovery point was created. |
| • | shouldRevertVmOwner: Reverts the VM owner information captured when the VM recovery pointwas created. |

Note: By default, the categories, owner, and project information of the VM remain unchanged during theVM revert operation if none of the optional parameters are set.

| • | FEAT-15116 FEAT-13852 A new parameter, guestInfo, is added, and the guestOsVersion parameter isnow deprecated. The guestInfo parameter consists of the following fields: |
| --- | --- |
| • | guestOsFullName |
| • | lastBootUpTime |
| • | dnsName |
| • | installedVirtIoVersion |
| • | guestOsBuildNumber |

The isVmMobilityDriversInstalled field is also added to the guestInfo parameter and are read-only fields.

| • | FEAT-7265 The guestCustomizationProfileConfig parameter is added to Clone a VM API request. Thisparameter replaces the existing configurations for the guestCustomizationProfileSpec. |
| --- | --- |

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

Security

| • | FEAT-16437 The following v4 APIs are now generally available in this release: STIG, System UserPasswords, Secure Snapshot, and SSL Certificates. |
| --- | --- |
| • | FEAT-15964 Cloud KMS v4 APIs are now GA. Use these APIs to create and manage Azure Cloud KeyManagement Service (KMS) on Prism Central. You can use Cloud KMS to store encryptions keys forSoftware Data-At-Rest Encryption feature. |

Data Policies

| • | FEAT-14586 The Storage Policy feature now offers a v4 API endpoint. The v4 API enables seamlessintegration with customer ecosystems and ensures compatibility with internal Nutanix products such asCNDS, providing a unified experience across Nutanix offerings. |
| --- | --- |

Cluster Management

| • | ENG- 700261 This release introduces a v4 API set for disk management on Prism Central and PrismElement. You can use these APIs to update the light-emitting diode (LED) state of a physical disk to helpidentify a particular disk. |
| --- | --- |

Prism

| • | FEAT- 16570 This release introduces a set of APIs for product enablement in Prism Central. You can usethese APIs to list all the available portfolio products, along with their status. It can also be used to enableNutanix Disaster Recovery product. |
| --- | --- |
| • | FEAT-15709 The enabling of Disaster Recovery workflow is now supported through v4 APIs. This releaseintroduces v4 APIs to fetch and update the enablement state of the disaster recovery feature in PrismCentral. The v4 APIs also provide features like idempotency and task management. |

IAM FEAT-15606The Self-Service Admin role is deprecated and replaced by the new Project Managerrole, which provides the same functionality.

### Resolved Issues

This v4 API release resolves the following critical issues: Namespace: VMM

| • | ENG-665011 Resolved an issue where the VM creation failed if you create a service account user, attachan API key to it, and add an access control policy (ACP) with the Prism admin role to it. |
| --- | --- |
| • | ENG-719499 Resolved an issue where the Update User API PUT /api/iam/v4.0.b3/authn/users/<extId>did not allow AD and other user types to update profile details. |

### Known Issues

You might encounter the following known issues in this or recent v4 API releases: Namespace: Data Protection

| • | ENG-697374 RBAC users who restore a Volume Group POST api/dataprotection/v4.0/config/recovery-points/{extId}/$actions/restore are not set as the owner of the restored Volume Group and areunable to access it. |
| --- | --- |

Workaround: Admin users can provide permission to RBAC users.

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

{% endraw %}
