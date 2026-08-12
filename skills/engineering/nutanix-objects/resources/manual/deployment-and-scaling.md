# Nutanix Objects Manual: Deployment and Storage Scaling

## NUTANIX OBJECTS DEPLOYMENT

To start using Nutanix Objects, you need to deploy the object store service.Nutanix Objects is a highly available and distributed object store service which can store petabytes of data. For more information, see Creating or Deploying an Object Store on Prism Central on page 67 or Deploying Object Store on Nutanix Cloud Clusters on page 85. You can also perform offline deployment (site without Internet access) of an object store. For more information, see Deploying Object Store at a Dark Site (Offline Deployment) on page 86.You can deploy multiple Nutanix Objects instance on a single Prism Element registered to a Prism Central, or you can register multiple Prism Elements to one Prism Central and deploy as many object stores on each of these Prism Elements provided you have sufficient storage.You can do the following operations after deploying Nutanix Objects instance:1. Configure the directory and generate the access key. For more information, see Directory Configuration and Access Key Generation on page 90.

### 2. Create buckets within the object store. For more information, see Creating and Configuring an S3

Bucket in Nutanix Objects on page 99

### 3. Upload objects and meta-data to the buckets by using the Simple Storage Service (S3) APIs. For more

information, see Nutanix Objects Supported APIs on page 250.

### Note:

- After registering Prism Element to Prism Central, wait for 10 minutes before starting the

deployment.

- Parallel object store deployments are not supported.• Nutanix Objects deployment is not supported on dual-stack IPv6-enabled Prism Central

instances.

- If your deployment fails due to precheck failures, you can resume the deployment after fixing

the configuration.

- For objects containers, the Erasure Coding delay will be reduced from 7 days to 3 days for old

and new deployments.

### Object Store and Domain Naming Conventions

Established guidelines for names of object stores and domains.

### Object Store Naming GuidelinesThe name of an object store must conform to the following rules:

- Be unique across all existing object store names in Nutanix Objects.• Begin with a letter, and end with a letter or number.• Can contain alphanumeric or hyphen characters.• Not contain any special character other than a hyphen.• Minimum of 1 and a maximum of 16 characters long.

Note: You cannot change the name after creating the object store. Objects | Nutanix Objects Deployment |

### Domain Naming GuidelinesThe name of the domain must conform to the following rules:

- Must include at least one period.• Can consist of alphanumeric characters, hyphens, or underscores.• Cannot start or end with a hyphen, underscore, or period.

### Prerequisites for Enabling FT1n:2d on an Object Store

Fault tolerance for 1 node or 2 disk failures (FT1n:2d) is enabled on a newly deployed object store when you meet the following criteria:.

- The cluster must consist of a minimum of five nodes.• The fault tolerance/redundancy factor 3 must be enabled on the cluster.• The AOS version must be 6.8 or later.• Each node in the dense node platform must host a minimum of 20 HDDs.• No non-internal (default, management, and self-service) FT2 containers must be present.• When adding another cluster using the multicluster option in an FT1n:2d cluster with redundancy

factor 3 enabled, ensure that the new cluster also has redundancy factor 3 enabled. Attempts to add a redundancy factor 2 cluster will be automatically rejected. For more information on fault tolerance, see Fault Tolerance vs Storage Efficiency on page 12.

### Prerequisites for Nutanix Objects Network Segmentation of an Object

### Store

Nutanix Objects Network segmentation is enabled on an object store when you meet the following criteria:

- The AOS version must be 7.3 or later.• The Prism Central version must be pc.7.3 or later.• The Prism Element cluster must consist of a minimum of three nodes.• Ensure that Nutanix Volumes network segmentation is enabled on the Prism Element cluster.

For more information, see Isolating Service-Specific Traffic and Nutanix Volumes in the Nutanix Security Guide.

- Ensure that the Nutanix Volumes network segmentation is configured on a different VLAN than Prism

Central to maintain a stable connection between Prism Central and Prism Element.

- When enabling network segmentation for the Nutanix Volumes service on a Prism Element cluster that

already has Nutanix Object Stores deployed (without network segmentation), make sure that the subnet IP address used for the Nutanix Volumes service network segmentation is different from the subnet IP address of any of the AHV subnets that are used as the storage network for any of the existing object stores.

### Warning: To avoid I/O failures and potential cluster recovery issues, do not use the same subnet IP

address for Nutanix Volumes network segmentation and the AHV subnet associated with object store storage networks.

- Nutanix recommends using the same network for management plane communication as Prism Element

for a network segmentation enabled Nutanix object store. Objects | Nutanix Objects Deployment |

- Ensure that for a network segmentation enabled Nutanix object store, the Nutanix Objects storage,

public, and management networks each have a different AHV subnet.

- Ensure that for a network segmentation enabled Nutanix object store, the Nutanix Objects management

network and the Nutanix Objects storage network are not routable to each other.

- When deploying a Nutanix object store without network segmentation on a Prism Element cluster that

has Nutanix Volumes network segmentation enabled, ensure that the subnet IP address used for the AHV subnet of the storage network of the object store is a different subnet IP address than the one used for Nutanix Volumes service network segmentation on the Prism Element cluster.

- Ensure that Nutanix Volumes network segmentation is not disabled after deploying a network-

segmented object store on a Prism Element cluster, even if the Prism Element user interface permits this action.

### Warning: Disabling network segmentation for Nutanix Volumes in a network-segmented object store

environment is not supported and might result in recovery failures.

### Creating or Deploying an Object Store on Prism Central

Before you create a bucket and upload objects, you must deploy an object store.

