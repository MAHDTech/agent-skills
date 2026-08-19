# Nutanix Objects Manual: Overview and Architecture

## NUTANIX OBJECTS OVERVIEW

Nutanix Objects provides a simple, scalable, and S3-compatible storage tier built on the Nutanix AOS platform.Nutanix Objects is a software-defined Object Store Service designed with an Amazon Web Services (AWS) Simple Storage Service (S3) compatible REST API interface, capable of handling petabytes of unstructured and machine-generated data. Nutanix Objects addresses storage-related use cases for backup, long-term retention, and data storage for your cloud-native applications by using standard S3 APIs. This service eliminates the need for an external, separately managed storage solution.Nutanix Objects is deployed and managed as part of the Nutanix Unified Storage. You can store and manage unstructured data on top of a highly scalable hyperconverged architecture. Using the on-premises model, you can manage object storage costs more effectively and have better visibility into where stored objects are located.You can deploy the Nutanix Objects instance on the following clusters:

- An existing cluster for small-scale object storage use cases• A dedicated cluster for high-performance workloads• Across multiple clusters to create a unified namespace that leverages underutilized storage resourcesNutanix Objects runs as a set of virtual machines that access storage across one or more clusters,

enabling seamless scalability from edge and remote office locations to core data centers. It also integrates with cloud storage, delivering a consistent data management experience across all deployment modes.You can manage objects by using Prism Central or the S3-compatible REST APIs after an administrator has authorized the applications and users to access buckets accordingly.For more information on Nutanix Objects architecture, see Solutions Documentation and Nutanix Bible

### Terminology Reference for Nutanix Objects

The following are the key terms related to Nutanix Objects:

**Table 1: Terminology Reference**

| Terminology | Description |
| --- | --- |
| Nutanix Objects™ | Nutanix Objects is deployed and managed as a part of Nutanix Unified Storage that delivers secure S3-compatible object storage at massive scale to hybrid cloud environments. |
| Bucket | An organizational unit exposed to the users and contains the objects. An object store deployment can have one or more buckets. |
| Object | Object represents the data that the user or application uploads. It refers to the actual unit of storage (blob) and interacts with the API using methods like GET or PUT. Objects | Nutanix Objects Overview | |

| Terminology | Description |
| --- | --- |
| S3 | The term S3 describes the Amazon Web Services (AWS) interface and is used interchangeably with object storage services. S3 also represents the object API that you use to interact with an object store. |
| Storage Network | A Virtual Local Area Network (VLAN) is required for communication between Nutanix Objects services. |
| Public Network | A VLAN that provides direct access to the object store endpoints from external sources. |
| Microservices Platform (MSP) | A platform based on Kubernetes where all the Nutanix Objects microservices run. |
| AHV IPAM | IP Address Management (IPAM) is a feature of AHV to assign IP addresses automatically to VMs by using DHCP. You can configure each virtual network with a specific IP address subnet, associated domain settings, and groups of IP address pools available for assignment to VMs. |
| Worker VMs | VMs that you create during the object store deployment to host various containerized Nutanix Objects services. You also refer to these worker VMs as Nutanix Objects nodes. |
| Nutanix Objects Browser | A user interface for the users to directly launch the object store instance in a web browser and perform bucket and object level operations. |

### Salient Features of Nutanix Objects

The salient features of Nutanix Objects are as follows: Write-Once-Read-Many You can create Write-Once-Read-Many (WORM) buckets using Nutanix Objects to prevent anyone, including administrators, from modifying or deleting data while the policy is active. WORM policies help ensure compliance with strict regulations often required by the healthcare, financial, and government sectors. Once applied, WORM policies prevent any updates to the data until the policy expires.Nutanix provides a 24-hour grace period to test the WORM policy. During this time, the bucket creator can undo the applied policy. After the grace period ends, no one can delete the policy or reduce the retention period, although there is an option to extend the retention period.To update an object in a WORM bucket, you must enable versioning. This ensures that the updated object is stored as a new version while retaining older versions, preventing any loss of data or overwriting.For more information, see WORM Bucket in Nutanix Objects on page 119. Immutability Immutability means that the data retrieved must be exactly the same as it was originally written. This feature ensures that you can access the stored object (along with its associated metadata) in the future without any alterations. If WORM is enabled, the object remains unchanged and is considered permanent. Objects | Nutanix Objects Overview |

Object Versioning When you enable object versioning, uploading multiple copies of the same object results in the creation of multiple versions. This feature protect your data from accidental overwrites or deletions and provides an option for reverting to a previous state. You can turn off object versioning on a bucket at any time and set the objects in that bucket to expire after a specified period. By default, there is no limit to the number of versions an object can have, as long as there is available storage space.For more information, see Object Versioning in Nutanix Objects on page 105. Data Life Cycle Management You use age-based data retention policies to enforce compliance with strict data retention regulations that dictate the time to store specific data. For example, the Health Insurance Portability and Accountability Act (HIPAA) regulation mandates you to retain medical data for six years from the creation date. You set a data retention policy to delete all data created in a specific bucket six years after its creation date. Note: The WORM policy supersedes any set retention policy you set. You can set retention policies at the bucket level where objects in a bucket expire after a certain amount of time you specify. If you do not set retention policies, the only limit to how many versions can be maintained depends on the storage space.For more information, see Lifecycle Policies in Nutanix Objects on page 105. Multipart Upload You can reduce slow upload times by breaking large pieces of data into chunks by using Multipart upload. Then, the system can handle the data separately to increase upload speeds if you upload them simultaneously. You can also use multipart uploads to prevent losing progress during an upload. For example, if there is a connectivity loss, most applications retry only the unsuccessful chunk. Hence, you do not have to upload the entire object again.For more information, see Nutanix Objects Supported APIs on page 250. Data-at-Rest Encryption with Native Key Management Nutanix Objects provides a FIPS 140-2 compliant data-at-rest encryption solution. To deliver this capability, Nutanix Objects uses the underlying AOS encryption capability. You can set encryption at an entire cluster level, always encrypting all data. With the native key management, the Nutanix cluster manages the keys, so the solution requires no additional device management or third-party costs. Identity and Access Management Native Identity and Access Management (IAM) functionality ensures that you have access only to the buckets and objects you created and granted access permissions. Each user gets a pair of access and secret keys that can be used by their applications to access Nutanix Objects. You can also generate access and secret keys for an Active Directory group. Administrators can revoke and regenerate keys at any time.For more information, see Directory Configuration and Access Key Generation on page 90. Multiprotocol Access You can create buckets using both S3 and NFS protocols in Nutanix Objects. NFS protocol support is natively implemented over Nutanix Objects and built on the same foundation that powers the S3 protocol.For more information, see the following references:

- Use Cases and Recommendations for Network File System on Nutanix Objects on page 14

