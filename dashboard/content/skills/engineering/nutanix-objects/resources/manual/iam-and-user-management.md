+++
title = "iam-and-user-management"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-objects"
+++

# Nutanix Objects Manual: IAM and User Management

## TYPES OF NUTANIX OBJECTS USERS

There are two ways to manage Nutanix Objects.

| • | Prism Central: Prism Central users can access Nutanix Objects by using the Prism Central web |
| --- | --- |

console. They have the ability to create user accounts and perform all operations except for object- specific actions such as PUT, DELETE, copy, and list objects in any bucket (own or others). Prism Central users can also access a bucket by using the S3 APIs. These users can view buckets of all the API users, and can also share buckets of any API user with any other users.The following additional types of Prism Central users with varying access levels, as described below, can be defined using role-based access control (RBAC).

- Nutanix Objects-only admin - Admin user having access to only Nutanix Objects service within Prism

Central. Within Nutanix Objects, you can create the following three different users:

- Full access to Nutanix Objects service (Nutanix Objects only - full access)• Read-only access to Nutanix Objects service (Nutanix Objects only - view access)• Restricted access to Nutanix Objects service (user can customize the access)

- Prism Central admin role with no access to Nutanix Objects service at all within Prism Central (No

Nutanix Objects access)

| • | API: S3 API users (IAM users) cannot access Nutanix Objects by using the Prism Central web console. |
| --- | --- |

They access buckets and perform operations only by using the S3 APIs. This includes S3-compatible applications. The API users have unconditional access to their own buckets and limited or no access to buckets of other users based on the shared policy. S3 API users are added using the Nutanix Objects GUI. For more information, see Generating Access Key for API Users on page 94. Objects | Types of Nutanix Objects Users |

## DIRECTORY CONFIGURATION AND

## ACCESS KEY GENERATION

You can configure the directory, add people, and generate access keys for the people. You can use these directories to search for people who can have access to the service. Only users with an access key can share buckets.

### Nutanix Objects IAM-High Availability Overview

The Identity and Access Management-High Availability (IAM-HA) feature is introduced to ensure the high availability of Nutanix Objects IAM across data centers. It provides the ability to withstand Prism Central or primary datacenter failures. Earlier, Nutanix Objects IAM was deployed only on the first Nutanix Objects cluster, and the rest of the Nutanix Objects clusters were dependent on it. IAM-HA ensures that each Nutanix Objects cluster is operational on its own, without depending on the other Nutanix Objects clusters, by deploying IAM on each Nutanix Objects cluster. With IAM-HA, any new deployments will use IAMv2 (the same version of IAM as Prism Central).IAM-HA is enabled in two stages: Note: The following two stages are not applicable if there are no Nutanix Objects clusters.

### Stage 1: Nutanix Objects Manager is Upgraded: When the Nutanix Objects Manager is upgraded to the

latest version, it performs migration of IAM directory service configuration and IAM users from older IAM in the primary Nutanix Objects cluster to the IAM in Prism Central. During this stage, the following limitations apply:

- You cannot update users or keys (such as adding or deleting users or keys) from UI during migration.

Only read operations can be performed.

- Operations such as deploy, upgrade, replace certificate, scale-out, set usage alert fails in prechecks.• S3 APIs are not affected during this stage and they continue to allow all operations.Stage 2: Nutanix Objects Service Upgrade:During the Nutanix Objects Service upgrade, in the primary Nutanix Objects cluster, the existing older IAM

service is deleted, and IAMv2 is deployed. In the secondary Nutanix Objects cluster, the IAM service was not available previously, so IAMv2 has been newly deployed.Once the IAM-HA is enabled, the following points are applicable:

- During IAM-HA enablement on clusters, all usernames are converted to lowercase.• Any newly created username is in lowercase.• If any existing user has a mixed case in the username, it is converted to lowercase when a new key is

added to that user.

### Note: This point is applicable only to the IAM-HA cluster deployed before Nutanix Objects 5.0. Any user

created or migrated after enabling IAM-HA with Nutanix Objects 5.0 always has lowercase user names.

- Bucket policy evaluation is case-insensitive.• IAM users cannot be deleted. However, once all keys are deleted, the user disappears from the

### Access

Keys page in the user interface.

### IAM-High Availability Prerequisites

This section lists the prerequisites for the IAM-HA feature. Objects | Directory Configuration and Access Key Generation |

The following are the prerequisites for the IAM-HA feature to be enabled automatically:

- Prism Central version must be pc.2024.1 or later.• The MSP version must be MSP 2.5.0 or later.• Make sure to open the additional ports needed for the IAM-HA feature. For more information, see Ports

and Protocols Guide.

### IAM-High Availability Limitations

This section lists the limitations of the IAM-High Availabilty (IAM-HA) feature.IAM-HA is not enabled under the following conditions:

- If the same user exists in ObjectsIAM (IAM currently used by Nutanix Objects) and Prism Central IAMv2

but with a different user UUID.

- If the same directory service is configured in both ObjectsIAM (IAM currently used by Nutanix Objects)

and Prism Central IAMv2 but with different names. To enable IAM-HA, the directory service name must be the same.