### Before you beginMake sure that you meet the following prerequisites:

- Prerequisites for deploying an object store. For more information, see Nutanix Objects Deployment

Prerequisites - AHV and ESXi on page 36.

- Prerequisites for enabling fault tolerance for one-node or two-disk failures (FT1n:2d) on a newly

deployed object store. For more information, see Prerequisites for Enabling FT1n:2d on an Object Store on page 66.

- Prerequisites for enabling network segmentation. For more information, see Prerequisites for Nutanix

Objects Network Segmentation of an Object Store on page 66.

- Ensure that you have the right user permissions. The access control policy must grant the following

minimum permissions to a non-admin user for creating or deploying an object store:

| • | Create Object Store access.The user who creates an object store instance automatically receives full access to that instance. An |
| --- | --- |

administrator can later modify or revoke this access.

| • | View Cluster access to the Prism Element instance on which you deploy the object store. |
| --- | --- |
| • | View Subnet access to the managed networks used for deployment if the Prism Element operates |

on AHV. This permission is applicable only for AHV. For more information, see Nutanix Objects Permissions on page 30 and Role-based Access Control for Nutanix Objects on page 29.

### About this taskYou can deploy multiple object stores on a single Prism Element cluster or on each Prism Element cluster

that is registered to a Prism Central instance, provided that you have enough storage. However, you cannot deploy a single object store across multiple Prism Element clusters.

### Note:

- Multicluster is not supported when network segmentation is enabled for Nutanix Objects.

Objects | Nutanix Objects Deployment |

- In a network segmentation-enabled object store deployment, replication traffic defaults to the

Nutanix Objects management network. To route replication traffic through the Nutanix Objects storage network, contact Nutanix Support for configuration assistance. To create an object store, follow these steps:

### Procedure

1. Log on to Prism Central.2. In the Application Switcher, click Objects.3. Click Create Object Store.

| The | Create Object Store: Prerequisites window appears. |
| --- | --- |

4. Click Confirm.

| The | Create Object Store window appears. |
| --- | --- |

### 5. In the General Details section of the Object Store Details, follow these steps and click Next:

| a. In | Object Store Name, enter the name of the object store. |
| --- | --- |

For guidelines on choosing a compliant name, see Object Store and Domain Naming Conventions on page 65.

| b. In | Domain, enter the domain name. |
| --- | --- |

This domain name is the default domain name for all the object store in that cluster.For guidelines on choosing a compliant name, see Object Store and Domain Naming Conventions on page 65.

- DNS configuration must be manually added on the client side to fully resolve the fully qualified

domain name (FQDN) of Nutanix Objects to the Nutanix Objects public IP address.

- When the Controller Microservices Platform (CMSP) is enabled, you cannot enter a domain

name for the object store deployment. The CMSP domain is used instead. To use a different Objects | Nutanix Objects Deployment |

domain, add Fully Qualified Domain Names (FQDNs) for the object store. For more information, see Managing FQDN and SSL Certificate on page 81.

| c. In | Cluster, select the cluster for the object store deployment. |
| --- | --- |

- Use an encrypted cluster for encrypting the bucket.• Object stores deployed on one or two-node clusters cannot be scaled out after deployment.

d. (Optional) To enable the Nutanix Objects fault tolerance for one-node or two-disk failures

| ( FT1n:2d), click | Enable FT1n:2d objects qualification. |
| --- | --- |
| e. In | Worker Nodes, add the number of worker nodes. |

VM configuration guidelines:

- Each VM is assigned 10 vCPUs and a DHCP IP address.• A minimum of 10vCPUs and 32 GiB of memory is required.• Each click on plus for worker nodes adds 10vCPUs and 32 GiB of memory.• vCPU and memory are linked. You must use multiples of 10 for vCPUs and multiples of 32 for

memory.

### Important:

- The number of configured worker nodes must not exceed the total nodes in the AOS

cluster.

- Resources added cannot be reduced later.• Actual performance relies on various factors.

f. (Optional) To optimize high-performance workloads on all-flash clusters, select the following options:

| » | Dense All-Flash Clusters: If a cluster is a dense all-flash setup, enable the high-performance |
| --- | --- |

configuration through the Prism Central user interface. If you require better performance on

| these clusters, select | Scale up Worker Nodes. This option is disabled by default. |
| --- | --- |
| » | Hybrid Clusters: If a cluster is a hybrid setup, enable the maximum number of load balancers |

(N-1) when the number of MSP worker nodes (N) is six or greater. If your workloads on Nutanix

| Objects require more concurrent connections or higher throughput, select | Scale Out Load |
| --- | --- |

Balancer. This option is also disabled by default.

| • If you close the window by clicking the | X while you create an object store, the system does not |
| --- | --- |

save any entered information or the precheck status.

- Hover over the help icon for more information about each field.• The diagram updates automatically to show the necessary worker nodes, load balancers, and

resources based on the number of selected worker nodes.

- The system highlights active components in the diagram.• A new container with name msp-<uuid> is created for Nutanix Objects deployment on ESXi. This

container downloads VM images for MSP worker nodes. Objects | Nutanix Objects Deployment |

### 6. In the Storage Network section, follow these steps:

| a. In | Storage Network, type the Nutanix Objects storage network name that is used for internal |
| --- | --- |

communication (data and management) between various VMs and nodes of the object store.For more information on the IP address requirements according to the deployment size, see Nutanix Objects Network Configuration on page 39.

| b. (Optional) To segment the network for AHV clusters, check the | Segment Management Traffic |
| --- | --- |
| checkbox and type the management network name in the | Management Network field. |