Objects | Nutanix Objects Overview |

- Network File System and Simple Storage Service Interoperability on page 15• Limitations of Network File System on page 49• Creating and Configuring an NFS Bucket in Nutanix Objects on page 101• Creating and Configuring an S3 Bucket in Nutanix Objects on page 99

Cloud Tiering You can move objects to another S3-compatible object store bucket for saving storage space in the Nutanix Objects cluster with the Cloud Tiering feature. Tiering can help you to save costs by sending the infrequently accessed objects to platforms such as AWS S3. The supported endpoints are AWS S3, Azure Blob Storage, Google Cloud Platform (GCP) and a different Nutanix Objects instance.For more information, see Cloud Tiering in Nutanix Objects on page 109. Nutanix Objects Streaming Replication You can do automatic and asynchronous copying of source buckets to the target buckets in different Nutanix Objects instances with the Nutanix Objects streaming replication feature. This provides you with the ability to replicate a single source bucket to a maximum of three destination buckets. These destination buckets can be on Nutanix Objects on the same Prism Central cluster or on a remote Prism Central cluster, or a Cloud Storage Endpoint.For more information, see Nutanix Objects Streaming Replication on page 141. Server-Side Encryption with Customer-Provided Keys (SSE-C) You can encrypt specific objects at rest using encryption keys that you provide with each request by using Server-side encryption with customer-provided keys (SSE-C). Unlike the data-at-rest encryption with native key management, which encrypts all data at the cluster level using system- managed keys, SSE-C gives you full control over both the encryption keys and the objects you choose to encrypt. Nutanix Objects encrypts the data before storing it but does not retain the encryption key, ensuring that only you can decrypt the data upon retrieval. Note: SSE-C does not support Network File System, federation, replication, or S3 select. For more information, see Server-Side Encryption with Customer-Provided Keys Supported APIs on page 263.

### Nutanix Objects Use Cases

Following are examples of solutions you can implement by using Nutanix Objects:

| • | Backup and Archiving:Nutanix Objects integrates with backup applications like Commvault, HYCU, Veeam, and Veritas. |
| --- | --- |

You can create backups to protect your data with a simple, scalable, and cost-effective active archive solution. You can start with a small storage footprint and scale up to petabytes while maintaining high performance.Nutanix Objects supports the multipart upload API. You can perform faster uploads by breaking large files such as documents, images, and video into smaller chunks. These chunks are uploaded to a global namespace. Objects | Nutanix Objects Overview |

| • | Long-term Retention:You can use Nutanix Objects for long-term data retention. With its built-in object versioning, you can |
| --- | --- |

protect your storage and simplify data searches without the complexities associated with tape systems. Versioning maintains previous copies of the object to avoid data loss from overwrites or deletes.If you work in sectors such as healthcare, legal, or government, you might need to comply with strict regulations regarding data accessibility and modifications. Nutanix Objects offers secure long-term data storage to comply with these regulations. For example, you can use Write Once Read Many (WORM) buckets to preserve your data, ensuring that no one, including administrators, can modify or delete it while the policy is active.

| • | DevOps:You can access your data through a global namespace by using the PUT and GET commands. Nutanix |
| --- | --- |

Objects supports most programming environments, and you can access the namespace using standard internet networking protocols, such as HTTP and HTTPS. With Nutanix Objects, you can easily access objects using S3-compatible REST API requests from various locations. The bucket-level metrics help you track resource utilization. Furthermore, the Identity and Access Management (IAM) support ensures secure access to Nutanix Objects resources and services.DevOps and IT operations can use the S3-compatible interface for cross-geographical and cross-team collaboration, promoting agile development. Consider the following scenario:Joe and Adam are developers working together on an image-processing application called XYZ. Joe lives in New York and is responsible for coding the data creation and storage of XYZ, while Adam, residing in Seattle, focuses on testing the application. Several buckets for the XYZ application are stored in Nutanix Objects instance. After completing the programming for the latest feature, Joe uploads the new version of XYZ to a bucket. Adam can then use GET request automation scripts to pull the latest version of the code and store the test results in another bucket using PUT requests. This collaboration helps the team to work more efficiently and quickly. Also, it highlights the immutable capabilities of object storage, as data of Joe remains unchanged while Adam performs testing.

### Advantages of Nutanix Objects

Following are the advantages of object storage with Nutanix Objects: No Silos Nutanix Objects integrates with Files Storage and Nutanix Volumes as part of the Acropolis Distributed Storage Fabric (DSF). This integration allows block, file, and object storage to coexist in a unified environment, eliminating silos and simplifying deployment and management. Security-First Approach Security is embedded throughout the Nutanix stack. For example, the stack conforms to Security Technical Implementation Guides (STIGs), which maintain a security baseline configuration based on common standards established by the National Institute of Standards and Technology (NIST). STIGs use machine-readable code to automate self-healing and compliance with the security standards for AOS and AHV. Nutanix Objects also complies with SEA 17a-4 for data retention and accessibility and supports Data-at-Rest Encryption. Capacity Optimization Nutanix Objects uses compression and Nutanix Erasure Coding (EC-X) to optimize storage capacity. Inline EC is enabled by default on Nutanix Objects deployments, with a three-day post- process policy for overflow data.In addition to compression savings, EC-X increases usable storage without impacting write performance and provides resilience through parity blocks. For example, in a cluster with the redundancy factor 2, two copies of the data are replicated among all nodes for resilience. Checksums of the data are stored with the metadata to ensure validity if corruption occurs. Objects | Nutanix Objects Overview |

Therefore, the cluster (redundancy factor 2) uses half of its raw storage capacity to store copies of its data. EC-X performs the OR operation on these copies of data to compute a parity block. The original data blocks and the parity form an erasure code strip. This process reduces the number of actual data copies needed to protect the environment from a single node failure. If objects are being deleted regularly, it might be best to disable EC due to the garbage generated.For example, you have three data blocks (A, B, and C) stored in a four-node cluster, with each data block distributed across three nodes. Without EC-X, you might have the original three blocks of data plus three more copies of those blocks, which sum up to a total of six blocks of data. With EC-X enabled, you can reduce the storage footprint to four blocks of data that include the original three blocks of data plus their parity. These blocks are present in different nodes. If a node fails, the system can reconstruct the data from the data blocks and parity data present on the available nodes. For example, If the node hosting block B fails, the system can reconstruct its data from blocks A, C, and P (parity).Additionally, when the disks are not getting used heavily, the system regularly performs background checksum validations and data restoration to maximize protection against data corruption. These techniques (combined with a high tolerance for disk and node failures in a self-healing and distributed system) form a resilient and reliable enterprise object-storage platform.

### Note: Erasure Coding can only be enabled on clusters with four or more nodes (redundancy factor