### Note: For a successful IAM-HA migration, the directory service names must match in letter case. For

example, NTNXdomain and ntnxdomain are considered different directory services.

- If multiple users in ObjectsIAM (IAM currently used by Nutanix Objects) have the same user name but

in a different case (user1@ntnx.com vs USER1@ntnx.com), only one such user can exist for IAM-HA to be enabled. To enable IAM-HA, delete any one user from the ObjectsIAM.

- If any stale Active Directory users exist in ObjectsIAM. To enable IAM-HA, delete any stale Active

Directory users.

### Note: Stale AD users are users belonging to a directory service that is no longer configured in

ObjectsIAM (IAM currently used by Nutanix Objects).

- If any of the Active Directory users present in IAM are disabled or do not exist in the Active Directory,

IAM migration fails, and IAM-HA is not enabled.

### Configuring Directories

If IAM-high availability is disabled, you can configure directories in Nutanix Objects. If enabled, use Prism Central to manage directory settings.

### About this taskIf IAM-high availability is disabled, you can add directories that Nutanix Objects can use to search for

users who might have access to the service. You can also configure multiple Active Directory servers in the user interface and perform searches across one or more Active Directories. However, if IAM-high availability is enabled, you can only view the directories; and cannot configure directories through the

| Nutanix Objects user interface. You can go to the | Directory Configuration page on Prism Central to |
| --- | --- |

configure the directory.Non-admin users can configure directories only if assigned a role with

### View Object Store and Create

### Object Store permissions by the admin user in Prism Central. Admins are Super Admins or Prism Admins

in Prism Central; non-admins are Prism Central users without admin privileges.For more information on the built-in roles in Prism Central, see Built-in Role Management in the Security Guide. Objects | Directory Configuration and Access Key Generation |

### About this taskTo configure the directories, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click Access Keys. Note: Deletion of the directory is not allowed if a user has been created and access keys are present.

### Figure 12: View Directories if IAM-High Availability is Enabled

4. Click Configure Directories.5. Click + Add Directory.

### Figure 13: Directory Configuration If IAM-High Availability is Disabled

If you have already added the directory earlier, you will see a list of directories in this screen, and you

| can edit or remove the directory from the | Edit or Remove link next to the directory name. Objects | Directory Configuration and Access Key Generation | |
| --- | --- |

### 6. In the Add Directory window, select any one of the following:

| a. | Active Directory: To add a directory through Active Directory, enter the following directory details |
| --- | --- |

and service account credentials:

| • | Name: Enter the name of the Active Directory server. |
| --- | --- |
| • | Domain: Enter the domain that represents the top of the Active Directory tree and uses Domain |

Name System (DNS) to define its namespace.Usually, the domain is the DNS name of the company.

| • | Directory URL: Enter the Active Directory URL with the port to access the Active Directory |
| --- | --- |

server.For example, ldap://10.2.3.111:389

| • | Username: Enter the username for accessing the Active Directory server to retrieve the user |
| --- | --- |

details.

| • | Password: Enter the password for accessing the Active Directory server to retrieve the user |
| --- | --- |

details. Note: To access Nutanix Objects, no expiry must be set on the Active Directory account.

| b. | LDAP: To add a directory through Lightweight Directory Access Protocol (LDAP), enter the following |
| --- | --- |

directory details, user and group details hierarchy (or tree under which the user details for generating access keys have to be retrieved), and service account credentials:

| • | Name: Enter the name of the OpenLDAP server. |
| --- | --- |
| • | Domain: Enter the domain that represents the top of the LDAP tree that uses DNS to define its |

namespace.Usually, the domain is the DNS name of the company.

| • | Directory URL: Enter the domain that represents the LDAP URL with port to access OpenLDAP |
| --- | --- |

server.For example, ldap://10.2.3.111:389

| • | User Object Class: Enter the LDAP object class value that defines users in the directory service. |
| --- | --- |

When the user is created, this list of user object classes is added to the attributes list of the user.

| • | User Search Base: Enter the location or the search starting point in the LDAP tree, which locates |
| --- | --- |

the users.For example, OU=people.

| • | Username Attribute: Enter the attribute names that are searched to retrieve users from the |
| --- | --- |

LDAP tree.

| • | Group Object Class: Enter the LDAP object class value that defines groups in the directory |
| --- | --- |

service to which the users belong.When the group is created, this list of group object classes is added to the attribute list of the user.

| • | Group Search Base: Represents the location or the search starting point in the LDAP tree under |
| --- | --- |

which groups are located.For example, OU=people

| • | Group Member Attribute: Enter the member attribute that specifies the group memberships. Objects | Directory Configuration and Access Key Generation | |
| --- | --- |

| • | Group Member Attribute Value: Enter the group entries for which the memberships are |
| --- | --- |

specified by using the member attribute.These member attributes can have member attribute values specifying group membership in Distinguished Names (DNs). Member attribute values are used for group membership resolution.

| • | Username: Enter the username for accessing the openLDAP server to retrieve the user details. |
| --- | --- |
| • | Password: Enter the password for accessing the openLDAP server to retrieve the user details. |

Note: To access Nutanix Objects, no expiry must be set on the LDAP account.

