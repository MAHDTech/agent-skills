# Nutanix Objects Manual: Prerequisites and Network Configuration

## NUTANIX OBJECTS PREREQUISITES AND

## LIMITATIONS

Ensure that you have met the deployment prerequisites, port requirements, and configured the network for AHV or VMware ESXi before proceeding with an object store deployment. This section also lists the Nutanix Objects and Network File System (NFS) limitations.

### Nutanix Objects Deployment Prerequisites - AHV and ESXi

Before deploying Nutanix Objects on AHV or VMware ESXi, carefully review this section to ensure all prerequisites are met. The requirements listed apply to both online (internet-accessible) and offline (dark site) deployments unless specified otherwise.

### General RequirementsEnsure that your environment conforms to the following requirements before running Nutanix Objects:

- The hypervisor must be either AHV or ESXi. ESXi is supported only with Nutanix Objects 3.0 or later

versions.

- The Prism Element version must be 5.11.2 and later, and the Prism Central version must be 5.17.1 and

later.

- The recommended browser is Google Chrome.• A cluster running AHV or ESXi must have at least one node present.

Nutanix Objects utilizes up to 12 vCPUs for each AHV or ESXi node, which includes includes 10 vCPUs for worker nodes and 2 vCPUs for load balancer. For dense all-flash nodes, Nutanix recommends using 20 vCPUs consisting of 16 vCPUs for worker node and 4 vCPUs for load balancer. If you need assistance with configuring all-flash nodes, please contact Nutanix Support.

- Make sure that no AHV, ESXi host, Prism Element, or Prism Central upgrades are in progress while

deploying Nutanix Objects.

- Ensure the object store domain is specifically for the object store deployment.

For example, if the main domain is mycompany.com, the object store domain can be a subdomain such as objectstore.mycompany.com.

- Ensure the proxy can access the guest VM. Nutanix recommends enabling Pulse for sites that have

Internet access.

- Nutanix recommends having a reliable, high-speed internet connection for online installations.

Image downloads timeout after 90 minutes, resulting in a failed deployment.

- Nutanix recommends upgrading to the latest version of the Microservices Platform (MSP) Controller for

deployment in a dark site.For more information on upgrading MSP Controller, see Microservices Platform on page 27.

- Ensure that the Prism Element can access the Life Cycle Manager (LCM) web server where Nutanix

Objects is deployed at a dark site.

- Ensure the LCM web server is accessible through the proxy if it is configured on Prism Central for dark

site deployment.

- Allow Prism Central and Prism Element to access the web server through port 80 for dark site

deployment. Objects | Nutanix Objects Prerequisites and Limitations |

- To expand the object store cluster, upgrade the MSP version to 2.0 or later and the Nutanix Objects

version to 3.0 or later.

- To scale the object store cluster, ensure that you upgrade to version 2.1 or later.

### Network RequirementsEnsure to configure the following network requirements before running Nutanix Objects:

- Ensure you meet the URL and port requirements. For more information, see Port Requirements for

Nutanix Objects on page 38.

- Configure Domain Name Servers (DNS) on both Prism Element and Prism Central.• Configure Network Time Protocol (NTP) servers on both Prism Element and Prism Central.• Set up the Virtual IP address and data services IP address on the Prism Element for deploying Nutanix

Objects. Also, configure the DSIP on the Prism Element cluster where Prism Central is deployed.

- Ensure that the communication between Nutanix Objects (Storage or Public network) and Prism Central

must be direct and not through a proxy server.

| • | AHV: Ensure that the Virtual Local Area Networks (VLANs) required for internal Nutanix Objects |
| --- | --- |

services and external access to the object store endpoints are configured correctly on Prism Element. For more information on the guidelines, see Nutanix Objects Network Configuration on page 39.ESXi: For more information, see ESXi Prerequisites.

- Ensure that both Prism Element and Prism Central have Internet connectivity for online deployment.

If Internet access is not available, deploy the object store on a dark site. For more information, see Deploying Object Store at a Dark Site (Offline Deployment) on page 86.

- Basic VLAN, Open Virtual Network (OVN) VLAN, and virtual private cloud (VPC) subnets are

supported. Note: Network segmentation only supports basic VLAN.

- The best practice is to keep Nutanix Objects Storage and Nutanix Objects Public network on separate

networks. Maximum ConfigurationsFor more information on Nutanix Objects configuration limits, see Nutanix Configurations Maximums.

### ESXi Prerequisites for Nutanix Objects Deployment

Before deploying Nutanix Objects on VMware ESXi, review this section to ensure that you meet all prerequisites.Unless specified otherwise, the requirements listed apply to both online (internet-accessible) and offline (dark site) deployments.

### Network PrerequisitesEnsure to configure the following network requirements before running Nutanix Objects:

Objects | Nutanix Objects Prerequisites and Limitations |

