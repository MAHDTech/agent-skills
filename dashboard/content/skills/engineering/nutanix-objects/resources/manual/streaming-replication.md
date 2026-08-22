+++
title = "streaming-replication"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-objects"
+++

# Nutanix Objects Manual: Streaming Replication and Multi-Site DR

## NUTANIX OBJECTS STREAMING

## REPLICATION

Nutanix Objects streaming replication enables automatic and asynchronous copying of source buckets to the target buckets in different Nutanix Objects instances.You can now create up to three replication rules. This provides you with the ability to replicate a single source bucket to a maximum of three destination buckets. These destination buckets can be on Nutanix Objects on the same PC cluster or on a remote PC cluster, or a Cloud Storage Endpoint.For more information, see Types of Bucket Replication on page 141.In Nutanix Objects, for version-enabled buckets, you can also replicate the delete markers to a destination bucket. You can only replicate a non-versioned bucket to an S3 destination bucket. Note: Nutanix Objects streaming replication is only supported for buckets created using the S3 protocol. Nutanix Objects streaming replication helps you with the following:

- Maintain another copy of data for disaster recovery.• Maintain another copy of data at different sites for local access. This helps to minimize latency in

accessing data.

### Types of Bucket Replication

The Nutanix Objects Streaming Replication feature supports two types of bucket replication: Replication to Nutanix Objects and Replication to S3. Replication to Nutanix Objects DestinationYou can replicate the data to a destination bucket in the same Prism Central instance or a remote Prism Central instance in Nutanix Objects. Replication is supported for both federated and non-federated buckets.You can replicate the bucket to Nutanix Objects destination in the following ways:

| • | Nutanix Objects instances within the same Prism Central instanceReplicating a bucket to Nutanix Objects instance within the same Prism Central instance involves the |
| --- | --- |

single step of creating a replication rule.For more information, see creating a replication rule.

| • | Nutanix Objects instances within different Prism Central instances Replicates buckets to Nutanix |
| --- | --- |

Objects instance within a different Prism Central instance. This involves three high-level steps.

- Add the remote Prism Central clusters as an availability zone. For more information, see Adding the

remote PC clusters as an availability zone.

- Perform IAM synchronization between the source Prism Central cluster and remote Prism Central

clusters. For more information, see Setting up IAM Synchronization with a Different Prism Central on page 148.

- Create a replication rule. For more information, see Creating a replication rule.

Objects | Nutanix Objects Streaming Replication |

Replication to an S3 DestinationYou can replicate the data from a non-versioned bucket to an S3 destination bucket. The destination bucket can be versioned or non-versioned.For more information, see Creating a replication rule.Object user metadata is replicated, but the destination object has its own system metadata. For example, the creation time of the object at the destination might differ from the creation time of the source object.After you set up the replication rule between the source and destination bucket, you can track the replication statistics for each bucket.For more information, see Viewing Replication Statistics for a Bucket. Points To Remember for Replication to Nutanix ObjectsNote the following points about Nutanix Objects streaming replication when buckets are replicated to Nutanix Objects:

- If the Nutanix Objects instances belong to different Prism Central clusters, a trust must be set up

between the Prism Central clusters by adding each other as Availability Zones.Availability Zone - A physically isolated site where you can replicate the data that you want to protect. A Prism Central instance represents an availability zone.

- For replication-enabled buckets, versioning, and WORM modifications are prevented.• Nutanix Objects streaming replication between buckets relies on the connection (for example, a secure

VPN connection) for encryption.

- Network Address Translation (NAT) performed by any device in between two Nutanix Objects instances

is not currently supported.

- Proxy between Nutanix Objects instances is not supported.

### Nutanix Objects Streaming Replication Guarantees and Topologies

This section covers the replication guarantees, how replication works, and supported replication topologies.

### Replication GuaranteesThe following object attributes are replicated:

- Object operations - Object PUT, Object Copy, Updates (PutTags, PutObjectLock), and Delete• Object metadata - ETag, create and modification time, and lock property• Version numbers, User metadata, and Tags

### Note:

- Creation and modification timestamps are not replicated to non-Nutanix targets such as AWS

S3 buckets or third-party S3-compatible endpoints.

- Source and destination buckets are independently managed and not replicated. Also, any

changes to the bucket policies (for example, access or lifecycle policies) are not replicated.

### How Replication WorksNote the following points about how the Nutanix Objects streaming replication works:

- Replication starts as soon as the object gets written on the source bucket.

Objects | Nutanix Objects Streaming Replication |

- The replication completion time might vary depending on the object size and other factors such as

available bandwidth, number of replications, and so on.

