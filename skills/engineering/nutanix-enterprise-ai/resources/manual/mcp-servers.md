# Nutanix Enterprise AI Manual: Model Context Protocol (MCP) Integration

Model Context Protocol (MCP) support in NAI, remote MCP servers and credentials, local MCP server deployment (container registry credentials, lifecycle), MCP connectors and client keys, MCP logging, and security considerations.

---

## MCP SERVERS

Model Context Protocol (MCP) servers in Nutanix Enterprise AI provide standardized, secure access control over the tools that your AI agents use.

MCP servers provide the following advantages:

Governance and Access Control

Implement governance with security, auditing, and role-based access control (RBAC) to define which agents can interact with specific external tools.

Simplified Integration

A single interface (

) aggregates multiple tool servers for your client applications, which reduces

```bash
/mcp
```

integration complexity.

Nutanix Enterprise AI supports management of two types of MCP servers:

Remote MCP Server

An externally hosted MCP server that Nutanix Enterprise AI connects to through a URL. You are responsible for hosting, scaling, and managing the MCP server.

Local MCP Server

A container-based MCP server that is deployed and managed within the Nutanix Enterprise AI cluster. Nutanix Enterprise AI manages the lifecycle, networking, and resource allocation for the server.

#### Adding a Third-Party Credential for a Remote MCP Server

Add a third-party credential for a remote MCP server in Nutanix Enterprise AI.

### About this task

To add a third-party credential, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

The

#### Add Credential

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

### 5.  From the

#### Platform

dropdown menu, select

#### Remote MCP Server

.

### 6.  In the

#### API Key

field, enter the API key.

### 7.  Click

#### Add Credential

.

The third party credential is displayed in the

#### Third Party Credentials

page.

### What to do next

Add a remote MCP server. For more information, see

Adding a Remote MCP Server

on page  292.

#### Adding a Remote MCP Server

Add a remote MCP server that uses Streamable HTTP transport in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Add a credential for the remote MCP server. For more information, see

Adding a Third-Party Credential for a

Remote MCP Server

on page 291.

- 

The remote MCP server must support Model Context Protocol version 2025-06-18. For more information, see

Model Context Protocol

.

- 

Ensure that the remote MCP server is reachable from the Nutanix Enterprise AI server and operational.

- 

Ensure that the remote MCP server supports Streamable HTTP transport and uses a public CA-signed certificate.

- 

Ensure the performance, reliability, safety, and output quality of the remote MCP Server.

- 

Ensure compliance with the security and configuration considerations for MCP Servers. For more information, see

Security and Configuration Considerations for MCP Servers

on page  299.

- 

Caution:   You cannot edit a remote MCP server.

### About this task

To add a remote MCP server, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### MCP Servers

.

The

#### MCP Servers

page is displayed.

### 3.  Select the

#### Servers

tab.

### 4.  From the

#### Add MCP Server

dropdown menu, select

#### Add Remote Server

.

#### Add Remote Server

The

dialog box is displayed.

### 5.  In the

#### Name

field, enter a name that your application can use to identify the server.

### 6.  In the

#### Server URL

field, enter the URL of the remote MCP server.

### 7.  (Optional) From the

#### Credential

dropdown menu, select the credential you created for the remote MCP server.

If the MCP server can be accessed anonymously, do not select a credential.

#### 8.  (Optional) To configure user-header forwarding request headers to the remote MCP server select

#### Enable

#### Request Header Forwarding

.

You can mark specific headers as required. Requests to tools on this server are blocked when a required header is missing or empty.

### 9.  Click

#### Add MCP Server

.

#### Servers

The remote MCP server is displayed on the

page.

### What to do next

Add a connector. For more information, see

Creating an MCP Connector

on page 296.

#### Adding an Image Registry Credential for a Local MCP Server

Add an image registry credential in Nutanix Enterprise AI to pull a local MCP server container image from a private registry.

### Before you begin

Ensure that you have the following:

- 

Access to a private container image registry that hosts the MCP server image.

- 

The registry URL, user name, and password for the private registry

### About this task

If your local MCP server container image is hosted in a private registry, you must add an image registry credential before you add the server. If the image is in a public registry, you can skip this procedure.

To add an image registry credential, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

The

#### Add Credential

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

### 5.  From the

#### Platform

dropdown menu, select

#### Registry (Docker)

.

### 6.  In the

#### Registry URL

field, enter the URL of the container image registry.

### 7.  In the

#### Username

field, enter the user name of the container image registry.

### 8.  In the

#### Password

field, enter the password of the container image registry.

### 9.  Click

#### Add Credential

.