- All ESXi hosts must have access to the network designated for Nutanix Objects deployment.

- When you set up the network using a standard vSwitch, all hosts must have the same network name

and VLAN configuration.

- When you configure the network using the VMware Distributed Virtual Switch (DVS), all hosts must

be part of it.

- Add nodes only from a single Prism Element cluster to an ESXi cluster in VMware vCenter server.• Configure the VMware NSX network on all hosts in both the primary and secondary VMware vSphere

Metro Storage Clusters (vMSC).

- Meet the Nutanix Objects ESXi IP Address Management (IPAM) requirements. For more information,

see ESXi Configuration for Nutanix Objects Deployment on page 44.

### vCenter and ESXi PrerequisitesMake sure that the ESXi cluster meets the following requirements:

- Register ESXi clusters with a vCenter server.• Enable VMware Distributed Resource Scheduler (DRS) and high availability solutions on the cluster.

For more information, see Create a Cluster in the VMware vSphere Guide.

- Map the AOS cluster that is used for deploying Nutanix Objects to a single ESXi cluster.• The ESXi hypervisor must be version 6.5 or later.• The Nutanix Objects version must be 3.0 or later.• The MSP version must be 2.0 or later.• The VMware NSX-T version must be 2.4 or later. For more information, see KB 8545.

### Prism Central VM PrerequisitesEnsure that the Prism Central VM meets the following requirements:

- The Prism Central VM is hosted and registered on the Prism Element cluster. Volume must be

provisioned from the hosted Prism Element cluster to the MSP controller for image conversion. Note: Prism Central deployments in a non-Nutanix ESXi environment do not support Nutanix Objects.

- Allow the proxy configuration to include the vCenter server IP address, and connect the vCenter server

to Prism Central VM.Worker VMs, Prism Central VM, and Prism Element VM must connect directly to vCenter without using a proxy server.

### Port Requirements for Nutanix Objects

Port requirements for Nutanix Objects instance deployment.For more information on the required ports and the port diagram for your Nutanix Objects deployment, see Ports and Protocols Guide for Nutanix Objects.

### Note:

- URLs are not required for the dark site deployment.

Objects | Nutanix Objects Prerequisites and Limitations |

- Internal network IP ranges, such as 10.100.x.x and 10.200.x.x, might appear in firewall logs

when traffic from the inter-Microservices Platform (MSP) routes through the firewall.

### Nutanix Objects Network Configuration

The Nutanix Objects architecture uses the following virtual networks:

| • | Nutanix Objects Storage Network: Helps internal communication among various VMs and nodes |
| --- | --- |

of the object store, handling both data and management traffic when Nutanix Objects network segmentation is disabled, and only data traffic when it is enabled.

| • | Nutanix Objects Public Network: Allows external clients to access the object store. |
| --- | --- |
| • | Nutanix Objects Management Network: Specifically isolates management traffic. It handles all |

communication from the Prism Central instance to the Microservices Platform (MSP) virtual machine, ensuring that this traffic remains securely segmented on its own dedicated network. The traffic destined to the management network of the Prism Element also originates from this network.

### Note: The Nutanix Objects management network is used only when network segmentation is enabled for

AHV clusters. This network option is not available for ESXi clusters.

### Figure 1: Nutanix Objects Network Architecture - Using Two Separate Networks (Storage and

### Public)

### Tip:Key recommendations for Nutanix Objects networks:

- You can have two virtual networks, each for Nutanix Objects storage network and Nutanix

Objects public network, but it is not mandatory. You can have the Nutanix Objects storage network and the Nutanix Objects public network on the same virtual network. However, Nutanix recommends configuring the Nutanix Objects storage network and the Nutanix Objects public network on different virtual networks for production deployments.

- Nutanix recommends that you configure the Nutanix Objects storage network to use the same

network as the Controller VM (CVM) or the hypervisor. A single network enables the traffic between Nutanix Objects and CVM to flow within the same network, avoiding cross-network hops that can limit performance in some deployments. The traffic between Nutanix Objects Objects | Nutanix Objects Prerequisites and Limitations |

and the CVM is influenced by the capabilities of the underlying AOS and is significant in a dedicated objects deployment.

- When enabling network segmentation, Nutanix recommends using the same network for

management plane communication as Prism Element.

- To use different networks for the Nutanix Objects storage network and CVM network, ensure

that the network bandwidth between the top-of-rack switch and the L3 device is fast enough to avoid network congestion. Alternatively, you can enable L3 functionality on the top-of-rack switch. Link Aggregation Control Protocol and Link AggregationNutanix recommends that you configure link aggregation with link aggregation control protocol (LACP) and balance-transmission control protocol (TCP). Nutanix Objects is a network-heavy workload and requires a high network bandwidth. You can achieve a faster network by aggregating multiple physical links into one logical network interface that can be used for all traffic.You require LACP and link aggregation to fully use the bandwidth from multiple links. In Open vSwitch (OVS), dynamic link aggregation is achieved through LACP, and load balancing is implemented using balance-TCP.For more information on how to enable, turn off, and verify LACP in AHV, see Enabling LACP and LAG (AHV Only) in the AHV Networking Guide.In ESXi-based Nutanix Objects deployments, you can also use the vSphere distributed switch with Load Based Teaming (LBT).

