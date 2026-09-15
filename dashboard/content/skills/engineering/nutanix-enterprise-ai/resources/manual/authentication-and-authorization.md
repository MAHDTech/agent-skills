+++
title = "authentication-and-authorization"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Authentication, IAM, and Fine-Grained Authorization

Initial login, password management, language settings, Identity & Access Management (IAM), local accounts, Active Directory, OpenLDAP, SAML SSO integration, user management, built-in and custom roles (entities & operations), and fine-grained authorization policies and permissions.

---

#### Logging in After Installing Nutanix Enterprise AI

Log in to Nutanix Enterprise AI for the first time after you install the latest version by using your NAI credentials.

### Before you begin

Retrieve the IP address for the NAI dashboard. For more information, see

Accessing the NAI Dashboard IP

Address

on page  129.

### About this task

To log in to Nutanix Enterprise AI, follow these steps:

### Procedure

#### 1.  Open a web browser, enter the configured fully qualified domain name (FQDN) or the external IP address of the

Envoy Ingress Gateway service in the address field, and press Enter. The Nutanix Enterprise AI login page appears.

#### Username

#### admin

### 2.  In the

field, enter the username

.

### 3.  In the

#### Password

field, enter the default password Nutanix.123.

### 4.  Click

#### Login

.

5. Change your password.

#### Type password

The system prompts you to change the default password. Enter a new password in the

and

#### Retype password

fields, then click

#### Submit

. After you successfully change the password, the system

synchronizes the new password across the database.

6. Accept the License Agreement.

The end-user license agreement (EULA) screen appears. Do the following:

a. Read the license agreement displayed on the left.
b. Enter the appropriate information in the Name, Company, and Job Title fields (on the right).
c. Select the

#### I have read and agree to the terms and conditions

checkbox.

d. Click

#### Accept

.

### What to do next

- 

Configure identity and access management.

Fine-Grained Authorization

For more information, see

on page 132.

- 

Changing Your Password

Change your password. For more information, see

on page 131.

- 

Change the language. For more information, see

Changing the Language

on page  132.

#### Changing Your Password

Change your password in Nutanix Enterprise AI.

### About this task

To change your password, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  Click your username and click

#### Update Password

.

#### Update Password

The

dialog box is displayed.

### 3.  In the

#### Current Password

field, enter the current password.

### 4.  In the

#### New Password

field, enter the new password.

### 5.  In the

#### Confirm New Password

field, enter the new password.

### 6.  Click

#### Update

.

#### Changing the Language

Change the language settings in Nutanix Enterprise AI. Nutanix Enterprise AI supports English and Japanese.

### About this task

You can change the user interface language from English (the default) to Japanese, or vice versa.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Settings

page opens.

### 3.  Click the

#### Language Settings

tab.

The system displays the current language settings.

### 4.  Click

#### Edit

.

#### Edit Language Settings

The system displays the

dialog box.

### 5.  From the

#### Language

dropdown menu, select the language.

### 6.  Click

#### Update

.

The system applies the selected language to the Nutanix Enterprise AI user interface.

#### Fine-Grained Authorization

Nutanix Enterprise AI uses fine-grained authorization to authorize every action against a specific permission, allowing administrators to grant users only the access they need.

### How Authorization Works

Each action in the Nutanix Enterprise AI UI, such as viewing a page, creating an endpoint, or deleting a model, maps to an IAM permission. When a user signs in, Nutanix Enterprise AI retrieves the permissions granted to that user through the authorization policies assigned to them and adjusts the UI as follows:

- 

Pages and navigation items that the user cannot access are hidden.

- 

Actions the user cannot perform are disabled and display an

You do not have access to this action.

Contact your administrator for additional access.

tooltip.

- 

If the user has no assigned roles, the

#### Access Denied

page is displayed instead of the dashboard.

### Authorization Model

The authorization model comprises the following components:

Identity

A user or user group that requires access.

Entity

A resource that the identity interacts with

Permission

An entity-and-operation pair . For example,

operation on the

entity that

```bash
Create Model
nai model
```

authorizes a single action.

Role

A collection of permissions. Nutanix Enterprise AI supports pre-defined system roles and custom roles that you compose from individual permissions.

Authorization Policy

A binding that grants a role to an identity (user or user group) and optionally restricts the role to a specific set of Nutanix Enterprise AI entities. To assign authorization policies, see

Assigning a User

to an Authorization Policy

on page 164.

Scope

The set of entities to which an authorization policy applies. A policy can grant one of the following:

- 

An authorization policy can provide access to all entities. For example, all models on the cluster.

- 

An authorization policy can restrict a user's permissions to only the Nutanix Enterprise AI entities the user created, so that each user sees and manages only their own resources. For example, only the models imported by the user.

- 

An authorization policy can share a user's resource with another user. For example, oa user can import a model and share it with another user.

### Owner-Scoped Access

An authorization policy can restrict a user's permissions to only the Nutanix Enterprise AI entities the user created, so that each user sees and manages only their own resources.

When an authorization policy is scoped to the owner of the entity, Nutanix Enterprise AI evaluates each row-level action against the identity of the user who created that entity. The effect on the UI is as follows:

- 

List pages, such as the

#### Models

,

#### Endpoints

,

#### API Keys

, and

#### MCP Servers

pages, display only the entities the

user created.

- 

Action buttons on rows that belong to other users are disabled with a

You do not have access to this action.

Contact your administrator for additional access.

tooltip.

Owner-scoped access is applied to Nutanix Enterprise AI entities that record the identity of the creating user, including models, local endpoints, unified endpoints, API keys, MCP client keys, MCP servers, MCP connectors, data sources, fine-tune jobs, and batch inference jobs. For more information about configuring owner-scoped access , see

Creating an Authorization Policy for Configurable Access

on page 161.

### Configure Fine-grained authorization

Admins can configure IAM in NAI to allow users to authenticate using an identity provider and access NAI resources based on assigned roles and authorization policies.

Configuring Identity and Access Management in Nutanix Enterprise AI

To configure IAM end-to-end, see

on

page 134.

After IAM is configured, users sign in through the login button that corresponds to their identity source. For more information, see

Log in to Nutanix Enterprise AI

on page 167.

Configuring Identity and Access Management in Nutanix Enterprise AI

Configure Identity and Access Management (IAM) in Nutanix Enterprise AI so that users authenticate through an identity provider and access resources based on the roles and authorization policies you assign to them.

### Before you begin

- 

Logging in After Installing

You must be able to log in to Nutanix Enterprise AI . For more information, see

Nutanix Enterprise AI

on page  131.

- 

You must have reviewed the IAM concepts and permissions model. For more information, see

Fine-Grained

Authorization

Entities and Operations

on page 132 and

on page 148.

- 

Depending on the identity source you plan to configure, you must have one of the following:

- 

The credentials of the local users you plan to create.

- 

The Active Directory (AD) service account credentials and the directory URL.

- 

The LDAP service account credentials, the directory URL, and the group object class, member attribute, and member attribute value.

- 

The SAML metadata file from your identity provider (IdP), along with the username, email, and (optional) group attribute names.

### About this task

The following high-level workflow ties together the identity, role, and authorization policy tasks to grant users access to Nutanix Enterprise AI resources. To configure IAM, follow these high-level steps:

### Procedure

1. Add users directly in Nutanix Enterprise AI or import them from an external identity provider.

»

To add users directly in Nutanix Enterprise AI, see

Adding a Local User

on page  136.

»

To import users from Active Directory, see

Adding an Active Directory Identity Provider

on page 137.

»

Adding an OpenLDAP Identity Provider

To import users from OpenLDAP, see

on page  138.

»

Adding a SAML Identity Provider

To add authentication through a SAML identity provider, see

on

page 139. After adding the SAML IdP, download the Nutanix Enterprise AI metadata file and upload it to your SAML IdP. For more information, see

Downloading Metadata for a SAML-based Identity Provider

on page 140.

The identity source is configured, and the login screen displays a button for each configured identity source.

2. (Optional) Verify the configured identity providers and imported identities.

Confirm that the identity providers and identities you configured are visible in Nutanix Enterprise AI. For more information, see

Viewing IdP Configuration

Viewing Identities or Users

on page 144 and

on page 141.

3. Select or create a role.

Choose a pre-defined system role or create a custom role that grants only the permissions the user needs.

»

To review the pre-defined system roles and their permissions, see

Viewing Roles

on page  146 .

