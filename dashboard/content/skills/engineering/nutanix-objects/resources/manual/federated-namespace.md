+++
title = "federated-namespace"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-objects"
+++

{% raw %}
# Nutanix Objects Manual: Federated Namespace

## FEDERATION OVERVIEW

In the context of a single Nutanix Objects instance, namespace is a set of unique buckets hosted by that instance. Federation provides a way to create a single namespace across multiple individual Nutanix Objects instances. One federation instance corresponds to one such namespace.A federated namespace can span across multiple Nutanix Objects instances that potentially reside in different geographical locations.Each Nutanix Objects instance has a default local namespace that is independent of any other Nutanix Objects instance. In addition, Nutanix Objects instance can join multiple federated namespaces by becoming a member of each federation.Any Nutanix Objects instance that joins a federation as a member can contribute to the federated namespace. A federated namespace can be accessed from any of the members using the Federation

| FQDN in the | Host field of the HTTP requests. |
| --- | --- |

Federated namespace supports the following properties:

- Federated namespace enforces unique bucket names within the namespace.

For example, if a client creates a bucket backup in a federated namespace, then another client that later tries creating the bucket backup would fail with the error message BucketAlreadyExists orBucketAlreadyOwnedByYou .

- All clients, irrespective of which federation member they choose to access the federated namespace

from, see the same consistent list of buckets.

- Any buckets and objects that are part of the federated namespace can be accessed from any of the

members.

| A federation instance is supported in the backend using a subset of the federation members called | Core |
| --- | --- |

### Members. Core members, together, run a distributed consensus service in the backend to provide fault

tolerance for the federated namespace.

### Figure 5: Federation Overview

Objects | Federation Overview |

### Federated Namespace Prerequisites

Before creating a federated namespace, review this section carefully to ensure you have met the prerequisites.The following are the prerequisites:

- You must have at least one object store instance managed by a single or multiple Prism Central

instances.

- For Nutanix Objects 4.0, it is necessary to upgrade Prism Central and Prism Element to compatible

versions, with a minimum required Prism Central version of 2022.9.

- Nutanix Objects Manager on all Prism Central instances must be upgraded to compatible versions.• Established availability zones pairing between Prism Central instances.• Set up Identity and Access Management (IAM) replication across Prism Central instances.

### Note: For IAM pairing, if the Nutanix Objects Manager version is 5.0 and above on either the source or

the target, then the other side should also have Nutanix Objects Manager version 5.0 and above. For more information on creating a federated namespace, see Creating Federated Namespace on page 58.

### Creating Federated Namespace

Nutanix federation namespace is a mechanism that enables multiple independent systems to share a common namespace or naming system. This allows different systems to refer to the same resources using the same names, even if those resources are located in different systems and locations.

### Before you beginTo create a federated namespace, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, select Objects > Federated Namespaces.3. Click Federated Namespaces.

### Note:

- Ensure to update the SSL certificates with the federation Fully Qualified Domain Names

(FQDN) for each object store that will be included in the federated namespace.

- Ensure your DNS is configured to map the federation FQDN to the object store public IP

addresses so that applications can access the federation namespace.

- The federation FQDN should not be a sub-domain of the Prism Central domain..

4. Click Create Federated NameSpace.

| After clicking | Get Started on the prerequisites pop-up window that provides the necessary details, you |
| --- | --- |

will be directed to a new window for creating a federated namespace. Objects | Federation Overview |

### 5. Update the fields as indicated:

| a. | Name: Enter the name of federated namespace.The federation name must meet the following guidelines. |
| --- | --- |

- Must be unique across all existing federations.• Must start with a letter.• Can have alphanumeric or hyphen characters. Hyphen can only be used in the middle of the

name.

- Must not contain special characters.• Must be a minimum of 1 and a maximum of 16 characters long.

| b. | Domain: Enter the domain.The federation domain must meet the following guidelines: |
| --- | --- |

- It must contain at least one dot.• It may contain either alphanumeric or hyphen characters.• It must not start or stop with a hyphen or a dot.• When selecting a federation fully qualified domain name (FQDN), it's important to avoid using a

subdomain that belongs to any of the MSP subdomains that are connected to the Prism Central web console.

| c. | Object Stores: Select and add at least one object store instance. It is recommended to have at least |
| --- | --- |

three instances from different clusters to set up federation. You can select all the required object store instances that run the federation services. Note: Once you select the Object stores, the visual representation on the right changes accordingly.

| d. | Federation core member: A federation instance is supported in the backend using a subset of the |
| --- | --- |
| federation members, called | Core Members. Core members, together, run a distributed consensus |

service in the backend to provide fault tolerance for the federated namespace.You can select and designate core members from the list of Object stores.

### Note: To ensure higher fault tolerance in case of cluster failure, you can add 1, 3, or 5 core members

from the same or different clusters.

| e. | Federation members: All object stores participating in the federation are called federation |
| --- | --- |

members. . Click Setup. The federation setup is ready.

### Figure 6: Federated Object Stores

### What to do nextAfter creating a federated namespace, you can view and perform operations on the namespace. For more