2) or six or more nodes (redundancy factor 3). For more information on Erasure Coding best practices and requirements, see Prism Element Web Console Guide.For more information on fault tolerance and storage efficiency, see Fault Tolerance vs Storage Efficiency on page 12. Ease of Use and Simple Management Nutanix Objects supports HTTP S3 REST APIs for bucket and object operations, including GET, PUT, POST, DELETE, and LIST. It inherits the intuitive Prism Central interface for management, monitoring, and reporting capabilities.Nutanix Objects inherits the simplicity of use and management of Prism Central and provides the managing, monitoring, and reporting capabilities that are simple to administer.For example, Prism Central automatically checks for interoperability and compatibility during a software update task. It provides alerts to inform users about specific conditions, such as object storage capacity thresholds and reduced high availability. It also reports metrics like object store size, bucket and object count, throughput, latency, and GET and PUT operation counts. Cost With Nutanix Objects, the cost to store a large amount of unstructured data is less compared to traditional storage solutions. Compared to public cloud providers, Nutanix Objects does not charge for data ingress or egress.

### Fault Tolerance vs Storage Efficiency

In Nutanix Objects, there is a tradeoff between how many faults a cluster can tolerate (for example, the number of drives or the number of node failures) and how efficiently a cluster can store the data.For example, with Fault Tolerance 1 (FT1), an AOS cluster can tolerate the failure of one node or one disk. Without any capacity optimizations, such as compression or Erasure Coding, two copies of the data must be maintained (redundancy factor of 2), resulting in an overhead of 2x. In the case of Fault Tolerance 2 (FT2), the AOS cluster can tolerate the failure of two nodes or two disks. Similarly, without capacity optimizations, three copies of the data need to be maintained (redundancy factor of 3), leading to an overhead of 3x. Objects | Nutanix Objects Overview |

Both fault tolerance modes can be represented as FT1n:1d (Fault Tolerance of 1 node and 1 disk failure) and FT2n:2d (Fault Tolerance of 2 nodes and 2 disk failures), indicating how many node or disk failures the cluster can withstand.Currently, the fault tolerance (FT) guarantees apply equally to both node and disk failures, based on the assumption that these failures occur with the same likelihood. However, in dense node configurations -  typically those with 20 or more drives - there are significantly more disks in a cluster than there are nodes. As a result, the likelihood of experiencing multiple disk failures is greater than that of multiple node failures.FT1n:2d offers some capacity savings compared to FT2, even without any capacity optimizations. However, these savings significantly increase when Erasure Coding is enabled. In traditional FT2 mode, each node can only store one member of the Erasure Code Strip. In contrast, FT1n:2d allows for up to two members of the Erasure Code Strip to be stored on the same node, provided they are on different disks. This capability enables larger strips to be formed in a smaller cluster, thereby enhancing overall storage efficiency. Fault Tolerance Mode Comparison Without Erasure CodingWithout Erasure Coding (EC), the only savings achieved between FT2 and FT1n:2d comes from the reduction in reserved rebuild capacity that needs to be allocated. With this in mind, the following table illustrates the overhead associated with different fault tolerance schemes, assuming no capacity optimizations (i.e., no compression, deduplication, or erasure coding). The values under each fault tolerance scheme represent the ratio of physical capacity to available logical capacity for various cluster sizes, with lower values indicating better efficiency.In the case of FT1, as the cluster size increases, the ratio of physical to logical capacity approaches 2, which is the minimum if we are storing two copies of the data. Similarly, for FT2, as the cluster size grows, the ratio approaches 3, which is the minimum if we are storing three copies of the data.FT1n:2d helps us create a new mode between FT1 and FT2. This mode offers better Fault Tolerance than FT1 and has less overhead than FT2.

| Number of Nodes | (Physical Capacity / Available Logical Capacity) without EC |
| --- | --- |
| FT1n:1d (FT1) | FT1n:2d FT2n:2d (FT2) |
| 2.5x | 3.75x 5.0x |
| 2.4x | 3.6x 4.5x |
| 2.29x | 3.43x 4.0x |
| 2.18x | 3.33x 3.6x |
| 2.13x | 3.2x 3.42x |
| 2.06x | 3.10x 3.2x |

Fault Tolerance Mode Comparison with Erasure CodingAs noted above, with EC, one can form large strips on a smaller cluster with FT1n:2d. The following table summarizes the effective logical capacity that different FT schemes offer if all the data is erasure-coded. Note that these calculations assume that the largest strip sizes possible today are 4,1 and 4,2.One can see that for FT1, as the cluster size increases, the ratio of physical to logical capacity approaches 1.25x, which is the lower bound if we are forming strips of size 4,1. Similarly, one can see that for FT2, as the cluster size increases, the ratio of logical to physical capacity approaches 1.5x, which is the lower bound if we are forming strips of size 4,2.With FT1n:2d, it is clear that we are able to carve out a mode between FT1 and FT2, which offers better fault tolerance than FT2 with some additional storage overheads. These overheads are significantly lower than that offered by the FT2 scheme, especially for mid-sized clusters. Objects | Nutanix Objects Overview |

| Number of Nodes | Physical Capacity / Available Logical Capacity) with EC |
| --- | --- |
| FT1n:1d(FT1) | FT1n:2d FT2n:2d (FT2) |
| 1.67x | 1.88x 5.0x (no EC) |
| 1.5x | 1.8x 3.0x |
| 1.43x | 1.71x 2.0x |
| 1.36x | 1.63x 1.8x |
| 1.33x | 1.6x 1.71x |
| 1.29x | 1.55x 1.60x |

### Larger Erasure Coding Strips

Increased the Erasure Coding (EC) strip size for large clusters to improve storage efficiency.EC strips are represented as N/K, where N is the number of information or data egroups, and K is the number of parity blocks.The strip size of the EC can be configured as a container parameter for each container.

### Note: Larger EC strips are enabled in a cluster that has fault tolerance of one node or two disk failures

(FT1n:2d) enabled. FT1n:2d is enabled on a newly deployed object store when the following criteria are met:

- The cluster has a minimum of five nodes.• Fault tolerance (FT2) or a redundancy factor of 3 (RF3) is enabled on the cluster.• The AOS version must be 6.8 or higher.• Each node in the dense node platform hosts a minimum of 20 HDDs or SSDs.• There are no non-internal containers (default, management, objects-lite, and self-service) with FT2

present.

**Table 2: Wider EC Strip Configuration Requirements**

| Effective EC Strip Configuration | Node Requirement |
| --- | --- |
| 6/2 | 5 nodes |
| 8/2 | 6 nodes |
| 10/2 | 7 nodes |
| 12/2 | 8 nodes or more |

### Use Cases and Recommendations for Network File System on Nutanix

### Objects

Nutanix Objects provides S3 and NFS protocol access for multiprotocol workloads and supports several use cases and configuration recommendations. Objects | Nutanix Objects Overview |