»

To create a new custom role from scratch, see

Creating a New Custom Role

on page 147.

»

Creating a Custom Role from

To create a custom role by copying a pre-defined or existing custom role, see

an Existing Role

on page 151.

4. Create an authorization policy that binds the role to a set of Nutanix Enterprise AI entities.

The authorization policy defines which entities the assigned users can act on with the selected role. Choose one of the following:

- 

Creating an Authorization Policy

To grant access to every entity type and instance covered by the role, see

for Full Access

on page 160.

- 

To restrict access to specific entity types and instances (including owner-scoped access), see

Creating an

Authorization Policy for Configurable Access

on page  161.

- 

Adding an Authorization Policy to a Role

To create the policy while adding it to a role in one step, see

on

page 163.

5. Assign users or user groups to the authorization policy.

Permissions are not granted to users by default. Assign each user to a policy that carries the required role and scope. For more information, see

Assigning a User to an Authorization Policy

on page 164.

The user can access Nutanix Enterprise AI entities based on the permissions carried by the assigned role and scope.

6. Ask the user to sign in and verify access.

The user signs in through the button that corresponds to their identity source. For more information, see the following topics:

- 

Logging in with your Nutanix Enterprise AI Local Account

on page 167

- 

Logging in with Active Directory Credentials

on page 168

- 

Logging in with LDAP Credentials

on page 168

- 

Logging in using SAML-based Single Sign-On

on page  169

The user sees only the pages, tabs, and actions that their permissions allow.

7. (Optional) Troubleshoot access issues.

#### Access Denied

If a user reports an

page or a disabled action, review the assigned role and policy scope.

After you update the role or the policy, ask the user to refresh so that Nutanix Enterprise AI re-evaluates the permissions.

### What to do next

After IAM is configured, maintain access over time with the following actions:

- 

Update or deactivate a local user. For more information, see

Editing a Local User

on page 142 and

Deactivating a Local User Account

on page 143.

- 

Editing an Identity Provider

Edit or delete an identity provider. For more information, see

on page  144 and

Deleting an Identity Provider

on page 145.

- 

Export the list of identities for audit or reporting. For more information, see

Exporting Identities

on page 141.

- 

Updating a Custom Role

Update or delete a custom role. For more information, see

on page  152 and

Deleting a Custom Role

on page 153.

- 

View, edit, duplicate, or delete an authorization policy. For more information, see

Viewing Authorization

Policies

Editing an Authorization Policy

Duplicating an Authorization

on page 154 ,

on page  166 ,

Policy

on page  166.

Adding a Local User Add a local user in Nutanix Enterprise AI.

### Before you begin

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page  155.

### About this task

To add a local user, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

For more information, see

Logging in After Installing Nutanix Enterprise AI

on page  131.

2. 

From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

3. 

Select the

#### Identities

tab.

The users are displayed.

4. 

Click

#### + Add Local User

.

5. 

In the

#### First Name

field, enter a first name.

6. 

In the

#### Last Name

field, enter a last name.

7. 

In the

#### Email

field, enter a valid user email address.

8. 

In the

#### Username

field, enter a user name.

9. 

In the

#### Password

field, enter a password.

Ensure that your password meets the following requirements:

- 

Between eight of 255 characters

- 

Contains at least one lowercase character

- 

Contains at least one uppercase character

- 

Contains at least one digit

- 

Contains at least one special character

- 

Does not contain more than three consecutive identical characters

### 10.  From the

#### Language

dropdown menu, select the language for the user.

### 11.  Turn on the

#### User Active

toggle.

Login access to the local user account is enabled.

### 12.  Click

#### Create

.

The local user is displayed in the

#### Identities

tab.

### What to do next

#### 1.  Permissions are not granted to users by default. To grant permissions, assign a user to an authorization policy. For

Assigning a User to an Authorization Policy

more information, see

on page 164.

#### 2.  (Optional) Edit the details of a local user. For more information, see

Editing a Local User

on page 142.

#### 3.  (Optional) Disable a local user. For more information, see

Deactivating a Local User Account

on page  143.

Adding an Active Directory Identity Provider Add an Active Directory (AD) identity provider in Nutanix Enterprise AI to enable single sign-on for users who already have Active Directory login credentials.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have the AD directory URL and service account credentials.

### About this task

To add an AD identity provider, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

3. 

Select the

#### IdP Configuration

tab.

4. 

From the

#### Add Identity Provider

dropdown menu, select

#### Active Directory

.

The

#### Configure Directory

window is displayed.

5. 

In the

#### Directory Type

dropdown menu, select the AD.

6. 

In the

#### Name

field, enter a name.

7. 

In the

#### Directory URL

field, enter the URL address of the directory in the following format:

```bash
ldap://
host:ldap_port_num
```

- 

host: The host value is either an IP address or a fully qualified domain name.

- 

ldap_port_num: The default LDAP port number is 389.

8. 

#### Search Type

#### Non Recursive (Default)

#### Recursive

From the

dropdown menu, select either

or

.

»

When users are present in the first level group, you can select

#### Non Recursive (Default)

.

»

#### Recursive

When you have multi-level group assignments, you can select

.

»

A recursive search performs a multi-level or nested search during authentication, which might cause slowness. If you experience slowness, select

#### Non-Recursive (Default)

.

9. 

#### Username

In the

field, enter the service account user name in the following format:

```bash
user_name@domain.com
```

### 10.  In the

#### Password

field, enter the service account password.

### 11.  Click

#### Save

.

#### IdP Configuration

The AD is displayed in the

tab and the login screen displays a button to login to the AD.

### What to do next

Assign a user to an authorization policy.

Permissions are not granted to users by default. To grant permissions, assign a user to an authorization policy. For more information, see

Assigning a User to an Authorization Policy

on page 164.

Adding an OpenLDAP Identity Provider Add an OpenLDAP identity provider in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have the LDAP directory URL, service account credentials, and the group object class, member attribute, and member attribute value.

### About this task

To add an OpenLDAP identity provider, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

3. 

Select the

#### IdP Configuration

tab.

4. 

From the

#### Add Identity Provider

dropdown menu, select

#### OpenLDAP

.

The

#### Configure Directory

window appears.

5. 

In the

#### Directory Type

dropdown menu, select the

#### OpenLDAP

.

6. 

In the

#### Name

field, enter a name.

7. 

In the

#### Directory URL

field, enter the URL address of the directory in

format:

```bash
ldap://host:ldap_port_num
```

- 

host: The host value is either an IP address or a fully qualified domain name.

- 

ldap_port_num: The default LDAP port number is 389.

8. 

#### Secondary URLs

(Optional) In the

field, enter multiple IP addresses for different domain controllers.

Update this field when you need high availability.

9. 

From the

#### Search Type

dropdown menu, select either

#### Non Recursive (Default)

or

#### Recursive

.

- 

#### Non Recursive (Default)

When users are present in the first level group, you can select

.

- 

When you have multi-level group assignments, you can select

#### Recursive

. A recursive search performs a

multi-level or nested search during authentication, which might cause slowness. If you experience slowness, select Non-Recursive (Default).

### 10.  In the

#### Group Object Class

field, enter the object class of groups in the directory service.

For example: posixGroup or groupOfNames

### 11.  In the

#### Group Search Base

field, enter the base domain name to search for user groups.

The base domain name must include the domain components, each represented in the format of dc=domain_component, separated by commas. For example, if your domain name is nutanix.com, the base domain name includes: cn=groups,dc=nutanix,dc=com; ou=groups,dc=nutanix,dc=com.

### 12.  In the

#### Group Member Attribute

field, enter the attribute of the group object that holds the user association.

For example: member or memberUid

### 13.  In the

#### Group Member Attribute Value

field, enter the attribute of the user object that is used to configure the

membership in the group object.

For example: uid

### 14.  In the

#### Username

field, enter the service account user name in

format.

```bash
user_name@domain.com
```

### 15.  In the

#### Password

field, enter the service account password.

### 16.  Click

#### Save

.

#### IdP Configuration

The LDAP identity provider now appears in the

tab and the login screen displays a button

to login to the LDAP identity provider.

### What to do next

Permissions to entities and operations are not granted to users by default. To grant permissions, assign a user to an authorization policy. For more information, see

Assigning a User to an Authorization Policy

on

page 164.

Adding a SAML Identity Provider Add a SAML identity provider in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have the SAML metadata file from your IdP and the username, email, and (optional) group attribute names.