Segmented networks separate data and management traffic. The management network handles all management plane traffic. You can enable Nutanix Objects network segmentation only if Nutanix Volumes network segmentation is enabled. If Nutanix Volumes network segmentation is disabled

| on the Prism Element cluster, the | Segment Management Traffic checkbox appears grayed out. |
| --- | --- |

Note: Network segmentation is not supported for ESXi clusters.

| c. (Only for AHV) In | Object Store Internal IPs (2), do one of the following: |
| --- | --- |

» If network segmentation is disabled, enter two Nutanix Objects storage network IP addresses, separated by a comma. » If network segmentation is enabled, enter two Nutanix Objects management network IP addresses, separated by a comma.

| If network segmentation is disabled, | Object Store Internal IPs (2) are within the Nutanix Objects |
| --- | --- |
| storage network. However, if network segmentation is enabled, | Object Store Internal IPs (2) are |

within the Nutanix Objects management network and Nutanix Objects storage and management network must be different.For ESXi, these two internal IP addresses are not required and are selected automatically from the IPAM range configured for the ESXi networks.

### 7. In the Public Network section, follow these steps and click Save & Continue:

| a. In | Public Network, select a public network to allow external clients access to the object store. |
| --- | --- |

This VLAN must have up to four IP addresses in the usable IP address range. This network can be the same as the storage network. For more information, see Shared Versus Single Network Configuration for Nutanix Objects on page 48.

| b. In the | Public Network Static IPs, enter the public access IP addresses (one for each load |
| --- | --- |

balancer) separated by a comma or as an IP address range.For example, if one Load Balancer is used, then only one IP address is required. Also, you can enter the IP addresses in a range, 10.2.3.1-10.2.3.4, or separated by a comma 10.2.3.1, 10.2.3.2,10.2.3.3, 10.2.3.4 .

### AHV: These IP addresses are within the Nutanix Objects public network and used to access the

### object store.ESXi: The public access IP addresses can be either within or outside the range of the IPAM

network.For more information on network configurations, see Nutanix Objects Network Configuration on page 39. 8. (Optional) To continue with the deployment later, click Save for Later. The object store is saved in the list of object stores. Objects | Nutanix Objects Deployment |

### 9. (Optional) To complete the deployment later for the saved object store configuration, select the object

| store and click | Actions > Complete Deployment. |
| --- | --- |

Note: Deployment takes at least 30 minutes. Prechecks start before the deployment begins. A list of checks performed is displayed in the UI. Also, a VM image named predeployment_port_vm, and two VMs named predeployment_objects_public and predeployment_objects_storage are created. 10. (Optional) To see a summary of the object store, click Show Summary in the right pane.11. Depending on the results of the precheck, follow these steps:

| » If the prechecks pass, click | Download Report to download the report, then click Create Object |
| --- | --- |

Store to start with the object store deployment.The report contains the name of the check, status, and message. All prechecks must pass before you can start the deployment.

| » If the prechecks fail, an error message is displayed in the UI. Click | Download Report to download |
| --- | --- |

the report. A Fail status is displayed next to the check name with a message. The object store is saved and you can complete the deployment after fixing the failed checks. After the prechecks pass, you can view the status of the object store deployment.You can view the deployment progress in percentage and each step in the grid by hovering over the loading icon.

### Warning: Do not delete MSP VMs (created with a specific prefix in Nutanix Objects) from vCenter or

Prism Central, because their deletion goes undetected. You can identify the MSP VMs by their names. The naming conventions are as follows:

- MSP Worker VM: deployment_name-XXXXXX-default-N• Load Balancer MSP VM: deployment_name-XXXXXX-XXXXXXXXXX-envoy-N

### What to do nextAfter deploying the object store, you can perform the following steps:

- Configure directory and generate access key. For more information, see Directory Configuration and

Access Key Generation on page 90.

- Access the endpoint provided as part of Client Access Network (S3 Endpoints) by using HTTP or

HTTPS. For more information, see Access Nutanix Objects Endpoints on page 74.

- Create and configure buckets. For more information, see Creating and Configuring an S3 Bucket in

Nutanix Objects on page 99.

- Create objects using S3 APIs. For more information, see Supported S3 APIs• Expand the object store storage. For more information, see Expanding Storage for an Object Store on

- Scale out an object store. For more information, see Scaling Out an Object Store on page 80• Share buckets. For more information, see Sharing a Bucket in Nutanix Objects on page 130.You can also deploy Nutanix Objects on Nutanix Cloud Clusters. For more information, see Deploying

Object Store on Nutanix Cloud Clusters on page 85. Objects | Nutanix Objects Deployment |

### Viewing Object Store Deployments

View a list of deployed object stores, and the general and networking details of the object stores.

### Before you beginA non-admin user can view an object store only after the administrator creates an access control policy

on a role in the Prism Central user interface for the non-admin user. The specific role must have minimum permission to view an object store and the non-admin user must be assigned to specific Nutanix Objects instances.

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator—A Super Admin or a Prism Admin in Prism Central.• Non-admin user—A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this taskTo view the object store, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. From the Application Switcher Function, click Objects.

### Figure 8: Object Store list

A list of existing object stores appears.The following steps describe the fields that appear in the object store table. You can click on the

| name of an object store to open the | Object Store page. You can click View By to view the General, Objects | Nutanix Objects Deployment | |
| --- | --- |

### Usage, or Networking details of an object store. A dash (-) is displayed in the field when a value is not

### available or applicable.General view:

| • | Name: Displays the name of the object store. A Federated tag is displayed next to the federated |
| --- | --- |

object stores.