### AHV Configuration for Nutanix Objects Deployment

Configure and manage virtual networks through the Prism Element web console and use these virtual networks to deploy an object store through Prism Central. Nutanix Objects Storage Network RequirementsAn object store uses the Nutanix Objects storage network, a private virtual network, to communicate between services. The number of IP addresses required for deploying your Nutanix Objects store instance varies according to the deployment size.For more information, see IP Address Consumption for an Object Store on page 41. For more information on network configurations and enabling AHV IPAM, see Network Configuration for VM Interfaces in Prism Web Console Guide.Requirements of a virtual network used for an object store:

- The virtual network must have AHV IPAM enabled.• Sufficient IP addresses must be available in the IPAM dynamic host configuration protocol (DHCP) pool.• Two static IP addresses outside the DHCP pool are required for each object-store. Later, while

deploying the object store from the Prism Central UI, use the static IP addresses for the Nutanix Objects storage network configuration based on the object store storage and resource requirements. For more information, see Table 6: IP Address Consumption Based on Deployment Size for an Object Store Without Segmented Network on page 42 and Table 7: IP Address Consumption Based on Deployment Size for an Object Store With Segmented Network on page 43.

### Note: Nutanix Objects internal services use the 10.100.0.0/16 and 10.200.0.0/16 subnets. Ensure that you do

not assign internal interface IP addresses from these subnets. Objects | Nutanix Objects Prerequisites and Limitations |

Nutanix Objects Public Network RequirementsAn object store uses the Nutanix Objects public network, a virtual network, to allow access from external clients. For more information, see the IP Address Consumption for an Object Store on page 41 section.Requirements of a virtual network to access an object store externally:

- AHV IPAM must be enabled, but no IP addresses are required in the IP pool.

| While configuring the virtual network, ensure that you specify details such as | Virtual Local Area |
| --- | --- |

### Network Identifier (VLAN ID), Network IP Address/Prefix Length, Gateway IP address, and

Domain Name System (DNS) Servers.

- While deploying an object store from the Prism Central UI, use up to four static IP addresses for

the Nutanix Objects public network configuration based on the object store storage and resource requirements. Nutanix Objects Management Network RequirementsAn object store uses the Nutanix Objects management network, a virtual network (with AHV IPAM enabled), to segment the network traffic and handle all management plane traffic. For more information, see the IP Address Consumption for an Object Store on page 41 section.Requirements of a virtual network to segment the network of an object store:

- The VLAN for Nutanix Volumes service on a Prism Element cluster that has Nutanix Volumes network

segmentation must be routable to the VLAN used for the Nutanix Objects storage network.

- When using the same VLAN for Nutanix Objects storage network and the Nutanix Volumes service on

a Prism Element cluster that has Nutanix Volumes network segmentation, ensure that the range of IP addresses defined for the Nutanix Objects storage network does not overlap with the VLAN used for the Nutanix Volumes service.

- When deploying network segmentation enabled object store, the static IP addresses must be from the

Nutanix Objects management network and not from Nutanix Objects storage network .

- When network segmentation is enabled, additional IP addresses are required for each worker VM and

load balancer VM. For more information, see Table 6: IP Address Consumption Based on Deployment Size for an Object Store Without Segmented Network on page 42 and Table 7: IP Address Consumption Based on Deployment Size for an Object Store With Segmented Network on page 43.For more information on the prerequisites, see Prerequisites for Nutanix Objects Network Segmentation of an Object Store on page 66.

### IP Address Consumption for an Object StoreNutanix Objects requires the following IP addresses for its virtual networks:

Nutanix Objects Public Network Nutanix Objects uses up to four static IP addresses to expose the Nutanix Objects Simple Storage Service (S3) endpoint, depending on the selected worker nodes. Clients connect to these IP addresses for all S3 requests. Each object store supports a maximum of four public IP addresses. Nutanix Objects Storage Network Ensure that you meet the following IP address requirements for Nutanix Objects storage network: Objects | Nutanix Objects Prerequisites and Limitations |

| • | From DHCP pool: |
| --- | --- |

- One IP address for each worker VM• One IP address for each load balancer to communicate with Nutanix Objects worker VMs• Two IP addresses for high availability of internal Microservices Platform (MSP) services

| • | Outside DHCP pool (Static IP address): Two static IP addresses for MSP DNS and API |
| --- | --- |

servers

### Note: When network segmentation is enabled in an object store, static IP addresses (outside