### About this task

To add a SAML identity provider, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### IdP Configuration

Select the

tab.

4. 

#### Add Identity Provider

#### SAML Identity Provider

From the

dropdown menu, select

.

#### Configure Identity Provider

The

window appears.

5. 

#### Configuration name

In the

field, enter a name for the identity provider.

The name you enter is displayed as a button name in the login screen.

6. 

In the

#### Username Attribute

field, enter the username attribute of the identity provider configuration.

7. 

In the

#### Email Attribute

field, enter the email attribute of the identity provider configuration.

8. 

In the

#### Redirect URL

field, enter the post-login redirection URL.

9. 

(Optional) In the

#### Group Attribute Name (Optional)

field, enter the group attribute name.

Ensure that this name matches the group attribute name provided in the IDP configuration.

### 10.  (Optional) In the

#### Group Attribute Delimiter (Optional)

field, enter a delimiter to be used when multiple

groups are selected for the Group attribute.

#### 11.  Upload a metadata file that contains the identity provider information:

a. Click

#### Import Metadata

.

b. Upload the metadata file.

### 12.  Click

#### Save

.

#### IdP Configuration

The SAML identity provider is displayed in the

tab and the login screen displays a button

to log in to the SAML identity provider.

### What to do next

#### 1.  Download the Nutanix Enterprise AI metadata file to configure the callback URL in the SAML-based identity

provider's website.

For more information, see

Downloading Metadata for a SAML-based Identity Provider

on page  140.

#### 2.  Permissions to entities and operations are not granted to users by default. To grant permissions, assign a user to an

authorization policy. For more information, see

Assigning a User to an Authorization Policy

on page  164.

#### Downloading Metadata for a SAML-based Identity Provider

Download the Nutanix Enterprise AI metadata file to configure the callback URL in the SAML-based identity provider's website.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Add SAML users.

For more information, see

Adding a SAML Identity Provider

on page  139.

### About this task

To download metadata, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### IdP Configuration

tab.

4. Select a SAML identity provider.

### 5.  Click

#### Download Metadata

.

A metadata file that describes the Nutanix Enterprise AI attributes is downloaded in XML format.

### What to do next

Go to identity provider's website and upload the metadata file that you downloaded.

Viewing Identities or Users View identities or users in Nutanix Enterprise AI.

### Before you begin

You must have at least one user or imported user. For more information, see

Adding a Local User

on

page 136 ,

Adding an Active Directory Identity Provider

on page  137,

Adding an OpenLDAP Identity

Provider

on page 138, or

Adding a SAML Identity Provider

on page  139.

### About this task

To view identities or users, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Select the

#### Identities

tab.

The users are displayed.

### 4.  (Optional) To view local users, click

#### Local Users

.

#### 5.  (Optional) To view users that you imported through Okta, Active Directory, or Open LDAP, click

#### Imported

#### Users

.

### 6.  (Optional) To view user groups, click

#### User Groups

.

#### 7.  (Optional) To view users you added in a prior version of NAI, click

#### Local Users

.

### What to do next

- 

Deactivate a local user. For more information, see

Editing a Local User

on page 142.

- 

Deactivating a Local User Account

Deactivate a local user. For more information, see

on page 143.

- 

Assigning a User to an Authorization

Assign an identity to an authorization policy. For more information, see

Policy

on page  164.

### Exporting Identities

Export identities in Nutanix Enterprise AI.

### Before you begin

You must have at least one identity. For more information, see

Viewing Identities or Users

on page  141.

### About this task

To export identities, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### Identities

tab.

### 4.  Click

#### Export

.

The details of the identities are downloaded to your machine.

### Editing a Local User

Edit a local user account in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Adding a Local User

You must have at least one local user. For more information, see

on page 136.

### About this task

To edit a local user account, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI .

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### Identities

Select the

tab.

The users are displayed.

4. 

Select a user.

5. 

Click

#### Actions

>

#### Edit

.

6. 

In the

#### First Name

field, enter a first name.

7. 

In the

#### Last Name

field, enter a last name.

8. 

In the

#### Display Name

field, enter the name.

9. 

In the

#### Email

field, enter a valid user email address.

### 10.  From the

#### Language

dropdown menu, select the language for the user.

### 11.  Turn on the

#### User Active

toggle.

Login access to the local user account is enabled.

### 12.  Click

#### Update

.

The user details are updated.

### Deactivating a Local User Account

Deactivate a local user account in Nutanix Enterprise AI.

### Before you begin

- 

You must have a role with

#### Update User

permission.

- 

You must have at least one local user. For more information, see

Adding a Local User

on page 136.

### About this task

To deactivate a local user account, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### Identities

tab.

#### Local Users

The list of local users are displayed in the

tab.

4. Select a user.

### 5.  Click

#### Actions

>

#### Deactivate

.

#### Deactivate User

The

confirmation dialog box is displayed.

### 6.  Click

#### Deactivate

.

The local user account is deactivated.

### What to do next

The deactivated user cannot sign in until you reactivate them . For more information, see

Reactivate a

Local User

on page 143.

### Reactivate a Local User

Reactivate a deactivated user account in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have at least one deactivated local user. For more information, see

Adding a Local User

on

page 136.

### About this task

To edit a local user account, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### Identities

tab.

The users are displayed.

4. Select a user.

### 5.  Click

#### Actions

#### Edit

>

.

### 6.  Turn on the

#### User Active

toggle.

Login access to the local user account is enabled.

### 7.  Click

#### Update

.

The user details are updated.

Viewing IdP Configuration View the IdP Configuration or identity providers in Nutanix Enterprise AI.

### Before you begin

- 

You must have at least one identity provider. For more information, see

- 

Adding an Active Directory Identity Provider

on page 137

- 

Adding an OpenLDAP Identity Provider

on page 138

- 

Adding a SAML Identity Provider

on page  139

### About this task

To view IdP configuration, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Select the

#### IdP Configuration

tab.

The IdP configuration is displayed.

### What to do next

- 

Editing an Identity Provider

Edit an identity provider. For more information, see

on page 144.

- 

Delete an identity provider. For more information, see

Deleting an Identity Provider

on page 145.

### Editing an Identity Provider

Edit an identity provider in Nutanix Enterprise AI.

### Before you begin

You must have at least one identity provider. For more information, see

- 

Adding an Active Directory Identity Provider

on page 137

- 

Adding an OpenLDAP Identity Provider

on page  138

- 

Adding a SAML Identity Provider

on page  139

### About this task

To edit an identity provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### IdP Configuration

tab.

#### 4.  (Optional) To edit an active directory identity provider, follow these steps:

a. Select an identity provider of

#### Type

Active Directory.

b. Click

#### Actions

#### Edit

>

.

c. Update the fields.

Step 5

Step 11

Adding an Active Directory Identity Provider

For more information, see

to

in

on page  137.

#### 5.  (Optional) To edit an Open LDAP identity provider, follow these steps:

a. Select an identity provider of

#### Type

Open LDAP.

b. Click

#### Actions

>

#### Edit

.

c. Update the fields.

Step 5

Step 16

Adding an OpenLDAP Identity Provider

For more information, see

to

in

on page 138.

#### 6.  (Optional) To edit a SAML identity provider, follow these steps:

a. Select an identity provider of

#### Type

SAML IDP.

a. Click

#### Actions

>

#### Edit

.

b. Update the fields.

For more information, see

Step 5

to

Step 11

in

Adding a SAML Identity Provider

on page 139.

### What to do next

Log in and verify that the identity provider button on the login screen reflects the updated configuration. For more information, see

Log in to Nutanix Enterprise AI

on page 167.

### Deleting an Identity Provider

Delete an identity provider in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Adding an Active Directory Identity

You must have at least one identity provider. For more information, see

Provider

on page  137 ,

Adding an OpenLDAP Identity Provider

on page 138 , or

Adding a SAML

Identity Provider

on page 139.

- 

Users authenticated through the identity provider you delete lose access to Nutanix Enterprise AI.

### About this task

To delete an identity provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Select the

#### IdP Configuration

tab.

4. Select an identity provider.

### 5.  Click

#### Actions

>

#### Delete

.

### 6.  In the

#### Delete Identity Provider

#### Delete

confirmation box, click

.

The identity provider is deleted.

### What to do next

Deleting an identity provider does not delete the authorization policies bound to it. Review and update related policies. For more information, see