information, see Viewing Federated Namespaces on page 62 section.

### Adding External Object Store

External object stores are AWS regions linked to an AWS account, allowing accessible buckets to be added to the federated global namespace.

### Before you beginTo add an external object store to a federated namespace, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, select Objects > Federated Namespaces.3. Select and click the required Federated namespace. . Click Add member > Add External Object Store.

| After clicking | add external endpoint on the prerequisites pop-up window that provides the necessary |
| --- | --- |

details, you will be directed to a new window for adding general details of external endpoint.

### Figure 7: Adding external endpoint

### 5. Update the fields as indicated:

| a. | Name: Enter the name of External Object Store. |
| --- | --- |
| b. | Provider: Select AWS. |
| c. | Service Host: Service host is the complete URL of your endpoint provider where the buckets are |

located.

| d. | Credentials: Provide AWS credentials with s3:ListAllMyBuckets permission and Owner-level access |
| --- | --- |

to all buckets you intend to add through this Endpoint .

| e. | Access key: provide an access key. |
| --- | --- |
| f. | Secret Key: provide a secret key. |

6. Click Save. The external endpoint is added.

### What to do nextAfter adding an external object store, you can add buckets from the endpoint. For more information, see

Adding External Bucket on page 212 section.

### Removing External Object Store

### Before you beginAll buckets associated with the external object store must be removed first. For more information, see

Deleting a Bucket in Nutanix Objects on page 139. Objects | Federation Overview |

### About this taskTo remove the external object store, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, select Objects > Federated Namespaces.3. Select the external object store you want to remove. New window with External Summary opens. 4. Click Remove External Object Store > Remove. External object store is removed.

### Managing Federated Namespace

After creating a federated namespace, you can view details and manage the object stores.

### About this taskTo manage and add object store, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In Application Switcher, select Objects > Federated Namespaces. A list of existing federated namespaces appears. 3. Select a Namespace, and click Manage Namespace .

| The | Federated Namespace page appears. |
| --- | --- |

4. (Optional) To remove an object store, first select an object store, and then click Remove Object Store. The object store is removed.

### 5. (Optional) To add an object store to the existing federation, click Add Object Store

The Add Object Store window appears. Select and add the required object stores.

### Note:

- You can add between 1 and 20 object stores. Overall a federation can have maximum 128

members.

- Core members cannot be added to a federation.• A core member cannot be removed.

### What to do nextYou can view the object store and check if they are part of the local namespace or federated namespace,

see Viewing Object Store Summary on page 74.

### Viewing Federated Namespaces

You can click any federated namespace to view all the details of the Summary and Object stores.The following table lists and defines the fields used in summary and object stores: Objects | Federation Overview |

**Table 9: Summary**

| Term | Description |
| --- | --- |
| Name | Displays the name of the object store. |
| Domain | Displays the domain of the object store. |
| FQDN(s) | Displays a complete address to access the Federation. |
| Object Stores | Displays the number of object stores. |
| Buckets | Displays the number of buckets in an object store. |
| Object Browser | Displays a link to access the federated namespace. |

**Table 10: Object Stores**

| Term | Description |
| --- | --- |
| Name | Name of the object store. |
| Buckets | Number of buckets for the particular object store |
| Objects | Number of objects for that particular object store. |
| Capacity Usage (logical) | Displays the object storage capacity usage. |
| Version | Displays the version of Nutanix Objects in which the object store was created. |
| Public IPs | Displays the public IP addresses of object store. These are presented as links that can be used to launch the Nutanix Objects Browser. |
| PC Location | Displays the location of the object store. |

### Deleting Federated Namespace

You can delete an existing federated namespace.

### About this task

### Note:

- A healthy federation can be deleted• Bulk deletion is not allowed.• If a federation service fails to start, it can be deleted.

To delete a federated namespace, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In Application Switcher, select Objects > Federated Namespaces. . Select the required Federated Namespace to delete. Note: Clear out all the objects and buckets stored in the selected object store.

| The | Federated Namespace page appears. |
| --- | --- |

4. Click Delete Federation. The federated namespace is deleted.

### What to do nextYou can view the different error messages in federated namespace. For more information, see Errors in

Federated Namespace on page 64.

### Errors in Federated Namespace

When an error occurs while creating a federated namespace, the description lists the type of error.Following are the different types of error seen in a federated namespace.

**Table 11: Errors**

| Error type | Description Steps to recover |
| --- | --- |
| Addition Failure | Inability to add a new object store 1. Check the object store. to an existing federation. 2. Get back the object store to a healthy state. |
| Backend Unreachable | When services of some of the 1. Check the object store. |
| object stores that are a part of the | 2. Get back the object store to a |
| federation are down. | healthy state. |
| Creation Failed | When the service cannot be 1. Check the object store. |
| created. | 2. Get back the object store to a healthy state. |
| Deletion Failure | Inability to delete an object store 1. Check the object store. as it is hosting buckets or objects. 2. Clear out all the objects and buckets stored in the selected object store. |
| Transport error | When core members are 1. Check the object store. unreachable. 2. Get back the services to a healthy state. Objects | Federation Overview | |

{% endraw %}