DHCP pool) from the Nutanix Objects storage network are not required. Nutanix Objects Management Network Ensure that you meet the following IP address requirements for Nutanix Objects management network:

| • | From DHCP pool: |
| --- | --- |

- One IP address for each worker VM• One IP address for each load balancer to communicate with Nutanix Objects worker VMs

| • | Outside DHCP pool (Static IP address): Two static IP addresses for internal MSP services |
| --- | --- |

**Table 6: IP Address Consumption Based on Deployment Size for an Object Store Without**

### Segmented Network

| Number | AHV Managed Networks |
| --- | --- |

### of Worker

### Nodes

| Nutanix Objects Storage NetworkFrom DHCP Pool | Nutanix Objects |
| --- | --- |
| Outside | Public |
| DHCP Pool | Network |
| Number of IP | Number of IP Number of IP Number of IP Number of Number of |
| Addresses | Addresses Addresses Addresses Static IP Static IP |
| Required for | Required for High Required for Addresses Addresses |
| Worker VMs | for the Load Availability of IPAM DHCP Required Required |
| Balance | Internal MSP Server for MSP, for Load |
| VMs to | Services DNS, and API Balancers to |
| Communicate | Server Communicate |
| with the | with External |
| Worker VMs | S3 Clients Objects | Nutanix Objects Prerequisites and Limitations | |

| Number | AHV Managed Networks |
| --- | --- |

### of Worker

### Nodes

| N | N 4 (without 4 (without |
| --- | --- |
| performance | performance |
| configuration) | configuration) |
| or N-1 (with | or N-1 (with |
| performance | performance |
| configuration) | configuration) |

**Table 7: IP Address Consumption Based on Deployment Size for an Object Store With Segmented**

### Network

| Number | AHV Managed Networks |
| --- | --- |

### of Worker

### Nodes

| Nutanix Objects Storage Network | Nutanix Nutanix Objects Management |
| --- | --- |
| Objects | Network Public |
| From DHCP Pool | Network From DHCP Pool Outside DHCP Pool |
| Number | Number Number Number Number Number Number Number |
| of IP | of IP of IP of IP of Static of IP of IP of Static |
| Addresses | Addresses Addresses Addresses IP Addresses Addresses IP |
| Required | Required for High Required Addresses Required Required Addresses |
| for Worker | for the Availability for IPAM Required for for the Required |
| VMs | Load of Internal DHCP for Load Worker Load for MSP |
| Balance | MSP Server Balancers VMs Balance |
| VMs to | Services to VMs to |
| Communicate | Communicate Communicate |
| with the | with with the |
| Worker | External Worker |
| VMs | S3 Clients VMs |
| N | N 4 (without 4 (without N 4 (without |
| performance | performance performance |
| configuration) | configuration) configuration) |
| or N-1 | or N-1 or N-1 |
| (with | (with (with |
| performance | performance performance |
| configuration) | configuration) configuration) Objects | Nutanix Objects Prerequisites and Limitations | |

### ESXi Configuration for Nutanix Objects Deployment

For ESXi clusters, you can perform the Nutanix Objects ESXi IP Address Management (IPAM) configuration.You can do the Nutanix Objects ESXi IPAM configuration from the

### Nutanix Objects service > vCenter

### Management available in the Prism Central web console. For more information, see Managing vCenter for

Nutanix Objects Service on page 46.Nutanix Objects requires you to add the IPAM range for the ESXi networks that you plan to use for object store deployment.Nutanix Objects uses these ESXi networks for two purposes:

- To deploy Nutanix Objects VMs that host the various Nutanix Objects services (also referred to as

Nutanix Objects Storage Network).

- To deploy load balancer VMs that provide object store endpoint to the Simple Storage Service (S3)

| clients (also referred to as | Nutanix Objects Public Network). |
| --- | --- |

### Note: Nutanix Objects management network is used only when network segmentation is enabled for AHV

clusters. This network option is not available for ESXi clusters. The following section describes the Nutanix Objects Storage (internal) and Nutanix Objects Public (external) networks in more detail: Nutanix Objects Storage Network An object store uses this private virtual network to communicate between services. The number of static IP addresses required for deploying your Nutanix Objects store instance varies according to the deployment size. For more information, see the IP Address Consumption section.Requirements of a virtual network used for an Object Store:

| • Add the IPAM range for the ESXi networks in the | vCenter Management page. |
| --- | --- |

- Sufficient IP addresses must be available. For more information, see IP Address Consumption

Based on Deployment Size.

- Subnet, Gateway, and DNS IP address to be used for the ESXi network. The provided values

must be valid.

### Note: Nutanix Objects uses the 10.100.0.0/16 and 10.200.0.0/16 subnets for internal services. Avoid