Editing an Authorization Policy

on page 166.

Viewing Roles View the pre-defined system roles, custom roles, and the permissions each role grants in Nutanix Enterprise AI.

### Before you begin

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page  155.

### About this task

To view the roles and permissions available in Nutanix Enterprise AI, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Select the

#### Roles

tab.

#### Roles

The

tab lists the pre-defined system roles and any custom roles that you created. The pre-defined roles are:

- 

ML Admin

- 

ML User

- 

Read Only

- 

License Manager

4. Click a role.

#### Role Details

The

page displays the Nutanix Enterprise AI and IAM permissions assigned to the

role_name

role.

5. Click a permission dropdown menu.

The operations assigned to the role for that permission are displayed.

### What to do next

- 

Create a new custom role. For more information, see

Creating a New Custom Role

on page  147.

- 

Creating a Custom Role from an

Create a custom role from an existing role. For more information, see

Existing Role

on page 151.

- 

Add an authorization policy to a role. For more information, see

Adding an Authorization Policy to a Role

on

page 163.

Creating a New Custom Role Create a new custom role to grant a specific set of Nutanix Enterprise AI permissions to the users you assign to it.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Adding a Local User

You must have at least one user. For more information about adding users, see

on

page 136,

Adding an Active Directory Identity Provider

on page 137,

Adding an OpenLDAP Identity

Provider

Adding a SAML Identity Provider

on page  138 , or

on page 139.

- 

A custom role cannot be assigned permissions to edit roles and authorization policies.

### About this task

Use custom roles to grant only the Nutanix Enterprise AI permissions a user needs for their job. For example, a data- science reviewer might need only

and

, while a model developer needs

```bash
NAI:View_Model
NAI:View_Endpoint
```

,

, and

. To create a custom role, follow these steps:

```bash
NAI:View_Model
NAI:Create_Model
NAI:Delete_Model
```

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### Roles

Select the

tab.

#### Roles

The

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

4. 

#### Roles

Select the

tab.

#### Roles

The

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

5. 

#### Create Role

#### New Role

From the

dropdown menu, select

.

#### Create Role

The

dialog box is displayed.

6. 

#### Role Name

In the

field, enter a unique name for the custom role.

7. 

#### Description

(Optional) In the

field, enter a description for the role.

8. 

#### Entity Type

Select

.

Type the name of an entity.

9. 

#### 10.  Grant permissions to an entity or operation. From an entity's dropdown menu, do one of the following:

»

To provide permissions to all the operations for an entity, select the entity.

»

To provide permissions to specific operations, select the required operations.

Entities and Operations

For more information, see

on page 148.

#### Related Operations

The

dialog box is displays the related permissions assigned by NAI . For example, if you

select only

#### Update API Key

, NAI also assigns

#### View API Key

.

#### 11.  To view, add, or edit permissions to related operations, follow these steps:

a. Hover over the

#### related operations

icon

The

#### Related Operations

dialog box is displays the related permissions assigned by NAI .

b. (Optional) Select additional related operations.

Entities and Operations

For more information, see

on page 148.

c. Click

#### Add

.

#### Added Operations

The

section displays the selected operations.

### 12.  In the

#### Added Operations

, section, review the operations you assigned.

### 13.  To remove permissions from the

#### Added Operations

#### delete

section, click the

icon

### 14.  Do one of the following:

»

#### Save

Create the role. Click

.

#### Roles

The custom role is displayed on the

tab.

»

To create the role and assign an authorization policy, Click

#### Save and Create Authorization Policy

.

#### Create New Authorization Policy

Creating

The

dialog box is displayed. For more information, see

an Authorization Policy for Full Access

on page 160  or

Creating an Authorization Policy for

Configurable Access

on page  161.

### What to do next

Add the role to an authorization policy. For more information, see

Adding an Authorization Policy to a Role

on page 163.

### Entities and Operations

Reference of the permissions that Nutanix Enterprise AI uses to authorize user actions. Assign these permissions to users through Prism Central IAM roles and authorization policies.

Nutanix Enterprise AI defines a set of entities and operations that authorize every action a user can perform in the UI. Each authorization policy binds one or more of these permissions to an identity (user or user group) and optionally scopes the permission to specific entities.

### Nutanix Enterprise AI Permissions

The following table lists the permissions that Nutanix Enterprise AI uses to authorize actions on Nutanix Enterprise AI entities.

**Table 44: Nutanix Enterprise AI Entities and Permissions**

| Entity Model | Permission View Model | Authorizes View models on the  Models  page. |
| --- | --- | --- |
|  | Create Model | Import a model from Hugging Face, NVIDIA NGC Catalog, or a manual upload. |
|  | Update Model | Modify an imported model. |
|  | Delete Model | Delete an imported model. |
|  | Update Catalog | Update  Model Access Control  settings for the model catalog. |
| Endpoint | View Endpoint | View local endpoints on the  Endpoints  page. |
|  | Create Endpoint | Create a local endpoint. |
|  | Update Endpoint | Edit, hibernate, resume, or validate a local endpoint. |
|  | Delete Endpoint | Delete a local endpoint. |
| Unified Endpoint | View Unified Endpoint | View unified endpoints on the  Unified Endpoints  page. |
|  | Create Unified Endpoint | Create a unified endpoint. |
|  | Update Unified Endpoint | Edit a unified endpoint, including rate-limit configuration. |
|  | Delete Unified Endpoint | Delete a unified endpoint. |
| Provider | View Provider | View third-party providers and provider credentials. |
|  | Create Provider | Add a third-party provider or provider credential. |
|  | Update Provider | Modify a third-party provider or provider credential. |
|  | Delete Provider | Delete a third-party provider or provider credential. |
| API Key | View API Key | View API keys on the  API Keys  page. |
|  | Create API Key | Create an API key. |
|  | Update API Key | Activate, deactivate, or update an API key. |
|  | Delete APIKey | Delete an API key. |
| MCP Client Key | View MCP Key | View MCP client keys. |
| Entity | Permission | Authorizes |
|  | Create MCP Key | Create an MCP client key. |
|  | Update MCP Key | Update an MCP client key. |
|  | Delete MCP Key | Delete an MCP client key. |
| MCP Server | View MCP Server | View MCP servers on the  Servers  tab. |
|  | Create MCP Server | Add or deploy a remote or local MCP server. |
|  | Update MCP Server | Update the configuration of a local or remote MCP server. |
|  | Delete MCPServer | Delete an MCP server. |
| MCP Connector | View MCP Connector | View MCP connectors. |
|  | Create MCP Connector | Create an MCP connector. |
|  | Update MCP Connector | Update an MCP connector. |
|  | Delete MCP Connector | Delete an MCP connector. |
| Data Source | View DataSource | View data sources used for fine-tuning and batch inference. |
|  | Create DataSource | Add a data source. |
|  | Delete DataSource | Delete a data source. |
| Fine-Tune Job | View Finetune | View fine-tune jobs. |
|  | Create Finetune | Start a fine-tune job. |
|  | Delete Finetune | Delete a fine-tuned model. |
| Batch Inference | View Batch Inference | View batch inference jobs. |
|  | Create Batch Inference | Start a batch inference job. |
|  | Delete Batch Inference | Delete a batch inference job. |
| Cluster | Update Cluster Config | Update cluster configuration, including Pulse, HTTP proxy, Syslog, and model access control settings. |
| Audit Log | View Audit Log | View audit events on the  Audit Events  page. |
| License | View License | View licenses on the  Licenses  page. |
|  | Manage License | Add, upgrade, or delete a license. |

**Table 45: IAM Permissions**

| Entity Role | Permission View Role | Authorizes View roles on the  Roles  tab. |
| --- | --- | --- |
| User | View User | View users and user groups on the  Identities  tab. |
|  | Update User | Update user or user group properties. |
| Entity | Permission | Authorizes |
| Authorization Policy | View Authorization Policy | View authorization policies on the Authorization Policies  tab. |
| Directory Service | View Directory Service | View configured identity providers (Active Directory, LDAP, SAML). |

Creating a Custom Role from an Existing Role Create a custom role using an existing built-in or custom role as a starting point that grants a specific set of Nutanix Enterprise AI permissions to users you assign to it.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Review the pre-defined system roles and any existing custom roles to identify a good starting point. For more information, see

Viewing Roles

on page 146.

- 

Entities and Operations

Review the list of permissions in

on page  148 and decide which permissions to

include in the custom role.