| • | Version: Displays the version of Nutanix Objects in which the object store was created. |
| --- | --- |
| • | Domain: Displays the domain of the object store. |
| • | Worker Nodes: Displays the number of worker nodes of the object store. |
| • | Usage (Logical): Displays the object store usage in GiB or TiB. |
| • | Buckets: Displays the number of buckets in an object store. |
| • | Objects: Displays the number of objects in an object store. |
| • | Notifications: Displays notifications if any. |
| • | Objects Public IPs: Displays the endpoints or the IP addresses used by the client.You can access these endpoints by using HyperText Transfer Protocol (HTTP) and HyperText |

Transfer Protocol Secure (HTTPS) protocols. For more information, see Access Nutanix Objects Endpoints on page 74.

### Networking view:

| • | Name: Displays the name of the object store. |
| --- | --- |
| • | Cluster: Displays the cluster in which the object store is deployed. |
| • | Objects Public Network: Displays the Virtual Local Area Network (VLAN) required for accessing |

object store endpoints externally.

| • | Objects Public IPs: Displays the public IP address of Nutanix Objects. This is used to access the |
| --- | --- |

Nutanix Objects Browser.

| • | Objects Storage Network: Displays the VLAN required internally for deploying Object Store |
| --- | --- |

Services on Prism Element.

| • | Objects Storage Network Static IPs: Displays the internal configured IP addresses. |
| --- | --- |

### Usage view:

| • | Name: Displays the name of the object store. |
| --- | --- |
| • | Usage (Logical): Displays the object store usage in GiB or TiB. |
| • | Local Usage: Displays the amount of data stored locally in the Nutanix Objects cluster. |
| • | Tiered Usage: Displays the amount of data tiered out and stored remotely based on lifecycle |

policies.

| • | Licensed Usage: Displays the usage from a single object store accounted against the licensed |
| --- | --- |
| capacity. It is calculated by adding | Local Usage and Tiered Usage. |

Note: Data tiered to another Nutanix Objects endpoint is not included. For information on network configurations, see Nutanix Objects Network Configuration on page 39. Objects | Nutanix Objects Deployment |

What to do nextYou can view the summary of an object store and perform various object store operations. For information, see Viewing Object Store Summary on page 74.

### Access Nutanix Objects Endpoints

Nutanix Objects endpoints are the entry point to Nutanix Objects. You can access these endpoints by using the Hypertext Transfer Protocol (HTTP) or Hypertext Transfer Protocol Secure (HTTPS) protocols through any third-party clients which support Simple Storage Service (S3) APIs. Accessing Buckets and Nutanix Objects within an Object Store InstanceNutanix Objects supports path-style and virtual hosted-style bucket access.

| • | Path-Style Access: The path-style syntax requires that you use the endpoint when attempting to |
| --- | --- |

access a bucket, and the request specifies a bucket by using the first slash-delimited component of the Request-URI path.For example, if you have a bucket with the bucket name as

### bucket-name and the object name as

### example.jpg, and you want to use the path-style syntax. Following is the correct request:

PUT /bucket-name/example.jpg HTTP/1.1 Host: object-store-name.domain-name

| • | Virtual Hosted-Style Access: The virtual hosted-style syntax is used to address a bucket in a REST |
| --- | --- |

API call by using the HTTP Host header. This method requires the bucket name to be Domain Name System (DNS)-compliant.For example, if you have a bucket with the bucket name as

### bucket-name and the object name as

### example.jpg, and you want to use the virtual hosted style. Following is the correct request:

PUT /example.jpg HTTP/1.1 Host: bucket-name.object-store-name.domain-name For virtual hosted-style access, allow the Nutanix Objects fully qualified domain name (FQDN) in the DNS server with the wild card allowlist. For example, the following are the expected DNS entries for the Nutanix Objects endpoint. objects.subdomain.example.com. IN A 192.168.5.101 objects.subdomain.example.com. IN A 192.168.5.102 objects.subdomain.example.com. IN A 192.168.5.103 objects.subdomain.example.com. IN A 192.168.5.104 *.objects.subdomain.example.com. IN A 192.168.5.101 *.objects.subdomain.example.com. IN A 192.168.5.102 *.objects.subdomain.example.com. IN A 192.168.5.103 *.objects.subdomain.example.com. IN A 192.168.5.104

### Viewing Object Store Summary

You can view the summary of an object store. The summary comprises of configuration, namespaces, and usage of the object store.

### About this taskTo view the object store summary, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects. Objects | Nutanix Objects Deployment |

### 3. Click the Object Stores tab, and then click the name of the object store from the table to view its

summary. 4. Click the Summary tab. You can view the object store configurations, federated namespaces, and usage.

### What to do nextYou can do the following from the object store summary page:

- Managing FQDNs and SSL certificates. For more information, see Managing FQDN and SSL Certificate

on page 81.

- Scaling out an object store. For more information, see Scaling Out an Object Store on page 80.• Expanding an object store. For more information, see Expanding Storage for an Object Store on

page 78.

- Deleting an object store. For more information, see Deleting an Object Store on page 75.• Launching Nutanix Objects Browser. For more information, see Launching the Nutanix Objects Browser

on page 208.

### Deleting an Object Store

You can delete both successful and failed object store deployments. However, for deletion of successful deployment, ensure to first delete the objects and buckets within that object store. Before you beginA non-admin user can delete an object store only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances: Delete Object Store and View Object Store.

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator—A Super Admin or a Prism Admin in Prism Central.• Non-admin user—A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this task

### Note:

- Deployments in progress cannot be deleted.• You cannot delete the primary cluster without deleting the secondary cluster as the primary

