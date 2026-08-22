+++
title = "bucket-management-and-features"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-objects"
+++

# Nutanix Objects Manual: Bucket Management and Advanced Features

## BUCKET CREATION, OPERATIONS, AND

## BUCKET POLICY CONFIGURATION

Buckets store objects in an object store. Learn how to create buckets and configure policies for S3 and NFS protocols.Before you can upload objects to a bucket, you must create a bucket in an object store. You can create a bucket using both S3 and NFS protocols. You can also configure a S3 bucket while creating it except applying the WORM policy.A user with Edit Buckets permissions can perform all bucket operations, such as create, update, and delete on all the buckets on the assigned Object Store.You can enable Lifecycle Policies, Versioning, WORM, Replication, Static Website, CORS, and Notifications to an S3 bucket. These S3 features cannot be enabled in a multi-protocol access bucket (NFS). However, you can also filter, update, share, and delete a bucket, and view all the existing buckets list created through both protocols.

### Bucket Naming Conventions

Naming a bucket requires adherence to specific rules.The name of a bucket must conform to the following rules:

- Be a unique DNS compliant name within a deployed bucket instance.• Can contain alphanumeric, dot, or hyphen characters.• Begin with a lowercase letter or a number.• Cannot contain uppercase and special characters.• Minimum of 3 and a maximum of 64 characters long.

Note: You cannot change the bucket name after creating the bucket.

### Creating and Configuring an S3 Bucket in Nutanix Objects

You can create and configure the bucket settings. There are multiple options available for configuring a bucket and all are accessible at the time of creating a bucket. However, you cannot configure a WORM bucket while creating a bucket. You can edit WORM policies only after creating a bucket. You cannot enable multi-protocol access on the S3 bucket. Before you beginEnsure that the bucket names are unique for all the users.Non-admin users can create and configure S3 buckets only if assigned a role with Edit Buckets, View Buckets, and View Object Store permissions by the admin user in Prism Central. Admins are Super Admins or Prism Admins in Prism Central; non-admins are Prism Central users without admin privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.Users with Edit Bucket permission can perform a create bucket operation on the buckets available in the object stores that are assigned to the respective users. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### About this task

### Note: While creating a new versioned bucket, the system will automatically create a lifecycle policy to expire

the delete markers if they are the only remaining object versions. To create and configure a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store in which you want to create a bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click Buckets > Create Bucket. You can also create a bucket without enabling versioning and life cycle policies. 5. On the General Settings section, type a name for your bucket and select a namespace. For more information on namespace, see Federation Overview on page 57 .For more information on naming buckets, see Buckets Naming Conventions. 6. On the Object Store section, select the object store name. This option becomes available after all federation members are upgraded to Nutanix Objects 5.3, and the Federation’s overall version reaches 5.3.

### 7. (Optional) On the Object Versions section, configure the following:

For more information on versions, see Object Versioning.

| a. | Enable Versioning: Select this check box to enable versioning on objects and to keep all the |
| --- | --- |

versions on the same bucket.To apply the life cycle policy with versioning enabled, see Rules for Expiration based Lifecycle Policy on page 107. When you select versioning, you are able to recover objects from accidental deletion or overwriting.

### 8. (Optional) On the Lifecycle Policies section, select the following:

For more information, see Lifecycle Policies.

| a. | Expire current objects version after:: Select to type a time period after which the current version |
| --- | --- |

of the object expires.You can specify the number in days, months, or years.

| b. | Expire previous objects versions after: Select this check box to enter a time period to delete all |
| --- | --- |

the previous versions of the objects. This option is available only if versioning is enabled.You can specify the number in days, months, or years.

- If versioning is not enabled, the current object deletes permanently. When you enable versioning, the

current object becomes a past object.

- Multi-protocol access cannot be enabled on the S3 bucket. If you want to create buckets with multi-

protocol access, see Creating and Configuring an NFS Bucket in Nutanix Objects on page 101. 9. Click Save. The bucket is created successfully. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### What to do nextAfter creating a bucket, you can perform object operations from the Nutanix Objects Browser or the

S3 APIs. For more information, see Object Operations in Nutanix Objects Browser on page 223 and Supported S3 APIs. You can also configure object-tiering lifecycle rules. For more information, see Creating a Lifecycle Rule in Nutanix Objects on page 115.

### Creating and Configuring an NFS Bucket in Nutanix Objects

Create and configure an NFS bucket with multi-protocol access in Nutanix Objects. S3-specific features are not supported on these buckets.

### Before you beginMake sure you are aware of the uses cases, recommendations, NFS-S3 interoperability, and limitations

of NFS on Nutanix Objects. See Use Cases and Recommendations for Network File System on Nutanix Objects on page 14, Network File System and Simple Storage Service Interoperability on page 15 and Limitations of Network File System on page 49.A non-admin user can create and configure NFS buckets only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can perform a create bucket operation on the buckets

available in the object stores that are assigned to the respective users.

### About this task

- If you deployed object store prior to Nutanix Objects 3.3 version, then you will not get the option to

enable NFS for the bucket of that object store. Similarly, if you upgrade to later versions of Nutanix Objects, then the option to enable NFS will not be available.

- Ensure that the bucket names are unique for all users.• As the Nutanix Objects-NFS does not support NLM (Network Lock Manager), no lock option is required

while mounting the NFS bucket.

- The total and available bytes returned in the FSSTAT response denote the logical capacity and logical

available space and not the physical capacity of the cluster which also takes RF2 into consideration. To create and configure a bucket with multi-protocol access, follow these steps: Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store in which you want to create a bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click Buckets > Create Bucket.

### Figure 16: Create Bucket Window

5. On the General Settings section, type a name for your bucket and select a namespace. For more information on namespace, see Federation Overview on page 57.For more information on naming buckets, see Buckets Naming Conventions.Versioning and lifecycle policies cannot be enabled on buckets with multi-protocol access.For more information on creating and configuring buckets for S3 features, see Creating and Configuring an S3 Bucket in Nutanix Objects on page 99 6. On the Multiprotocol Access section, select Enable NFS v3 Access. Access cannot be enabled or disabled after the bucket is created. . For owner and default permissions for S3 written objects, do the following. For files written using NFS protocol, these settings are inherited from the client.

| a. In the | Owner section, enter the UID and GID. |
| --- | --- |

Any object (file or directory) created from S3 protocol have UserID (UID) and GroupID (GID) in the NFS namespace.

| b. In the | Default Access Permissions section, by default read, write, and execute permissions are |
| --- | --- |

set for the files and directories for the owner, group (other users in a group), and others (users that are not part of any group).The following are the default permissions for files:

| • | Owner: Read, Write |
| --- | --- |
| • | Group: Read |
| • | Others: Read |

The following are the default permissions for directories:

| • | Owner: Read, Write, Execute |
| --- | --- |
| • | Group: Read, Execute |
| • | Others: Read, Execute |

You can change these permissions as needed. 8. In the Advanced Settings section, select any one of the following squash options.

| » | None: Select this option if you do not want to convert the UID and GID of the users on the server. |
| --- | --- |
| » | Root Squash: Select this option if the user has root privileges and you want to convert the |

UID and GID to an anonymous UID and GID on the server. The anonymous UID and GID are automatically generated, however, you can change it.

| » | All Squash: Select this option if you want to map all users to a single identity. This will convert the |
| --- | --- |

UID and GID of all users to an anonymous UID and GID on the server. 9. Click Create. The bucket is created successfully. 10.

### Note: Before mounting a bucket, add the client to the NFS allowlist. Only the client present in the NFS

allowlist will be given access to the NFS buckets.For more information on adding and managing clients to NFS allowlist, see Managing NFS Allowlist in Nutanix Objects on page 104. Mount the bucket from the client VM. $ sudo mount -tnfs -o nfsvers=3,proto=tcp,soft -v objectstore-endpoint-ip- address:/bucketname path/to/mount Here objectstore-endpoint-ip-address is the Nutanix Objects Public IP address which is used to send any S3 request. You can find this Public IP address in the Object Store table. For more Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