- 

A custom role cannot be assigned permissions to edit roles and authorization policies.

### About this task

Use custom roles to grant only the Nutanix Enterprise AI permissions a user needs for their job. For example, a data- science reviewer might need only

and

, while a model developer needs

```bash
NAI:View_Model
NAI:View_Endpoint
```

,

, and

. To create a custom role, follow these steps:

```bash
NAI:View_Model
NAI:Create_Model
NAI:Delete_Model
```

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

3. 

Select the

#### Roles

tab.

The

#### Roles

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

4. 

From the

#### Create Role

dropdown menu, select

#### From existing role

.

The

#### Choose Role

dialog box is displayed.

5. 

From the

#### Search roles

dropdown menu, type a system role or custom role name and select the role from the

search result.

The operations associated with the selected role are displayed.

6. 

#### Role Name

In the

field, enter a unique name for the custom role.

7. 

#### Description

(Optional) In the

field, enter a description for the role.

8. 

#### Entity Type

Select

.

9. 

Type the name of an entity.

#### 10.  Grant permissions to an entity or operation. From an entity's dropdown menu, do one of the following:

»

To provide permissions to all the operations for an entity, select the entity.

»

To provide permissions to specific operations, select the required operations.

For more information, see

Entities and Operations

on page 148.

The

#### Related Operations

dialog box is displays the related permissions assigned by NAI . For example, if you

#### Update API Key

#### View API Key

select only

, NAI also assigns

.

#### 11.  To view, add, or edit permissions to related operations, follow these steps:

a. Hover over the

#### related operations

icon

#### Related Operations

The

dialog box is displays the related permissions assigned by NAI .

b. (Optional) Select additional related operations.

Entities and Operations

For more information, see

on page 148.

c. Click

#### Add

.

The

#### Added Operations

section displays the selected operations.

### 12.  In the

#### Added Operations

, section, review the operations you assigned.

### 13.  To remove permissions from the

#### Added Operations

section, click the

#### delete

icon

### 14.  Do one of the following:

»

Create the role. Click

#### Save

.

#### Roles

The custom role is displayed on the

tab.

»

#### Save and Create Authorization Policy

To create the role and assign an authorization policy, Click

.

The

#### Create New Authorization Policy

dialog box is displayed. For more information, see

Creating

an Authorization Policy for Full Access

Creating an Authorization Policy for

on page 160  or

Configurable Access

on page  161.

### What to do next

#### If you clicked  Save  instead of  Save and Create Authorization Policy , you must assign the role to an authorization policy. For more information, see

Adding an Authorization Policy to a Role

on page  163.

```bash
Updating a Custom Role Update a custom role
```

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have at least one custom role. For more information, see

Creating a New Custom Role

on

Creating a Custom Role from an Existing Role

page 147  or

on page 151.

### About this task

To update a custom role, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI .

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### Roles

Select the

tab.

#### Roles

The

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

4. 

Select a role.

5. 

Select

#### Actions

>

#### Update

.

The

#### Update

dialog box is displayed with the selected role.

6. 

In the

#### Role Name

field, enter a unique name for the custom role.

7. 

(Optional) In the

#### Description

field, enter a description for the role.

8. 

#### Entity Type

Select

.

9. 

Type the name of an entity.

#### 10.  Grant permissions to an entity or operation. From an entity's dropdown menu, do one of the following:

»

To provide permissions to all the operations for an entity, select the entity.

»

To provide permissions to specific operations, select the required operations.

For more information, see

Entities and Operations

on page 148.

The

#### Related Operations

dialog box is displays the related permissions assigned by NAI . For example, if you

#### Update API Key

#### View API Key

select only

, NAI also assigns

.

#### 11.  To view, add, or edit permissions to related operations, follow these steps:

a. Hover over the

#### related operations

icon

#### Related Operations

The

dialog box is displays the related permissions assigned by NAI .

b. (Optional) Select additional related operations.

For more information, see

Entities and Operations

on page 148.

c. Click

#### Add

.

The

#### Added Operations

section displays the selected operations.

### 12.  In the

#### Added Operations

, section, review the operations you assigned.

### 13.  To remove permissions from the

#### Added Operations

#### delete

section, click the

icon

### 14.  Do one of the following:

»

#### Save

Create the role. Click

.

The custom role is displayed on the

#### Roles

tab.

»

#### Save and Create Authorization Policy

To create the role and assign an authorization policy, Click

.

The

#### Create New Authorization Policy

dialog box is displayed. For more information, see

Creating

an Authorization Policy for Full Access

Creating an Authorization Policy for

on page 160  or

Configurable Access

on page  161.

### Deleting a Custom Role Delete a custom role

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Creating a New Custom Role

You must have at least one custom role. For more information, see

on

page 147  or

Creating a Custom Role from an Existing Role

on page 151.

- 

You can delete a role only when the role is not attached to an authorization policy.

### About this task

To delete a custom role, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Select the

#### Roles

tab.

The

#### Roles

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

4. Select a role.

#### Actions

#### Delete

### 5.  Select

>

.

#### Delete Role

The

confirmation dialog box is displayed.

### 6.  Click

#### Delete

.

The custom role is deleted from Nutanix Enterprise AI and is no longer displayed on the

#### Roles

page.

#### Viewing Authorization Policies

View authorization policies in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Creating an Authorization Policy

You must have at least one authorization policy. For more information, see

for Full Access

on page 160  or

Creating an Authorization Policy for Configurable Access

on page 161.

### About this task

To view authorization policies, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Click

#### Authorization Policies

.

The authorization policies are displayed.

4. Select an authorization policy.

The details of the authorization policy are displayed.

### What to do next

- 

Assign a user to an authorization policy. For more information, see

Assigning a User to an Authorization

Policy

on page  164.

- 

Adding an Authorization Policy to a Role

Assign a role to an authorization policy. For more information, see

on page 163.

- 

Create a new authorization policy using the selected authorization policy as a starting point. For more information, see

Duplicating an Authorization Policy

on page  166.

- 

Edit the authorization policy. For more information, see

Editing an Authorization Policy

on page  166.

#### Authorization Permissions

Authorization permissions required for operations in Nutanix Enterprise AI.

**Table 46: Authz Permission Map**