### Use CasesNFS protocol support provides multiprotocol access (NFS 3.0 and Simple Storage Service (S3)) to data

stored within Nutanix Objects buckets. NFS protocol support is natively implemented over Nutanix Objects, built on the same foundation as the S3 protocol.NFS access over Nutanix Objects is ideally suited for large-scale, read-heavy workloads with sequential accesses where data is ingested once and minimally or never modified later.Following are some ideal use cases:

| • | Backup applications: Use Nutanix Objects as a large-scale NFS repository for backups while |
| --- | --- |

migrating the underlying storage to Nutanix Objects.

| • | Analytics: Use multiprotocol access in Nutanix Objects for in-place analytics by ingesting data through |
| --- | --- |

either the object or S3 interface and accessing it through file or NFS interfaces required by analytic systems.

### RecommendationsNutanix recommends the following guidelines for NFS access:

- Do not use NFS access over Nutanix Objects for use cases that require data modifications, such as

file edits and renames. For example, end-user computing, home shares, and application data that is frequently updated, including presentations or computer-aided design (CAD) files.

- Limit the number of subdirectories within a directory to 100, and maintain a maximum total of 100 million

objects within an NFS bucket.

- Use the nconnect=1 mount option for NFS buckets.

### Network File System and Simple Storage Service Interoperability

Nutanix Objects supports interoperability between the Simple Storage Service (S3) and Network File System (NFS) namespaces, enabling seamless data access across both protocols. Network File System to Simple Storage ServiceObjects in the NFS namespace is mapped to files and directories in the S3 namespace in the following ways:

- Files and symbolic links created in the NFS namespace appear as objects in the S3 namespace.• Directories created in NFS do not appear in the S3 namespace, as the S3 protocol lacks native

directory support.

- S3 operations like ObjectHead, ObjectGet, or ObjectDelete on directories fails with the error

message NfsDirectoryOperationNotAllowed. Therefore, files within nested directories in NFS appear as single objects in S3 namespace. Objects | Nutanix Objects Overview |

**Table 3: Example Mapping: Network File System to Simple Storage Service**

| NFS | S3 dir1/dir2/file |
| --- | --- |

- dir1/

- dir2/

- file

Simple Storage Service to Network File SystemObjects in the S3 namespace are mapped to files and directories in the NFS namespace in the following ways:

- Objects created through S3 appear as files and directories in NFS based on their naming structure.• If an object name includes a directory-like structure, it is stored in a hierarchical namespace as

identified by its name.Implicit directories are created when an object with a directory hierarchy is accessed from the NFS namespace.Folders created from the Nutanix Objects Browser also appear as a directory in the NFS namespace. The following table shows an example where an object a/b/c created from S3 protocol appears in the NFS namespace as a/directory, which contains the subdirectory b/ , which in turn contains the file c.

**Table 4: Example Mapping: Simple Storage Service to Network File System**

| S3 | NFS |
| --- | --- |

a/b/c

- a/

- b/

- c

For more information, see Limitations of Network File System on page 49.

### Nutanix Objects Workflow

Outlines the basic workflow of Nutanix Objects. Subsequent sections provide detailed information about each step involved.

### About this taskThe Nutanix Objects workflow is as follows:

### Procedure

1. Enable Nutanix Objects from the Prism Central web console. For more information, see Enabling Nutanix Objects on page 18. . Deploy the object store on your desired cluster. For more information, see Nutanix Objects Deployment on page 65. 3. Generate the access keys for aunthentication. For more information, see Generating Access Key for API Users on page 94. 4. Set up the Secure Sockets Layer (SSL) certificates for the object store. For more information, see Setting up SSL Certificate for an Object Store on page 84. 5. Access the object store endpoints using third-party clients or the Nutanix Objects Browser. For more information, see Access Nutanix Objects Endpoints on page 74 and Nutanix Objects Browser on page 206. 6. Create buckets using either the Simple Storage Service (S3) or Network File System (NFS) protocol. For more information, see Bucket Creation, Operations, and Bucket Policy Configuration on page 99 and Bucket Operations in Nutanix Objects Browser on page 209. 7. Upload objects and perform object operations using the Nutanix Objects Browser or S3 APIs. For more information, see Supported Operations in Nutanix Objects Browser on page 209 and Nutanix Objects Supported APIs on page 250. 8. Expand the object store if the storage is approaching the limit. For more information, see Expanding Storage for an Object Store on page 78. Objects | Nutanix Objects Overview |

## ENABLING NUTANIX OBJECTS

To create an object store, enable Nutanix Objects from the Prism Central web console and add a license in Prism Element or Prism Central instance, based on the version used.

### About this taskEnabling Nutanix Objects in Prism Central web console is a one-time operation. Only Super Admins or

Prism Admins can perform this action. Non-admin users (Prism Central users without admin privileges) can enable Nutanix Objects only if granted a role with the necessary permissions by an admin. Users with the Nutanix Objects Admin role do not have permission to enable the service.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.After enabling Nutanix Objects, ensure you run an LCM inventory and upgrade both the MSP and Nutanix Objects Manager to the latest versions before proceeding with deployment. For more information, see Nutanix Objects Life Cycle Manager Upgrades on page 26.To enable Nutanix Objects in Prism Central, follow these steps:

### Procedure

1. Log on to Prism Central web console as an administrator.2. From the Application Switcher Function, click Admin Center.3. In the navigation bar, click Marketplace.4. In the Nutanix Apps section, click Get for the Objects application. The application details page appears. 5. Click Deploy. Note: Deployment process takes around 4–5 minutes to complete. 6. From the Application Switcher Function, click Objects. A welcome page appears with prerequisite details to create an object store. 7. Click Download Creation Checklist. A list of prerequisites for deploying an object store is downloaded. 8. (Only for ESXi clusters) To deploy the object store on an ESXi cluster, click vCenter Registration. Ensure to provide the vCenter credentials and configure the IPAM for the ESXi networks. For more information, see Managing vCenter for Nutanix Objects Service on page 46.

### Note: A non-admin user can register vCenter only after an admin assigns a role with these minimum

| permissions: | Create Object Store and View Object Store. Admins are Super Admins or Prism |
| --- | --- |

Admins; non-admins lack admin privileges. For more information on the built-in roles in Prism Central,

| see | Built-in Role Management in the Security Guide. |
| --- | --- |

9. Click Next.10. Click Create Object Store. For more information, see Creating or Deploying an Object Store on Prism Central on page 67.Nutanix Objects is enabled and ready for object store deployment. Objects | Enabling Nutanix Objects |

### What to do nextAfter deployment, the Nutanix Objects application appears on the

### My Apps page. You can also select

Objects from the Application Switcher to access and use the application. Objects | Enabling Nutanix Objects |

## VIEWING NUTANIX OBJECTS VERSION