### What to do nextYou can now generate an access key for the API users. For more information, see Generating Access Key

for API Users on page 94.

### Generating Access Key for API Users

You can add people (users) from the directory or through email address, and generate access key for the users. Only users with access keys can share buckets.

### Before you beginA non-admin user can generate access keys only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions: View Object Store and Create an Object Store.

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this taskTo generate access keys for the users, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click Access Keys.4. Click + Add People. Objects | Directory Configuration and Access Key Generation |

### 5. In the Add People window, select any of the following options:

| a. | Search for people in a directory service: Select to add people from the directory.For more information on adding a directory, see Configuring Directories on page 91.You can use the Active Directory (AD) group to generate key pairs. Nutanix Objects IAM generates |
| --- | --- |

key pairs for each user as a separate file (inside a single zip file). The administrator can distribute these individual key pair files to the end users.

| b. | Add people not in a directory service: Add the email addresses of the people. Also, you can add a |
| --- | --- |

display name for the user.Adding the display name is optional and can contain up to 255 characters.Click +Add to add multiple users. 6. Click Next. In this page, you generate and download the access keys. You can also add a tag to the access keys for key management.

### Figure 14: Generate and Download Keys Page

| The | Generate and Download Keys page appears. |
| --- | --- |

7. (Optional) Select Apply tag to keys check box and enter a tag name for the access keys. If you added multiple users, the same tag applies to all. The tags can contain up to 255 characters.Tags cannot be changed later. If tags are not set, auto-generated tags apply in version pc.2024.3 with Nutanix Objects Manager 5.0. 8. Click Generate Keys. The keys are generated for the selected people. 9. To download the keys, click Download Keys. The keys are downloaded.

### Caution: For Google Chrome, Microsoft Edge, and Internet Explorer, you can directly download the

| keys. For Safari and Firefox, after you click | Download keys, a new tab opens listing the keys. You must |
| --- | --- |

copy and paste the keys at the desired location manually from the tab. You no longer have access to the keys after you close the tab. Objects | Directory Configuration and Access Key Generation |

### Viewing API Users

You can view the list of people who can have access to Nutanix Objects.

### Before you beginFor a user with non-administrator privileges to view API users, the administrator must create an access

control policy for the non-administrator user on a role that must have the minimum permission to View Object Store.

### About this task

### Note: If the Identity and Access Management-High Availability (IAM-HA) feature is enabled, the users with 0

| access keys are not displayed on the | Access Keys page. |
| --- | --- |

To view the list of API users of Nutanix Objects, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Objects.3. Click Access Keys. Note: You can add keys for multiple users, but you cannot delete multiple users at the same time. The table displays the list of API users.

### Managing API Keys

If you lose or forget your access keys, you cannot retrieve the same key. However, you can add the key and get access. You can also delete a key.

### Before you beginA non-admin user can manage API keys only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions:

- View Object Store• Create an Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.For more information on the built-in roles in Prism Central, see

Built-in Role Management in the Security Guide.

### About this taskTo add or delete the access keys, follow these steps:

### Procedure

1. Log on to the Prism Central web console. . In the Application Switcher, click Objects.3. Click Access Keys. The table displays the list of API users. 4. Click Manage against a user.

### Figure 15: Manage Keys Page

| The | Manage Keys page appears. This page provides you with the option to add or delete access keys. |
| --- | --- |

### 5. To manage a key, you can do one of the following:

| » | Add Key: Click this button to generate the access key for the user. |
| --- | --- |
| » | Apply tag to keys: Select this option if you want to associate a tag with the access key. |

Note: You can add one key at a time and up to five keys for a user.

| » | Delete: Select the access key and click this button to delete the access key.The access key of the user gets deleted and has no access to the object. |
| --- | --- |

### Note:

- You can delete one key at a time.• If all keys for a user are deleted, then that user is not displayed on the

### Access Keys

page. If you want to add a key to that user, you can generate an access key. For more information, see Generating Access Key for API Users on page 94.

### Deleting API Users

This section describes the procedure to delete an API user.

### Before you beginA non-admin user can delete API users only after the administrator creates an access control policy

on a role in Prism Central for the non-admin user. The specific role must have the following minimum permissions:

- View Object Store• Create an Object Store

### Note: Administrators and non-admin users are Prism Central users with the following roles:

- Administrator - A Super Admin or a Prism Admin in Prism Central.• Non-admin user - A Prism Central user without any administrator privileges.

Objects | Directory Configuration and Access Key Generation |

| For more information on the built-in roles in Prism Central, see | Built-in Role Management in the Security |
| --- | --- |

Guide.

### About this task

### Note:

- You cannot delete multiple API users at the same time.• A deleted user can work up to 3 hours after deletion.

To delete an API user, follow these steps:

### Procedure

1. Log on to the Prism Central web console.2. In the Application Switcher, click Nutanix Objects.3. Click Access Keys. The table displays the list of API users. 4. Select the user you want to delete, and then click Delete User.

| The | Delete User button appears at the top after you select a user. |
| --- | --- |

The user is deleted. Objects | Directory Configuration and Access Key Generation |