| Property 1 | UI capability key | IAM key | Result |
| --- | --- | --- | --- |
|  | model_view | NAI:View_Model | Hides the Models page (also sidebar entry, model pickers in create flows, import-model widget) |
|  | model_create | NAI:Create_Model | Disables Create/Import-Model button & no-data CTAs |
|  | model_update | NAI:Update_Model | Disables Edit-Model action |
|  | model_delete | NAI:Delete_Model | Disables Delete-Model action |
|  | model_catalog_update | NAI:Update_Catalog | Hides catalog-update control on Model Access Control page |
|  | model_accessControl_manage | NAI:Update_ClusterConfig / NAI:Update_Catalog (any) | Hides Model Access Control page and tab |
|  | localEndpoint_view | NAI:View_Endpoint | Hides Local Endpoints page/tab/ sidebar; disables endpoint pickers in API-Key/Batch/UEP modals |
|  | localEndpoint_create | NAI:Create_Endpoint | Disables Create-Endpoint button & Deploy-from-Model action |
|  | localEndpoint_update | NAI:Update_Endpoint | Disables Edit/Hibernate/Resume/ Scan endpoint actions |
|  | localEndpoint_delete | NAI:Delete_Endpoint | Disables the Delete-Endpoint action |
|  | localEndpoint_create_kvAwareRouting | -  (always-on) | Hides KV-aware-routing toggle in the  Create Endpoint  dialog box |
|  | UI capability key | IAM key | Result |
|  | unifiedEndpoint_view | NAI:View_UnifiedEndpoint | Hides Unified Endpoints page & sidebar; disables UEP pickers in key/dashboard modals |
|  | unifiedEndpoint_create | NAI:Create_UnifiedEndpoint | Disables Create-Unified-Endpoint button |
|  | unifiedEndpoint_update | NAI:Update_UnifiedEndpoint | Disables Edit-UEP action |
|  | unifiedEndpoint_delete | NAI:Delete_UnifiedEndpoint | Disables Delete-UEP action |
|  | unifiedEndpoint_rateLimit_technicalPreview | -  (always-on) | Hides Tech-Preview badge on UEP rate limit |
|  | provider_view | NAI:View_Provider | Hides Providers page & sidebar; disables provider picker in UEP modal |
|  | provider_create | NAI:Create_Provider | Disables Add-Provider button |
|  | provider_update | NAI:Update_Provider | Disables Edit-Provider action |
|  | provider_delete | NAI:Delete_Provider | Disables Delete-Provider action |
|  | batchInference_view | NAI:View_BatchInference | Hides Batch Inferencing tab/page; hides batch purpose option & columns; hides Batch audit filter |
|  | batchInference_create | NAI:Create_BatchInference | Disables Create-Batch-Inference button/action |
|  | batchInference_delete | NAI:Delete_BatchInference | Disables Delete-Batch-Inference action |
|  | finetune_view | NAI:View_Finetune | Hides Finetuning page & Models sidebar entry (with model_view); hides Finetune audit filter |
|  | finetune_create | NAI:Create_Finetune | Disables Create-Finetune button & Finetune-from-Model action |
|  | finetune_delete | NAI:Delete_Finetune | Disables Delete-Finetune action |
|  | finetune_technicalPreview | -  (always-on) | Hides Tech-Preview badge on Finetune route |
|  | cluster_updateConfigs | NAI:Update_ClusterConfig | Hides cluster-config controls, Model Access Control cluster section, onboarding step; hides Settings Security/Pulse/Proxy/ Syslog tabs |
|  | mcpServer_view | NAI:View_MCPServer | Hides MCP Servers page & sidebar; disables server picker & tool selection in Connector modal |
|  | mcpServer_create | NAI:Create_MCPServer | Disables Add-MCP-Server action |
|  | mcpServer_update | NAI:Update_MCPServer | Disables Edit-MCP-Server action |
|  | UI capability key | IAM key | Result |
|  | mcpServer_delete | NAI:Delete_MCPServer | Disables Delete-MCP-Server action |
|  | mcpConnector_view | NAI:View_MCPConnector | Hides MCP Connectors page, sidebar & dashboard widgets; disables connector picker in MCP-Key form |
|  | mcpConnector_create | NAI:Create_MCPConnector | Disables Add-Connector button |
|  | mcpConnector_update | NAI:Update_MCPConnector | Disables Edit-Connector action |
|  | mcpConnector_delete | NAI:Delete_MCPConnector | Disables Delete-Connector action |
|  | mcpKey_view | NAI:View_MCPKey | Hides MCP Client Keys tab/page & sidebar; disables MCP-Key attachment form; hides MCP-Key audit filter |
|  | mcpKey_create | NAI:Create_MCPKey | Disables Create-MCP-Key button |
|  | mcpKey_update | NAI:Update_MCPKey | Disables Edit/Activate/Deactivate MCP-Key actions |
|  | mcpKey_delete | NAI:Delete_MCPKey | Disables Delete-MCP-Key action |
|  | apiKey_view | NAI:View_APIKey | Hides API Keys tab/page & sidebar; disables API-Key attachment form; hides API-Key audit filter |
|  | apiKey_create | NAI:Create_APIKey | Disables Create-API-Key button |
|  | apiKey_update | NAI:Update_APIKey | Disables Edit/Activate/Deactivate API-Key actions |
|  | apiKey_delete | NAI:Delete_APIKey | Disables Delete-API-Key action |
|  | dataSource_view | NAI:View_DataSource | Hides Data Sources page & sidebar; disables data-source picker in Finetune/Batch modals |
|  | dataSource_create | NAI:Create_DataSource | Disables Add-Data-Source button |
|  | dataSource_delete | NAI:Delete_DataSource | Disables Delete-Data-Source action |
|  | auditLog_view | NAI:View_AuditLog | Hides Audit Logs page & sidebar |
|  | license_view | NAI:View_License | Hides License option in audit-log filter |
|  | license_manage | NAI:Manage_License | Disables edit/update on Settings # Licensing; hides Licensing tab |
|  | iam_roles_view | IAM:View_Role | Hides Users # Roles; hides Role audit filter |
|  | iam_users_view | IAM:View_User | Hides Users # Users / User Groups; hides User audit filter |
|  | UI capability key | IAM key | Result |
|  | iam_users_update | IAM:Update_User | Hides Settings # Language tab |
|  | iam_access_policies_view | IAM:View_Authorization_Policy | Hides Users # Access Policies; hides Access-Policy audit filter |
|  | iam_directory_services_view | IAM:View_Directory_Service | Hides Users # Directory Services; hides Directory-Service audit filter |
|  | user_view | IAM:View_Role / IAM:View_User / IAM:View_Authorization_Policy / IAM:View_Directory_Service (any) | Hides Users page |
|  | credentials_platform_aws | NAI:View_Provider | Hides AWS tile in Add-Credential flow |
|  | credentials_platform_nai | NAI:View_Provider | Hides NAI tile in Add-Credential flow |
|  | credentials_platform_gcp | NAI:View_Provider | Hides GCP tile in Add-Credential flow |
|  | credentials_platform_azure | NAI:View_Provider | Hides Azure tile in Add-Credential flow |
|  | credentials_platform_openai | NAI:View_Provider | Hides OpenAI tile in Add- Credential flow |
|  | credentials_platform_anthropic | NAI:View_Provider | Hides Anthropic tile in Add- Credential flow |
|  | credentials_platform_mistral | NAI:View_Provider | Hides Mistral tile in Add- Credential flow |
|  | credentials_platform_cohere | NAI:View_Provider | Hides Cohere tile in Add- Credential flow |
|  | credentials_platform_googleGeminiNAI:View_Provider |  | Hides Google Gemini tile in Add- Credential flow |
|  | credentials_platform_mcp | NAI:View_MCPServer | Hides MCP tile in Add-Credential flow |
|  | credentials_platform_mcpregistry | NAI:View_MCPServer | Hides MCP Registry tile in Add- Credential flow |
|  | credentials_platform_hf | -  (always-on) | Hides Hugging Face tile in Add- Credential flow |
|  | credentials_platform_ngc | -  (always-on) | Hides NGC tile in Add-Credential flow |
|  | credentials_platform_s3 | -  (always-on) | Hides S3 tile in Add-Credential flow |
|  | settings_credentials | -  (always-on) | Hides Settings # Credentials tab |
|  | settings_security | NAI:Update_ClusterConfig | Hides Settings # Security tab |
|  | settings_pulse | NAI:Update_ClusterConfig | Hides Settings # Pulse tab |
|  | UI capability key | IAM key | Result |
|  | settings_language | IAM:Update_User | Hides  Settings   >  Language  tab |
|  | settings_licensing | NAI:Manage_License | Hides Settings # Licensing tab |
|  | settings_proxy | NAI:Update_ClusterConfig | Hides Settings # Proxy tab |
|  | settings_syslog | NAI:Update_ClusterConfig | Hides Settings # Syslog tab |
|  | sampleApp_chat | -  (flag enableLabs) | Disables Labs # Chat sample app |
|  | sampleApp_rag | -  (flag enableLabs) | Disables Labs # RAG sample app |
|  | sampleApp_agent | -  (flag enableAgent) | Disables Labs # Agent sample app |
|  | sidebar_providers | NAI:View_Provider | Hides Providers sidebar entry |
|  | sidebar_localEndpoints | NAI:View_Endpoint / NAI:View_BatchInference (any) | Hides Local Endpoints sidebar entry |
|  | sidebar_unifiedEndpoints | NAI:View_UnifiedEndpoint | Hides Unified Endpoints sidebar entry |
|  | sidebar_mcp | NAI:View_MCPServer / NAI:View_MCPConnector (any) | Hides MCP sidebar entry |
|  | sidebar_dataSources | NAI:View_DataSource | Hides Data Sources sidebar entry |
|  | sidebar_apiKeys | NAI:View_APIKey / NAI:View_MCPKey (any) | Hides API Keys sidebar entry |
|  | sidebar_users | IAM:View_Role / IAM:View_User / IAM:View_Authorization_Policy / IAM:View_Directory_Service (any) | Hides Users sidebar entry |
|  | sidebar_auditLogs | NAI:View_AuditLog | Hides Audit Logs sidebar entry |
|  | sidebar_models | NAI:View_Model / NAI:View_Finetune (any) | Hides Models sidebar entry |
|  | dashboardWidget_summary | NAI:View_MCPConnector / NAI:View_UnifiedEndpoint / NAI:View_Endpoint / NAI:View_Provider / NAI:View_MCPServer (any) | Hides Dashboard summary widget |
|  | dashboardWidget_apiAndMcpKeysNAI:View_APIKey / |  | Hides Dashboard API & MCP keys widget |
|  |  | NAI:View_MCPKey (any) |  |
|  | dashboardWidget_endpointRequests | NAI:View_UnifiedEndpoint / NAI:View_Endpoint (any) | Hides Dashboard endpoint- requests widget |
|  | dashboardWidgets_topFiveEndpoints | NAI:View_UnifiedEndpoint / NAI:View_Endpoint (any) | Hides Dashboard top-5-endpoints widget |
|  | dashboardWidget_requestTrend | NAI:View_UnifiedEndpoint / NAI:View_Endpoint (any) | Hides Dashboard request-trend widget |
|  | dashboardWidget_mcpRequestSummary | NAI:View_MCPConnector | Hides Dashboard MCP request- summary widget |
|  | UI capability key | IAM key | Result |
|  | dashboardWidget_mcpConnectors NAI:View_MCPConnector |  | Hides Dashboard MCP connectors widget |
| Creating an Authorization Policy for Full Access |  |  |  |