information, see Viewing Object Store Deployments on page 72. bucketname is the name of the bucket.For example, $ sudo mount -tnfs -o nfsvers=3,proto=tcp,soft -v 10.45.53.75:/test-nfs- bucket home/folder-1/mnt-point The bucket is mounted successfully. What to do nextAfter creating a bucket, you create directories, files, and symbolic links from the NFS namespace. You can also perform object operations from the Nutanix Objects Browser or the S3 APIs. For more information, see Object Operations in Nutanix Objects Browser on page 223 and Supported S3 APIs.

### Managing NFS Allowlist in Nutanix Objects

You need to add the IP addresses of the client VMs that are allowed to access an NFS bucket in an object store.

### About this task

### Note: Make sure that you add the required client IP addresses to the allowed list before you mount a

bucket. To add clients to the NFS, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store in which the bucket exists.

| The | Object Store page appears. |
| --- | --- |

4. Click Settings > NFS Clients Allowlist, and then click Add Client.

| The | Add NFS Clients window appears. |
| --- | --- |

a. Enter the IP address of the client VM in Classless Inter-Domain Routing (CIDR) format and click Add.

| b. Click | Save. |
| --- | --- |
| The newly added IP address is listed in the | Added Clients list. |

### Note: The Add button appears only when you add the first IP address to the allowlist. To add more

| clients or to manage the existing clients, click | Manage Clients. |
| --- | --- |

5. Click Manage Clients to add or remove the clients.

| The | Manage NFS Clients window appears. |
| --- | --- |

a. (Optional) Enter the IP address of the client VM in Classless Inter-Domain Routing (CIDR) format

| and click | Add to add client. |
| --- | --- |
| b. (Optional) Select one or more IP addresses from | Client IP(s) and click Remove to remove the |

clients.

| c. Click | Save. |
| --- | --- |
| The IP address is removed from the | Added Clients list. Objects | Bucket Creation, Operations, and Bucket Policy Configuration | |

What to do nextAfter adding the required clients to the NFS allowlist, you can mount the buckets and then perform tasks, such as creating files, directories, or symbolic links. For more information on mounting the bucket, see Creating and Configuring an NFS Bucket in Nutanix Objects on page 101.

### Bucket Policy Configuration in Nutanix Objects

You can configure multiple policies on a bucket, including versioning, lifecycle, WORM, static website hosting, and CORS.You can configure multiple policies in a bucket such as object versioning, lifecycle policies, WORM, static website hosting and Cross-Origin Resource Sharing (CORS). Object versioning and lifecycle policies are accessible at the time of creating a bucket, however, you can apply WORM policy only after creating a bucket.

### Note: Lifecycle Policies, Versioning, WORM, Replication, Static Website, CORS, and Notifications cannot

be enabled for buckets created using NFS protocol.

### Object Versioning in Nutanix Objects

Object versioning preserves multiple versions of objects in a bucket, supports lifecycle policies, and requires specific permissions in the Prism Central.Object versioning enables you to keep multiple versions of an object in one bucket. By default, versioning is disabled for a new bucket. You can enable versioning while creating a bucket or editing a bucket. See Creating and Configuring an S3 Bucket in Nutanix Objects on page 99.With the object versioning option, you can enable versioning on objects of that particular bucket. Object versioning varies depending on the lifecycle policies applied to the object.For more information on the lifecycle rules for object versioning, see Rules for lifecycle policy.

### Note:

- Versioning cannot be enabled for the buckets created using the NFS protocol.• For a versioned bucket, the number of objects shown for each bucket is indicative of the

number of versions of all the objects present in the bucket.

- You cannot disable but can suspend the object versioning at any time.

When you suspend versioning, the accumulation of the new object versions is stopped and previous object versions are retained.Non-admin users can configure directories only if assigned a role with Edit Buckets, View Buckets, and View Object Store permissions by the admin user in Prism Central. Admins are Super Admins or Prism Admins in Prism Central; non-admins are Prism Central users without admin privileges. For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide.Users with Edit Bucket permission can perform object versioning on the buckets available in the object stores that are assigned to the respective users.

### Lifecycle Policies in Nutanix Objects

Lifecycle policy enables you to create or update a set of rules that define actions that Nutanix Objects applies to a group of objects.With these policies, you can expire objects when no longer required or move them to a low-cost storage tier to preserve for a longer-term. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

A non-admin user can create lifecycle policies only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can apply lifecycle policies on the buckets available in the

object stores that are assigned to the respective users.

- Lifecycle policies cannot be applied for the buckets created using NFS protocol.• Rules or any updates to the rules get applied to the new objects that you create and will not

apply to the objects existing before the rule creation or update.

- Starting from Nutanix Objects 5.2 the non- retroactive lifecycle policies only apply to new or

modified objects added after you create the policy. The retroactive lifecycle policies apply to all existing objects, plus the new objects that are added later. On the older buckets, you have an option in the UI to enable retroactive lifecycle policies.To change to retroactive, see Evaluating Lifecycle Policy on page 108. You can apply these policies while creating a bucket. You can create multiple rules within a lifecycle policy. This means that different objects within the bucket can have different rules based on prefixes and tags.Example: You can create rule 1 to expire the current versions of the objects with the tag value as a dev. Similarly, you can create multiple rules with different tiering and expiring configurations and apply them to other objects using prefixes and tags.With lifecycle policies, you can configure a lifecycle policy rule to:

- Automatically delete objects after a specified number of days or months or years from the date of object

creation.

- Tier objects to an S3-compatible object storage bucket after a specified number of days or months, or

years from the date of tiering rule creation.

- Expire the current version and previous versions of an object independently. This means that you can

set different expiration durations for the current version and the previous versions.

- Tier the current version and previous versions of an object independently on buckets with retroactive

lifecycle policies enabled.

- Expire the incomplete multi-part uploads of an object.• Apply to all objects or a subset of objects based on prefixes, tags, or both.For example, if you want to store log files or business transaction records for a fixed period and after that

period, you want to delete them. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

Note: You cannot recover the objects once it is deleted.To create or update a lifecycle rule, see Creating a Lifecycle Rule in Nutanix Objects on page 115. If retroactive lifecycle policies are not enabled on the bucket, lifecycle policies with the scope of a tag are not retroactive. Only tags that were applied along with Object PUT are considered. Tags applied to an object after its upload is not considered by lifecycle policies.With retroactive lifecycle policies enabled, all rules (with both tag and prefix filters) are retroactively applied to the objects

### Rules for Expiration based Lifecycle Policy

Expiration-based lifecycle policies manage object and version deletion timelines in versioned buckets, with specific rules for current, past, and multipart uploads.Following are the rules for expiration based lifecycle policy:

| • If you apply lifecycle policy | Expire current objects after # days/months/years on an object with |
| --- | --- |

versioning enabled, it deletes the current version of the objects after the specified time, and does not delete any past versions of the objects.

| • If you apply lifecycle policy | Expire previous objects versions after # days/months/years on an |
| --- | --- |

object with versioning enabled, it deletes all the past versions after the specified time. This specified time gets calculated from the day the object version becomes non-current or past. This operation does not delete the current version. Note: This policy cannot be configured on a bucket with suspended versioning.

| • If you apply both the lifecycle policies | Expire current objects after # days/months/years and Expire |
| --- | --- |

### previous objects versions after # days/months/years on an object with versioning enabled, it deletes

| all the past versions based on the time specified in | Expire previous objects versions after # days/ |
| --- | --- |

### months/years, and the current version expires based on the time specified in Expire current objects

after # days/months/years.

| • If you apply expiration based lifecycle policy | Expire or Abort incomplete multipart uploads after # |
| --- | --- |

### days/months/years on an object, it deletes the parts associated with the multipart uploads after the

specified time.

- Lifecycle policies do not apply to WORM buckets with versioning enabled until the WORM retention

period ends.

- When you create a new versioned bucket, the system automatically creates a lifecycle policy to expire

the delete markers if they are the only remaining object versions.

### Rules for Tiering based Lifecycle Policy

Expiration-based lifecycle policies manage object and version deletion timelines in versioned buckets, with specific rules for current, past, and multipart uploads.Following are the rules for the lifecycle policy when you enable tiering to a given endpoint:

| • If you apply lifecycle policy | Tier current objects after # days/months/years on an object with |
| --- | --- |

versioning enabled, it tiers out the current version of the objects after the specified time, and does not tier out any past versions of the objects.

| • If you apply lifecycle policy | Tier previous objects versions after # days/months/years on an object |
| --- | --- |

with versioning enabled, it tiers out all the past versions after the specified time. This specified time gets Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

calculated from the day the object version becomes non-current or past. This operation does not tier out the current version. Note: This policy cannot be configured on a bucket with suspended versioning.

| • If you apply both the lifecycle policies | Tier current objects after # days/months/years and Tier |
| --- | --- |

### previous objects versions after # days/months/years on an object with versioning enabled, it tiers all

| the past versions based on the time specified in | Tier previous objects versions after # days/months/ |
| --- | --- |

### years, and the current version tiers based on the time specified in Tier current objects after # days/

months/years.

### Evaluating Lifecycle Policy

Enabling Retroactive policy enforcement will apply your latest configured lifecycle policy to all existing objects.

### About this task

### Note: All the lifecycle policies on buckets created after upgrading to Nutanix Objects 5.2 will be retroactive

by default. On older buckets, there is an option to enable retroactive lifecycle policy management on the bucket. To enable lifecycle policy evaluation, do the following:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store that contains the bucket where you want to edit the lifecycle configuration.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab.5. In the Buckets table, click the bucket that you want to edit the lifecycle configuration.

| The | Bucket page appears. |
| --- | --- |

6. Click the Lifecycle tab. You can view the rules that you defined while creating the bucket.

### 7. In the Policy Enforcement table, click Change to Retroactive

| The | Change enforcement to Retroactive page appears. |
| --- | --- |

8. Enter the Bucket Name in the text field.9. Click Change.

| In the | Lifecycle tab, the Policy Enforcement Type is Retroactive. |
| --- | --- |

### Retroactive Policy Limitation

Retroactive lifecycle policy enforcement has limitations related to tiering behavior, object size, and endpoint compatibility.Enabling retroactive lifecycle policy enforcement introduces the following limitations:

- If an object is already tiered to a remote end point and the policy is changed to use a different endpoint,

the object will not be re-tiered.

- S3 imposes a limit of 10,000 parts per object. Objects that exceed this limit cannot be tiered out.

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- Large objects that exceed the maximum supported size for S3-compatible or Azure endpoints are not

eligible for tiering.For example, Azure supports a maximum single object PUT size of 5000 MiB, while Nutanix supports up to 5120 MiB. As a result, objects between 5000 MiB and 5120 MiB will not be tiered when using Azure as the target endpoint.

- Retroactive tiering increases API costs for remote endpoints because operations are performed on a

per-object basis rather than in bulk.

### Cloud Tiering in Nutanix Objects

Cloud tiering enables you to move objects to another S3-compatible object store bucket for saving storage space in the Nutanix Objects cluster. Tiering can help you to save costs by sending the infrequently accessed objects to platforms such as AWS S3, Microsoft Azure Blob Storage, and Google Cloud Platform (GCP). The supported endpoints are AWS S3, Azure Blob Storage, Google Cloud Platform (GCP) and a different Nutanix Objects instance.A non-admin user can configure a tiering endpoint only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Create Endpoint• Update Endpoint• View Endpoint• View Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide. Cloud tiering is managed through lifecycle policies. You can configure multiple lifecycle rules for different objects within a bucket.Cloud tiering configuration consists of the following steps:

| • | Step 1: Configure a remote endpoint in the object storeYou can configure remote endpoints using the Cloud Bucket Endpoints page in the object store |
| --- | --- |

instance. The objects within the bucket get moved to the endpoint according to the rules you configure. For more information, see Configuring a Cloud Bucket Endpoint in Nutanix Objects on page 110.

| • | Step 2: Create lifecycle rules for a bucketYou can create a tiering rule for a bucket where you define the scope (all objects or a subset of objects), |
| --- | --- |

select the remote endpoint, and specify the details for the rule. For more information, see Creating a Lifecycle Rule in Nutanix Objects on page 115.

### Points to NoteBefore you start with object tiering, note the following points:

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- The following points are strongly recommended:

- Only encrypted data is stored on buckets for which tiering to the cloud is enabled.• Object store admins enable audit trails for the S3 bucket or other storage endpoints to ensure data

is not being accessed or tampered with external malicious entities and that all access is only coming from the object store instance.

- Admins follow recommended security best practices of AWS or other storage endpoints while setting

up buckets for tiering.

- Tiering lifecycle policies are non-retroactive. The policy gets applied to the new objects that you create.

The policy will not apply to existing objects.

- Tiering lifecycle policies get applied to both versioned and non-versioned objects in the same way. A

separate non-current version transition action in lifecycle policies is not supported.

- Do not perform write operations on the configured-endpoint bucket. Ensure that the endpoint bucket

gets used only for the object store instance.

- Removing a configured endpoint in the object store instance is not supported.• There is a N:N relationship between the source bucket and the configured endpoint bucket. You can

create multiple tiering-lifecycle rules for different objects within a bucket and use a separate endpoint for each tiering rule. Also, an endpoint bucket can be a destination to many Object Store buckets.

- Nutanix Objects within a WORM-enabled bucket will continue to adhere to their WORM property even

after getting tiered out to the endpoint bucket.

- Only the object data gets tiered to the endpoint bucket. The metadata of the object is not tiered.• The behavior to access tiered objects using the Object Get method remains the same. If you send a

request to retrieve an object from the Object store and the object is already tiered out, the Object store fetches the data from the endpoint bucket and fulfills your request.

| • Users with | Edit Bucket permission can create lifecycle rules bucket operation on the buckets available |
| --- | --- |

in the Object stores that are assigned to the respective users.

- Tiered objects are not readable on the tiering endpoint as they are moved in an unstructured form,

without metadata.

| • You can check if the objects are being tiered on the source Bucket | Summary > Tiering. Also tiering |
| --- | --- |

endpoint bucket will grow in size and show data blocks.

- Once the specified time period has elapsed, an object becomes eligible for tiering out. However, it is

important to note that the actual movement of the object may occur up to one day later.

- When reading a tiered object, the Nutanix Objects cluster retrieves the object data from the tiering

endpoint cluster and presents it to the client, without storing it locally that is no caching is performed.

### Configuring a Cloud Bucket Endpoint in Nutanix Objects

You can configure a cloud bucket endpoint for both tiering and replication. The infrequently used objects are moved to the configured endpoints according to the configured lifecycle rule for tiering. The supported endpoints for tiering are AWS S3, Microsoft Azure Blob Storage, Google Cloud Platform (GCP), and a different Nutanix Objects instance.

### Before you beginMake sure the bucket used for replication must be created with the recommended settings. For more

information, see Cloud Bucket Endpoint Recommended Settings on page 114. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### About this task

### Note:

- Cloud tiering supports GCP endpoint as Other S3 compatible endpoint type.• For tiering the objects to GCP, you need to create a bucket in GCP with the following default

configurations. This bucket will be used as the tiering endpoint.

| • | Location type: Multi-region |
| --- | --- |
| • | Default storage class: Standard |
| • | Public access prevention: Off |
| • | Access control: Uniform |
| • | Protection tools: None |

- Ensure that the tiering endpoint has a Certificate Authority (CA) signed certificate. Note that

self-signed certificates are not supported.

- To enable tiering and replication to the cloud, the endpoint must be connected to the storage

network. For example, tiering to S3 requires the S3 ports (80 and 443) to be open and accessible, and the endpoint s3.amazonaws.com to be reachable from the Nutanix Objects storage network. To configure a cloud bucket endpoint, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to configure the cloud bucket endpoint.

| The | Object Store page appears. |
| --- | --- |

4. Click Endpoints > Cloud Bucket Endpoints.5. To open the Add Cloud Bucket Endpoint page, click Create.

| If no endpoints are configured, the | Add Endpoint button appears in the center of the page. Click Add |
| --- | --- |