- Nutanix Objects replications might not strictly follow the same time order in which they are written on the

source bucket.

- Replication of the versions of an object might not happen in sequential order, but they are replicated

eventually.

### Replication TopologiesSee the Replication Topologies Supported for Nutanix Objects as Destination on page 143 and

Replication Topologies Supported for S3 Destination on page 146 section.

### Replication Topologies Supported for Nutanix Objects as Destination

This section lists the topologies that you can use for your Nutanix Objects replication scenarios.

### Note:

- Ensure that applications do not perform conflicting write operations (objects with the same

name) on the remote bucket while replication is enabled.

- The I/O operations performed on the remote bucket are unrestricted. You can perform read-

and-write operations on a remote bucket.

- For an N:1 replication relation, a bucket can have a maximum of five inbound relationships. For

example, Bucket A can be the destination bucket for a maximum of five source buckets.

- For a 1:N replication relation, a bucket can have a maximum of three outbound relationships.

For example, Bucket A can be the source bucket for a maximum of three destination buckets.

### Replication TopologiesThe following are the topologies that you can use for your replication scenarios:

Single-Replication RelationIn this topology, you replicate objects one way from the source to the destination.

### Figure 23: Single Replication - Unidirectional

Different buckets on the source Nutanix Objects instance can replicate to buckets belonging to different Nutanix Objects instances. Objects | Nutanix Objects Streaming Replication |

### Figure 24: Single Replication - Different Nutanix Objects Instances

Bidirectional Replication RelationIn this topology, you can set up a bidirectional-replication relation between a pair of buckets. Nutanix Objects written on one side get replicated on the other side.Independent replication relations between the buckets have to be created, that is, create a replication rule with Bucket A as the source and Bucket B as the destination and the other way around.

### Note: Ensure that the application does not write conflicting objects (objects with the same name) on both

buckets while replication is enabled.

### Figure 25: Bidirectional Replication

Chain Replication RelationIn this topology, objects from Bucket A can be replicated to Bucket B, and objects that originated in Bucket B can be replicated to Bucket C Objects | Nutanix Objects Streaming Replication |

### Figure 26: Chain Replication

N:1 Replication RelationIn this topology, a bucket can be a destination to many source buckets.

### Figure 27: N:1 Replication

1:N Replication RelationIn this topology, a source bucket can replicate to a maximum of three destination buckets. You can have a maximum of three rules in this topology. By default, the priority of replication on all three destination buckets is equal. Objects | Nutanix Objects Streaming Replication |

### Figure 28: 1:N Replication

### Replication Topologies Supported for S3 Destination

This section lists the topologies that you can use for your S3 replication scenarios.

### Replication TopologiesThe following are the topologies that you can use for your S3 replication scenarios:

Note: These replication relations are unidirectional. 1:1 Replication RelationIn this topology, you replicate objects one way from the source to the destination.

### Figure 29: 1:1 Replication - Unidirectional

N:1 Replication RelationIn this topology, a bucket can be a destination for many source buckets. Objects | Nutanix Objects Streaming Replication |

### Figure 30: N:1 Replication - Unidirectional

### Prerequisites for Replication to Nutanix Objects

The following are the prerequisites for bucket replication to Nutanix Objects:

- Nutanix Objects instances containing the source and destination buckets must be deployed with

Nutanix Objects version 3.0 or later.

- For Nutanix Objects instances within different Prism Central instances replication, ensure to deploy

Nutanix Objects instances with IAM users if you want the same users to have access to the replicated objects.

- For Nutanix Objects instances within different Prism Central instances replication, you must establish a

secure VPN connection between the Prism Central clusters.For Nutanix Objects instances within the same Prism Central cluster, you must establish a secure VPN connection between the Prism Element clusters.

- For IAM pairing, both the source and target must have Nutanix Objects Manager version 5.0 or later.• Ensure that the firewall allows traffic to port 7100 on the load balancers for Nutanix Objects instances

for replication.

- The pod network of the source Nutanix Objects cluster must not overlap with the public network of the

target Nutanix Objects cluster. Also, the public network of the target Nutanix Objects cluster must not use the 10.100.x.x network range.

### Adding Remote Prism Central as Availability Zone

For Nutanix Objects instances within different PC instances replication, you need to add the remote PC clusters as an Availability Zone (AZ) on the source PC cluster to establish a secure connection.

### About this taskTo add the remote PC as an availability zone, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . In the Application Switcher, select Infrastructure > Administration > Availability Zones.

### Figure 31: Administration - Availability Zones Page