Create an authorization policy with full access to all entity types and operations for the added users in the associated role.

### Before you begin

- 

You must have at least one role. For more information, see

Viewing Roles

on page  146 or

Creating a New

Custom Role

on page  147.

- 

Adding a Local User

Adding an

You must have at least one user. For more information, see

on page  136 ,

OpenLDAP Identity Provider

on page 138,

Adding an Active Directory Identity Provider

on page 137,

Adding a SAML Identity Provider

or

on page  139.

### About this task

To create an authorization policy, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### Authorization Policies

Click

.

The authorization policies are displayed.

4. 

In the

#### Select Role

field, enter the built-in or custom role's name and select the role from the list of suggestions.

The system displays role details for the selected role.

5. 

#### Next

Click

.

6. 

#### Full Access: all entity type & instances

Select

.

7. 

#### View entity types

To view the entity types, click

.

8. 

(Optional) To automatically gain access to any new entity types that are added to this selected role in the future, select the

Automatically grant access to new entity types that are added to this role in the future

checkbox.

#### Next

9. 

Click

.

#### 10.  (Optional) To assign a local user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

select

#### Local user

.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The local user is assigned to the authorization policy.

#### 11.  (Optional) To assign an Active Directory user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

#### IDP_name_(AD)

select

.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The AD user or user group is assigned to the authorization policy.

#### 12.  (Optional) To assign a SAML user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

select a SAML user or a SAML user group.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The SAML user or user group is assigned to the authorization policy.

### 13.  Click

#### Save

.

14. (Optional) Assign multiple roles to a single user.

For example, you can create two separate roles for downloading models and creating endpoints. If the same user perrforms both the actions, then you can assign both these roles to the same user. To assign, follow these steps:

a. Create the required number of roles.

For example, you can create two separate roles for downloading models and creating endpoints.

b. Create separate authorization policies for each of these roles.

For example, you can create two separate authorization policies for downloading models and creating endpoints.

c. Assign the user to all the authorization policies you create.

For example, assign the same user to the authorization policy for downloading models and the authorization policy for creating endpoints

#### Creating an Authorization Policy for Configurable Access

Create an authorization policy with configurable access to the selected entity types and instances for the added users in the associated role.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Viewing Roles

Creating a New

You must have at least one role. For more information, see

on page  146 or

Custom Role

on page  147.

- 

You must have at least one user. For more information, see

Adding a Local User

on page  136 ,

Adding an

OpenLDAP Identity Provider

Adding an Active Directory Identity Provider

on page 138,

on page 137,

or

Adding a SAML Identity Provider

on page  139.

### About this task

To create an authorization policy, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

3. 

Click

#### Authorization Policies

.

The authorization policies are displayed.

4. 

#### Select Role

In the

field, enter the built-in or custom role's name and select the role from the list of suggestions.

The system displays role details for the selected role.

5. 

Click

#### Next

.

6. 

Select

#### Configure access: select entity types & instances

.

7. 

From the

#### Entity Type

dropdown menu, select an entity.

The list of available entities depends on the role that you select.

Do any of then following:

8. 

»

#### Filters

#### Individual Entity

#### Search

From the

dropdown menu, select

and from the

dropdown menu select

#### All

.

entity_name

»

From the

#### Filters

dropdown menu, select

#### Owner

and from the

#### Search

dropdown menu ,select

#### Self_Owned

.

9. 

Share a model that you imported to NAI with a user or user group that you are adding to this authorization policy. To share, do the following:

Sharing allows the users or user groups assigned to this authorization policy to view the model in the

#### Models

page and can select it during endpoint creation.

a. From the

#### Entity Type

dropdown menu, select Model.

b. From the

#### Filters

dropdown menu, select

#### Individual Entity

.

c. From the

#### Search

dropdown menu , type the initial letter of the model name and select the model.

10. Configure users access to entities created by them.

»

To automatically allow users to access the entities they create in addition to the entities created that you assign to them, select

#### Allow users access to entities created by them

.

»

To allow users to access only to the entities that you configured, do not select the checkbox.

### 11.  Click

#### Next

.

#### 12.  (Optional) To assign a local user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

select

#### Local user

.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The local user is assigned to the authorization policy.

#### 13.  (Optional) To assign an Active Directory user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

#### IDP_name_(AD)

select

.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The AD user or user group is assigned to the authorization policy.

#### 14.  (Optional) To assign a SAML user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

select a SAML user or a SAML user group.

b. In the search box, enter the first few letters of the user or user group's name and select the correct user or user

group from the list of suggestions.

c. Click

#### Next

.

The SAML user or user group is assigned to the authorization policy.

### 15.  Click

#### Save

.

16. (Optional) Assign multiple roles to a single user.

For example, you can create two separate roles for downloading models and creating endpoints. If the same user perrforms both the actions, then you can assign both these roles to the same user. To assign, follow these steps:

a. Create the required number of roles.

For example, you can create two separate roles for downloading models and creating endpoints.

b. Create separate authorization policies for each of these roles.

For example, you can create two separate authorization policies for downloading models and creating endpoints.

c. Assign the user to all the authorization policies you create.

For example, assign the same user to the authorization policy for downloading models and the authorization policy for creating endpoints

#### Adding an Authorization Policy to a Role

Adding an authorization policy to a role.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Viewing Roles

Creating a New

You must have at least one role. For more information, see

on page  146 ,

Custom Role

on page  147 , or

Creating a Custom Role from an Existing Role

on page 151.

### About this task

To add an authorization policy to a role, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI .

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Select the

#### Roles

tab.

#### Roles

The

tab displays the pre-defined system roles and custom roles if you created custom roles earlier.

4. Select a role.

### 5.  Select

#### Actions

>

#### Add Authorization Policy

.

The

#### Create New Authorization Policy

dialog box is displayed with the selected role.

6. Create an authorization policy.

»

Create an authorization policy with full access to all entity types and operations for the added users in the associated role. For more information, see

Creating an Authorization Policy for Full Access

on

page 160.

»

Create an authorization policy with configurable access to the selected entity types and instances for the added users in the associated role. For more information, see

Creating an Authorization Policy for Configurable

Access

on page 161.

#### Assigning a User to an Authorization Policy

Assign local users to an authorization policy in Nutanix Enterprise AI.

### Before you begin

Ensure that you have the following:

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have at least one of the following users:

- 

Local user.

Adding a Local User

For more information, see

on page  136.

- 

Active Directory user

For more information, see

Adding an Active Directory Identity Provider

on page  137.

- 

LDAP user

For more information, see

Adding an OpenLDAP Identity Provider

on page  138.

- 

SAML user

Adding a SAML Identity Provider

For more information, see

on page 139.

- 

You must have at least one authorization policy.

- 

Viewing

Review the permissions that the authorization policy will grant. For more information, see

Authorization Policies

on page 154.

### About this task

To assign local users to an authorization policy, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Users

From the navigation bar, select

.

#### Identity and Access Management

The

page is displayed.

3. 

#### Authorization Policies

Select the

tab.

The authorization policies are displayed.

4. 

Select an authorization policy.

5. 

#### Actions

#### Edit

Click

>

.

#### Edit Authorization Policy

The

window appears.

6. 

#### Edit

Click

.

7. 

#### Choose Role

#### Next

In the

tab, click

.

(Optional) To assign a local user, follow these steps:

8. 
a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