Endpoint to start an endpoint configuration. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### 6. (Only for Nutanix Objects as the endpoint) Enter the following details:

| a. | Description: Enter a description that identifies your cloud bucket endpoint. |
| --- | --- |
| b. | Endpoint Provider: Select Nutanix as the endpoint type from the pull-down list. |
| c. | Service Host: Enter the complete URL of the endpoint that you want to use for tiering the objects.For example, the Nutanix Objects public IP address or the domain name of an Object store |

instance.Example of a Nutanix Objects endpoint: example.buckets.company.com.

| d. | Bucket Name: Enter the name of the bucket within the service host to which you want the objects |
| --- | --- |

to tier out.

| e. | Access Key: Enter the access key of the bucket owner. |
| --- | --- |
| f. | Secret Key: Enter the secret key of the bucket owner. |
| g. | Skip SSL Certificate Validation: Select this check box to skip SSL certificate validation. |

### 7. (Only for AWS as the endpoint) Enter the following details:

| a. | Description: Enter a description that identifies your cloud bucket endpoint. |
| --- | --- |
| b. | Endpoint Provider: Select AWS as the endpoint type from the pull-down list. |
| c. | Service Host: Enter the complete URL of the endpoint that you want to use for replication or tiering |

the objects.The following AWS S3 URL formats are supported:

- s3.region.amazonaws.com• s3-region.amazonaws.com• s3.region.amazonaws.comExample of an AWS S3 endpoint: s3.us-east-1.amazonaws.com.

| d. | Bucket Name: Enter the name of the bucket within the service host to which you want the objects |
| --- | --- |

to tier out or replicated.

| e. | Access Key: Enter the access key of the bucket owner. |
| --- | --- |
| f. | Secret Key: Enter the secret key of the bucket owner. |

### 8. (Only for Azure Blob Storage as the endpoint) Enter the following details:

| a. | Description: Enter a description that identifies your cloud bucket endpoint. |
| --- | --- |
| b. | Endpoint Provider: Select Azure as the endpoint type from the pull-down list. |
| c. | Container Name: Enter the name of the container within the service host to which you want the |

objects to tier out.

| d. | Account Name: Enter the Azure account name where the containers are located. |
| --- | --- |
| e. | Secret Key: Enter the secret key of the container owner. |

For more information on supported endpoints for tiering on Microsoft Azure Blob Storage, see Configuring a Cloud Bucket Endpoint in Microsoft Azure Blob Storage on page 113. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### 9. (Only for Other S3 Compatible as the endpoint) Enter the following details:

| a. | Description: Enter a description that identifies your cloud bucket endpoint. |
| --- | --- |
| b. | Endpoint Provider: Select Other S3 Compatible as the endpoint type from the pull-down list. |
| c. | Service Host: Enter the complete URL of the endpoint that you want to use for tiering the objects.The AWS S3 URL format is s3.region.amazonaws.comExample of an AWS S3 endpoint: s3.us-east-1.amazonaws.com. |
| d. | Bucket Name: Enter the name of the bucket within the service host to which you want the objects |

to tier out.

| e. | Access Key: Enter the access key of the bucket owner. |
| --- | --- |
| f. | Secret Key: Enter the secret key of the bucket owner. |

10. Click Save to complete configuring the cloud bucket endpoint. You can also update the access and secret keys of the bucket for the configured endpoints. For example, if the bucket owner generates new access and secret keys, the owner can update the

| configured endpoint with the new keys using the | Update option. |
| --- | --- |
| Select the endpoint and click | Update to open the Update Endpoint page. Update the required fields |
| and click | Save to complete the workflow. |

What to do nextAfter you configure an endpoint, you can view the cloud bucket configured endpoints, create lifecycle rules for tiering objects within your bucket, or create a replication relation. For more information, see Viewing Cloud Bucket Endpoints on page 118, Creating a Lifecycle Rule in Nutanix Objects on page 115 or Creating Replication Relation for Buckets on page 151.

### Configuring a Cloud Bucket Endpoint in Microsoft Azure Blob Storage

You can configure a cloud bucket endpoint for both tiering and replication.

### About this taskTo create a container in the Azure portal, follow these steps:

### Procedure

### 1. In the portal navigation pane on the left side of the screen, select Storage accounts and choose a

storage account.

### 2. In the navigation pane for the storage account, scroll to the Data storage section and select

Containers. 3. Within the Containers pane, select the + Container button to open the New container pane. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### 4. Within the New Container pane, provide a Name for your new container and enter the following details:

| a. | Description: Enter a description that identifies your cloud bucket endpoint. |
| --- | --- |
| b. | Endpoint Provider: Select Azure as the endpoint type from the pull-down list. |
| c. | Container Name: Enter the name of the container within the service host to which you want the |

objects to tier out.

| d. | Account Name: Enter the Azure account name where the containers are located. |
| --- | --- |
| e. | Secret Key: Enter the secret key of the container owner. |

What to do nextAfter you configure an endpoint, you can view the cloud bucket configured endpoints, create lifecycle rules for tiering objects within your bucket, or create a replication relation. For more information, see Viewing Cloud Bucket Endpoints on page 118, Creating a Lifecycle Rule in Nutanix Objects on page 115 or Creating Replication Relation for Buckets on page 151.

### Cloud Bucket Endpoint Recommended Settings

This section describes the permission settings required on the destination S3 bucket configuration and lock configuration.

### Permissions Required for the Destination S3 Bucket Configuration

- Creating an endpoint involves testing read and write permissions by adding and deleting objects on

the destination bucket. Therefore, the destination bucket should not have WORM (Write Once Read Many) enabled when creating an endpoint, as deleting it will fail. After creating the endpoint, you can enable WORM on the destination bucket. If the destination bucket is already WORM-enabled, you see the following error message: Method Not Allowed: An object from the object-lock enabled bucket can not bemodified or deleted unless the retention period is elapsed. If you are unable to remove WORM to configure the endpoint, contact Nutanix support for assistance with configuring it.

- The destination bucket must have all the object-level (such as HEAD, GET, PUT, DELETE object,

WORM, tags-related permission, and so on) and bucket-level permissions (such as HEAD, tags, and so on) same as the source bucket.

- The lock permission on the source and destination must be the same.For example, the following is a safe configuration with only access to a particular bucket:

{ "Version": "2012-10-17", "Statement": [ { "Sid": "S30013200001BbN8ZAAVqa", "Effect": "Allow", "Action": [ "s3:*" ], "Resource": [ "arn:aws:s3:::bucket-name", "arn:aws:s3:::bucket-name/*", ] } ] } Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### Caution: Changing these permissions after replication rule creation might cause replication operation

failure.

### Creating a Lifecycle Rule in Nutanix Objects

You can create a lifecycle rule for a bucket where you define the scope (all objects or a subset of objects), select the remote endpoint, and specify the details for the rule. Before you beginMake sure to configure a tiering endpoint before creating object-tiering lifecycle rules.A non-admin user can create lifecycle rules for a bucket in cloud tiering only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Endpoint• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide. About this taskWhen creating a bucket, you can enable versioning, set a rule to delete past versions, and add a lifecycle policy to expire the current objects after the specified time. Once the bucket gets created, you can go to the bucket page and configure object-tiering lifecycle rules.To create a lifecycle rule to tier out objects, do the following:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store that contains the bucket where you want to create tiering rules.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab.5. In the Buckets table, click the bucket that you want to tier out.

| The | Bucket page appears. |
| --- | --- |

6. Click the Lifecycle tab. You can view the rules that you defined while creating the bucket. . Click Create Rule to open the Create Rule page.

| You can also import lifecycle rule by clicking | Create Rule > Import XML. Type or copy the lifecycle |
| --- | --- |

rule in XML format.

### 8. In the Scope page, do the following, and then click Next:

| a. In the | Name box, enter a name that identifies the rule you are creating. |
| --- | --- |
| b. In the | Scope list, select All objects or Tags/Prefix. |
| • Select | All objects to apply the tiering rule to all the objects within the bucket. |
| • Select | Tags/Prefix to apply the tiering rule to specific objects. You can filter objects by entering |