object store cluster hosts all the common services. To delete the primary cluster, ensure all the secondary clusters are deleted.

- Multi-cluster containers are not deleted on secondary Prism Elements if the Nutanix Objects

deployment is not successful.

- If a failure occurs while replacing an SSL certificate for an object store, you cannot delete that

object store deployment; however, you can try replacing the SSL certificate again. Objects | Nutanix Objects Deployment |

- Deletion of an object on the local object store may not immediately lead to space reclamation

on the AWS S3. Space reclamation happens when all the source objects mapped to the corresponding objects in the AWS S3 get deleted. To delete an object store deployment, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the Object Stores table, select the object store which you want to delete, and then click Actions > Delete. 4. On the confirmation dialog box, click Delete. A message appears to confirm the object store deletion.

### Object Store Expansion

This section describes the expansion options available with object store. Storage ExpansionIf your existing object store storage is less than 85% full or you plan to use a different cluster for object store storage, you can expand the storage of your object store cluster by adding new nodes to the existing cluster or by adding additional clusters.For example, if you have an object store deployed on a 10 TB cluster named Cluster1 and your storage is almost or less than 8.5 TB full, then you can either add a node to the existing Cluster1 or add a new cluster or existing clusters (such as Cluster2 and Cluster3) if they have sufficient storage capacity. In this example, Cluster1 is the existing primary cluster on which the object store was initially deployed and hosts the worker VMs for the object store. Cluster2 and Cluster3 are secondary clusters that are later added for capacity expansion.In a multicluster setup, clusters are managed using a two-tier system: a preferred tier and a secondary tier.

- Clusters that use less than 85% of their physical capacity quota (where the quota is defined as a

percentage of total physical capacity) start in the preferred tier. Once the usage of the cluster exceeds 85%, it is moved to the secondary tier.For example, if the quota is set at 60% of the physical capacity (for example, 100TB), the object store cannot exceed a physical storage consumption of 60% (which equals 60TB). The 85% threshold for tier migration is applied to the defined capacity, meaning 85% of 60TB equals 51TB. Therefore, if the storage consumption goes beyond 51TB, the cluster will be moved to the secondary tier.

- When selecting a cluster for storage allocation, the preferred tier is prioritized. If the preferred tier is

empty, the secondary tier is used.

- Within each tier, storage is allocated in a round-robin manner among the available clusters.• Storage allocation from any single cluster cannot exceed 95% of its total physical capacity.Quotas can only be configured for secondary clusters and are specified as a percentage of the total

physical capacity. The primary cluster is assigned a default quota of 100%. Objects | Nutanix Objects Deployment |

Scale-Out (Compute and Memory Expansion)Scaling out an object store enables you to add more resources to an existing object store cluster. You can scale out the CPU count and memory, and optionally, add more storage in addition to the current usage. Scale out uses an extra 10 vCPUs and 32 GiB memory for the newly added node.

### Prerequisites for Expanding Storage for an Object Store in Nutanix Objects

Make sure you meet the following requirements before expanding storage for an object store:

- Ensure to upgrade to Microservices Platform (MSP) 1.0.5 or later and Nutanix Objects 2.0 or later

before expanding the object store cluster.

- Ensure that the new clusters are registered to the same Prism Central cluster where the object store is

deployed.

- Both the primary and secondary clusters must be running AOS versions later than 5.11.2.• Data services IP address must be configured in each secondary cluster.• Firewall must be running on the Controller Virtual Machines (CVMs) in the cluster.• Maximum latency required for adding secondary clusters is less than 5 milliseconds.• Up to 4 secondary clusters are supported.• VMware ESXi and Nutanix AHV clusters are supported. Microsoft Hyper-V cluster is not supported.

### Secondary Cluster Addition or Removal Guidelines

Key considerations and operational rules for adding, updating, or removing secondary clusters in a Nutanix Objects object store.

### Secondary Cluster Addition GuidelinesFollowing are the guidelines for adding a secondary cluster in an object store:

- Add one secondary cluster at a time to expand the object store.• Do not add secondary clusters if the primary cluster is full.• Add secondary clusters before primary storage reaches 80% capacity.• If a secondary cluster is added to an empty object store and becomes unreachable:

- No alert is shown in Prism Element.• The cluster is hidden in the UI.• After objects are added, an alert is triggered and the unreachable cluster is marked in the UI within a

minute. Cluster Removal and Update GuidelinesFollowing are the guidelines for removing or updating a secondary cluster in an object store:

| • Use | Update Multi-Cluster only to update capacity or delete an added cluster. |
| --- | --- |

- You cannot remove or reduce the limit of a cluster while adding another cluster.• You can add a cluster while removing or reducing the limit of another.

Objects | Nutanix Objects Deployment |

Timing ConsiderationsThe following are the timing implications of various actions, such as removal or modification of storage limits:

- If the secondary cluster is empty, then the removal of that cluster takes up to three hours.• If the secondary cluster has some data, and the storage limit is reduced or the cluster is removed, then

starting the data migration process might take up to seven hours.

- When you reduce the limit of the secondary cluster and if the used capacity of the cluster is less than

the updated limit capacity, then no data migration takes place and limit is changed without any delay.

### Operational RestrictionsThe following are the operational restriction for updating or removing the secondary clusters:

- When adding a cluster, you cannot remove another cluster or reduce the limit of an existing cluster.

However, while removing a cluster or reducing its limit, you can add another cluster or increase the limit of an existing secondary cluster.

- If you cancel removing the cluster or decreasing the limit, then the last updated limit remains the same