#### Local user

select

.

b. Type the user names.
c. Click

#### Next

.

The local user is assigned to the authorization policy.

9. 

(Optional) To assign an Active Directory user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown menu,

authorization_policy_name

select

#### IDP_name_(AD)

.

b. Type the user names.
c. Click

#### Next

.

The AD user or user group is assigned to the authorization policy.

#### 10.  (Optional) To assign a SAML user, follow these steps:

a. From the

#### Select users or user groups to assign to

dropdown

authorization_policy_name

menu, select a SAML user or a SAML user group.

b. Type the user names.
c. Click

#### Next

.

The SAML user or user group is assigned to the authorization policy.

### What to do next

Verify if the user can access entites and operations as per the assigned permissions. To verify, do any of the following:

- 

Log in to NAI  as a local user. For more information, see

Logging in with your Nutanix Enterprise AI Local

Account

on page 167.

- 

Log in to NAI as a AD user. For more information, see

Logging in with Active Directory Credentials

on

page 168.

- 

Log in to NAI  as a LDAP user. For more information, see

Logging in with LDAP Credentials

on page 168.

- 

Log in to NAI as a SAML user. For more information, see

Logging in using SAML-based Single Sign-On

on

page 169.

#### Editing an Authorization Policy

Edit an authorization policy.

### Before you begin

You must have at least one authorization policy. For more information, see

Creating an Authorization

Policy for Full Access

on page 160  or

Creating an Authorization Policy for Configurable Access

on

page 161. You must have Update Authorization Policy permission.

### About this task

To edit an authorization policy, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

#### Identity and Access Management

The

page is displayed.

### 3.  Click

#### Authorization Policies

.

The authorization policies are displayed.

### 4.  Select an authorization policy

#### Actions

#### Edit

### 5.  Select

>

.

#### Select Role

### 6.  (Optional) In the

field, enter the built-in or custom role's name and select the role from the list of

suggestions.

### 7.  Click

#### Next

.

### 8.  Do any of the following:

»

To grant full access to all entity types and operations for the added users in the associated role, select

#### Full

#### Access: all entity type & instances

Creating an Authorization Policy for

. For more information, see

Full Access

on page 160.

»

To grant configurable access to the selected entity types and instances for the added users in the associated role, select

#### Configure access: select entity types & instances

Creating an

. For more information, see

Authorization Policy for Configurable Access

on page  161.

#### Duplicating an Authorization Policy

Create an authorization policy using an existing authorization policy as a starting point.

### Before you begin

- 

You need View Authorization Policy and Create Authorization Ploicy permissions.

- 

Creating an Authorization Policy

You must have at least one authorization policy. For more information, see

for Full Access

on page 160  or

Creating an Authorization Policy for Configurable Access

on page 161.

### About this task

To duplicate an authorization policy, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### Users

.

The

#### Identity and Access Management

page is displayed.

### 3.  Click

#### Authorization Policies

.

The authorization policies are displayed.

### 4.  Select an authorization policy

### 5.  Click

#### Actions

>

#### Duplicate

.

The

#### Duplicate Authorization Policy

dialog box is displayed.

6. Create an authorization policy.

»

Create an authorization policy with full access to all entity types and operations for the added users in the associated role. For more information, see

Creating an Authorization Policy for Full Access

on

page 160.

»

Create an authorization policy with configurable access to the selected entity types and instances for the added users in the associated role. For more information, see

Creating an Authorization Policy for Configurable

Access

on page 161.

#### Log in to Nutanix Enterprise AI

Log in to Nutanix Enterprise AI after install or upgrade.

- 

After installing the latest version of Nutanix Enterprise AI, log in to Nutanix Enterprise AI. For more information, see

Logging in After Installing Nutanix Enterprise AI

on page 131. An user must configure IAM and assign

users to authorization policies. For more information, see

Fine-Grained Authorization

on page 132 and

Configuring Identity and Access Management in Nutanix Enterprise AI

on page 134.

- 

After a user with permissions configures IAM, users can log in with any of the following account types:

- 

Nutanix Enterprise AI local user accounts: You can log in using accounts that the admin creates in Nutanix Enterprise AI  directly. Nutanix credentials. For more information, see

Logging in with your Nutanix

Enterprise AI Local Account

on page 167.

- 

Active Directory account

Logging in with Active Directory Credentials

For more information, see

on page  168.

- 

Open LDAP credentials

Logging in with LDAP Credentials

For more information, see

on page 168.

- 

SAML-based single sign-on

Logging in using SAML-based Single Sign-On

For more information, see

on page  169.

Logging in with your Nutanix Enterprise AI Local Account Log in to Nutanix Enterprise AI (NAI) using your NAI credentials.

### Before you begin

- 

Ensure that a local user is added to NAI. For more information, see

Adding a Local User

on page  136.

- 

Assigning a User to

Ensure that the local user is assigned to an authorization policy. For more information, see

an Authorization Policy

on page 164.

### About this task

To log in to Nutanix Enterprise AI as a local user, follow these steps:

### Procedure

#### 1.  Open a web browser, enter the configured fully qualified domain name (FQDN) or the external IP address of the

#### Enter

Envoy Ingress Gateway service in the address field, and press

.

The Nutanix Enterprise AI login page differs based on whether you have configured IdP.

- 

#### Username

#### Password

#### Log In

The

field,

field, and the

button are displayed if you have not configured IdP.

#### 1.  In the

#### Username

field, enter the username.

#### 2.  In the

#### Password

field, enter the password.

- 

#### Log in with your Nutanix Local Account

The

link is displayed after you have configured IdP.

#### 1.  Click

#### Log in with your Nutanix Local Account

.

2. Enter your NAI credentials.

### 2.  Click

#### Login

.

Logging in with Active Directory Credentials Log in to Nutanix Enterprise AI using your Active Directory (AD) credentials.

### Before you begin

- 

Ensure that AD is configured as an identity provider. For more information, see

Adding an Active Directory

Identity Provider

on page 137.

- 

Assigning a User to

Ensure that the AD user is assigned to an authorization policy. For more information, see

an Authorization Policy

on page 164.

### About this task

To log in to Nutanix Enterprise AI using AD credentials, follow these steps:

### Procedure

#### 1.  Open a web browser, enter the configured fully qualified domain name (FQDN) or the external IP address of the

#### Enter

Envoy Ingress Gateway service in the address field, and press

.

The Nutanix Enterprise AI login page opens and displays a button to log in with the

.

Active Directory_name

### 2.  Click

#### Log in with Active Directory_name

.

3. Enter your AD credentials.

### 4.  Click

#### Login

.

Logging in with LDAP Credentials Log in to Nutanix Enterprise AI using your LDAP credentials.

### Before you begin

- 

Adding an OpenLDAP

Ensure that LDAP is configured as an identity provider. For more information, see

Identity Provider

on page 138.

- 

Ensure that the LDAP user is assigned to an authorization policy. For more information, see

Assigning a User to

an Authorization Policy

on page 164.

### About this task

To log in to Nutanix Enterprise AI using LDAP credentials, follow these steps:

### Procedure

#### 1.  Open a web browser, enter the configured fully qualified domain name (FQDN) or the external IP address of the

Envoy Ingress Gateway service in the address field, and press

#### Enter

.

The Nutanix Enterprise AI login page opens and displays a button to log in with the

.

Active LDAP_name

### 2.  Click

#### Log in with LDAP_name

.

3. Enter your LDAP credentials.

### 4.  Click

#### Login

.

Logging in using SAML-based Single Sign-On Log in to Nutanix Enterprise AI using single sign-on through your SAML identity provider.

### Before you begin

- 

Ensure that the SAML identity provider (IdP) is configured using the IdP's metadata. For more information, see

Adding a SAML Identity Provider

on page  139.

- 

Assigning a User to

Ensure that the SAML user is assigned to an authorization policy. For more information, see

an Authorization Policy

on page 164.

### About this task

To log in using SAML-based single sign-on, follow these steps:

### Procedure

#### 1.  Open a web browser, enter the configured fully qualified domain name (FQDN) or the external IP address of the

#### Enter

Envoy Ingress Gateway service in the address field, and press

.

The Nutanix Enterprise AI login page opens and displays a button to log in with the

.

SAML_identity_provider_name

### 2.  Click

#### Log in with SAML_identity_provider_name

.

You are redirected to the SAML provider's login screen.

3. Authenticate as per the SAML provider.