#### Third Party Credentials

The image registry credential is displayed in the

page and is available for local MCP

server deployments.

### What to do next

Deploy a local MCP server. For more information, see

Deploying a Local MCP Server

on page 293.

#### Deploying a Local MCP Server

Deploy a local MCP server as a managed container in the Nutanix Enterprise AI cluster.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

The local MCP server must support Model Context Protocol version 2025-06-18. For more information, see

Model Context Protocol

.

- 

The container image for the MCP server must be accessible from the Nutanix Enterprise AI cluster.

- 

If the container image is hosted in a private registry, add an image registry credential. For more information, see

Adding an Image Registry Credential for a Local MCP Server

on page  293.

- 

The MCP server must support Streamable HTTP transport.

- 

Ensure compliance with the security and configuration considerations for MCP Servers.. For more information, see

Security and Configuration Considerations for MCP Servers

on page  299.

- 

Ensure the performance, reliability, safety, and output quality of the MCP server.

- 

The local MCP server is deployed as a single instance. You cannot configure multiple instances for a local MCP server.

### About this task

To add a local MCP server, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### MCP Servers

From the left navigation pane, select

.

The

#### MCP Servers

page is displayed.

3. 

Select the

#### Servers

tab.

4. 

From the

#### Add MCP Server

dropdown menu, select

#### Deploy Local Server

.

#### Deploy Local Server

The

dialog box is displayed.

5. 

#### Instance Name

In the

field, enter a name that your application can use to identify the local MCP server.

You cannot change the instance name after creation because it associates the local MCP server with the MCP connector.

6. 

(Optional) From the

#### Registry Credential

dropdown menu, select the registry credential that you created for

the MCP server.

7. 

In the

#### Server Package

field, enter the name of the MCP server package.

8. 

Click

#### Next

.

#### Configuration

The

tab is displayed.

9. 

#### Compute

In the

field, enter the CPU allocation in milliCores.

### 10.  In the

#### Memory

field, enter the memory allocation in MiB.

### 11.  In the

#### Port

field, enter the port number on which the MCP server listens.

Supported values are 1 to 65535.

#### 12.  (Optional) To add more arguments, follow these steps:

a. Click

#### + Add Argument

b. Enter the command-line arguments for the container.

Include

if needed.

```bash
--
```

#### 13.  (Optional) To add environment variables, follow these steps:

a. Click

#### Add Environment Variable

.

b. Add environment variables as key-value pairs.

Environment variable values are redacted in API responses.

#### 14.  (Optional) To configure user-header forwarding request headers to the local MCP server, select

#### Enable

#### Request Header Forwarding

.

You can mark specific headers as required. Requests to tools on this server are blocked when a required header is missing or empty.

### 15.  Click

#### Deploy

.

#### Servers

The local MCP server is displayed in the

page.

### What to do next

Add a connector. For more information, see

Creating an MCP Connector

on page 296.

#### Updating a Local MCP Server

Update the configuration of a local MCP server in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Deploying a Local MCP Server

You must have at least one local MCP server. For more information, see

on

page 293.

- 

Review local and remote MCP server guidelines. For more information, see

Security and Configuration

Considerations for MCP Servers

on page  299.

- 

Local MCP servers run as a single instance. You cannot increase or decrease instance count when you update a local MCP server. Existing multi-instance local MCP servers from earlier releases are scaled down to a single instance in this release.

- 

You cannot edit request headers.

### About this task

To update a local MCP server, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI .

### 2.  From the left navigation pane, select

#### MCP Servers

.

#### MCP Servers

The

page is displayed.

### 3.  Select the

#### Servers

tab.

4. Select the local MCP server you want to update.

### 5.  From the

#### Actions

#### Edit

dropdown menu, select

.

The

#### Edit MCP Local Server

dialog box is displayed.

6. (Optional) Update any of the fields.

For more information, see

step

in

Deploying a Local MCP Server

on page  293.

### 7.  Click

#### Update MCP Server

.

The local MCP server is updated.

#### Creating an MCP Connector

Add an MCP connector in Nutanix Enterprise AI to associate MCP servers and define tool access.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have at least one MCP Server. For more information, see

Adding a Remote MCP Server

on

Deploying a Local MCP Server

page 292  or

on page 293.

- 

Create a MCP client key

### About this task

To add a connector, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### MCP Servers

.

#### MCP Servers

The

page is displayed.

3. 

#### Connectors

Select

.

4. 

#### Create MCP Connector

Click

.

#### Create MCP Connector

The

dialog box is displayed.

5. 

#### Connector Name

In the

field, enter a name.

6. 