3. Click Connect to Availability Zone.4. In the Availability Zone Type list, click Physical Location.5. Enter the IP address and login credentials of the remote PC in the corresponding boxes.6. Click Connect. The remote PC gets added as an AZ in the source PC and the connectivity status is shown as Reachable.

### Setting up IAM Synchronization with a Different Prism Central

To replicate Nutanix Objects, perform IAM synchronization between the source and remote Prism Central instances.

### About this taskThe IAM users of the source Nutanix Objects instance is replicated to the destination Nutanix Objects

instance belonging to a different Prism Central with the same access key and secret key pair. The admin or bucket owner needs to provide permission to the users for the replicated buckets.

### Note: For IAM pairing, if either the source or target uses Nutanix Objects Manager version 5.0 or later, the

other must also be on version 5.0 or later. A non-admin user can set up IAM synchronization with a different Prism Central only after an administrator assigns a role with the following permissions:

- Create Object Store• View Object StoreFor more information on the built-in roles in Prism Central, see Built-in Role Management in the Security

Guide.To perform the IAM pairing, follow these steps:

### Procedure

1. Log on to the source Prism Central web console. . In the Application Switcher, click Objects.3. Click the Access Keys tab.4. Click IAM Replication Settings, and then click Add IAM Pairing.5. In the Target Prism Central list, select the remote Prism Central that you added as an Availability Zone (AZ) in the source Prism Central.

| • Only the Prism Central instances added in | Administration > Availability Zones appear in this list. |
| --- | --- |

- The target Prism Central IAM must have all the Active Directory configured in this source Prism

Central IAM.

### Figure 32: Add IAM Pairing

6. Click Connect to complete the pairing.

| After you click | Connect, all existing users get replicated to the target Prism Central IAM. You can |
| --- | --- |
| monitor the progress of the replication on the | IAM Replication Settings page. Once the IAM pairing is |

complete, all the future users and keys that are created and deleted get replicated to the target Prism Central IAM.If any of the replication fails, go to the

### IAM Replication Settings page and click Sync to replicate any

unreplicated users to the target Prism Central. The administrators can view the users for whom the replication failed and use the export option to download the list of errors.

### Note:

- You cannot add more than five IAM replication targets.• The

### Sync option does not replicate user or key deletions. For failed delete replications, log

in to the target Prism Central and manually delete the users and keys.

### Prerequisites for Replication to S3

This section lists the prerequisites for bucket replication to S3.The prerequisites for replicating buckets to S3 are as follows:

- The source bucket must be non-versioned.

Objects | Nutanix Objects Streaming Replication |

- While configuring the replication rule, the source and destination bucket must have the same WORM

policies (such as retention period). Changing the WORM policy parameter on the destination bucket after setting the replication rule might result in unexpected behavior. For example, if the object is deleted on the source, the destination bucket might reject the delete request.

- You must set up a default lifecycle policy on the destination S3 bucket.

| On AWS, the lifecycle policy can be set by the S3 bucket. Click | Management > Add Lifecycle Rule. |
| --- | --- |
| Select | Clean up incomplete multipart uploads checkbox. Enter the number of days as expiry |

duration. Here, the number of days depends on the object size, connection bandwidth, workload, and system resources. For example, 30 days is a good starting point.

- Make sure that the destination bucket has the recommended settings. For more information, see Cloud

Bucket Endpoint Recommended Settings on page 114.

### Prerequisites for Creating a Replication Relation

Before you create a replication relation, ensure that you meet the following requirements:

- For remote replication of Prism Central clusters, ensure that the fully qualified domain names (FQDN) of

the object store instances in the two different Prism Central clusters are different.

- If you intend to replicate data to a bucket on another Prism Central cluster, make sure to synchronize

IAM between the source and the remote Prism Central cluster.

- Nutanix Objects does not automatically replicate the bucket access permissions to the destination

bucket. You must manually assign them to the destination bucket.

- Lifecycle policies are not replicated and can be assigned independently to both source and destination

buckets.

- When you replicate a bucket created in Nutanix Objects version 3.1 with a legal hold to a bucket

created in Nutanix Objects version 3.0, the system does not replicate the legal hold attributes.

- The source and destination buckets must have the same versioning, WORM state, and WORM

retention period.

### Note: For S3 replication, the source bucket must be non-versioned, while the destination bucket can be

either versioned or non-versioned.

- When you create a replication rule, only the objects that you add to the buckets or modify afterward is

transferred to the destination bucket. For more information on replicating existing objects in the bucket, see Creating Replication Relation for Buckets.

- Versioning and WORM modifications are not allowed for replication-enabled buckets. To make changes,

you must first delete the replication rule, make the necessary edits on the buckets, then recreate the replication rule.

- When creating a replication rule for buckets located in different object storage instances, first establish