You can view the installed versions of Nutanix Objects Manager and Nutanix Objects Service from the Prism Central web console.

### About this taskTo view the Nutanix Objects version, follow these steps:

### Procedure

1. Log on to Prism Central web console.2. From the Application Switcher Function, click Admin Center > LCM > Inventory.3. Click Perform Inventory.

| The | Installed Versions list displays the version of Nutanix Objects Manager and Nutanix Objects |
| --- | --- |

Service. Objects | Viewing Nutanix Objects Version |

## NUTANIX OBJECTS SECURITY

## HARDENING INSTRUCTIONS

Outlines the guidelines for enhancing the security of Nutanix Objects.Before performing Nutanix Objects hardening configurations, ensure that Prism Central and Prism Element are hardened.For more information, see Security Management Using Prism Element and Security Management Using Prism Central in the Nutanix Security Guide.

### Networking GuidelinesThe following are the instructions for hardening the network for Nutanix Objects:

- Ensure that only the Nutanix Objects Simple Storage Service (S3) endpoint is accessible to the users.• Establish a firewall to protect access to Prism Central and Prism Element, ensuring it is not open to end

users.

- The Nutanix Objects storage network must only be accessible to Prism Element and Prism Central, and

must be secured with a firewall.

### Note: Use the Nutanix Objects storage network only for deploying Nutanix Objects. Do not use this

network for any other workloads or applications.

- For the Nutanix Objects replication, only open the required ports between the source and destination

Nutanix Objects cluster IP addresses. All other access must remain protected. Firewall RulesFirewall rules must be configured to protect the infrastructure of Nutanix Objects.For more information, see URL and Port Requirements.The following is a list of networks:

- Nutanix Objects Storage Network• Nutanix Objects Public Network• Internet connection• Other services• Prism Central• Prism Element

Flow Networking RulesWhen Flow Networking is enabled in a setup, rules can be configured as security policies (application policy) either manually or automatically, instead of relying on an external firewall.For more information, see Protecting Nutanix Objects Network with Flow Networking (Microsegmentation) on page 23.If Flow Networking is not enabled, the same rules can be applied by using an external firewall.Inbound traffic allowlist: Objects | Nutanix Objects Security Hardening Instructions |

- Controller Virtual Machine (CVM) IP addresses (including secondary/segmented addresses)• Prism Central IP address• Pod network range, typically 10.100.0.0/16• All IP addresses, such as 0.0.0.0/0 for ports TCP 80 and 443.

Nutanix recommends not to allow port 80. Outbound traffic allowlist:

- CVM IP addresses (including secondary/segmented)• Prism Central IP address• Pod network range, usually 10.100.0.0/16• All IP addresses such as 0.0.0.0/0 for port TCP ports 53, 7100, 5553, and UDP ports 53, 123

### Nutanix Objects Bucket SettingsThe following are the guidelines to enhance security for Nutanix Objects buckets:

- Disable HTTP Access:

Configure the network firewall to disable Hypertext Transfer Protocol (HTTP) access to the Nutanix Objects S3 endpoint. This ensures that access is restricted to HTTPS only.

- Enable Write-Once-Read-Many (WORM) for buckets containing sensitive data.

This feature prevents anyone, including administrators, from modifying or deleting data while the policy is active. For more information, see WORM Bucket.

- Enable Nutanix Objects versioning:

Activate object versioning to create new versions whenever new data is uploaded to the same object. For more information, see Creating and Configuring an S3 Bucket in Nutanix Objects on page 99.To manage storage consumption, set up a lifecycle policy to automatically expire old versions and markers after a set interval. For more information, see Rules for Expiration based Lifecycle Policy on page 107.

- Enable Data-at-Rest Encryption with Native Key Management:

Nutanix Objects provides a FIPS 140-2 compliant data-at-rest encryption solution. To deliver this capability, Nutanix Objects uses the underlying AOS encryption capabilities to set encryption at the entire cluster level to ensure all data is always encrypted. For more information, see Data-at-Rest Encryption in the Nutanix Security Guide.

- Enable Identity and Access Management:

Review and delete any unused or extra access keys for buckets.

- Configure replication:

Set up replication for buckets to maintain at least one backup copy. For more information, see Creating Replication Rules between Buckets.

- Set up Secure Sockets Layer (SSL) Certificate:

Upload a trusted Certificate Authority certificate to the Nutanix Objects cluster to ensure secure communication between the Nutanix Objects S3 endpoint and end users. For more information, see Setting up SSL Certificate for an Object Store. Objects | Nutanix Objects Security Hardening Instructions |

- User Access Management:

In Prism Central, the Nutanix Objects user and Prism Central admin users possess root or administrative privileges, allowing them to perform all operations on the buckets and generate new API keys.

- Use role-based access control (RBAC) in Prism Central to configure and customize user access

based on their assigned roles for object entities.

- Set up RBAC policies to grant Nutanix Objects access to Prism Central users, and use these

credentials to perform operations on Nutanix Objects from Prism Central. For more information, see Role-based Access Control for Nutanix Objects.

- Configure notifications:

Set up notifications to send completed event logs to the configured endpoints within your Nutanix Objects instance. For more information, see Configuring Events Notification.

- Credential Security:

Ensure that tiering endpoint credentials are protected and never shared publicly.

### Remember: Never save your credentials in plain text form on your computer. Securely store your

credentials in encrypted form by using credential storage applications.

### Protecting Nutanix Objects Network with Flow Networking

### (Microsegmentation)

Outlines the steps to protect Nutanix Objects networks with Flow Networking.

### About this taskTo protect the Nutanix Objects Network with Flow Networking, follow these steps:

### Procedure

1. Enable Flow Networking microsegmentation. For more information, see Enabling Microsegmentation in the Flow Networking Microsegmentation Guide. 2. Update the AppType category to Objects_security. For more information, see Modifying a Category in the Prism Self Service Administration Guide. 3. Assign the updated category to Nutanix Objects cluster VMs. For more information, see Assigning a Category in the Prism Self Service Administration Guide. . Create an application security policy for AppType:Objects_security.

| Choose a relevant name, such as Objects_security_policy, and include the following data for | Inbounds |
| --- | --- |
| and | Outbounds traffic. For more information, see Application Security Policy Configuration in the Flow |

Networking Microsegmentation Guide.Inbound traffic allowlist:

- CVM IP addresses (including secondary/segmented)• Prism Central IP address• Pod network range, typically 10.100.0.0/16• All IP addresses such as 0.0.0.0/0 for port TCP 443.Outbound traffic allowlist:

- CVM IP addresses (including secondary/segmented)• Prism Central IP address• Pod network range, typically 10.100.0.0/16• All IP addresses, such as 0.0.0.0/0 for TCP ports 53, 7100, 5553, and UDP ports 53, 123