Select an MCP client key.

- 

#### MCP Client Keys

Select a MCP client key that you created earlier. From the

dropdown menu, select a key

that you created earlier.

- 

Create a new key.

#### 1.  Click

#### Create a New Key

.

2. Copy the key value and save it.

#### 3.  From the

#### MCP Client Keys

dropdown menu, select the key you created.

7. 

#### MCP Servers

From the

dropdown menu, select the server that you created earlier.

Requests sent through the connector forward the MCP user header to the selected MCP server.

8. 

In the

#### Actions

column, click the

#### edit

icon.

The

#### Add Tool Access

dialog box is displayed.

9. 

Grant the tools which the connector can access for the selected servers.

a. Select the tools.
b. Click

#### Update

.

Make tools available only when absolutely necessary.

#### 10.  (Optional) To edit the selected tools, repeat

8

on page 296 and

9

on page 297.

#### 11.  (Optional) To delete the selected tools, click the

#### Delete

icon.

### 12.  Click

#### Create MCP Connector

.

The connector is displayed in the

#### Connectors

tab.

### What to do next

Update the MCP client key-to-connector association from the  MCP Client Keys  page.

#### Creating an MCP Client Key

Create an MCP client key in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must have an MCP connector. For more information, see

Creating an MCP Connector

on page 296.

### About this task

To create an MCP client key, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### API Client Keys

.

#### API Client Keys

The

page is displayed.

### 3.  Click

#### MCP Client Keys

.

### 4.  Click

#### Create a New Key

.

The

#### Create MCP Client Key

dialog box is displayed.

### 5.  In the

#### Key Name

field, enter a name.

Nutanix recommends that you enter a meaningful and identifiable name.

### 6.  From the

#### MCP Connectors

dropdown menu, select an MCP connector that you created earlier.

### 7.  Click

#### Create

.

The

#### MCP Client Key Details

dialog box is displays the details of the MCP client key you created..

### 8.  Click

#### Copy Key

.

> [!NOTE]
> Warning:   You cannot view the MCP client key after you close the

#### MCP Client Key Details

dialog box. If you

lose the key, you must generate a new one.

The key is copied to the clipboard.

9. Save the key securely.

### What to do next

Updating MCP Client

Update the connectors attached to an existing MCP client key. For more information, see

keys

on page 298

#### Updating MCP Client keys

Update the MCP Client keys attached to a MCP Connector.

### About this task

To update the MCP Client keys attached to a MCP Connector, follow these steps.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### API Client Keys

.

The

#### API Client Keys

page is displayed.

### 3.  Select

#### MCP Client Keys

.

4. Select the required MCP client key.

### 5.  From the

#### Actions

menu, select

#### Update

.

The

#### Update Client Key

dialog box is displayed.

### 6.  From the

#### MCP Connectors

dropdown menu, update the connectors.

7. Click Update.

#### Viewing the Logs of an MCP Server

View logs of an MCP server in Nutanix Enterprise AI to monitor activity or troubleshoot issues.

### Before you begin

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page  155.

### About this task

To view the logs of an MCP server, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Agentic Tools and Data

>

#### MCP Servers

.

#### Servers

The

page is displayed.

3. Select an MCP server.

The details page of the MCP server is displayed.

### 4.  Click the

#### Logs

tab.

### 5.  From the

#### Instance

dropdown menu, select an instance.

The logs of the selected MCP server instance are displayed.

### 6.  (Optional) To view the latest logs, click

#### Refresh

.

The latest logs are displayed.

### What to do next

To download the displayed logs, see

Downloading the Logs of an MCP Server

on page 299.

#### Downloading the Logs of an MCP Server

Download MCP server logs to your local system for troubleshooting and analysis.

### Before you begin

Display the logs of the MCP server instance you want to download. For more information, see

Viewing the

Logs of an MCP Server

on page 298.

### About this task

To download MCP server logs, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Agentic Tools and Data

#### MCP Servers

>

.

The

#### Servers

page is displayed.

3. Select an MCP server.

The details page of the MCP server is displayed.

### 4.  Click the

#### Logs

tab.

### 5.  From the

#### Instance

dropdown menu, select an instance.

The logs of the selected MCP server instance are displayed.

### 6.  (Optional) To view the latest logs, click

#### Refresh

.

The latest logs are displayed.

### 7.  Click the

#### Download Logs

icon.

Logs are stored on the Kubernetes pod and rotated by the Kubernetes log rotation policy. Only the logs retained by the rotation policy are available for download.

> [!NOTE]
> Note:   You can download a maximum of 10 MB of log lines.