a relationship with the object store that contains the destination bucket using the Prism Central user interface. For example, to replicate Bucket 2 in object store A to Bucket 6 in object store B, use the user interface for the first step. After that, you can create additional replication rules for any buckets in object store A to any other bucket in object store B using either the user interface or S3 API.

- A non-admin user can create replication rules between buckets only after the administrator creates an

access control policy for the non-admin user on a role in Prism Central. The specific role must have the following minimum permissions, and the non-admin user must be assigned to specific Nutanix Objects instances:

- Edit Buckets

Objects | Nutanix Objects Streaming Replication |

- View Buckets• View Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator: A Super Admin or a Prism Admin in Prism Central.• Non-admin user: A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

| • Users with | Edit Buckets permission can configure bucket replication on all the buckets on an assigned |
| --- | --- |

object store.

| • If the target object store is on the same Prism Central cluster, the user must have | View Object Store |
| --- | --- |

access to that target object store as well. However, if the target object store is on a different Prism Central cluster, the user is allowed to select any object store available on the target Prism Central cluster.

### Creating Replication Relation for Buckets

You can create a replication relation for buckets that allows you to replicate data in a bucket to other buckets. Nutanix Objects supports replication only to buckets configured for the AWS S3 endpoint, or to any Nutanix Objects bucket hosted on the same Prism Central or a remote Prism Central.

### Before you begin

- Ensure that you meet the prerequisites for creating a replication relation and replication to Nutanix

Objects and S3. For more information, see Prerequisites for Creating a Replication Relation on page 150, Prerequisites for Replication to Nutanix Objects on page 147, and Prerequisites for Replication to S3 on page 149.

- If you want to replicate the data to an S3 destination bucket, make sure you configure a Cloud Bucket

Endpoint for the destination bucket. See Configuring a Cloud Bucket Endpoint in Nutanix Objects on page 110.

### About this taskTo create a replication relation for buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which stores the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, and then from the buckets list table, select the source bucket.5. Click Actions > Replication Rules or click the bucket name and then Replication. . Click Outgoing Replication Relation.

| The | Create Replication Relation page appears. |
| --- | --- |

### Figure 33: Create Replication Relation Page

### 7. In the Target tab, select the Endpoint Type as Nutanix Objects or Cloud Storage Endpoint configured

for the bucket.For more information, see Viewing Cloud Bucket Endpoints on page 118 .Nutanix Objects supports only the S3 Cloud Bucket Endpoint for replication.

### 8. (Only for Nutanix Objects endpoint) Select Nutanix Objects from the Endpoint Type dropdown, enter

| the following details and click | Next: |
| --- | --- |
| a. | Namespace: Select a namespace to replicate the object. You can select any local or federated |

namespace.You can replicate the bucket on the same Prism Central cluster or on a remote Prism Central

| cluster. You can select | Local AZ to replicate the source bucket to a destination bucket on the same |
| --- | --- |
| Prism Central cluster. You can select a remote Prism Central from the | Federated list to replicate |

your bucket to a destination bucket on the remote Prism Central cluster.

| b. | Bucket: Enter the name of the destination bucket. |
| --- | --- |

### 9. (Only for S3-compatible endpoint) Select AWS from the Endpoint Type dropdown, select the Bucket

Name from the dropdown, and click Next.Nutanix Objects supports only buckets configured for the AWS S3 endpoint for replication. Objects | Nutanix Objects Streaming Replication |

### 10. In the Rules tab, under the Sync section, select one of the following options:

| » | Yes: Select this option if you want to replicate the existing objects upon rule creation. |
| --- | --- |
| » | No: Select this option if you do not want to replicate the existing objects upon rule creation. |

Replication is skipped for objects that do not match the current versioning state of the bucket and generates an alert.

| • | When versioning is enabled: Nutanix Objects are replicated along with their available version |
| --- | --- |

history. However, replication is skipped for objects that do not have a version history, such as NULL versioned objects.

| • | When versioning is disabled: Only NULL versioned objects are replicated, and replication |
| --- | --- |

excludes any associated version history. In this case, objects that have versions and their respective version histories are not replicated. For more information on replication alerts, see Nutanix Objects Specific Alerts on page 174.If the user specifies filters, only the objects that comply with those filters will be replicated.If the bucket already contains a mix of versioned and non-versioned objects, see Combination of Versioned and Non-Versioned Nutanix Objects Replication on page 159. 11. Under the Rules section, click + Add to add a new rule for bucket replication. A maximum of 1000 rules can be created across all destinations. For more information on the sample replication configurations, see Sample Replication Configurations on page 156.