a prefix, tags, or both. Nutanix Objects with the prefix and tags you specified get filtered and the tiering rule applies to those objects. Nutanix Objects is a flat namespace and directories are a file construct. Directories may be simulated in Nutanix Objects but they are not true directories.Lifecycle policy prefix should include the entire directory hierarchy.Some examples of prefix and explanation which files (objects) it will apply to:

| • | log - Only files/folders in the root of the bucket that start with "log". |
| --- | --- |
| • | serverA- All files in folder "serverA", which is in the root of the bucket. |
| • | serverB/log - Files in folder "serverB" that start with "log". |
| • | server*/log- Variable prefixes (for matching multiple folders) are not supported. . In the Configure Rule page, do the following, and then click Next.

| a. To create a tiering rule, select the | Tiering check box. |
| --- | --- |

1. In the Tier list, select current version according to your requirement.

| For a retroactive lifecycle policies enabled bucket, | Previous version is available for tiering. |
| --- | --- |

2. In the Endpoint list, select the endpoint where you want to tier out the objects.

### 3. Enter a time period after which you want to move the objects to the selected endpoint. You can

specify the number in days, months, or years.Tiering time must be less than expiration time.

### Figure 17: Creating Tiering Rule

| b. To create an expiration rule, select the | Expiration check box. |
| --- | --- |

### 1. In Expire, select Current version, Previous version, Multipart uploads, or Delete markers

according to your requirement.The Previous version and Delete markers options appear only for a version-enabled bucket. The expiration of delete marker and the current version cannot be configured in a single rule.

### Tip: It is recommended to create a rule to automatically expire delete markers to enhance the

performance of your object stores.

### 2. Enter a time period after which you want the objects to expire. You can specify the number in

days, months, or years.

| You can click | Add Action to add multiple expiration rules. You can create expiration rules for the |
| --- | --- |

current version, previous version, and multipart uploads. 10. On the Review page, check your configurations. Then, click Done to create the rule. The rule you just created gets enabled and appears in the Rules table.

| • You can select a bucket, and update, delete, disable, and enable the rule using the | Actions drop- |
| --- | --- |

down.

| • You can also export multiple rules to an XML file by clicking | Export to XML. Objects | Bucket Creation, Operations, and Bucket Policy Configuration | |
| --- | --- |

### Viewing Cloud Bucket Endpoints

You can view all the cloud bucket endpoints configured for both replication and tiering.

### About this taskTo view all the configured cloud bucket endpoints, follow this steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store to configure the cloud bucket endpoint.

| The | Object Store page appears. |
| --- | --- |

4. Click Endpoints > Cloud Bucket Endpoints. The endpoints are listed in the Cloud Bucket Endpoint table.

### Tiering Statistics

Cloud tiering supports endpoints such as, Nutanix Objects, Other S3 Compatible, and Azure Blob Storage. You can view the amount of object data moved to the endpoint bucket and the amount of pending data. You can also view the statistics for the source bucket (Object Store bucket) and the endpoint bucket.

### Object Store Bucket - StatisticsIn the object store instance page, click the

### Bucket tab. Click the bucket name, and then click the

### Summary tab to view its tiering statistics.You can view the following information:

| • | Space pending reclamation: The amount of the deleted data and metadata left out due to incomplete |
| --- | --- |

operations.

| • | Total size of objects marked for tiering (pending): The amount of the object data (eligible for tiering) |
| --- | --- |

that is in the process of tiering.

| • | Tier out object size: The amount of object data that has been tiered from this source bucket to the |
| --- | --- |

endpoint buckets.

### Note: If all tiering endpoints are in idle state for a long period of time, such as a day or two, then the usage

statistics displays zero usage. However, the actual usage statistics reflects in the user interface some time (approximately 30 minutes or so) after the next tiering task starts.The tiering statistics are updated in the user interface after a tiering task is completed. However, when large amount of data is tiered to an endpoint, you might experience a delay in seeing the updated statistics in the user interface.

### Legal Hold for Nutanix Objects

You can use legal hold to lock an object indefinitely with no expiration date.A legal hold remains active until an authorized user explicitly removes it. Implementing a legal hold prevents the data of the object from being modified or deleted. You can implement a legal hold on your object data in various cases. For example, put an indefinite lock on objects that you plan to preserve for legal cases, audits, or compliance purposes.Note the following points about the legal hold functionality in Nutanix Objects:

- A user with the write permission on the bucket can apply a legal hold to the objects within the bucket.

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- Only an administrator or bucket owner can remove the legal hold applied to the objects.• A legal hold can be applied while writing an object or to an existing object also.• If a bucket is created using the Prism Central web console and a user with write permissions applies

a legal hold to the objects within the bucket, an administrator user needs to be established to remove the legal hold. Contact Nutanix support to set up an administrator user with the necessary permissions. Nutanix recommends that the user creating the bucket be the bucket owner. In this case, the user will be the bucket owner and will have permission to remove the legal hold.

- If you are replicating a bucket created in Object v3.1 (with the legal hold applied) to a bucket created in

Nutanix Objects v3.0, the legal hold attributes is not replicated. A legal hold can be applied to the objects using the AWS S3 APIs. For more information, see Nutanix Objects Supported APIs on page 250.

### WORM Bucket in Nutanix Objects

Write-once-read-many (WORM) buckets protect your data and metadata. You can configure a WORM bucket to allow the creation of new objects and prevent overwrites or deletion of the existing content for a particular retention period. By default, versioning is not enabled on a bucket. When you apply the WORM policy on a bucket, you can choose to enable versioning. In some industries, regulations or compliance rules mandate long-term records retention, sometimes for more than 7 years. For example, in the financial and health services industry, you must maintain the records in their original state, which cannot be overwritten or erased.A non-admin user can perform WORM bucket operations only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can create WORM buckets on the buckets available in the

Object stores that are assigned to the respective users. When you increase the retention period of a bucket, the new retention period applies to the existing objects as well as the newly added objects.When you apply any lifecycle policy to a WORM bucket with or without versions enabled, the policy is not applicable until the WORM retention period has passed.

### Note:

- WORM can not be enabled for the buckets with NFS protocol support enabled.• You cannot enable the WORM policy while creating a bucket.

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- You can set 0 days as the minimum and 100 years as the maximum limit for the retention

period.

### Operations on a WORM Bucket in Nutanix Objects

This topic lists the valid and invalid operations on a WORM bucket. You can refer to this list before performing any operation on a WORM bucket.

### Valid OperationsFollowing are the valid operations:

- Delete objects

### Note: This operation does not delete the data, but creates a delete marker on top of the existing

versions of objects.

- Delete the delete marker• Delete the target version of the object after the retention period• Create a new version of the object and retain old versions• Extend the retention period

### Invalid OperationsFollowing are the invalid operations:

- Enable WORM policy while creating a bucket• Enable WORM policy without specifying the retention period• Reduce the retention period• Delete the targeted version before the retention period• Change the version state• Does not support retention in governance mode

### Applying WORM Policy to Buckets from Prism Central

You can apply WORM policy only after the bucket is created. A WORM bucket allows you to create new objects and prevents you to overwrite or delete the existing content for a particular retention period.

### About this task

Warning: You cannot modify or delete objects inside a WORM bucket for the specified time period. To apply WORM policies to buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab. . In the Buckets table, select the bucket to apply the WORM policy.6. Click tripple dot horizontal icon > Configure WORM.

| The | Configure WORM on bucketname window appears. |
| --- | --- |

### Note:

- If versioning is disabled on a bucket, you can enable versioning while enabling WORM.

However, once the WORM is enabled, you cannot change the versioning state.

- You cannot suspend versioning on WORM buckets.

### Caution:

- For legal compliance reasons, the setting becomes permanent after 24 hours. You can

disable the WORM policy within the first 24 hours of the grace period.

- You can only edit the retention period to increase the length of retention. You cannot

decrease the retention period. 7. (Optional) Click the Enable Version check box to enable versioning.8. Click the Enable WORM checkbox.9. Type the retention period (in years or months or days) in the Retention Period field by entering a number, and then selecting the time period from the drop-down menu. 10. Click Enable WORM. This procedure successfully applies the WORM policy to the bucket.

