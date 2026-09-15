# Nutanix Enterprise AI Manual: NAI Labs, Vector Databases, and Support Tools

NAI Labs sample applications (Chat, Talk to My Data, Agent application), Milvus vector database integration, application configuration resets, Nutanix Enterprise AI support bundle generation (NKP and non-NKP), online help, and glossary of terms.

---

## NAI LABS

NAI Labs  includes the Chat, Talk To My Data, and the Agent applications to quickly verify inference endpoints, observe end-to-end functionality, and visualize operational workflows without developing custom applications.

The guidelines to use NAI Labs are as follows:

- 

NAI Labs has limited file processing capabilities. Ensure that the files that you upload meet the following criteria:

- 

The filetypes can be .pdf, .docx, .txt, .md, .markdown, .rtf, .json, .xml, .yaml, .yml, and .log.

- 

The file size can be up to 5 MB.

- 

The number of files can be up to five.

- 

The data uploaded is removed after two hours of the logged in user's inactivity and remains only on the NAI cluster.

- 

The data you upload to the applications is stored only in your environment.

- 

NAI Labs is available only in preview mode.

- 

NAI Labs is for testing and validation purposes only.

- 

NAI Labs supports integration with Milvus, an external vector database, to improve performance and scalability. You can either connect to Milvus for higher-performance workloads or continue using the database included with NAI Labs.

Connecting NAI Labs to an External Milvus Vector Database

For more information, see

on page 319.

- 

The number of identities assigned to the ML Users role who can access NAI Labs simultaneously depends on the following database configurations:

- 

NAI with Milvus: All ML Users can access NAI Labs simultaneously.

- 

NAI with the database included in NAI: A maximum of five ML Users can access NAI Labs simultaneously.

You can perform several actions with NAI Labs such as configuring and interacting with the following applications: Chat, Talk To My Data, and Agent.

In Agent, you can configure a chat model, register connectors and tools, optionally upload documents for grounded answers, and review source citations in responses.

#### Configuring the Chat Application in NAI Labs

Configure the  Chat  application in  NAI Labs .

### Before you begin

- 

Create an LLM endpoint with text-to-text capability using a large language model of your choice. For more information, see

Local Endpoints in Nutanix Enterprise AI

on page 214.

- 

(Optional) Connect NAI Labs to an external Milvus vector database to improve performance and scalability. For more information, see

Connecting NAI Labs to an External Milvus Vector Database

on page  319.

- 

The Chat application is disabled by default. To enable, see

Deploy Nutanix Enterprise AI

on page 27.

### About this task

To configure the Chat application in NAI Labs, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI as follows:.

### 2.  From the navigation bar, select

#### NAI Labs

>

#### Chat

.

### 3.  From the

#### LLM Endpoint Name

dropdown menu, select an endpoint that you created earlier.

Only endpoints with the model capability text generation are displayed.

### 4.  In the

#### LLM API Key

field, enter the API key associated with the text-to-text LLM endpoint that you selected

above.

### 5.  Click

#### Save

.

You can interact with the endpoint. For more information, see

Interacting with the Chat Application

on

page 314.

#### Interacting with the Chat Application

Interact with the Chat application.

### Before you begin

Configure Chat. For more information, see

Configuring the Chat Application in NAI Labs

on page 313.

### About this task

To interact with the Chat application, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### NAI Labs

>

#### Chat

.

### 3.  In the

#### Chat Preview

section, enter your query in the message bar.

### 4.  Click

#### Ask Me

.

The results of your query are displayed.

#### Configuring the Talk To My Data Application in NAI Labs

The  Talk To My Data  application retrieves information from a specific document. You can configure the Talk To My Data  application in  NAI Labs .

### Before you begin

- 

Create endpoints. For more information, see

Local Endpoints in Nutanix Enterprise AI

on page 214.

- 

(Optional) Connect NAI Labs to an external Milvus vector database to improve performance and scalability. For more information, see

Connecting NAI Labs to an External Milvus Vector Database

on page  319.

- 

Deploy Nutanix Enterprise AI