| a. | Name/ID: Enter a valid name for the filter. |
| --- | --- |

### Note:

- Each rule must have a unique alphanumeric ID. For example, rule_10. The ID must be

unique across all the rules set up among all the destinations.

- Name/ID can be a maximum of 255 characters.

| b. | Priority: Each rule must have a unique numeric priority. The priority must be unique across all the |
| --- | --- |

rules set up among all the destinations. The higher the number, the higher the priority of that rule.

| c. (Optional) Select the | Replicate delete operations check box to replicate delete markers to the |
| --- | --- |

destination bucket.

| • If | Replicate delete operations is enabled, Tag is disabled. Tag based filtering cannot be used |
| --- | --- |

with Replicate delete operations.

| • Selecting the | Replicate delete operations check box enables the DeleteMarkerReplication |
| --- | --- |

parameter and replicates delete markers to the destination bucket.

- The DeleteMarkerReplication status now appears in the Nutanix Objects user interface.

You can also find the DeleteMarkerReplication status using S3 API. The relations Objects | Nutanix Objects Streaming Replication |

created using Nutanix Objects versions earlier than Nutanix Objects 3.5 return the correctDeleteMarkerReplication enabled status.

- Set DeleteMarkerReplication as Disabled when you create a replication relationship. This

setting ensures that the replicated objects are not deleted from the destination bucket when the corresponding object is deleted from the source bucket.However, if objects that are pending replication are deleted from the source, these objects are not replicated to the destination bucket. Hence, the objects are not be available on the destination bucket.

- You can create a maximum of three rules for the 1:N replication.• For more information on the delete marker replication scenarios, see Delete Marker Replication

Scenarios on page 155.

- When you have an overlapping set of rules where both rules have different values for

DeleteMarkerReplicationStatus, only the delete operation is replicated if the highest priority matching rule has the DeleteMarkerReplicationStatus enabled. However, tag-based rules are not considered even if they have the highest priority.For example, consider a scenario where the highest priority rule has tags and DeleteMarkerReplicationStatus disabled, but a lower priority matching rule (with no tags) has the delete marker replication enabled. In this case, the delete operation is replicated. If there is no rule matching, the delete operation is not replicated.

| d. (Optional) | Prefix: You can add any prefix. You can replicate only the object whose name matches |
| --- | --- |

the prefix.Prefix length must be less than 255 characters.

| e. (Optional) | Tag: You can replicate only if the object tags match with the rule tags. |
| --- | --- |

- A maximum of 255 tags per replication rule is allowed.• The object must match with both prefix and tag if specified. If you do not provide a prefix or tag,

all the objects in the bucket are replicated.

| • If | Tag is enabled, Replicate delete operations is disabled. Replicate delete operations cannot |
| --- | --- |

be used with Tag based filtering.

- Prefix or tag replication does not work retroactively. Replication takes place when an object

is uploaded. If you edit the replication rule, all the objects uploaded after edit will be matched against the new rule.

- Replication Status (Paused/Enabled) is at an endpoint level. All the replication rules with the

same destination must have the same replication status.

- When you upload an object, the object is matched against the filters set for the replication

relations. If at least one of the filters matches, a replication_info_map entry is created for that object and replicates the object immediately. For the background path, it is possible that the replication filter no longer matches the object, therefore, before replicating the object in the background path, the object against the filters is matched one more time to ensure the decision of replication.

- A tag consists of a key-value pair where the value field is optional. Also, the keys provided

must be unique within the replication rule. For example, user cannot specify <key1, val1> and<key1, val2> .

| f. Click | Add to add the rule. |
| --- | --- |
| The list of | Rules appears in new page. |

12. (Optional) Select an existing replication relation, and click Action > Add to add multiple rules. Objects | Nutanix Objects Streaming Replication |

### 13. (Optional) Select an existing replication relation, and click Action > Remove to delete an existing

replication relationship.

### 14. (Optional) Select an existing replication rule, and click Action > Sync to sync existing objects of

already created replication rule.The operation might have a significant impact on bandwidth and performance for several hours.

| The status is changed to | Active on completion of sync and an alert is raised. |
| --- | --- |

For more information on replication alerts, see Nutanix Objects Specific Alerts on page 174. 15. Click Save.

| The new replication rule appears in the | Replication Relation table. The replication relation between |
| --- | --- |
| the buckets is established. You can see the status in the | Status column. The replication time might |

vary and depends on various factors, such as object size, workloads on the objects cluster, and so on.

| This replication rule is applied to the objects based on the | Sync option selected in Step 10. The status |
| --- | --- |
| is changed to | Active on completion of sync and an alert is raised. |