subnet conflicts when assigning internal interface IP addresses. Nutanix Objects Public Network An object store uses this virtual network to allow access from external clients.Requirements of a virtual network to access an object store externally:Up to four static IP addresses that can either be part of or outside the IPAM range. Later, while deploying the object store from Prism Central, use the static IP addresses for the Nutanix Objects Public Network configuration based on the object store storage and resource requirements. You can have two virtual networks, each for Nutanix Objects Storage Network and Nutanix Objects Public Network, but it is not mandatory. You can have the Nutanix Objects Storage Network and the Nutanix Objects Public Network on the same virtual network. However, Nutanix recommends to have the Nutanix Objects Storage Network and the Nutanix Objects Public Network on different virtual networks for production deployments.

### Note:

Objects | Nutanix Objects Prerequisites and Limitations |

| • If the | Nutanix Objects Public Network is different from the Nutanix Objects Storage |
| --- | --- |

### Network, then only subnet and Gateway values are needed. IPAM range and DNS IP address

values are optional.

- All the IP addresses might not be used during the deployment. The number of IP addresses

used depends on the size of your deployment. The unused IP addresses are reserved for future usage. You can view a list of deployed object stores and the general and networking details of the object stores. For example, you can view the Nutanix Objects Public IP addresses for your deployment. For more information, see Viewing Object Store Deployments on page 72. IP Address ConsumptionThis section describes the IP address consumption based on the deployment size. Nutanix Objects Public Network Up to four static IP addresses are required to expose the Nutanix Objects S3 Endpoint based on the selected worker nodes. The client connects to these IP addresses for all S3 requests. Nutanix Objects supports a maximum of four public IP addresses for each object store. The IP addresses can be within or outside the range of the IPAM network. Nutanix Objects Storage Network The IP addresses are consumed from the IPAM range configured for the ESXi networks as follows:

- Each Worker VM requires one IP address.• One IP address for each load balancer for communications with Nutanix Objects workers.• Two IP addresses for High Availability of internal Microservices Platform (MSP) services.• Two static IP addresses required for MSP Domain Name System (DNS) and Application

Programming Interface (API) servers.

**Table 8: IP Address Consumption Based on Deployment Size**

| Number | Nutanix Objects Storage Network Nutanix |
| --- | --- |
| of Worker | Objects |
| Nodes | Public Network |

### Nutanix Objects ESXi IPAM Range Configured for the ESXi NetworkNumber of

| Number of | Number of Number of Number Number |
| --- | --- |
| IP addresses | IP addresses IP addresses IP addresses of static IP of static IP |
| required for | required for for high required for addresses addresses |
| Worker VMs | the Load availability of IPAM DHCP required for required |
| Balance | internal MSP Server MSP, DNS, for Load |
| VMs to | services API Server Balancers to |
| communicate | communicate |
| with the | with external |
| Worker VMs | S3 clients Objects | Nutanix Objects Prerequisites and Limitations | |

| Number | Nutanix Objects Storage Network Nutanix |
| --- | --- |
| of Worker | Objects |
| Nodes | Public Network |
| N | N 4 (without 4 (without |
| performance | performance |
| configuration) | configuration) |
| or N-1 (with | or N-1 (with |
| performance | performance |
| configuration) | configuration) |

### Managing vCenter for Nutanix Objects Service

To deploy Object Stores on ESXi clusters, you need to provide the vCenter credentials and configure the IP Address Management (IPAM) for ESXi networks.

### About this taskFor AHV clusters, IPAM is configured in the Prism Element. For ESXi clusters, you can perform the IPAM

configuration in the Prism Central as described in this section.Managing vCenter for object service consists of the following two steps:

- Add the vCenter IP address and login credentials in the Object service within Prism Central to create

a trust relationship. Nutanix does not store the login credentials after the connection is established between the vCenter and Prism Central.

- Add IPAM for one or more ESXi networks.

When deploying the object store instance, you need to define the Nutanix Objects Storage Network and Object Public Network. You can use these pre-configured networks for Nutanix Objects internal and public access. To register the vCenter in the Prism Central, follow these steps:

### Procedure

1. Log on to the source Prism Central web console, and click the Entity menu > Services > Objects2. Click vCenter Management > Add vCenter.

### Figure 2: vCenter Management Page in Nutanix Objects

. In the vCenter page, enter the IP address and login credentials of the vCenter. The details you enter are used to generate a certificate and build a trust relationship between the vCenter and Prism Central.Nutanix does not store the login credentials after the connection is established between the vCenter and Prism Central.

### Note: For a user with non-administrator privileges to perform vCenter registration, the administrator

must create an access control policy for the non-administrator user on a role that must have the following minimum permissions:

- Create Object Store• View Object Store

### 4. Select one the following options:

| » | Next: Click this option to continue adding IPAM details. |
| --- | --- |
| » | Save & Close: Click this option to add the IPAM details later.In the |

### vCenter Management page, you can click Configure Network in the Actions column to