### Configuring a Bucket for Static Website Hosting in Nutanix Objects

You can use Nutanix Objects to host a static website that has individual web pages with static content. To host a static website on Nutanix Objects, you can configure a bucket for website hosting, and then upload your website files (objects) to the bucket. When you configure a bucket as a static website, you enable static website hosting, and optionally, add an index document and an error page. You can upload files (such as index documents and error pages) to the bucket using an S3 browser. The S3 browser uses the S3 protocols by providing access and the secret key. You can also choose to redirect to a website. Once you have configured your bucket as a static website, you can access the bucket through the object store endpoints for your bucket.

### Before you beginA non-admin user can configure static website hosting only after the administrator creates an access

control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Website hosting implicitly grants anonymous public read access of all objects.

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can configure buckets for static website hosting on the

buckets available in the Object stores that are assigned to the respective users.

### About this task

### Note:

- Static website hosting cannot be configured for the buckets created using the NFS protocol.• Once you configure the static website for a bucket, you cannot turn off this feature from the

Nutanix Objects user interface. To turn off the static website for buckets, contact Nutanix Support. To configure a bucket for static website hosting, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab.5. In the Buckets table, select the bucket to configure for static website hosting.6. Click tripple dot horizontal icon > Static Website.

| The | Configure Static Website window appears. |
| --- | --- |

7. By default, the endpoint is auto-populated when you click Save at the last step. For example, when an endpoint auto populates, the URL will be in the formatobjectstorename

| .domain/bucketname | . For example, testobjectstore.nutanix.com/teamobjects. |
| --- | --- |

However, if they have set up the DNS correctly, then you can also access the website withbucketname

| .objectstorename.domain | endpoint using HTTP or HTTPS. For example, https:// |
| --- | --- |
| teamobjects.testobjectstore.nutanix.com | . |

8. Click the Host Website or Redirect check box.

| » | Use this bucket to host a website: Select this option to use the bucket to host the website. |
| --- | --- |

Optionally, you can enter the name of the index document (for example, myindex.html) and an error page.An index document is a web page that Nutanix Objects returns when you request the root of a website. It is the default page that loads when you are not requesting any specific page. After you enable static website hosting for your bucket, you can upload an HTML file with the index document name (for example, myindex.html) to your bucket. For example, if you specify no object in the URL, then the website loads the index page (myindex.html) that you have configured. If you have not Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

configured an index document, then website access to the root will return an access denied error message.A custom error page is a web page that Object returns when an error occurs. For example, if you are trying to load an object that does not exist, the website loads the error page that you have configured.

| » | Redirect: Select this option to enter a website URL to redirect to that website. For example, when |
| --- | --- |

you try to access the bucket endpoint, you will be redirected to this website. The protocol used is either HTTP or HTTPS. 9. Click Save.

| An endpoint is auto-generated when you click | Save. This endpoint will be the object store endpoint for |
| --- | --- |

your bucket and is used as the website address.You can now use your bucket as a static website. You can use the endpoint to test your static website.

### Cross-Origin Resource Sharing (CORS) in Nutanix Objects Overview

Cross-Origin Resource Sharing (CORS) allows a web application loaded in one domain to access the restricted resources that are requested from another domain. With CORS support in Nutanix Objects, you can create rich web applications and allow cross-origin sharing of resources from Nutanix Objects.For example, if you upload an image (for example, image1.png) in a bucket (for example, first-bucket) in domain1 that contains some security-related information, and you are not allowed to make access toimage1.png from a website (for example, www.example.com) on domain2. Then you can configure a CORS policy for that first-bucket to allow www.example.com to access the resources of the first-bucket.

### Note:

- CORS cannot be configured for the buckets created using NFS protocol.• When you configure a bucket for static website hosting, public can have only read access to

that bucket. POST, PUT, and DELETE requests on the bucket will be denied. For configuring CORS for a bucket, create an XML document with the following. The document size is limited to 64 KB.

- Rules that identify the origins that you will allow to access your bucket.• The operations (HTTP methods) that will support each origin.• Other operation-specific information.When Nutanix Objects receives a cross-origin request for a bucket, it checks the CORS configuration on

the bucket and uses the first CORSRule rule that matches the incoming browser request to enable a cross- origin request. You can add up to 100 rules to the configuration.Following are the conditions to match the rules.

- The Origin header of the request must match AllowedOrigin elements.• The request method (for example, GET, PUT, HEAD, and so on) or the Access-Control-Request-

header for a pre-flight OPTIONS request must be one of the AllowedMethod elements. Method

- Every header specified in the Access-Control-Request-Headers request header of a pre-flight

request must match an AllowedHeader element. Following is an example of CORS configuration with two CORSRule: <CORSConfiguration> <CORSRule> Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

<AllowedOrigin>http://www.example.com</AllowedOrigin> <AllowedMethod>PUT</AllowedMethod> <AllowedMethod>DELETE</AllowedMethod> <AllowedHeader>*</AllowedHeader> </CORSRule> <CORSRule> <AllowedOrigin>*</AllowedOrigin> <AllowedMethod>GET</AllowedMethod> </CORSRule> </CORSConfiguration>

- The first CORSRule allows cross-origin PUT and DELETE requests whose origin is http://

origin. The rule also allows all headers in a pre-flight OPTIONS request through the www.example.com header. So, in response to any pre-flight OPTIONS request, Nutanix Access-Control-Request-Headers Objects will return any requested headers.

### Note: Other than pre-flight OPTIONS request, no other requests are denied that fail the CORS policy

checks.

- The second rule allows cross-origin GET requests from all the origins.

The * wild-card character refers to all the origins.

### Configuring CORS on a Bucket in Nutanix Objects

Cross-Origin Resource Sharing (CORS) allows a web application loaded in one domain to access the restricted resources that are requested from another domain.

### Before you beginA non-admin user can configure CORS only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can configure CORS on the buckets available in the object

stores that are assigned to the respective users. About this taskYou set this configuration on a bucket so that the bucket can service cross-origin requests.To configure CORS for a bucket, follow these steps: Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab.5. In the Buckets table, select the bucket to configure CORS.6. Click tripple dot horizontal icon > CORS.

| The | Configure CORS window appears. |
| --- | --- |

7. Type or copy and paste a configuration file, or edit an existing configuration. The configuration file must be an XML file. 8. Click Save. The CORS configurations are saved for the bucket.

### Tag-Based Conditions in Bucket Policies

Tag-based conditions in bucket policies enable fine-grained access control for objects based on their associated tags.Nutanix Objects supports tag-based conditions in bucket policies to enable fine-grained access control at the object level. You can restrict access to objects based on their associated tag key-value pairs instead of applying permissions to all objects in a bucket.The StringEquals condition type performs an exact, case-sensitive match between the specified tag key and value and the existing object tags.Use the s3:ExistingObjectTag/tag-key; condition key to evaluate object tags in the bucket policy.

### Note:In release 5.3.1, you can configure tag-based bucket policies only by using S3-compatible tools

such as CLI or API clients. The Prism UI does not support creating or editing such policies. When a bucket policy includes tag-based conditions, the policy becomes read-only in the Prism UI.

### Supported OperationsThe following operations support tag-based conditions:

- GetObject• PutObjectTagging• PutObjectVersionTagging• GetObjectTagging• GetObjectVersionTagging• DeleteObjectTagging• DeleteObjectVersionTagging

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### Scope and LimitationsThe following scope and limitations apply to tag-based conditions in bucket policies:

- Only the StringEquals condition type is supported.• Only existing object tags can be used for condition evaluation.• Tag key and value comparisons are case-sensitive and must match exactly.• Only one condition type and one condition key are supported per policy statement.• Tag-based conditions are not supported on external federated buckets.

Error ResponsesStandard S3 error handling applies when evaluating requests with tag-based conditions.

- AccessDenied is returned when the request does not satisfy the policy conditions.• NoSuchKey or NoSuchUpload errors can occur when the requested object or upload does not exist.• MalformedPolicy is returned for invalid bucket policy definitions.