For more information on replication alerts, see Nutanix Objects Specific Alerts on page 174.

- You cannot pause, delete, or edit the replication rule while synchronization is in progress. You can

only abort the synchronization.

- For disconnected Availability Zones: If you remove a Prism Central from the Availability Zones after

setting up the IAM synchronization and bucket replication:

- Successive IAM user additions and deletions are not replicated.• Existing bucket replication is not affected. However, you cannot create new replication rules to

the object store deployed in the disconnected availability zone.

| You can select the destination bucket and click | View on the Replication page to view the replication |
| --- | --- |

rule details.You can also view the overview or statistics of the replication rule by clicking the

### Overview tab. For

more information, see Viewing Bucket Replication Statistics on page 161. What to do nextYou can also delete a replication rule. For more information, see Deleting a Replication Rule of a Bucket on

### Delete Marker Replication Scenarios

This section describes the delete marker behavior in a few replication scenarios. Note: Only non-versioned source buckets are replicated to S3 destination buckets. In the case of unidirectional replication, the following is the behavior of the delete marker:When the DeleteMarkerReplication is enabled:

- For a non-versioned bucket, the delete operation is replicated to the destination bucket.• For a versioned bucket, the delete marker is replicated to the destination bucket.When the DeleteMarkerReplication is disabled:

- For a non-versioned bucket, the delete operation is not replicated to the destination bucket.• For a versioned bucket, the delete marker is not replicated to the destination bucket.

Objects | Nutanix Objects Streaming Replication |

### Sample Replication Configurations

This section provides some examples of the replication configurations. Replication Configuration with One RuleThe following basic replication configuration specifies one rule. This rule replicates all the objects in the source bucket to the destination bucket. For example, bucket-example in this case. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <ReplicationConfiguration> To choose a subset of objects to replicate, you can add a filter. In the following configuration, the filter specifies an object key prefix. This rule applies to objects that have the prefix Tax/ in their object names. Priority is irrelevant because there is only one rule. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <Prefix>Tax/</Prefix> </Filter> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <ReplicationConfiguration> In the following configuration, the filter specifies an object key tag. This rule applies to objects that have the tag vm1, us-west-1 in its list of tags. Note: The DeleteMarkerReplicationStatus must always be disabled for tag-based rules. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <Tag> <Key>vm1</Key> <Value>us-west-1</Value> </Tag> </Filter> <DeleteMarkerReplication> Objects | Nutanix Objects Streaming Replication |

<Status>Disabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <ReplicationConfiguration> In the following configuration, the filter specifies one prefix and two tags. The rule applies to the subset of objects that have the specified key prefix and tags. Specifically, it applies to object that have the Tax/ prefix in their key names and the two specified object tags in its list of object tags. Priority doesn't apply because there is only one rule.

### Note: The DeleteMarkerReplicationStatus must always be disabled for tag-based rules

<ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <And> <Prefix>Tax/</Prefix> <Tag> <Key>IRS</Key> <Value>2024</Value> </Tag> <Tag> <Key>IRS2</Key> <Value>2023</Value> </Tag> </And> </Filter> <DeleteMarkerReplication> <Status>Disabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <ReplicationConfiguration>

### Replication Configuration with Two RulesIn the following replication configuration:

- Each rule filters on a different key prefix so that each rule applies to a distinct subset of objects. In this

example, we will replicate objects with the key names Tax/doc1.pdf and Finance/project1.pdf, but it does not replicate objects with the key name Logs/snapshot1.txt.

- Rule priority is irrelevant because the rules apply to two distinct sets of objects. The next example

shows what happens when rule priority is applied. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <Prefix>Tax/</Prefix> Objects | Nutanix Objects Streaming Replication |

</Filter> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <Rule> <ID>rule_id_2</ID> <Status>Enabled</Status> <Priority>2</Priority> <Filter> <Prefix>Finance/</Prefix> </Filter> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> </ReplicationConfiguration> Replication Configuration with Two Overlapping RulesIn this configuration, the two rules specify filters with overlapping key prefixes with same destination, Doc and Document. Both rules apply to objects with the key name Document1. In this case, Nutanix Objects uses the rule priority to determine which rule to apply. The higher the number, the higher the priority. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <Prefix>Doc</Prefix> </Filter> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <Rule> <ID>rule_id_2</ID> <Status>Enabled</Status> <Priority>2</Priority> <Filter> <Prefix>Document</Prefix> </Filter> <DeleteMarkerReplication> <Status>Disabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> </ReplicationConfiguration> Objects | Nutanix Objects Streaming Replication |

If you delete the object Document1, the delete will not be replicated to the destination as the rule with