Objects | Nutanix Objects Security Hardening Instructions |

## NUTANIX OBJECTS PRISM CENTRAL

## DISASTER RECOVERY

Nutanix Objects includes built-in disaster recovery capabilities to recover automatically without any manual intervention.This capability improves resilience and reduce manual intervention during Prism Central disaster recovery (PCDR). This topic is applicable for Prism Central version pc.7.3 or later.Nutanix Objects Manager performs the following self-recovery operations after a PCDR event is triggered:1. The Nutanix Objects Manager automatically runs a series of system prechecks.

### Note: The Nutanix Objects user interface access is temporarily disabled during the precheck and

recovery stages, with restoration expected in under an hour.

### 2. If the prechecks pass, the system automatically recovers without manual intervention. If the prechecks

fail, the system does not trigger automatic recovery, and manual support is required.If prechecks fail, contact Nutanix Support to restore Nutanix Objects functionality.

### 3. The Nutanix Objects user interface reflects the current recovery status, including any failure

notifications.

### Caution: Data remains secure; however, users might intermittently lose access during recovery progress or

failure. Objects | Nutanix Objects Prism Central Disaster Recovery |

## NUTANIX OBJECTS LIFE CYCLE

## MANAGER UPGRADES

You can upgrade Nutanix Objects by using the Life Cycle Manager (LCM) feature in Prism Central.As part of the Prism Central upgrades module, LCM handles updates for key components including

### Nutanix Objects Manager and Nutanix Objects Service.Upgrades occur in the following order:1. Prism Central

### 2. Prism Element

3. MSP Controller. For more information, see Microservices Platform on page 27.

### 4. Nutanix Objects Manager

5. Nutanix Objects ServiceIf Nutanix Objects is not enabled, upgrading the Prism Central instance automatically upgrades the Nutanix Objects Manager. However, if Nutanix Objects is enabled, Nutanix Objects Manager must be upgraded manually.For more information, see Performing Firmware and Software Updates in a Connected Site Setup from Prism Central in the Life Cycle Manager Guide. Nutanix Objects ManagerNutanix Objects Manager is a containerized service that runs on a Prism Central VM. It handles user input for object store deployment, validates inputs, manages certificates, deploys Nutanix Objects Service, and acts as an interface between Prism Central and object store backups.A single Nutanix Objects Manager can manage one or more object stores. In a scale-out Prism Central VM, the service runs on each Prism Central node to provide high availability.You can upgrade the Nutanix Objects Manager after MSP is upgraded. When multiple versions of Nutanix Objects Manager are available, always select the latest version. You can upgrade MSP, Nutanix Objects Manager, and Nutanix Objects Service together. When multiple upgrade options are selected, the upgrade happens serially, not in parallel.

### Note:

- LCM disables the Nutanix Objects Manager upgrade if there is any unresolved Nutanix Objects

Service upgrade failure.

- During Nutanix Objects Manager upgrade, input and output operations continue without

disruption. However, the user interface becomes temporarily unavailable for statistics and management.

- If you encounter an LCM upgrade failure, contact Nutanix Support. Backup snapshots for

recovery are retained for a limited duration (30 days). For more information, see Performing Firmware and Software Updates in a Connected Site Setup from Prism Central in the Life Cycle Manager Guide. Nutanix Objects ServiceNutanix Objects Service provides the object store interface and handles storing and retrieving objects. It stores objects and metadata on selected Prism Element clusters. The service components are containerized and run on the Kubernetes platform, with each instance offering a single global namespace.During deployment, the system creates the required VMs to run Kubernetes pods and the load balancer. Objects | Nutanix Objects Life Cycle Manager Upgrades |

You can upgrade the Nutanix Objects Service after upgrading the Nutanix Objects Manager. Multiple Nutanix Objects Service instances appear separately in the upgrade list. You can upgrade them individually or together. You can upgrade MSP, Nutanix Objects Manager, and Nutanix Objects Service together. When multiple upgrade options are selected, the upgrade happens serially, not in parallel.

### Note:

- The Nutanix Objects Service becomes available for upgrade only after deployment.• During Nutanix Objects Service upgrade, input and output operations may experience a brief

disruption of a few minutes as internal services upgrade. The overall upgrade process takes around 15 to 30 minutes. For more information, see Performing Firmware and Software Updates in a Connected Site Setup from Prism Central in the Life Cycle Manager Guide.

### Installing Nutanix Objects (First-time Users)

You can install Nutanix Objects by upgrading Prism Central through Life Cycle Manager (LCM), enabling Nutanix Objects, and updating to the latest version through LCM.

### About this taskTo install and update Nutanix Objects in your environment, follow these steps:

### Procedure

### 1. Perform an inventory and upgrade to a compatible version of Prism Central by using the LCM feature in

Prism Central.For more information, see Performing Inventory with the Life Cycle Manager section in the Life Cycle Manager Guide. 2. Enable Nutanix Objects. For more information, see Enabling Nutanix Objects on page 18. 3. In LCM, perform inventory and update Nutanix Objects to the latest available version. For more information, see Nutanix Objects Life Cycle Manager Upgrades on page 26.The latest Nutanix Objects version is now installed.

### Microservices Platform

Microservices Platform (MSP) is a Kubernetes-based platform that runs all Nutanix Objects microservices.The MSP Controller, a service on the Prism Central VM, manages the MSP lifecycle. You must upgrade the MSP Controller before upgrading Nutanix Objects Manager and Nutanix Objects Service. The first object- store cluster deployed on Prism Central acts as the primary MSP or primary object store. All other clusters are secondary MSP clusters or secondary object stores.For Nutanix Objects deployments, always upgrade to the latest MSP Controller version. Upgrade MSP from 1.0.8 to 2.2.1 version to make Nutanix Objects visible for upgrade in dark site mode. For more information, see Performing Inventory with the Life Cycle Manager section in the Life Cycle Manager Guide.

### Note: The primary object store cluster hosts all the common services, such as Identity and Access

Management (IAM). You cannot delete the primary cluster without first deleting all secondary clusters. This note is applicable only when the IAM-high availability feature is not enabled. For more information, see the Nutanix Objects IAM-High Availability Overview on page 90 section. Objects | Nutanix Objects Life Cycle Manager Upgrades |

### Finding the Microservices Platform Controller Version

Find the Microservices Platform (MSP) Controller version by using the command line interface.

### About this taskTo find the MSP Controller version, follow these steps:

### Procedure

### 1. Log in to Prism Central VM.2. Find the MSP Controller version:

admin@pcvm$ mspctl controller version

### Note: If the mspctl controller version command does not work, use mspctlcontroller_version

command since you might be using an older version of MSP (2.2.0 or earlier). The system displays the MSP Controller version.

### Finding the Primary and Secondary Microservices Platform Clusters

Find the primary and secondary Microservices Platform (MSP) clusters by using the command line interface (CLI).