and any data migrated to other clusters is not migrated back to this cluster.

- While the limit reduction for one cluster is in progress, you can increase the limit of another cluster, but

you cannot decrease the limit of that cluster.

### Expanding Storage for an Object Store

Expand object store storage by adding nodes or clusters if usage is less than 85 percentage or if using a different cluster for storage. Before you beginSee Prerequisites for Expanding Storage for an Object Store in Nutanix Objects on page 77.Non-admin users can expand object store storage only if assigned a role with

### View, Add, Update

### Multicluster and View Object Store permissions by the admin user in Prism Central. Admins are Super

Admins or Prism Admins in Prism Central; non-admins are Prism Central users without admin privileges.In addition to the preceding permissions, the user must have

### View Cluster permission on the Prism

Element cluster that needs to be added to the object store.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.

### About this taskTo expand the object store storage, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the Object Stores tab, and then click the name of the object store for which you plan to expand the storage. The object store page appears. . Click Clusters. Caution: Once a primary cluster has been successfully added to an object store, it cannot be removed. The table lists the Usage, Max Usable and Free Capacity of the cluster. All the capacities listed are physical capacities, not logical.

| • | Usage (Physical): Physical capacity used by this object store on the cluster. |
| --- | --- |
| • | Max Usable (Physical): Maximum physical capacity on the cluster can be used by this object |

store. This is calculated as the total physical capacity of the cluster * any limit set for this cluster capacity used by other workloads. Your available capacity might be less than you planned for as other workloads are taking up the space.

| • | Free Capacity (Physical): Additional capacity the object store can consume on this cluster within |
| --- | --- |

the specified limit. It is possible that there might not be free capacity available for the object store to consume even if current consumed capacity is less than max usable since there might be other workloads consuming capacity from this cluster. For example, Cluster A is a primary cluster with 10 TB as the total capacity. No hard limit can be set on this cluster as it is a primary cluster. So, the Max Usable Physical Capacity of this object store cluster is 10 TB. However, the current object store usage is 5 TB and other workloads usage is 2 TB. So, the additional Physical Free Capacity available for the object store is 3 TB. However, in case of Cluster B which is the secondary cluster with total capacity as 15 TB, the hard limit is 50%. So, the Max Usable Physical Capacity of this object store cluster is 50 % * 15 TB = 7.5 TB. However, the current object store usage is 2 TB and other workloads usage is 10 TB. So, the additional Physical Free Capacity available for the object store is 5.5 TB, but as the other workloads consume 2.5 TB of the Max Usable Physical Capacity of the object store, so the remaining Physical Free Capacity available for the object store is only 3 TB. So, other workloads can consume the Max Usable Physical Capacity of the object store; however, an object store cannot go beyond the set limit.Your primary cluster where the object store is deployed will be displayed in the table and this cluster cannot be removed. You can also view the free and used storage space. 5. Click Add Clusters. For more information on guidelines, see Secondary Cluster Addition Guidelines on page 77.After adding 4 secondary clusters, the Add Clusters button is automatically disabled. A list of clusters registered to the Prism Central will be displayed. Hypervisor type, total physical capacity and free physical capacity of the clusters will also be displayed. However, the clusters that are already added as secondary clusters to an object store will not be displayed.

### 6. Once you select the cluster, under Set up hard limit section, select the usage limit in percentage, or

| select | Custom and enter a custom limit for an object store and click Done. |
| --- | --- |

This limits the object store to use a maximum capacity on the selected cluster. If you exceed the limit, an alert is generated.You can change the limit (increase or decrease) once a secondary cluster is added. Select the cluster,

| and then click | Update Limit. You cannot update the limit for the primary cluster. You can also remove Objects | Nutanix Objects Deployment | |
| --- | --- |

| the secondary clusters once added. Select the cluster, and then click | Remove. The removed cluster |
| --- | --- |

can be added back to the multicluster.For more information on secondary clusters considerations, see Secondary Cluster Addition or Removal Guidelines on page 77.A new cluster is added to the object store. If any secondary cluster addition fails, you can remove that

| cluster. You can also see the usage of these clusters in the | Usage tab. For more information, see |
| --- | --- |

Viewing Object Store Usage on page 168. Note: It will take a minute to add a new cluster.

### Scaling Out an Object Store

Scaling out an object store enables you to add more resources to an existing object store cluster. You can add a worker node, and optionally, add more storage in addition to the current usage. An additional 10 vCPUs and 32 GiB memory get added for each worker node.

### Before you begin

- Make sure that you have at least a three-worker node cluster for performing scale-out, but it cannot

exceed the maximum number of back-end AOS nodes.

- Make sure that physical resources are available.A non-admin user can scale out an object store only after the administrator creates an access control

policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Scale-Out• View Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator—A Super Admin or a Prism Admin in Prism Central.• Non-admin user—A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this task

### Note:

- Nutanix Objects version lower than Nutanix Objects 2.1 does not support scale-out. Upgrade

Nutanix Objects to the latest version to use the scale-out feature.

- You can scale out one node or VM at a time.• During scale out of an object store, no disruption happens. You can launch the object store

during scale-out.

- If physical resources (VMs) are deployed, rollback is not supported. However, if the

deployment of physical resources fails or if the deployment fails prior to deploying physical resources and your cluster is not scaling, you can roll back. For rolling back scale out of the object store, contact Nutanix Support at http://portal.nutanix.com. You can perform compute scale out (adding worker nodes) and storage scale out (adding additional storage capacity) for an object store. Objects | Nutanix Objects Deployment |

To scale out an object store, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the Object Stores tab, and then from the object store table, select the object store that you want to scale out.