#### Security and Configuration Considerations for MCP Servers

Default security and configuration constraints that Nutanix Enterprise AI applies to local and remote MCP servers to maintain cluster security and workload isolation.

### Security Configuration for Local MCP Servers

Local MCP servers are deployed with strict controls on network access, runtime permissions, file system usage, and naming conventions. Failure to comply with these constraints can cause runtime failures.

Network Policies for Local MCP Servers

Nutanix Enterprise AI enforces a default NetworkPolicy named

```bash
nai-admin-extensions-network-
```

in the

namespace. The policy enforces the following rules:

```bash
policy
nai-admin-extensions
```

- 

Ingress is restricted to the

and

namespaces.

```bash
nai-system
envoy-gateway-system
```

- 

Egress is allowed to all destinations except the Kubernetes API server.

If the MCP server requires additional ingress, add a

in the

```bash
NetworkPolicy
nai-admin-extensions
```

namespace by using the following selector to target MCP server pods:

```bash
spec:
podSelector:
matchLabels:
nai.nutanix.com/feature: mcp
```

Nutanix recommends that you do not edit the

to allow egress to the Kubernetes API server.

```bash
NetworkPolicy
```

For non-NKP deployments on Amazon Elastic Kubernetes Service, Google Kubernetes Engine, and Azure Kubernetes Service, the Kubernetes API server might have direct Fully Qualified Domain Name access that resolves to load balancer IP addresses. To block access through that endpoint, include the IP address in the

block of egress IPs. The following template excludes the Kubernetes service ClusterIP from the

```bash
except
```

egress allow-list:

```bash
- to:
- ipBlock:
cidr: 0.0.0.0/0
{{- $kubeService := (lookup "v1" "Service" "default" "kubernetes") }}
{{- if $kubeService }}
except:
- {{ $kubeService.spec.clusterIP }}/32
{{- end }}
```

Deployment Security Flags for Local MCP Servers

Every local MCP server deployment specification includes the following security flags:

- 

and

```bash
RunAsNonRootUser
RunAsUser 65534
```

MCP servers are blocked from running as the root user or any privileged user other than the

user

```bash
Nobody
```

. Any local MCP server that requires root user or privileged user access fails.

```bash
65534
```

- 

Root filesystem access is read-only

Root filesystem write access is blocked in all MCP server deployments. Servers that require write access to the root filesystem fail.

Configuration for Local MCP Servers

```bash
EmptyDir
```

When you deploy a local MCP server, account for any directories that the server creates at run time.

If your server creates directories internally such as

,

,

, or

, only the following paths

```bash
/tmp
/.cache
/app
/logs
```

are supported:

- 
```bash
/.cache
```

- 
```bash
/tmp
```

Nutanix Enterprise AI does not validate or restrict directory paths created inside the server container.

Directories under

and

are supported. Directories under other paths, such as

or

,

```bash
/.cache
/tmp
/app
/logs
```

are not supported and cause startup failure.

Write all temporary files, caches, and runtime-generated data to

or

only. Do not hardcode or

```bash
/.cache
/tmp
```

dynamically create directories outside these paths. If your application requires file writes, refactor it to use one of the supported directories.

### Requirements for Remote MCP Servers

Remote MCP servers must meet the following URL, connectivity, and transport requirements:

- 

Include

in the URL.

```bash
https
```

- 

Use a DNS host name in the URL. IP addresses are not allowed.

- 

If you do not specify port, the default port

is used.

```bash
443
```

- 

Use a public CA-signed TLS certificate.

- 

Supported security protocols: TLS or SSL

Example URL:

```bash
https://api.githubcopilot.com/mcp/
```

#### Tool Name Length Limit for Local and Remote MCP Servers

Limit tool names to 34 characters.

The Model Context Protocol allows tool names up to 128 characters. However, most MCP clients impose a practical limit of 64 to 70 characters and ignore tool names that are longer than 64 to 70 characters.

#### Forward Header Considerations for Local and Remote MCP Servers

When you configure header forwarding for an MCP server, use only headers that meet the following constraints:

- 

Configure up to 32 forwarded headers per MCP server.

- 

Limit each header name to 1024 characters.

- 

Use only alphanumeric characters and the following special characters in header names:

,

,

,

, and

.

```bash
_
.
-
+
$
```

- 

Do not configure duplicate headers. Header names are case-insensitive.

- 

Do not use reserved header prefixes such as

,

,

, and

.

```bash
X-Nutanix
x-ai-eg
Authorization
x-vsr
```

- 

If a forwarded header is marked as required, requests to tools on that MCP server are denied when that header is missing or empty.