| priority 2 has the | DeleteMarkerReplicationStatus disabled. |
| --- | --- |

In this configuration, the two rules specify filters with overlapping key prefixes with different destinations,Doc and Document. Since both the rules have different destinations, priority is not applicable even if the rules are overlapping. Both rules will be applied to objects with the key name Document1 and object will be replicated to both the destinations. <ReplicationConfiguration> <Role/> <Rule> <ID>rule_id_1</ID> <Status>Enabled</Status> <Priority>1</Priority> <Filter> <Prefix>Doc</Prefix> </Filter> <DeleteMarkerReplication> <Status>Enabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example:example_fqdn</Bucket> </Destination> </Rule> <Rule> <ID>rule_id_2</ID> <Status>Enabled</Status> <Priority>2</Priority> <Filter> <Prefix>Document</Prefix> </Filter> <DeleteMarkerReplication> <Status>Disabled</Status> </DeleteMarkerReplication> <Destination> <Bucket>bucket-example2:example_fqdn</Bucket> </Destination> </Rule> </ReplicationConfiguration>

### Combination of Versioned and Non-Versioned Nutanix Objects Replication

If the bucket already contains a mix of versioned and non-versioned objects, follow this procedure.

### About this taskTo replicate a combination of versioned and non-versioned object, follow these steps:

### Procedure

1. Replicate all objects that correspond to the current versioning state. For more information, see Creating Replication Relation for Buckets on page 151. 2. Delete the replication relationship. For more information, see Deleting a Replication Rule of a Bucket on page 161. 3. Change the versioning state of both the source and destination buckets.4. Re-enable replication and perform another sync. All objects are successfully replicated, regardless of their versioning state. Objects | Nutanix Objects Streaming Replication |

### Pausing a Replication Rule for a Bucket

You can pause an active replication rule at any point in time. When you pause a replication, the objects are accumulated by the source. When the replication resumes, all the objects uploaded during the pause state are replicated to the destination bucket.

### About this task

### Note: Nutanix recommends not pausing the replication for more than 24 hours to avoid bloating the number

of objects pending for replication. If the replication rule is paused for more than 24 hours and if at least one object was uploaded during the pause state, the alert PauseReplicationWarningDurationTimeExceeded is raised

| to resume the replication. For more information on the alerts, see | Nutanix Objects Specific Alerts on |
| --- | --- |

page 174. To pause a replication rule for buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store that stores the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, then from the buckets list table, click the name of the source bucket.5. Click the Replication tab.6. To pause an active replication, select the replication rule and click Actions > Pause.

| The replication rule is paused and the status is changed from | Active to Paused. The objects is |
| --- | --- |

accumulated by the source.

### What to do nextYou can also resume or delete a replication rule. For more information, see Resuming a Replication Rule

for a Bucket on page 160 and Deleting a Replication Rule of a Bucket on page 161

### Resuming a Replication Rule for a Bucket

You can resume a paused replication rule at any point in time. When you pause a replication, the objects are accumulated by the source. When the replication is resumed, all the objects uploaded during the pause state are replicated to the destination bucket.

### About this taskTo resume a replication rule for buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store that stores the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, then from the buckets list table, click the name of the source bucket.5. Click the Replication tab. . To resume a paused replication, select the replication rule and click Actions > Resume.

| The replication rule is resumed and the status changes from | Paused to Active. The objects uploaded |
| --- | --- |

during the paused state are replicated to the destination bucket. What to do nextYou can also pause or delete a replication rule. For more information, see Pausing a Replication Rule for a Bucket on page 160 and Deleting a Replication Rule of a Bucket on page 161

### Deleting a Replication Rule of a Bucket

This section provides the steps to delete a replication relation. About this taskA typical scenario where you may require to delete a replication rule is if you want to modify the versioning and WORM state of a replication-enabled bucket. Since versioning and WORM changes are prevented, you need to delete the replication rule, perform the required edits on the bucket, and then create the replication rule again.

### Note: Deleting a replication relation would cause all pending replications for that relationship to be dropped

immediately. It is recommended to wait for the pending replications to complete, then start making changes by deleting the relation. To delete a replication rule for a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click the name of the object store where the bucket is deployed.

| The | Object Store page appears. |
| --- | --- |

4. Click Buckets.5. In the Buckets table, select the replication-enabled bucket.6. Click Actions > Replication Rules.

| The | Replication page appears. |
| --- | --- |

7. In the Replication Rule table, select the replication destination bucket.8. Click Delete. After the replication rule is deleted, new objects are not replicated to the destination bucket.The replication rule is deleted.

### What to do nextYou can also pause a replication rule. For more information, see Pausing a Replication Rule for a Bucket