proceed with adding the IPAM for ESXi networks. IPAM is added for the ESXi network. 5. Click Add Network.

| The | Add Network page appears. |
| --- | --- |

### 6. In the Add Network page, follow these steps:

| a. In the | Data Center, select the datacenter where your ESXi cluster is located. |
| --- | --- |
| The | ESXi Network drop-down will be populated with a list of supported ESXi networks belonging to |

the datacenter.

| b. In the | ESXi Network, select the ESXi network you want to use for deployment. |
| --- | --- |

c. Enter the IP address range, subnet mask, Gateway IP address, and DNS IP address. The IP address range you provide will be used for the Nutanix Objects Storage Network. The IP address range and DNS IP address are optional if the IPAM will be used only for the Nutanix Objects Pubic Network.

| d. Click | Add to complete your IPAM configuration. |
| --- | --- |
| If you want to add more networks, click | Add Network on the Configure page and enter the details. |

It is recommended to use separate networks for the Nutanix Objects Storage Network and Nutanix Objects Public Network.

| e. Click | Update . |
| --- | --- |
| In the | vCenter Management page, in the Network column, you can view the IPAM count. |
| In the | Actions column, you can click the Delete option to delete the vCenter. |

Even if you delete a vCenter, the IPAM details will still be available. If you add the deleted vCenter again, the IPAM details previously added get recovered.

### What to do nextYou can start with the deployment of the object store on the ESXi clusters. For more information, see

Object Store Service Deployment. Objects | Nutanix Objects Prerequisites and Limitations |

### Shared Versus Single Network Configuration for Nutanix Objects

You can configure Nutanix Objects to use a single network for all services or separate the subnets used for internal communications and client communications.You can have two virtual networks, each for Nutanix Objects Storage Network and Nutanix Objects Public Network, but it is not mandatory. You can have the Nutanix Objects Storage Network and the Nutanix Objects Public Network on the same virtual network. However, it is recommended to have the Nutanix Objects Storage Network and the Nutanix Objects Public Network on different virtual networks for production deployments.

### Note: If you choose to deploy a network segmentation-enabled object store, you need to have the Nutanix

Objects storage, public, and management networks on separate virtual networks; shared networks are not supported.

### Figure 3: Nutanix Objects Network Architecture - Using Two Separate Networks (Storage and

### Public)

### Figure 4: Nutanix Objects Network Architecture - Using One Network

### Nutanix Objects Limitations

The following section lists the limitations for Nutanix Objects. Objects | Nutanix Objects Prerequisites and Limitations |

If the Nutanix Objects Public network is identical to the Prism Central VM network, and the Nutanix Objects Storage network is different, a firewall might block TCP handshakes between Prism Central and the Nutanix Objects Storage network. This might lead to connectivity issues.To address this limitation, you must either ensure that the Nutanix Objects Public network is different from the Prism Central network or configure the Nutanix Objects Storage network to align with both the Prism Central and the Nutanix Objects Public network.

### System Limitations

- Once an object store gets deployed, you cannot change the Data Services, Controller VM,

Microservices Platform (MSP), and Prism Central IP addresses.

- Prism Central and Prism Element de-registration and reregistration is not supported.

### Limitations of Network File System

This section lists the limitations of multiprotocol access.

- Network File System (NFS) adapter in Nutanix Objects does not support Windows NFS client.• You can enable NFS access only at the time of bucket creation.• NFS-enabled buckets are exposed as NFS shares, which the NFS clients can mount.• You can perform updates (only multiprotocol access configurations), delete, and share actions on NFS

buckets.

### Note: You can delete NFS-enabled buckets only if they do not have any object and any directory

explicitly created from the NFS protocol.

- You cannot enable other S3 bucket features, such as Lifecycle Policies, Versioning, Write once read

many (WORM), Replication, Static Website, CORS, and Notifications on NFS-enabled buckets.

- You can only create symbolic links through NFS. Hard links are not supported.• You can rename files and links; however, you cannot rename a directory. After renaming, the file handle

of the renamed file changes.

- In addition to the Simple Storage Service (S3) object naming convention, the following are some cases

that are not allowed for an object name in the NFS-enabled bucket:

| • Object name cannot be | . or .. |
| --- | --- |
| • Object name cannot start with | /,./ or ../ |
| • Object name cannot contain | //, /./ or /../ pattern. |
| • Object names cannot end with | /. or /.. |

- If an object name contains a directory hierarchy, then each component in the object name separated

| by | / cannot be more than 255 bytes in length. |
| --- | --- |
| • You cannot delete objects with | / as a suffix or objects-browser-created folders from the NFS protocol, |

which appear as directories in the NFS namespace.

- If the objects are created from the S3 protocol in such a way that they end up creating a conflict in