### 4. If you want to add a worker node (compute scale out), follow these steps:

| a. Click | Scale out. |
| --- | --- |
| The | Compute Scale Out window appears. |
| b. Click | Add Nodes. |

The object store is now scaling out. This process takes about 5 to 10 minutes. You can track step by step deployment progress with scale out workflow progress for the object store.Once the object store scale out is completed, new node is added to the object store. The object store takes an additional 10 vCPUs and 32 GiB of memory for the worker node.

### 5. If you want to set usage alert to the current usage, follow these steps:

| a. In the | Actions list, click Set Usage Alert. |
| --- | --- |
| The | Set Usage Alert page appears. |
| b. Enter the limit you want to add to the current usage and click | Add to complete. |

### Note: Nutanix Objects generates an alert if the logical usage of your object store reaches 90% of the

| specified value. For more information on alerts, see | Viewing Alerts. |
| --- | --- |

### Managing FQDN and SSL Certificate

You can add multiple fully qualified domain names (FQDNs), download the Certificate Authority (CA) certificate, and set up or replace the secure sockets layer (SSL) certificate for your object store.A non-admin user can manage FQDN and SSL certificates only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Regenerate Self Signed Certificate• Replace Certificates• Download Certificate• View Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator—A Super Admin or a Prism Admin in Prism Central.• Non-admin user—A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide. Objects | Nutanix Objects Deployment |

### Adding FQDNs

You can create multiple fully qualified domain names (FQDNs) for an object store. The FQDN that is used when creating an object store is considered the default FQDN, while the rest of the FQDNs are categorized as alternate FQDNs.

### About this taskTo add a FQDN, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the Object Stores tab, and then from the object stores table, select the object store for which you want to create a FQDN. 4. Click Manage FQDNs and SSL Certificates. . In the New FQDN field, type the FQDN and then click +FQDN. Following are the guidelines for naming a FQDN:

- Must contain at least two dots.• Can contain either alphanumeric, hyphen, or underscore characters.• Cannot start or end with a hyphen or underscore or dot.• Duplicate domain names are not allowed.• A FQDN cannot be at a sub-domain of another FQDN.

### Figure 9: Adding an FQDN

| You can select one or more FQDNs, and click | Delete to delete the FQDNs. However, you cannot delete |
| --- | --- |

the default FQDN (the FQDN used while creating an object store).The new FQDN is listed in the table.

### Note: A warning message is shown if the new FQDN is missing in the DNS configuration of the

certificate. To add the FQDN to the certificate do either of the following:

- Regenerate the secure sockets layer (SSL) certificate to add all the newly added domains

to the certificate.

- Replace the SSL certificate by importing a Certificate Authority (CA)-signed certificate.

Make sure to add all the domains to that certificate.

- If using a wildcard SSL certificate, ensure that each newly added FQDN for the object store

is listed in the Subject Alternative Name (SAN) field of the wildcard certificate. 6. Click Save. A confirmation dialog box appears to replace the SSL certificate. 7. Click Yes, Replace. The new FQDN is added and the SSL certificate remains the same.

### Note: The object store will be unreachable for 2-3 minutes, and you will not be able to perform any

operations on that object store. What to do nextYou can also regenerate or replace the SSL certificate. For more informtion,see Setting up SSL Certificate for an Object Store on page 84. Objects | Nutanix Objects Deployment |

### Setting up SSL Certificate for an Object Store

By default, self-signed secure sockets layer (SSL) certificates are generated. If you have strong security requirements, you can replace the default certificate for the object store to securely connect to the object store while you use the Hypertext Transfer Protocol Secure (HTTPS) protocol. You can replace the certificates either by regenerating a self-signed certificate or if you have a Certificate Authority (CA)- signed certificate, then you can import your private key and certificate files. When you replace the existing certificate, it removes the web browser certificate error warnings.

### Before you beginEnsure that you have met the following requirements:

- The private key should be Rivest–Shamir–Adleman (RSA) key type with key size 2048- or 4096-

bit. Contents of the private key can be in Public-Key Cryptography Standards (PKCS)#1 standard, unencrypted PKCS#8 standard, and Privacy Enhanced Mail (PEM) format.

- The provided public certificate must be signed by the provided CA.

### Note: If you want the server to return the server certificate and the chain of intermediate certificates,

upload the server certificate and the chain of intermediate certificates as a public certificate in a single file.

- The public certificate must have the fully qualified domain name (FQDN) of the Object Store Service

along with the wild card in either CN or SAN. For example, if the object store name is objects-2021 and the domain is companyname.com, then the FQDN should be objects-2021.companyname.com. Then, the certificate must have *.objects-2021.companyname.com, objects-2021.companyname.com.

### About this taskTo set the SSL certificate, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the Object Stores tab, and then from the object stores table, select the object store for which you want to set the certificate. 4. Click Manage FQDNs and SSL Certificates.5. Under SSL Certificates, click Replace SSL Certificate. Objects | Nutanix Objects Deployment |

### 6. In the Replace SSL Certificate window, select one of the following:

### Figure 10: Replace SSL Certificate

| a. | By regenerating self signed certificate: Uses RSA 2048 bit as the private key type.A self-signed certificate is a certificate signed by the same entity that verifies the certificate. |
| --- | --- |
| b. | By importing key and certificate: Upload your private key and certificate files. |
| • | Private Key: Click Upload to upload the private key.This key is used to decrypt the message. |
| • | Public Certificate: Click Upload to upload the public certificate.A public key certificate is an electronic document used to prove the ownership of a public key. |