on page 160.

### Viewing Bucket Replication Statistics

This section describes the steps to view the replication statistics, such as the replication properties and the average bandwidth for a bucket. Objects | Nutanix Objects Streaming Replication |

### Before you beginA non-admin user can view replication statistics only after the administrator creates an access control

policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with View Buckets permission can view the replication statistics on all the buckets on an

assigned object store.

### About this taskTo view the replication statistics, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which stores the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, and then from the Buckets list table, select the replication-enabled bucket.5. Click tripple dot horizontal icon > Replication Rules.

| The | Replication page appears. Objects | Nutanix Objects Streaming Replication | |
| --- | --- |

6. Click the Overview tab.

| The replication properties and the average bandwidth appears on the | Overview page. |
| --- | --- |

### Figure 34: View Replication Overview

If the bucket has both inbound and outbound traffic, you can select either of the traffic types from the Traffic Type list and then select the corresponding source or destination bucket.

| • | Outbound from this bucket - Shows the statistics for replication of objects from the selected |
| --- | --- |

bucket (source) to the destination bucket.

| • | Inbound to this bucket - The selected bucket can be a destination to many source buckets. |
| --- | --- |

Inbound statistics show the data for the replication of objects to the selected bucket. You can select the source bucket and view the inbound statistics from that bucket.

### Note: The updates to the destination bucket happen periodically. This might result in a lag in the

destination bucket statistics display.

| • | Destination Bucket - The destination bucket for replication. |
| --- | --- |
| • | Destination Bucket Object Store - The name of the object store that contains the destination |

bucket.

| • | Last replication point - Point-in-time up to which all the objects created on the source bucket have |
| --- | --- |

been replicated.

| • | Number of objects pending replication - Object count pending replication. |
| --- | --- |
| • | Objects size pending replication - Total amount of data pending replication. |
| • | Average bandwidth - The rate of the amount of data transferred from the source to the destination. |

For inbound relationships, the average bandwidth is the cumulative value of all the incoming data from the source buckets. The bandwidth graph helps you to visualize the progress of the replication. What to do nextYou can create more replication rules for buckets. For more information, see Creating Replication Relation for Buckets on page 151.

### Achieving Fault Tolerance for IAM on Nutanix Objects

This section describes how you can use IAM replication and bucket replication to withstand data center failures. Objects | Nutanix Objects Streaming Replication |

### Note: This section is applicable only if IAM-HA is not enabled. For more information, see Nutanix Objects

IAM-High Availability Overview on page 90. IAM Service on Nutanix ObjectsAt present, all the Object Stores (Nutanix Objects instance) in the same PC share a single IAM. This IAM instance resides in the first object store (primary) that is deployed in the PC. If that primary object store becomes unavailable, the IAM service will not be available for all secondary object stores of that PC.

### Figure 35: IAM Service on Nutanix Objects

For example, you have one PC managing all your clusters that is PC1. This PC1 has four object stores (OSS 1, 2, 3 and 4). However, IAM 1 resides only in the first object store OSS 1 (primary object store). All other secondary object stores on that PC (OSS 2, 3 and 4) are for back up of OSS 1 and relies on IAM 1 for authentication. If OSS 1 goes down, then all other back ups also goes down. In this case, Nutanix recommends you the following IAM replication configuration. Note: The Nutanix Objects cluster will work even when the PC fails. Recommended IAM Replication ConfigurationsNutanix recommends to transition to multiple PC, that is multiple IAM solution for achieving fault tolerance for IAM replication. You can have one or more PCs to manage IAM replication. The remote PC can have different object stores, and you can set up IAM replication between primary PC and remote PC.

### Figure 36: Nutanix Objects IAM Service with Replication across Prism Centrals

Objects | Nutanix Objects Streaming Replication |

For example, in the preceding image, you have three PCs managing all your clusters that is PC1, PC2, and PC3. PC1 has four object stores (OSS 1, 2, 3 and 4) with IAM 1 residing only in the first object store OSS 1 (primary object store). All other secondary object stores on that PC (OSS 2, 3 and 4) relies on IAM 1 for authentication.Similarly, PC2 and PC3 have different object stores respectively (OSS 5, 6, 7, 8 and OSS 9, 10, 11, 12) with IAM 2 and IAM 3 residing only in the first object stores OSS 5 and OSS 9 (primary object stores) of PC2 and PC3.Now as recommended, you use PC2 and PC3 for replication of PC1. In this case, even if any object store in PC1 goes down, then replicated buckets will be available in PC2 and PC3 without any disruption. Objects | Nutanix Objects Streaming Replication |