the object and directory name, then only the file is visible in the NFS namespace, and the directory is hidden. Only upon renaming or removing the file to some other name, the directory become visible again.For example, suppose we create two objects dir1/dir2/file and dir1/dir2 from the S3 protocol. As we can see, dir2 is a directory as well as a file, so it creates a conflict. Now, while traversing this namespace Objects | Nutanix Objects Prerequisites and Limitations |

from the NFS protocol, once we are inside the dir1/ directory, only the object dir2 would be visible and not the subdirectory dir2/. So, Nutanix recommendeds not to use conflicting names for objects from the S3 protocol while creating them. Objects | Nutanix Objects Prerequisites and Limitations |

## CONFIGURING VIRTUAL PRIVATE CLOUD

## MODES FOR NUTANIX OBJECTS WITHOUT

## NETWORK ADDRESS TRANSLATION

Nutanix Objects supports Virtual Private Clouds (VPCs) in underlay, mixed (underlay and overlay), and overlay modes for No-Network Address Translation (NAT).

### Before you beginYou must meet the following requirements for configuring VPC modes with No-NAT for Nutanix Objects:

- Prism Central version must be pc.2024.3 and later.• Prism Element version must be 6.10 and later.• Nutanix Objects version must be 5.1 and later.• MSP version must be 2.6.0.0 and later.• AHV version must be 10.0 and later.

### About this taskTo configure VPC for Nutanix Objects with No-NAT, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, select Infrastructure.3. In the navigation bar, click Network & Security > Subnets.

| The | Subnets page opens the list of subnets. |
| --- | --- |

4. Create an external subnet network with No-NAT. For more information, see Creating a Subnet in the Flow Virtual Networking Guide. 5. Create an underlay network. For more information, see Creating a Subnet in the Flow Virtual Networking Guide. 6. In the navigation bar, click Network & Security > Virtual Private Clouds.

| The | Virtual Private Clouds page opens the list of VPCs. |
| --- | --- |

7. Click Create VPC. For more information, see Creating Virtual Private Cloud in the Flow Virtual Networking Guide.The Create VPC page appears. . Create VPC and associate the external subnet.

| a. In | Name, enter the VPC name. |
| --- | --- |
| b. In | External Connectivity, click Associate External Subnet. |

For more information, see the External Connectivity and Associate External Subnet window details rows in the table in Creating Virtual Private Cloud topic of the Flow Virtual Networking Guide.

| c. In | Subnet Type, select VLAN. |
| --- | --- |
| d. In | Static Routes, check Set this subnet as the default next hop for outbound traffic. |
| e. In | Destination Prefix(es), enter 0.0.0.0/0. |
| f. | In SNAT IP/ Router IP, select IP Assignment Mode as Custom Defined. |
| g. In | Custom SNAT IP / Router IP, enter four IP addresses which are configured as next hop. |
| h. In | External Gateway Configuration, enter 4 as Number of Active Hosts. |
| i. | Click Save. |
| The external subnet is setup and the | Create VPC page appears. |
| j. | Enter the externally routable IP addresses.Externally routable IP addresses is the flow VPC network. |
| k. In | Domain Name Servers (DNS), add the DNS server IP addresses. |

For more information, see the Other details on the Create VPC page row in the table in Creating Virtual Private Cloud topic of the Flow Virtual Networking Guide.

| l. | Click Create. |
| --- | --- |

The VPC is now created and associated with the external subnet.

### 9. On the Virtual Private Clouds List page, click the Flow Virtual Networking transit VPC name where

| you want to create an overlay network and click | Create Subnet. |
| --- | --- |
| The | Create Subnet page appears. |

10. Configure the overlay network. For more information, see Creating a Subnet in the Flow Virtual Networking Guide.

| a. In | Name, enter the name of the subnet. |
| --- | --- |
| b. In | Type, confirm that the type is Overlay. |
| c. In | Network IP Address / Prefix, enter the network IP address and prefix, and the gateway IP |

address.This network IP address and prefix would be the same as the externally routable IP addresses provided in Step 8 j.

| d. Click | Add IP Pool and enter the first and last IP address of the range, then click the checkbox |
| --- | --- |
| under | Actions. |
| e. Expand and configure | Domain Settings. |
| f. Click | Create. |

The external subnet, underlay, and overlay networks are created and listed in the Virtual Private Clouds list. All three external subnet, underlay, and overlay networks are created and listed in the Virtual Private Clouds list. Objects | Configuring Virtual Private Cloud Modes for Nutanix Objects Without Network Address Translation |

### What to do nextAfter creating all the networks, you can start the Nutanix Objects deployment. For more information, see

Nutanix Objects Deployment on page 65. For more information on configuring VPC modes with NAT, see Configuring Virtual Private Cloud Modes for Nutanix Objects with Network Address Translation on page 54. Objects | Configuring Virtual Private Cloud Modes for Nutanix Objects Without Network Address Translation |

## CONFIGURING VIRTUAL PRIVATE CLOUD