The certificate includes information about the key, information about the identity of its owner (called the subject), and the digital signature of an entity that has verified the contents of the certificate (called the issuer).

| • | CA Certificate/Chain: Upload the CA certificate.A certificate chain is the certificate of a particular CA, plus the certificates of any higher CAs up |
| --- | --- |

through the root CA. You can also add or delete the FQDNs while regenerating or replacing the SSL certificate. For more information, see Adding FQDNs on page 82. 7. Click Save. The default SSL certificate is replaced.

### Deploying Object Store on Nutanix Cloud Clusters

Outlines the steps to begin with Nutanix Cloud Clusters (NC2) on Nutanix Objects.

### Before you beginEnsure that you have registered for NC2 on Amazon Web Services (AWS) from the My Nutanix portal. For

more information, see Getting Started with NC2. Objects | Nutanix Objects Deployment |

### Note: Nutanix Cloud Clusters (NC2) only support single-node deployments. If you need assistance with

multinode deployments, contact the Nutanix Account Team.

### About this taskTo deploy an object store on Nutanix Cloud Clusters (NC2), follow these steps:

### Procedure

1. Enable Nutanix Objects. For more information, see Enabling Nutanix Objects on page 18. 2. Create an object store.

- Use the IP addresses from the subnet that you created on the Prism Element web console.• Select only the necessary number of IP addresses instead of using all 255 available.

### 3. Ensure that the name servers are configured correctly and copy the name servers from the Prism

Element web console and add them to the Prism Central web console. 4. Use the load balancer to log on to Prism Central.5. Copy the IP address from the subnets created on Prism Element instance and proceed to deploy the object store. Note: You can use any available free IP addresses within the subnet. The object store is successfully deployed.

### 6. Set up a load balancer for the Nutanix Objects endpoint IP address to access the Nutanix Objects

Browser.

### Deploying Object Store at a Dark Site (Offline Deployment)

Dark site deployment is a process for deploying Nutanix Objects at a site without internet access.

### Before you beginMake sure that you satisfy the deployment prerequisites before starting the deployment. For more

information, see Nutanix Objects Deployment Prerequisites - AHV and ESXi on page 36.

### About this taskTo deploy an object store in a dark site, follow these steps:

### Procedure

1. Deploy Prism Central 2023.1.0.1 or later version.2. Check the compatibility matrix to find the latest Objects Service and Objects Manager versions from the Support Portal.

### 3. Download the latest Objects Service LCM bundle and Objects Manager LCM bundle from the

Support Portal.

### 4. Check the compatibility matrix to find the latest Microservices Platform (MSP) version from the

Support Portal. . Download the latest MSP dark site bundle from the Support Portal.

### Note: If the Prism Central version is pc.7.5 or later and the Nutanix Objects version is 5.3 or later,

use the MSP Platform bundle instead of the MSP LCM bundle. For all other cases, use the MSP LCM

| bundle. For version compatibility, see | MSP Release Notes. |
| --- | --- |

6. Download the latest Life Cycle Manager (LCM) Framework Bundle from the Support Portal.7. Set up the dark site web server, and copy the latest bundles for Objects Manager, Objects Service, compatibility, MSP Platform or MSP LCM, and LCM to the web server in the folder named darksite.For more information on viewing example of setting up a web server, see Setting Up a Local Web Server in the LCM Dark Site Guide.

### 8. Run the following commands to untar all the bundles within the same location (darksite folder):

tar -xvf lcm_dark_site_bundle_version.tar.gz tar -xvf lcm_buckets-manager_version.tar.gz tar -xvf lcm_buckets-service_version.tar.gz tar -xvf lcm_msp-platform_msp-version.tar.gz or tar -xvf lcm_msp_version.tar.gz

### 9. Log in to the Prism Central UI and from the Application Switcher, click Admin Center and follow these

steps: a. Enable the Nutanix Marketplace.b. Obtain the Nutanix Objects application. The Objects Service is enabled on Prism Central. 10. From the Application Switcher, click Objects. The Nutanix Objects landing page appears. Objects | Nutanix Objects Deployment |

### 11. Configure the dark site server in LCM:

| a. From the Application Switcher, click | Admin Center. |
| --- | --- |
| b. Click | LCM > Prism Central. |
| c. In the table, click the Prism Central location and then click | Settings. |
| d. Under | Upgrade Source, select Local Web Server from the dropdown menu, and enter the web |

server URL from Step 8 where all dark site bundles are untarred.For example, http://IP address of the web server/darksite/

| darksite | is the folder where all the bundles are untarred. |
| --- | --- |

You can also upload the LCM bundle using direct upload method. For more information, see Uploading the Firmware and Software LCM Bundle using the Upload Bundle option from Prism Central in the Life Cycle Manager Guide.

### Figure 11: LCM Dark Site Settings

### 12. SSH to Prism Central and run the following command to check the AirGap enabled status:

nutanix@PCVM~$ mspctl controller airgap get 13. Log on to the Prism Central UI and go to LCM > Inventory, then click Perform Inventory. The LCM inventory completes.

### 14. (Optional) To upgrade Objects Manager to the latest version, select Objects Manager, click View

Upgrade Plan > Apply Updates. After the Objects Manager is upgraded to the latest version, the setup is ready to start the Nutanix Objects deployment.

### 15. SSH to Prism Central and run the following command to ensure that MSP Controller version on the

Prism Central UI matches the version indicated in Step 4: nutanix@PCVM~$ mspctl controller version 16. From the Application Switcher, click Objects, then click Create Object Store to start the deployment. Objects | Nutanix Objects Deployment |