### About this taskTo find the primary and secondary MSP clusters, follow these steps:

### Procedure

### 1. Log on to Prism Central VM.2. List the clusters:

admin@pcvm$ mspctl cluster list If the cluster_type value is primary_msp, the cluster is a primary MSP cluster. All other clusters are secondary.

### Note: If the mspctl cluster list command does not work, use mspctl cluster_list

command since you might be using an older version of MSP (2.2.0 or earlier). A list of cluster appears. Objects | Nutanix Objects Life Cycle Manager Upgrades |

## ROLE-BASED ACCESS CONTROL FOR

## NUTANIX OBJECTS

Prism Central supports role-based access control (RBAC) that allows you to configure and provide customized access to the users based on their assigned roles for the objects entities.The access policies and permissions are user-based and apply only to the actions performed on the Prism Central web console. These policies and permissions do not apply to the actions performed on Nutanix Objects Browser or any S3-compatible client application, even if the same users have Prism Central access and Nutanix Objects Browser access.From the Prism Central roles dashboard, you can define and assign the following additional roles to users or user groups:

- A set of predefined built-in roles (system roles)• Additional custom rolesNutanix Objects support the following built-in roles (system roles) that are defined by default:

- Nutanix Objects Admin - permissions required to create and manage object stores.• Nutanix Objects Viewer - permissions required to perform read-only operations on object stores.

### Note: For more information on Role Based Access Control, see the following references:

- Controlling User Access (RBAC) in the Security Guide.• Built-in Role Management in the Security Guide• Operations Management in the Prism Central Guide.

### Prerequisites for Configuring Role-based Access Control for Nutanix

### Objects

Make sure that your system meets the following requirements before you start configuring role-based access control for Nutanix Objects.

- Prism Central version must be pc.2021.9 or later.• Nutanix Objects Manager version must be 3.4 or later.• Nutanix Objects Service version must be 3.4 or later (recommended version).• Microservices Infrastructure must be enabled in Prism Central.

For information on enabling Microservices Infrastructure, see Enabling Microservices Infrastructure Manually in the Prism Central Infrastructure Guide. For more information on upgrading Nutanix Objects Manager and Nutanix Objects Service, see Nutanix Objects Life Cycle Manager Upgrades on page 26. Only administrators (Super Admin or a Prism Admin in Prism Central) can create roles and access control policies for objects.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Nutanix Security Guide. Objects | Role-based Access Control for Nutanix Objects |

### Note: To view the Prism Central version, log on to the Prism Central web console. In the Application

| Switcher, click | Infrastructure. In the left navigation bar, select Prism Central Settings > Prism Central |
| --- | --- |

Management. The Prism Central version is listed in the Prism Central Summary section.

### Role-Based Access Control Workflows for Nutanix Objects

Role-based access control (RBAC) allows users to configure and provide different levels of access to the users based on the role of the user within an organization.You can create the following roles and assign them to the users based on your requirements:

- Full administrator• View-only administrator• Buckets administrator• Infra administrator• Custom

### Note: For more information on role-based access control, see the following references:

- Built-in Role Management in the Nutanix Security Guide.• Controlling User Access (RBAC) in the Nutanix Security Guide.

### Nutanix Objects Permissions

Administrators can use the Prism Central RBAC feature to create access control policies and attach them to Nutanix Objects users.A non-admin user can perform object-related tasks only after an administrator creates an access control policy on a role in Prism Central for the non-admin user. The administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.

### Note: For more information on the built-in roles in Prism Central, see Built-in Role Management in the

Nutanix Security Guide The following table lists the permissions that an administrator can grant to a non-admin user to perform various objects tasks. These permissions allow the users to perform the following tasks:

**Table 5: Nutanix Objects Permissions**

| Permission | Tasks Users Can Perform |
| --- | --- |
| Add Multicluster | Add Prism Element clusters to expand the storage. |
| Create Endpoint | Create remote endpoints for tiering. |
| Create Object Store | Create object stores. |
| Create Quota Policy | Create quota policies for objects. |
| Delete Object Store | Remove object stores. Objects | Role-based Access Control for Nutanix Objects | |

| Permission | Tasks Users Can Perform |
| --- | --- |
| Delete Quota Policy | Remove the quota policies created by the respective user for objects. |
| Download Certificate | Download the CA-signed certificates for an object store. |
| Edit Buckets | Edit the buckets in an object store. |
| Regenerate Self Signed Certificate | Regenerate self-signed certificates for an object store. |
| Replace Certificates | Replace certificates for an object store. |
| Scale Out | Add more resources to an existing object store cluster. |
| Set Capacity | Add additional storage capacity to an existing object store cluster. |
| Set Notification Endpoint | Add the notification endpoints. |
| Update Endpoint | Update the existing tiering endpoints. |
| Update Multicluster | Remove or modify the existing expanded storage (Prism Element cluster). |
| Update Quota Policy | Update the quota policies. |
| View Buckets | View the buckets in an object store. |
| View Endpoint | View the existing tiering endpoints. |
| View Multicluster | View the Prism Element clusters added for expanded storage. |
| View Notification Endpoint | View the notification endpoints. |
| View Object Store | View the object stores. |
| View Quota Policy | View the quota policies. |

### Creating Full Administrator Role for Nutanix Objects

You can create a full access administrator policy for users or user groups and grant full access rights to all operations on Nutanix Objects instances using role-based access control (RBAC) from the Prism Central web console.

### About this task

### Note:

- Only users with administrator privileges create a full access policy for Nutanix Objects

instances.

- It might take some time (around five minutes or so) for the changes made to the policy to be

effective. To create a full access policy for Nutanix Objects instances, follow these steps:

### Procedure

1. Log on to the Prism Central web console as an Admin user. . In the Application Switcher, select Admin Center > IAM > Roles.3. Select the Nutanix Objects Admin role, and then click Manage Assignment.

| The | Role Assignment page appears. |
| --- | --- |

4. Click Add New to create a new role.5. From the Select Users or Groups section, select the required Active Directory group from the pull-

| down list, and then select one or more users or user groups using the | Search User field. |
| --- | --- |

Note: You can only add users who are part of an Active Directory group.

### 6. From the Entities section, select object store as the Entity Type, and then select the following:

- Individual Entity• Select

### All Nutanix Objects or enter the name of the Nutanix Objects instance on which you need to

grant custom permissions for the user. 7. Click Save. An administrator user with full access is created.

### Creating Bucket Administrator Role for Nutanix Objects

You can create a bucket administrator policy to manage all bucket activities on Nutanix Objects instances using role-based access control (RBAC) from the Prism Central web console.

### About this task

Note: Users with only administrator privileges can do this task. To create a buckets admin policy, follow these steps:

### Procedure