## MODES FOR NUTANIX OBJECTS WITH

## NETWORK ADDRESS TRANSLATION

Nutanix Objects supports Virtual Private Clouds (VPCs) in mixed mode for Network Address Translation (NAT).

### Before you beginYou must meet the following requirements for configuring VPC modes with NAT for Nutanix Objects:

- Prism Central version must be pc.7.3 or later.• Prism Element version must be 6.10 or later and compatible with Prism Central version pc.7.3.• Nutanix Objects version must be 5.2 or later.• MSP version must be 2.7.0.0 or later.• AHV version must be 10.0 or later and compatible with Prism Central version pc.7.3.

### About this taskMixed mode includes Nutanix Objects storage network in underlay network and Nutanix Objects public

network in overlay network.To configure VPC for Nutanix Objects with NAT, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. From the Application Switcher Funtion, select Infrastructure.3. In the navigation bar, click Network & Security > Subnets.

| The | Subnets page opens the list of subnets. |
| --- | --- |

4. Create an external subnet network with NAT. For more information, see Creating a Subnet in the Flow Virtual Networking Guide. 5. Create an underlay network. For more information, see Creating a Subnet in the Flow Virtual Networking Guide. 6. In the navigation bar, click Network & Security > Virtual Private Clouds.

| The | Virtual Private Clouds page opens the list of VPCs. |
| --- | --- |

7. Click Create VPC. For more information, see Creating Virtual Private Cloud in the Flow Virtual Networking Guide.The Create VPC page appears. . Create VPC and associate the external subnet.

| a. In | Name, enter the VPC name. |
| --- | --- |
| b. In | External Connectivity, click Associate External Subnet. |

For more information, see the External Connectivity and Associate External Subnet window details rows in the table in Creating Virtual Private Cloud topic of the Flow Virtual Networking Guide.

| c. In | Subnet Type, select VLAN. |
| --- | --- |
| d. In | Static Routes, check Set this subnet as the default next hop for outbound traffic. |
| e. In | Destination Prefix(es), enter 0.0.0.0/0. |
| f. | In SNAT IP/ Router IP, select IP Assignment Mode as Custom Defined. |
| g. In | Custom SNAT IP / Router IP, enter four IP addresses which are configured as next hop. |
| h. In | External Gateway Configuration, enter 4 as Number of Active Hosts. |
| i. | Click Save. |
| The external subnet is setup and the | Create VPC page appears. |
| j. | In Domain Name Servers (DNS), add the DNS server IP addresses. |

For more information, see the Other details on the Create VPC page row in the table in Creating Virtual Private Cloud topic of the Flow Virtual Networking Guide.

| k. Click | Create. |
| --- | --- |

The VPC is now created and associated with the external subnet.

### 9. On the Virtual Private Clouds List page, click the Flow Virtual Networking transit VPC name where

| you plan to create an overlay network and click | Create Subnet. |
| --- | --- |
| The | Create Subnet page appears. |

10. Configure the overlay network. For more information, see Creating a Subnet in the Flow Virtual Networking Guide.

| a. In | Name, enter the name of the subnet. |
| --- | --- |
| b. In | Type, confirm that the type is Overlay. |
| c. In | Network IP Address / Prefix, enter the network IP address and prefix, and the gateway IP |

address.

| d. Click | Add IP Pool and enter the first and last IP address of the range, then click the checkbox |
| --- | --- |
| under | Actions. |
| e. Expand and configure | Domain Settings. |
| f. Click | Create. |

The external subnet, underlay, and overlay networks are created and listed in the Virtual Private Clouds list. . Request a floating IP address.

| a. From the Application Switcher Funtion, select | Infrastructure. |
| --- | --- |
| b. Click | Network & Security > Floating IPs. |
| The | floating IPs details page appears. |
| c. Click | Request Floating IP. |
| d. In the | Number of Floating IPs field, enter the required number of floating IP addresses. |

For example, if you plan to assign floating IP addresses to two VMs, type 2 in this field. This floating IP address must match the number of Nutanix Objects public IP addresses required for the Nutanix Objects deployment later.

| e. Select the assignment type as | Assign Floating IPs and select the floating IP address. |
| --- | --- |

f. Select the VPC and enter Nutanix Objects public IP addresses required for the Nutanix Objects deployment.Repeat this step for all public IP addresses associated with the deployment of Nutanix Objects.

| g. Click | Submit. |
| --- | --- |

The floating IP addresses are assigned. What to do nextAfter creating all the networks and assignment of floating IP addresses, you can start the Nutanix Objects deployment. For more information, see Nutanix Objects Deployment on page 65. You can also configure VPC modes without NAT. For more information, see Configuring Virtual Private Cloud Modes for Nutanix Objects Without Network Address Translation on page 51. Objects | Configuring Virtual Private Cloud Modes for Nutanix Objects with Network Address Translation |