The Talk To My Data application is disabled by default. To enable, see

on

page 27.

### About this task

To configure the Talk To My Data application in NAI Labs, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the navigation bar, select

#### NAI Labs

>

#### Talk To My Data

.

3. 

From the

#### LLM Endpoint Name

dropdown menu, select an endpoint that you created earlier.

Only endpoints with the model capacity of text generation are displayed.

4. 

#### LLM API Key

In the

field, enter the assigned API key for that model.

5. 

#### Embedding Endpoint Name

From the

dropdown menu, select an endpoint that you created earlier.

6. 

#### Embedding API Key

In the

field, enter the assigned API key for that model.

7. 

#### Reranker Endpoint Name

From the

dropdown menu, select an endpoint that you created earlier.

8. 

#### Reranker API Key

In the

field, enter the assigned API key for that model.

Upload a document.

9. 

You can upload up to five files with a maximum size of 5MB each.

#### Optional Configuration

### 10.  Click the

dropdown menu.

#### Safeguard Endpoint Name

### 11.  From the

dropdown menu, select an endpoint that you created earlier.

Only the endpoints created using the following models are displayed:

- 

meta-llama/Llama-Guard-3-8B llamaguard from Hugging Face

- 

llama-3.1-nemoguard-8b-content-safety Nemoguard from NVIDIA NGC

### 12.  In the

#### Safeguard API Key

field, enter the assigned API key for that model.

### 13.  From the

#### Object Detection Endpoint Name

dropdown menu, select an endpoint that you created earlier.

Only the endpoints created using the nemoretriever-parse model from the NVIDIA NGC catalog are displayed.

### 14.  In the

#### Object Detection API Key

field, enter the assigned API key for that model.

### 15.  Click

#### Save

.

Interacting with the Talk To My Data

You can interact with the endpoint. For more information, see

Application

on page  315.

#### Interacting with the Talk To My Data Application

Interact with the Talk To My Data application.

### Before you begin

Configure Talk To My Data. For more information, see

Configuring the Talk To My Data Application in NAI

Labs

on page 314.

### About this task

To interact with the Talk To My Data application, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### NAI Labs

>

#### Talk To My Data

.

### 3.  In the

#### Talk To My Data Preview

section, enter your query in the message bar.

### 4.  Click

#### Ask Me

.

The results of your query are displayed along with the link to the document.

Resetting the Configuration for the Chat or Talk To My Data Applications

Reset the configuration for the  Chat  or  Talk To My Data  applications.

### About this task

To reset the configuration, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the navigation bar, select

#### NAI Labs

.

#### 3.  (Optional) To reset the configuration for the Chat application, follow these steps:

a. Click

#### Chat

.

b. From the left pane, click

#### Reset

.

c. In the confirmation box, click

#### Confirm

.

#### 4.  (Optional) To reset the configuration for the Talk To My Data application, follow these steps:

a. Click

#### Talk To My Data

.

b. From the left pane, click

#### Reset

.

A confirmation box is displayed.

c. (Optional) Select the

#### Keep the endpoint values in the form

checkbox.

d. Click

#### Confirm

.

#### Configuring the Agent application in NAI Labs

Configure the Agent application in Nutanix Enterprise AI.

### Before you begin

- 

You must have at least one local endpoint or an unified endpoint.

For more information, see

Local Endpoints in Nutanix Enterprise AI

on page  214 or

Unified Endpoints in

Nutanix Enterprise AI

on page  253.

- 

You must have at least one local MCP server or remote MCP server configured.

MCP Servers

For more information, see

on page 291.

- 

For document grounding, ensure that do have the following:

- 

Create at least one embedding endpoint.

- 

Create at least one reranker endpoint.

- 

Provide files.

### About this task

Configure the Agent application to connect to an LLM endpoint, register an MCP connector, and enable the tools for the Agent to use. You can also optionally configure document grounding to enable the Agent to provide citation- backed responses based on uploaded documents.

To configure the NAI Agent application, follow these steps:

### Procedure

### 1.  From the navigation bar, select

#### NAI Labs