1. Log on to the Prism Central web console as an Admin user.2. In the Application Switcher, select Admin Center > IAM > Roles.3. Click Create Role.

| The | Roles page appears. |
| --- | --- |

### 4. Enter the following details on the Roles page:

| a. | Role Name: Enter a name for the role. |
| --- | --- |
| b. | Description: Enter a brief description of the role. |

5. From the Object Store section, click Change next to the Set Custom Permissions option.

| The | Custom Object Store Permissions window appears. |
| --- | --- |
| (Optional) You can also enter Object Store in | Filter Entities to go to the object store section. Objects | Role-based Access Control for Nutanix Objects | |

### 6. Select the following permissions from the list and click Save:

- Edit Buckets• View Buckets• View Object Store

- When you select certain permissions, other additional permissions might automatically get added

to the permission list. Read the on-screen instructions to understand the additional permissions that are automatically granted to the new custom role.

- Users with Edit Bucket permission can perform bucket operations, such as create, update, and

delete on the buckets available in the object stores that are assigned to the respective users. 7. Click the newly created role, and click Manage Assignment.

| The | Role Assignment page appears. |
| --- | --- |

8. Click Add New. You can assign the role to more users or user groups.

### 9. From the Select Users or Groups section, select the Windows AD group, and then select the user or

| user groups using the | Search User field. |
| --- | --- |

- You can add only those users who are part of an Active Directory group.• When permission is granted to a role, the specific permission is not always inherited by the access

control policy that is created for that role. This role needs to be assigned to a specific Nutanix Objects instance.

### 10. From the Entities section, select object store as the Entity Type, and then select the following:

- Individual Entity• Select All Nutanix Objects or enter the name of the Nutanix Objects instance on which you need to

grant custom permissions for the user. 11. Click Save. Some bucket actions, such as setting a lifecycle policy for tiering, require additional permissions. For more information on the required permissions, see Tiering.The Buckets Admin access control policy is saved and listed on the Roles page. The user now has access to perform bucket management tasks (create, update, and delete buckets) on all buckets in that Nutanix Objects instance.

### Creating Infra Administrator Role for Nutanix Objects

An administrator can provide access to users or to user groups to manage all the existing Nutanix Objects instances without providing any write access to buckets on the objects instances from the Prism Central web console.

### About this task

Note: Users with only administrator privileges can perform this task. To create an infra administrator access control policy, follow these steps:

### Procedure

1. Log on to the Prism Central web console as an Admin user. . In the Application Switcher, select Admin Center > IAM > Roles.3. Click Create Role.

| The | Roles page appears. |
| --- | --- |

### 4. Enter the following details on the Roles page:

| a. | Role Name: Enter a name for the role. |
| --- | --- |
| b. | Description: Enter a brief description of the role. |

5. From the object Store section, select Set Custom Permissions and click Change.

| The | Custom Object Store Permissions window appears. |
| --- | --- |
| (Optional) You can also enter Object Store in | Filter Entities to go to the Object Store section. |

### 6. Select the following permissions from the list and click Save:

Add Multicluster, Create Quota Policy, Delete Quota Policy, Download Certificate, Regenerate Self Signed Certificate, Replace Certificates, Scale Out, Set Capacity, Set Notification Endpoint, Update Multicluster, Update Quota Policy, View Buckets, Edit Buckets, View Multicluster, View Notification Endpoint, View Object Store, and View Quota Policy.When you select certain permissions, other additional permissions might automatically get added to the permission list. Read the on-screen instructions to understand the additional permissions that are automatically granted to the new custom role. 7. Click the newly created role, and click Manage Assignment.

| The | Role Assignment page appears. |
| --- | --- |

8. Click Add New to assign the role to users or user groups.9. From the Select Users or Groups section, select the Windows AD group, and then select the users

| or user groups using the | Search User field. |
| --- | --- |

- You can add only those users who are part of an Active Directory group.• When permission is granted to a role, the specific permission is not always inherited by the access

control policy that is created with that role. This role needs to be assigned to a specific Nutanix Objects instance.

### 10. From the Entities section, select object store as the Entity Type, and then select the following:

- Individual Entity• Select

### All Nutanix Objects or enter the name of the objects instance on which you need to grant

custom permissions for the user. When permission is granted to a role, the specific permission is not always inherited from the access control policy that is created with that role. This role needs to be assigned to a specific Nutanix Objects instance. 11. Click Save.

| The Infra Administration access control policy is saved and listed on the | Roles page. The user now |
| --- | --- |

has access to manage existing Nutanix Objects instances. However, the user cannot perform any bucket update operations, create any objects instances, or delete any objects instances.

### Creating a Custom Role for Nutanix Objects

An administrator can create custom roles and provide access to users or user groups from the Prism Central web console. Objects | Role-based Access Control for Nutanix Objects |

### About this taskTo create a custom access control policy (custom role), follow these steps:

### Procedure

1. Log on to the Prism Central web console as an Admin user. Only users with administrator privileges can do this task.For information on creating a custom role, see Creating a Custom Role in the Security Guide. 2. In the Application Switcher, select Admin Center > IAM > Roles.3. Click Create Role.

| The | Roles page appears. |
| --- | --- |

### 4. Enter the following details on the Roles page:

| a. | Role Name: Enter a name for the role. |
| --- | --- |
| b. | Description: Enter a brief description of the role. |

5. From the Object Store section, select Set Custom Permissions and click Change.

| The | Custom Object Store Permissions window appears. |
| --- | --- |
| You can also enter Object Store in | Filter Entities to go to the Object Store section. |

6. Select the required permissions and click Save. When you select certain permissions, other additional permissions might automatically get added to the permission list. Read the on-screen instructions to understand the additional permissions that are automatically granted to the new custom role. 7. Select the newly created role, and then click Manage Assignment.

| The | Role Assignment page appears. |
| --- | --- |

8. Click Add New to assign the role to users or user groups.9. From the Select Users or Groups section, select the required Active Directory group from the pull-

| down list, and then select one or more users or user groups using the | Search User field. |
| --- | --- |

- You can add only those users who are part of an Active Directory group.• When permission is granted to a role, the specific permission is not always inherited by the access

control policy that is created with that role. This role needs to be assigned to a specific Nutanix Objects instance.

### 10. From the Entities section, select object store as the Entity Type, and then select the following:

| • | Individual Entity |
| --- | --- |
| • Select | All Nutanix Objects or enter the name of the Nutanix Objects instance on which you need |

to grant custom permissions for the user. When permission is granted to a role, the specific permission is not always inherited from the access control policy that is created with that role. This role needs to be assigned to a specific Nutanix Objects instance. 11. Click Save.

| The custom access control policy is saved and listed in the | Roles page. The user now has access to |
| --- | --- |

manage existing Nutanix Objects instances granted to him in the policy. Objects | Role-based Access Control for Nutanix Objects |