### Adding Tags to a Bucket in Nutanix Objects

A tag is a label that you assign to a bucket, and it consists of a key and a value pair that you can define. You can add or remove tags from your buckets.

### About this taskTo add tags to a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket.4. Click the Buckets tab, and from the Buckets table, select the bucket to add tags.5. Click tripple dot horizontal icon > Tags.

| The | Add Tags window appears. |
| --- | --- |

6. To add a new tag, click + add tag. The existing tags will be auto-populated. You can click the delete icon to delete a tag.

### Note:

- The maximum number of tags allowed per bucket is 50.• Tag keys can be up to 128 Unicode characters in length, and tag values can be up to 256

Unicode characters in length.

- Tag keys and values are case-sensitive.• Tag key should be unique and is a mandatory field, but tag value is optional.

A new row appears to fill the key value of the tag. . Enter the key and value.8. Click Save. The changes are saved.

### Viewing Buckets in Nutanix Objects

The buckets view allows you to view the list of buckets in the object store and access detailed information about each bucket. Before you beginA non-admin user can view buckets only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- View buckets• View object store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with View Buckets permission can view all the buckets on an assigned object store.

### About this taskTo view the list of buckets, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store in which you want to create a bucket.

| The | Object Store page appears. Objects | Bucket Creation, Operations, and Bucket Policy Configuration | |
| --- | --- |

4. Click the Buckets tab. A list of buckets appears in a tabular view.The following list describes the fields that appear in the buckets list. A dash (-) is displayed in a field when a value is not available or applicable.

### Note:In Prism Central, the object store only displays the federated buckets that it hosts and not all

buckets that are part of the federation. To view all the buckets in a federation namespace, you need to use the Object Browser or any S3 client.

| • | Name: Displays the name of the bucket. Click the name to display the bucket summary. |
| --- | --- |
| • | Size: Displays the size of the bucket. |
| • | Number of Objects: Displays the number of objects in a bucket. |
| • | Versioning: Displays if versioning is enabled or disabled in a bucket. |
| • | WORM: Displays if WORM is enabled or disabled in a bucket. |
| • | Outbound Replication: Displays the outbound replication status. The following statuses are |

displayed:

- Enabled - If a replication rule is enabled.• Disabled - If all the replication rules are disabled.• None - If no replication rule is created on the bucket.

| • | Multiprotocol Access: Displays if multiprotocol access is enabled or disabled for the bucket. |
| --- | --- |
| • | Notifications Displays if notifications are enabled or disabled. |
| • | Static Website & CORS: Displays if static website or CORS is configured for a bucket. |
| You can also launch the Nutanix Objects Browser for this object store, click | Launch Nutanix Objects |

Browser.

### Bucket Summary in Nutanix Objects

This page displays the bucket properties and tiering details.To view the

### Summary page of a bucket, click the name of the bucket in the buckets table, and then click

Summary. See Viewing Buckets in Nutanix Objects on page 127.

### Figure 18: Bucket Summary

Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

### Updating a Bucket in Nutanix Objects

You can update the bucket settings after creating the buckets and adding the objects to the buckets.

### Before you beginA non-admin user can update buckets only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can perform an update bucket operation on the buckets

available in the object stores that are assigned to the respective users.

### About this task

### Note:

- You cannot disable versioning (if enabled) but you can suspend it.• You cannot edit multiple buckets at a time.

To edit a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket. A new object store window appears. 4. Click the Buckets tab.5. In the Buckets table, select the bucket for which you want to change the settings.6. Click Update.

| The | Update Bucket window appears. |
| --- | --- |

7. Edit the settings that you want to change. For more information on the bucket settings, see Bucket Configuration. . Click Save. The changes are saved.

### Sharing a Bucket in Nutanix Objects

You can share a bucket with multiple users that have access keys. Before you beginA non-admin user can share a bucket only after the administrator creates an access control policy on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions, and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide.

- Users with Edit Bucket permission can share a bucket available in the object stores that are

assigned to the respective users.

- You can share only one bucket at a time.

### About this taskTo share a bucket,follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the Object Store that contains the bucket.

| The | Object Store page appears. |
| --- | --- |

### 4. Click the Buckets tab, and in the Buckets table, select the bucket that you want to share with other

users.

### Figure 19: Sharing the Bucket

. Click tripple dot horizontal icon > Share.

| The | Bucket Access Configuration window appears. |
| --- | --- |

6. The name of the bucket owner is visible under Owner. You cannot edit the name of the bucket owner. Removing or changing the permission for the user who is a bucket owner does not affect the bucket. The owner is still allowed to do all the operations on a bucket.

### 7. Type the email address of the users and set the required permission for that user. You can also select

### Anyone with Link as a user to share a bucket for public access and set the required permission.When you select

### Anyone with Link as a user, a public bucket access link is provided. You can copy

and share the link with any user to directly access the bucket from the web browser.You can only add users who have access keys.

### Caution: Creating permissions with Anyone with Link as a user, allows anonymous users to use the

bucket without requiring authentication.

### Figure 20: Share Bucket Window

To generate access keys, see Generating Access Key for API Users on page 94.Nutanix supports the following list of permissions on a bucket:

| • | Read Only: Provides read-only access to the user. |
| --- | --- |
| • | Full Access: Provides all access to the user. |
| • | Custom: Provides customizing of varied levels of access to the user. Click Set under Permissions |

to set custom permission for a user. To know more about Bucket Permissions, see Bucket Access Policies. 8. (Optional) To add more users, click + Add User.

| • If you add | Anyone with Link as Users and grant them Full Access, the user will be able to |
| --- | --- |

perform all operations without any authentication.

- While adding a new user, auto suggestions are provided.• You can enter multiple user names by providing space in between each user's name.

9. (Optional) Click the delete icon to remove an existing set of users and permission. . Click Save.

- It is not possible to save a policy with an empty set of permissions. It is required to have at least

one user and permission.

- Modifying or revoking permissions for a user who is the bucket owner does not affect the bucket.

The owner retains unrestricted access and can continue to perform all operations on the bucket. The bucket is now shared with the listed users with the allotted permissions. What to do nextYou can list the buckets that are shared with you.For more information, refer to Listing the Shared Buckets on page 135.

### Bucket Access Policies in Nutanix Objects

This section describes the authorization and access policies implemented in Nutanix Objects.Nutanix supports the following list of policies on a bucket:

| • | Read Only: Provides read-only access to the user. |
| --- | --- |
| • | Full Access: Provides all access to the user. |
| • | Custom: Provides customizing of varied levels of access to the user. |

### Note: To evaluate the policy for a user, the union of the user specific policy and the anonymous policy is

computed. For example, if the user specific policy contains one set of S3 actions and the anonymous policy for the bucket contains another set of S3 actions, then the resulting policy for the user for that bucket will be union of both set.

**Table 12: Access Roles**

| Access Role | Description |
| --- | --- |

User context with respect to bucketsOwner The user who creates the bucket. The owner can grant and revoke access to the non- admin users on the bucket. Owner can implicitly perform all operations.

| Admin | PC admin who can manage the object-store from the UI. |
| --- | --- |
| Shared user | The non-owner user who gains access to the bucket based on the bucket policy assigned. A shared user can apply a bucket policy and grant access to various operations to any other user, provided they have the necessary access to perform PutBucketPolicy API. |
| Other (non-shared) users | The bucket is not shared with them and they do not have access to the bucket. |
| Anonymous | Refers to someone who does not authenticate themselves, such as static website users. Objects | Bucket Creation, Operations, and Bucket Policy Configuration | |

| Access Role | Description |
| --- | --- |

User privilege levels (IAM)Admin user Admin user will be implicitly allowed all the bucket resource related operations as mentioned in Permissions for Nutanix Objects Operations on page 133 .

| Standard user | Any user who is not an admin. They have full control on the bucket which they create. |
| --- | --- |

### Permissions for Nutanix Objects Operations