#### Agent

>

.

### 2.  Click

#### +

#### Models

and then click

.

#### Chat model

The

dialog box is displayed.

### 3.  In the

#### Chat model

dialog box, follow these steps:

a. From the

#### LLM Endpoint Name

dropdown menu, select a local endpoint or an unified endpoint that you

created earlier.

b. In the

#### API Key

field, enter the API key for the selected LLM endpoint.

c. Click

#### Save

.

### 4.  Click

#### +

and then click

#### Connectors and Sources

.

#### Connectors and Sources

The

dialog box is displayed.

### 5.  In the

#### Connectors and Sources

dialog box, follow these steps:

a. From the

#### Add MCP Connector

dropdown menu, select MCP connector that you configured earlier.

b. In the

#### MCP API Key (optional)

field, enter the API key

c. Click

#### Add Server

.

The registered server and the available tools are displayed.

d. Select the tools to use.
e. Click

#### Save

.

The Agent application is configured and ready for use.

#### 6.  (Optional) Configure document grounding to enable citation-backed responses, and interact with the Agent

application to submit prompts and review grounded answers.To configure document-grounded answers, follow these steps:

a. Click

#### +

and then click

#### Documents

.

The Document dialog box is displayed.

b. Click

#### Enable Document Retrieval

.

c. From the

#### Embedding Endpoint

dropdown menu, select an embedding endpoint that you created earlier.

d. In the

#### Embedding API Key

field, enter the API key that you created earlier.

e. From the

#### Reranker Endpoint

dropdown menu, select a reranker endpoint that you created earlier.

f. In the

#### Reranker API Key

field, enter the API key that you created earlier.

g. Click

#### Save Settings

.

h. Click

#### Upload documents

.

i. Select a document.

Ensure that the document youn upload meets the following requirements:

- 

You can upload up to five documents per thread.

- 

The maximum upload size is 5 MB per file.

- 

Supported file extensions include .pdf, .docx, .txt, .md, .markdown, .rtf, .json, .xml, .yaml, .yml, and .log.

#### Ready

Wait till documents are displayed below the message bar and the status changes to

.

Agent can use uploaded documents to provide grounded answers with citations.

### What to do next

Interact with the Agent application. For more information, see

Interacting with the Agent Application

on

page 318.

#### Interacting with the Agent Application

Interact with the Agent application in Nutanix Enterprise AI.

### Before you begin

Configure the Agent application. For more information, see

Configuring the Agent application in NAI Labs

on page 316.

### About this task

Use the Agent application to submit prompts to the configured language model and, when required, approve requests to use configured tools. If document grounding is enabled, you can also review the sources and document content used to generate a response.

To interact with the Agent application, follow these steps:

### Procedure

### 1.  From the navigation bar, select

#### NAI Labs

>

#### Agent

.

2. Enter your prompt in the message bar.

### 3.  Click the

#### send prompt

icon.

A prompt requesting approval to use the tool is displayed.

### 4.  Click

#### Approve

.

The results for your query are displayed.

#### 5.  Verify the source for document-grounded responses. To verify follow these steps:

a. Click the source indicator to view citations and supporting document chunks.
b. Review the citation list and select a source item.
c. In the source viewer, review the cited text chunk and surrounding content that were used to generate the

response.

6. (Optional) To create a separate conversation, use the new-thread tab and enter a thread name.

#### Connecting NAI Labs to an External Milvus Vector Database

Connect NAI Labs to an external Milvus vector database for improved scalability and multi-user support.

### Before you begin

- 

A Milvus vector database instance accessible from your Kubernetes cluster. You can set up Milvus Database in one of the following ways:

- 

Self-hosted Milvus

1. Ensure a Milvus v2.6 or higher version instance is installed and running.
2. Confirm that the Milvus instance is reachable from your Kubernetes cluster.
- 

Zilliz Cloud or managed Milvus

#### 1.  Sign up at

https://cloud.zilliz.com

.

2. Create a Milvus cluster.

#### 3.  Note down the following:

- 

endpoint URL

- 

API token

- 

Collect the following:

- 

Host/IP: Milvus server hostname or IP address

- 

Port: You need one of the following:

- 

19530 for self-hosted.

- 

443 for Zilliz Cloud.

- 

Token: Authentication token (if required)

- 

(Optional) If your Milvus setup requires TLS encryption, you need the following:

- 

CA certificate file.

For example,

file.

```bash
ca.pem
```

- 

Server hostname for certificate verification.

### About this task

To connect Milvus, follow these steps:

### Procedure

### 1.  In the NAI Core

file, search for

.

```yaml
values.yaml
naiLabs
```

»

For a basic connection without TLS, copy the following under

:

```bash
naiLabs
naiLabs:
enabled: true
vectorDb:
external: true  # Enable external Milvus
milvus:
host: "10.52.48.98"        # Your Milvus IP/hostname
port: "19530"              # Milvus port
token: ""                  # Leave empty if no auth
milvusTLSEnabled: false
```

»

For a connection with TLS encryption, copy the following under

:

```bash
naiLabs
```

The following example is for Zilliz Cloud:

```bash
naiLabs:
enabled: true
vectorDb:
external: true
milvus:
host: "your-cluster.zilliz.cloud"
port: "443"
token: "your-api-token"           # Your Zilliz token
# TLS Configuration
milvusTLSEnabled: true
milvusTLSSecretName: ""
milvusTLSCACertName: ""     # Leave empty as Zilliz cloud is public
milvusTLSServerName: ""
```

#### 2.  (Optional) To use custom CA, create a TLS secret:

a. Create a TLS secret:
```bash
kubectl create secret generic nai-milvus-tls-certs \
--from-file=ca.pem=/path/to/your/ca-certificate.pem \
-n nai-system
```

b. Verify if the secret was created:
```bash
kubectl get secret nai-milvus-tls-certs -n nai-system
kubectl describe secret nai-milvus-tls-certs -n nai-system
```

Your

must be as follows:

```yaml
values.yaml
naiLabs:
enabled: true
vectorDb:
external: true  # Enable external Milvus
milvus:
# Connection details
host: "10.52.48.98"              # Your Milvus server IP/hostname
port: "19530"                     # Milvus port (19530 for self-hosted, 443
for Zilliz)
token: "your-token-here"          # Auth token (leave empty "" if not
required)
# TLS Configuration (one-way TLS)
milvusTLSEnabled: true
milvusTLSSecretName: "nai-milvus-tls-certs"    # Must match secret name
above
milvusTLSCACertName: "ca.pem"                   # Must match KEY NAME inside
secret
milvusTLSServerName: ""                         # Optional: leave empty to
use 'host' value
```

### 3.  Apply the Milvus configuration:

Install or upgrade the Helm chart:

```bash
helm upgrade --install nai-core ./charts/nai-core \
-n nai-system \
-f values.yaml
```

### 4.  Verify the deployment:

```bash
kubectl get pods -n nai-system | grep nai-labs
kubectl logs -n nai-system deployment/nai-labs
```

## NUTANIX ENTERPRISE AI SUPPORT BUNDLE

The Nutanix Enterprise AI Support Bundle contains all the necessary Kubernetes pod logs and resource information required to debug an ongoing issue for your Nutanix Enterprise AI deployment.

The Nutanix Enterprise AI Support Bundle assists the Nutanix support team with troubleshooting. The procedures to generate the support bundle are as follows:

- 

Generating Support Bundle in an NKP

If you have installed Nutanix Kubernetes Platform (NKP), see

Environment

on page  322.

- 

If you have not installed Nutanix Kubernetes Platform (NKP), see

Generating Support Bundle in a Non-NKP

Environment

on page  322.

#### Generating Support Bundle in an NKP Environment

Generate support bundle in an NKP environment.

### About this task

To generate support bundle in an NKP environment, follow these steps:

### Procedure

1. Download NKP CLI.

Downloading NKP

For more information, see

.

#### 2.  Collect the logs, metrics, and configuration data from your Kubernetes cluster in a zipped support bundle:

```yaml
nkp diagnose --kubeconfig=cluster-kubeconfig.yaml
```

For more information on the NKP diagnose command, see

NKP Troubleshooting Guide

.

The zipped support bundle is downloaded.

3. Extract the zipped support bundle.

The downloaded artifacts are displayed.

### What to do next

Share the generated bundle with the Nutanix Support Team.

For more information, see

KB-1294 Uploading Files for Nutanix Support Using FTP, SFTP, or the Customer

Portal

.

#### Generating Support Bundle in a Non-NKP Environment

Generate support bundles in a non-NKP environment.

### Before you begin

Ensure that you have a Kubernetes cluster and local

access to the cluster. Install

from

```bash
kubectl
kubectl
```

kubectl

.

### About this task

To generate support bundles in a non-NKP environment, follow these steps:

### Procedure

#### 1.  Download the binary for your operating system and architecture from GitHub Releases:

»

For Linux AMD64:

```bash
curl -L https://github.com/replicatedhq/troubleshoot/releases/download/v0.119.0/
support-bundle_linux_amd64.tar.gz | tar xzvf -
```

»

For Linux ARM64:

```bash
curl -L https://github.com/replicatedhq/troubleshoot/releases/download/v0.119.0/
support-bundle_linux_arm64.tar.gz | tar xzvf -
```

»

For macOS ARM64 (Apple Silicon):

```bash
curl -L https://github.com/replicatedhq/troubleshoot/releases/download/v0.119.0/
support-bundle_darwin_arm64.tar.gz | tar xzvf -
```

Releases

For any other OS architecture, see

.

### 2.  Verify if the plug-in is installed correctly:

```yaml
./support-bundle version
```

### 3.  Ensure your cluster's

is configured:

```bash
KUBECONFIG
export KUBECONFIG=/path/to/your/kubeconfig
```

### 4.  Collect the support bundle:

Run the support bundle collection with your collector configuration:

```yaml
./support-bundle support-bundle-collector.yaml
```

This creates a zipped archive containing logs, metrics, and configuration data from your Kubernetes cluster.

The format of the

file is as follows:

```yaml
support-bundle-collector.yaml
apiVersion: troubleshoot.sh/v1beta2
kind: SupportBundle
metadata:
name: nai-support-bundle
spec:
collectors:
- helm: {} # collects information about all helm releases in all namespaces
- nodeMetrics: {} # collects metrics for all the nodes, all containers within the
nodes
- logs:
collectorName: "nai-system-logs"
name: "nai-system-logs"
namespace: nai-system # collects logs for all the containers in nai-system
namespace
- logs:
collectorName: "nai-admin-logs"
name: "nai-admin-logs"
namespace: nai-admin
```

The zipped support bundle is downloaded.

5. Extract the zipped support bundle.

The downloaded artifacts are displayed.

6. Ensure the support bundle doesn't contain any sensitive information.

You can redact sensitive information by using the

https://troubleshoot.sh/docs/redact/

feature.

### What to do next

Share the generated bundle with the Nutanix Support Team.

KB-1294 Uploading Files for Nutanix Support Using FTP, SFTP, or the Customer

For more information, see

Portal

.

## ACCESSING ONLINE HELP

NAI includes online help documentation that you can access at any time.

### About this task

To access the online help documentation, follow these steps.

### Procedure

1. Log in to NAI.
2. Click the user name in the upper-right corner.

#### Online Documentation

### 3.  From the dropdown menu, select

.

The online help documentation appears in a new tab or window. These pages are located on the Nutanix support portal.

## GLOSSARY

Nutanix Glossary

For terms used in this guide, see

.

## COPYRIGHT

Copyright 2026 Nutanix, Inc.

Nutanix, Inc.

1740 Technology Drive, Suite 150

San Jose, CA 95110

All rights reserved. This product is protected by U.S. and international copyright and intellectual property laws. Nutanix and the Nutanix logo are registered trademarks of Nutanix, Inc. in the United States and/or other jurisdictions. All other brand and product names mentioned herein are for identification purposes only and may be trademarks of their respective holders.