This section describes the operations that can be performed by the owners and the admin users. Owner PrivilegesThe owner can perform any S3 operations on a bucket. They have full control over the buckets they create and can grant access to the buckets to other users (non-admin users).Any user can make changes as long as they have the necessary access permissions specified in the policy to perform the desired operation.Bucket Owner has all actions permissions.

**Table 13: Operations Performed by an Owner**

AbortMultipartUploadDeleteBucketDeleteBucketWebsiteDeleteObjectDeleteBucketPolicyDeleteObjectTaggingDeleteObjectVersionDeleteObjectVersionTaggingGetAccelerateConfigurationGetAnalyticsConfigurationGetBucketAclGetBucketCORSGetBucketLocationGetBucketLoggingGetBucketNfsConfigurationGetBucketNotificationGetBucketObjectLockConfigurationGetBucketPolicyGetBucketPolicyStatusGetBucketPublicAccessBlock Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

GetBucketRequestPaymentGetBucketTaggingGetBucketVersioningGetBucketWebsiteGetEncryptionConfigurationGetInventoryConfigurationGetLifecycleConfigurationGetMetricsConfigurationGetObjectGetObjectLegalHoldGetObjectRetentionGetObjectTaggingGetObjectVersionTaggingListBucketListBucketMultipartUploadsListBucketVersionsListMultipartUploadPartsPutBucketCORSPutBucketNfsConfigurationPutBucketNotificationPutBucketObjectLockConfigurationPutBucketPolicyPutBucketTaggingPutBucketVersioningPutBucketWebsitePutEncryptionConfigurationPutInventoryConfiguration Admin PrivilegesAn admin user is implicitly granted permission to perform all operations related to bucket resources.An admin can perform the following actions:

**Table 14: Operations Performed by an Admin**

DeleteBucketDeleteBucketWebsiteDeleteBucketPolicyGetAccelerateConfiguration Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

GetAnalyticsConfigurationGetBucketAclGetBucketCORSGetBucketLocationGetBucketLoggingGetBucketNfsConfigurationGetBucketNotificationGetBucketObjectLockConfigurationGetBucketPolicyGetBucketPolicyStatusGetBucketRequestPaymentGetBucketTaggingGetBucketVersioningGetBucketWebsiteGetEncryptionConfigurationGetInventoryConfigurationGetLifecycleConfigurationGetMetricsConfigurationGetReplicationConfigurationListBucketListBucketMultipartUploadsListBucketVersionsPutBucketCORSPutBucketNfsConfigurationPutBucketNotificationPutBucketObjectLockConfigurationPutBucketPolicyPutBucketTaggingPutBucketVersioningPutBucketWebsitePutInventoryConfigurationPutLifecycleConfigurationPutReplicationConfiguration

### Listing the Shared Buckets

Listing the Shared Buckets feature extends the S3 ListBuckets API and displays the buckets owned by or shared with the current user.The current behavior of the S3 API for ListBuckets is to list the buckets that the current owner (caller of the API) owns. If you have created a bucket, then you are the owner of that bucket. For example, when Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

an admin creates a bucket using the Nutanix Objects user interface, the owner of all such buckets is the admin user. You (owner of the buckets) can choose to share buckets with one or more IAM users with Read Only or Full Access or Custom permissions. IAM users with whom a bucket is shared can access the bucket and its objects subject to the permissions granted. However, the Vanilla S3 API for ListBuckets does not allow for the discovery (listing) of buckets that were shared with them.In earlier versions of Nutanix Objects, if you share a bucket, you have to inform the user which bucket is shared with them. However, with Nutanix Objects 2.1 and later versions, Listing the Shared Bucket feature extends the S3 ListBuckets API and displays the buckets owned by or shared with the current user. If you delete a bucket that is shared, the bucket will automatically be removed from the list.

### Note:

- This feature is a departure from the S3 specifications and is enabled by default. To disable the

feature, contact Nutanix Support.

- This feature is only enabled for sharing relations created using Nutanix Objects 2.1 or later

versions. You cannot list the buckets created using earlier versions of Nutanix Objects. A workaround to enable this feature retroactively is to have the owner of the bucket remove all the sharing relations and recreate them using the latest Nutanix Objects version. For example, an Admin created three buckets B0, B1, and B2. B1 and User2 were shared with User1 with any set of S3 actions. Following are the outputs of the ListBuckets API call before and after the introduction of Listing the Shared Buckets feature.Before the introduction of Listing the Shared Bucket feature: Admin B0 B1 B2 User1 User2 After the introduction of Listing the Shared Bucket feature: Admin B0 B1 B2 User1 B1 User2 B2

### Viewing Bucket Users in Nutanix Objects

You can view all the users and their permissions for a particular bucket.

### Before you beginA non-admin user can view bucket users only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- Users with View Buckets permission can view all the buckets on an assigned object store.

### About this taskTo view the usage of the bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which stores the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, and then from the Buckets list table, click the name of the bucket.5. Click User Access.

### Figure 21: List of Users

The table displays the list of users and their permissions.You can edit the user access by clicking Edit User Access. For more information on editing user access, see Editing User Access in Nutanix Objects on page 137 topic.

### Editing User Access in Nutanix Objects

You can modify user access to grant different permissions, including read-only access, full access, or custom access to provide varied levels of access.

### Before you beginThe non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object Store

### Note:

- Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in

the Security Guide. Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

- If you access any of the buckets that are still utilizing the old-style policy, they will automatically

be upgraded to the new policy. During the upgrade process, the set of access permissions will remain the same as it was before.

### About this taskTo edit user access, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the Object Store which contains the bucket.

| The | Object Store page appears. |
| --- | --- |

4. Click the Buckets tab, and then from the buckets list table, click the name of the bucket.5. Click User Access > Edit User Access to edit.

| The | Bucket Access Configuration page appears. |
| --- | --- |

6. Type the email address of the user and set the permission for that user.

### Figure 22: Share Bucket Window

Nutanix supports the following list of permissions on a bucket:

| • | Read Only: Provides read-only access to the user. |
| --- | --- |
| • | Full Access: Provides all access to the user. |
| • | Custom: Provides customizing of varied levels of access to the user. |

7. To add more users and permissions, click + Add User and Permissions.

### Note:

- Ensure that multiple users that share the same set of permissions, are added as a single

entry. This approach reduces the overall policy size and enables the sharing of buckets with a larger number of users without reaching the policy document size limit.

| • If you add | All users as Users and grant them Full Access, the user will be able to perform |
| --- | --- |

all operations without any authentication. 8. Click the delete icon to remove an existing set of user and permission. . Click Save. The bucket is now shared with the listed users with the allotted permissions.

### Note: It is not possible to save a policy with an empty set of permissions. It is required to have at least

one user and permission.

### Deleting a Bucket in Nutanix Objects

The owner and admin can delete an empty bucket directly. However, non-empty buckets can be deleted only by admins, provided those buckets do not have any WORM or replication rules enabled, and are not part of any Federation. An admin can also delete a bucket with versioning enabled; however, it permanently deletes all the versions of the bucket.

### Before you beginA non-admin user can delete a bucket only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions and the non-admin user must be assigned to specific objects instances:

- Edit Buckets• View Buckets• View Object StoreAdministrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security

Guide.

### Note:

| • Users with | Edit Bucket permission can delete only the buckets they created in the object |
| --- | --- |

stores. They cannot delete buckets created by others.

- If the bucket deletion fails, the bucket will be marked as a read-only bucket until it is deleted.• The space reclamation from AOS is not instant when you delete the non-empty buckets.

### About this taskTo delete a bucket, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. In the object stores table, click the name of the object store which contains the bucket.

| The | Object Store page appears. |
| --- | --- |

### 4. Click the Buckets tab, and then from the Buckets list table, click the check box next to the name of the

bucket that you want to delete. . Click Actions > Delete. While performing this action for a bucket on AWS S3, the cloud bucket is deleted from the federated global namespace. It continues to exist in AWS S3. 6. In the popup, click Confirm. A message appears to confirm the deletion of the bucket. What to do next(optional) After removing all the S3 bucket, you can remove the external endpoint from the Prism Central. For more information refer Removing External Object Store on page 61 Objects | Bucket Creation, Operations, and Bucket Policy Configuration |

