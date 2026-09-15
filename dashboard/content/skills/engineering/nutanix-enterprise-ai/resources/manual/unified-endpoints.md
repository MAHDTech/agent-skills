+++
title = "unified-endpoints"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Unified Endpoints and External Model Providers

Unified endpoints architecture, multi-model routing across local endpoints, external model provider integration (Anthropic, AWS Bedrock, Azure OpenAI, Cohere, GCP Vertex AI, Google Gemini, Mistral, Remote NAI, OpenAI), provider credentials, rate limit management, and OpenAI-compatible client access.

---

#### UNIFIED ENDPOINTS IN NUTANIX ENTERPRISE AI

You can perform the following actions:

- 

Creating a Unified Endpoint using Local Endpoints

Create a unified endpoint using local endpoint.

on

page 253

- 

Managing rate limits for a unified endpoint, including global rate limits and per-client rate limits by API client key or request header value.

Managing Rate Limit for Unified Endpoints

on page  269

- 

Deleting a Unified Endpoint

Deleting a unified endpoint.

on page  271

- 

Access an endpoint using Open AI compatible clients.

#### Creating a Unified Endpoint using Local Endpoints

Create unified endpoints to create a common interface to an LLM and back it up with multiple endpoints from multiple clusters. A unified endpoint gives you a single API address that connects to one or more inference services even if they run in different clusters so your applications can access multiple models through one simple interface.

### Before you begin

Create at least one local endpoint.

### About this task

To create unified endpoints, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Unified Endpoints

From the left navigation pane, select

.

The

#### Unified Endpoints

page displays all the endpoints that you created earlier.

3. 

Click

#### Create Unified Endpoint

.

#### Basics

#### Create a Unified Endpoint

The

tab of the

dialog box is displayed.

4. 

#### Endpoint Name

In the

field, enter a name for the endpoint.

5. 

#### API Spec

From the

dropdown menu, select a value based on the required input and output.

6. 

#### Models

#### Add Model

In the

section, click

.

#### Add Model

The

dialog box is displayed.

7. 

#### Local

Select

.

8. 

Select a local endpoint running on the same cluster.

a. Select the

#### Type

as

#### Local

.

b. From the

#### Endpoint

dropdown menu, select an endpoint you created earlier.

The

#### Model Instance Name

field is auto populated.

c. Click

#### Add Model

.

#### Models

The model is displayed in the

section.

9. 

(Optional) To add multiple models, repeat either step 7 or step 8.

10. (Optional) To edit a model, click the edit icon.
11. (Optional) To delete a model, click the delete icon.

### 12.  Click

#### Next

.

The

#### Configuration

tab is displayed.

»

#### Load Balance

»

#### Fallback Order

#### 13.  (Optional) To select the load balance strategy, follow these steps:

a. Select

#### Load Balance

.

The models are displayed.

b. In the

#### Load

column, enter the load for each model.

#### 14.  (Optional) To select the fallback order strategy, follow these steps:

a. Select

#### Fallback Order

.

### 15.  From the

#### API Client Keys

dropdown menu, select an API Key that you created for this unified endpoint.

### 16.  Click

#### Next

.

The

#### Summary

tab is displayed.

17. Review the information.

### 18.  Click

#### Create

.

The system creates the unified endpoint, which is displayed on the

#### List

page.

#### Creating a Unified Endpoint using Providers

Create a unified endpoint to provide a single API address for one or more inference services across clusters, so your applications can access multiple models through one interface.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

You must add at least one of the following providers:

- 

Anthropic

Adding an Anthropic Provider

For more information, see

on page  256.

- 

AWS Bedrock

Adding an AWS Bedrock Provider

For more information, see

on page  258.

- 

Azure Open AI

For more information, see

Adding an Azure Open AI Provider

on page  259.

- 

Cohere

For more information, see

Adding a Cohere Provider

on page 261.

- 

GCP Vertex AI

Adding a GCP Vertex AI Provider

For more information, see

on page 262.

- 

Google Gemini

Adding a Google Gemini Provider

For more information, see

on page  263.

- 

Mistral

For more information, see

Adding a Mistral Provider

on page 265.

- 

Remote NAI

For more information, see

Adding a Remote NAI Provider

on page  266.

- 

Open AI

Adding a Open AI Provider

For more information, see

on page  267.

### About this task

To create unified endpoints, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI .

2. 

From the left navigation pane, select

#### Unified Endpoints

.

#### Unified Endpoints

The

page displays all the unified endpoints that you created earlier.

3. 

#### Create Unified Endpoint

Click

.

The

#### Basics

tab of the

#### Create a Unified Endpoint

dialog box is displayed.

4. 

In the

#### Endpoint Name

field, enter a name for the endpoint.

5. 

From the

#### Task Type

dropdown menu, select

#### Messages

only for providers or endpoints created using

Anthropic models that support messages ingress.

6. 

In the

#### Models

section, click

#### Add Model

.

The

#### Add Model

dialog box is displayed.

7. 

Select

#### Provider

.

8. 

Select the

#### Type

as

#### Provider

.

9. 

From the

#### Provider

dropdown menu, select a provider you created earlier.

### 10.  From the

#### Model

dropdown menu, select a model.

### 11.  In the

#### Model Name

field, enter the exact name of the model.

### 12.  Click

#### Add Model

.

### 13.  (Optional) To add multiple models, repeat

Add Model

Click Add Model

to

.

### 14.  (Optional) To edit a model, click the

#### edit

icon.

### 15.  (Optional) To delete a model, click the

#### delete

icon.

### 16.  Click

#### Next

.

#### Configuration

The

tab is displayed.

»

#### Load Balance

»

#### Fallback Order

#### 17.  (Optional) To select the load balance strategy, follow these steps:

a. Select

#### Load Balance

.

The models are displayed.

b. In the

#### Load

column, enter the load for each model.

#### 18.  (Optional) To select the fallback order strategy, select

#### Fallback Order

.

### 19.  From the

#### API Client Keys

dropdown menu, select an API Key that you created for this unified endpoint.

### 20.  Click

#### Next

.

The

#### Summary

tab is displayed.

21. Review the information.

### 22.  Click

#### Create

.

The unified endpoint is displayed in the

#### List

page.

#### Adding an Anthropic Provider

Add an Anthropic provider in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Add a third party credential for Anthropic.

For more information, see

Adding a Third Party Credential for Anthropic

on page  257.

### About this task

To add an Anthropic provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

#### Providers

The

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

#### Anthropic

dropdown menu, select

.

6. Assign a third party credential.

»

#### Platform-Credential

Select a credential that you created earlier. From the

dropdown menu, select the

credential you created for Anthropic.

»

#### Add Credential

Add a new credential. Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party Credential

for Anthropic

on page  257.

### 7.  Click

#### Add Provider

.

#### Providers

The provider is displayed in the

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Anthropic Add a credential for Anthropic in Nutanix Enterprise AI to enable inference features.

### About this task

To add a credential for Anthropic, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

#### Platform

#### Anthropic

### 5.  From the

dropdown menu, select

#### API Key

### 6.  In the

field, enter the API Key.

### 7.  Click

#### Add Credential

.

The third party credential is displayed in the

#### Third Party Credentials

page.

### What to do next

Add an Anthropic provider. For more information, see

Adding an Anthropic Provider

on page 256.

#### Adding an AWS Bedrock Provider

Add an AWS Bedrock provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Amazon Web Services (AWS).

For more information, see

Adding a Third Party Credential for Amazon Web Services

on page  258.

### About this task

To add an AWS Bedrock provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

The

#### Providers

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

#### AWS Bedrock

dropdown menu, select

.

### 6.  Do any of the following:

- 

#### Platform-Credential

From the

dropdown menu, select the credential you created for AWS Bedrock.

- 

Click

#### Add Credential

.

#### Add Credential

Adding a Third Party Credential

The

dialog box is displayed. For more information, see

for Amazon Web Services

on page 258.

### 7.  Click

#### Add Provider

.

The provider is displayed in the

#### Providers

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Amazon Web Services Add a credential for Amazon Web Services (AWS) so that you can configure an AWS Bedrock provider in Nutanix Enterprise AI (NAI).

### About this task

To add a third party credential for AWS, follow these steps:

### Procedure

Log in to Nutanix Enterprise AI.

1. 
2. 

From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed. You can view, update, or delete only the credentials that you

created.

3. 

#### Add Credential

Click

.

#### Add Credential

The

dialog box is displayed.

4. 

#### Credential Name

In the

field, enter the name for the credential.

5. 

#### Platform

#### Amazon Web Services (AWS)

From the

dropdown menu, select

.

6. 

#### Region

In the

field, enter an AWS region.

7. 

#### Access Key ID

In the

field, enter the AWS access key ID.

8. 

#### Secret Access Key

In the

field, enter the AWS secret access key.

#### Hostname

9. 

In the

field, enter the fully qualified domain name of the AWS Bedrock runtime endpoint.

Use the format

. For example,

```bash
bedrock-runtime. region .amazonaws.com
bedrock-runtime.us-
```

.

```bash
east-1.amazonaws.com
```

### 10.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

### What to do next

Add an AWS Bedrock provider. For more information, see

Adding an AWS Bedrock Provider

on

page 258.

#### Adding an Azure Open AI Provider

Add an Azure Open AI provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Azure Open AI.

Adding a Third Party Credential for Open AI

For more information, see

on page 268.

### About this task

To add an Azure Open AI provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

#### Providers

The

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  (Optional) Add an Open AI provider

a. From the

#### Platform

dropdown menu, select

#### Azure Open AI

.

b. Do any of the following:
- 

From the

#### Platform-Credential

dropdown menu, select the credential you created for Azure Open AI.

- 

#### Add Credential

Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party

Credential for Microsoft Azure

on page 260.

### 6.  Click

#### Add Provider

.

#### Providers

The provider is displayed in the

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Microsoft Azure Add a third party credential for Microsoft Azure in Nutanix Enterprise AI.

### About this task

To add a third party credential for Microsoft Azure, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### Settings

### 2.  From the left navigation pane, select

.

#### Third Party Credentials

The

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

#### Microsoft Azure

.

### 6.  In the

#### API Key

field, enter the Azure API Key.

### 7.  In the

#### Hostname

field, enter the fully qualified domain name of the Azure OpenAI endpoint.

Use the format

. For example,

.

tenant-id .openai.azure.com

```bash
nai.openai.azure.com
```

### 8.  In the

#### Port

field, enter the port number.

### 9.  Click

#### Add Credential

.

The third party credential is displayed in the

#### Third Party Credentials

page.

### What to do next

Add an Azure Open AI provider. For more information, see

Adding an Azure Open AI Provider

on

page 259.

#### Adding a Cohere Provider

Add a Cohere provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Cohere.

For more information, see

Adding a Third Party Credential for Cohere

on page 261.

### About this task

To add a Cohere provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

The

#### Providers

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

#### Cohere

dropdown menu, select

.

### 6.  Do any of the following:

- 

#### Platform-Credential

From the

dropdown menu, select the credential you created for Cohere.

- 

Click

#### Add Credential

.

#### Add Credential

Adding a Third Party Credential

The

dialog box is displayed. For more information, see

for Cohere

on page  261.

### 7.  Click

#### Add Provider

.

The provider is displayed in the

#### Providers

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Cohere Add a credential for Cohere in Nutanix Enterprise AI (NAI).

### About this task

To add a third party credential for AWS, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

### 5.  From the

#### Platform

#### Cohere

dropdown menu, select

### 6.  In the

#### API Key

field, enter an API key.

### 7.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

### What to do next

Add a Cohere provider. For more information, see

Adding a Cohere Provider

on page 261.

#### Adding a GCP Vertex AI Provider

Add a GCP Vertex AI provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for GCP Vertex AI.

Adding a Third Party Credential for Google Cloud Platform

on page  263.

### About this task

To add a provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

The

#### Providers

page is displayed.

### 3.  Click

#### Add Provider

.

The

#### Add Provider

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

dropdown menu, select

#### GCP Vertex AI

.

### 6.  Do any of the following:

- 

From the

#### Platform Credential

dropdown menu, select the credential you created for From the

#### Platform

#### GCP Vertex

dropdown menu, select

..

- 

#### Add Credential

Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party Credential

for Google Cloud Platform

on page 263.

### 7.  Click

#### Add Provider

.

#### Providers

The provider is displayed in the

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Google Cloud Platform Add a credential for Google Cloud Platform (GCP) in Nutanix Enterprise AI to enable inference features.

### About this task

To add a credential for GCP, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed. You can view, update, or delete only the credentials that you

created.

3. 

Click

#### Add Credential

.

The

#### Add Credential

dialog box is displayed.

4. 

In the

#### Credential Name

field, enter the name for the credential.

5. 

From the

#### Platform

dropdown menu, select

#### Google Cloud Platform (GCP)

.

6. 

#### Project Name

In the

field, enter the project name.

#### Region

7. 

In the

field, enter a region.

#### Service Account JSON

8. 

In the

field, paste the JSON for the GCP service account.

9. 

In the

#### Hostname

field, enter the fully qualified domain name of the GCP Vertex AI endpoint.

Use the format

. For example,

region -aiplatform.googleapis.com

```bash
us-central1-
```

.

```bash
aiplatform.googleapis.com
```

### 10.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

### What to do next

Add a GCP Vertex AI provider. For more information, see

Adding a GCP Vertex AI Provider

on page  262.

#### Adding a Google Gemini Provider

Add a Google Gemini provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Google Gemini.

Adding a Third Party Credential for Google Gemini

For more information, see

on page 264.

### About this task

To add a Google Gemini provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AIn.

### 2.  From the left navigation pane, select

#### Providers

.

The

#### Providers

page is displayed.

### 3.  Click

#### Add Provider

.

The

#### Add Provider

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

dropdown menu, select

#### Google Gemini

.

### 6.  Do any of the following:

- 

From the

#### Platform Credential

dropdown menu, select the credential you created for Google Gemini.

- 

Click

#### Add Credential

.

#### Add Credential

Adding a Third Party Credential

The

dialog box is displayed. For more information, see

for Google Gemini

on page  264.

### 7.  Click

#### Add Provider

.

The provider is displayed in the

#### Providers

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Google Gemini Add a credential for Google Gemini in Nutanix Enterprise AI.

### Before you begin

### About this task

To add a credential for Google Gemini, follow these steps:

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

#### Google Gemini

.

### 6.  In the

#### API Key

field, enter an API key.

### 7.  In the

#### Hostname

field, enter a FQDN address.

### 8.  In the

#### Port

field, enter the port number.

### 9.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

### What to do next

Add a Google Gemini provider. For more information, see

Adding a Google Gemini Provider

on

page 263.

#### Adding a Mistral Provider

Add a Mistral provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Mistral.

Adding a Third Party Credential for Mistral

For more information, see

on page 265.

### About this task

To add a Google Gemini provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AIn.

#### Providers

### 2.  From the left navigation pane, select

.

The

#### Providers

page is displayed.

### 3.  Click

#### Add Provider

.

The

#### Add Provider

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

dropdown menu, select

#### Mistral

.

### 6.  Do any of the following:

- 

From the

#### Platform Credential

dropdown menu, select the credential you created for Mistral.

- 

#### Add Credential

Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party Credential

for Mistral

on page  265.

#### Add Provider

### 7.  Click

.

The provider is displayed in the

#### Providers

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Mistral Add a credential for Google Gemini in Nutanix Enterprise AI.

### About this task

To add a credential for Google Gemini, follow these steps:

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

#### Mistral

.

### 6.  In the

#### API Key

field, enter an API key.

### 7.  In the

#### Hostname

field, enter a FQDN address.

### 8.  In the

#### Port

field, enter the port number.

### 9.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

### What to do next

Add a Mistral provider. For more information, see

Adding a Mistral Provider

on page 265.

#### Adding a Remote NAI Provider

Add a remote NAI provider in Nutanix Enterprise AI.

### About this task

- 

Add a third party credential for remote NAI .

Adding a Third Party Credential for Remote NAI

For more information, see

on page  267.

### About this task

To add a remote NAI provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Providers

.

#### Providers

The

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

#### Remote NAI

dropdown menu, select

.

### 6.  Do any of the following:

- 

#### Platform-Credential

From the

dropdown menu, select the credential you created for remote NAI.

- 

#### Add Credential

Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party Credential

for Remote NAI

on page 267.

### 7.  Click

#### Add Provider

.

#### Providers

The provider is displayed in the

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Remote NAI Add a credential in Nutanix Enterprise AI to enable inference features.

### About this task

To add a credential, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

### 5.  From the

#### Platform

#### Remote NAI

dropdown menu, select

.

### 6.  In the

#### API Key

field, enter the key.

### 7.  In the

#### Hostname

field, enter the fully qualified domain name (FQDN) of the other NAI instance.

- 

If on port 443, FQDN must be CA signed

- 

Do not enter an IP address.

### 8.  In the

#### Port

field, enter the port number of the other NAI instance.

### 9.  Click

#### Add Credential

.

The third party credential is displayed in the

#### Third Party Credentials

page.

#### Adding a Open AI Provider

Add a Open AI provider in Nutanix Enterprise AI.

### Before you begin

- 

Add a third party credential for Open AI .

Adding a Third Party Credential for Open AI

For more information, see

on page 268.

.

### About this task

To add a Open AI provider, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AIn.

### 2.  From the left navigation pane, select

#### Providers

.

#### Providers

The

page is displayed.

### 3.  Click

#### Add Provider

.

#### Add Provider

The

dialog box is displayed.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Platform

#### Open AI

dropdown menu, select

.

### 6.  Do any of the following:

- 

#### Platform-Credential

From the

dropdown menu, select the credential you created for Open AI earlier.

- 

#### Add Credential

Click

.

The

#### Add Credential

dialog box is displayed. For more information, see

Adding a Third Party Credential

for Open AI

on page  268.

### 7.  Click

#### Add Provider

.

#### Providers

The provider is displayed in the

page.

### What to do next

Create a unified endpoint using the provider you created. For more information, see

Creating a Unified

Endpoint using Providers

on page 254.

Adding a Third Party Credential for Open AI Add a credential for Open AI in Nutanix Enterprise AI to enable inference features.

### About this task

To add a credential, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed. You can view, update, or delete only the credentials that you

created.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter the name for the credential.

### 5.  From the

#### Platform

#### Open AI

dropdown menu, select

### 6.  In the

#### API Key

field, enter the API Key.

### 7.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential is displayed in the

page.

#### Managing Rate Limit for Unified Endpoints

Manage rate limits for embedding or chat unified endpoints in Nutanix Enterprise AI.

### Before you begin

You must have at least one embedding or chat unified endpoint. For more information, see

Creating a

Unified Endpoint using Local Endpoints

on page  253. This feature is available as a tech preview feature

### About this task

To manage rate limit for unified endpoints, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Unified Endpoints

From the left navigation pane, select

.

The

#### Unified Endpoints

page displays all the endpoints that you created earlier.

3. 

Select either an embedding or chat unified endpoint.

4. 

#### Actions

#### Manage Rate Limit

Select

>

.

5. 

Select values for the following fields:

»

#### Measured Unit

»

#### Frequency

6. 

(Optional) Limit endpoint access across all clients.

a. Select the

#### Enable Global Rate Limit on Token Usage

checkbox.

b. In the

#### Rate Limit Value

field, enter the value for the selected measured unit and frequency.

7. 

#### Enable Per-Client Rate Limit on Token Usage

(Optional) To configure per-client rate limits, select the

checkbox.

Per-client limits are optional and can be configured either by API client key or by request header value. To switch between these configurations, delete the configuration you set initially and then configure the other.

8. 

To configure API client key-based limits, follow these steps:

a. In

#### Identify Clients By

#### API Client Key

#### Rate Limit Value

select

, and then enter a

for each API client key

that must be rate limited.

Enter rate limit must be for the selected measured unit and frequency. For example, enter rate limit for total tokens per minute.

9. 

To configure request header value-based limits, follow these steps:

a. In

#### Identify Clients By

select

#### Request Header Value

.

b. In

#### Header Name

, enter the request header used to identify clients.

You are not allowed to enter the following reserved header prefixes:

- 

X-Nutanix

- 

x-ai-eg

- 

Authorization

- 

x-vsr

Avoid using raw non ASCII characters in header matching criteria.

c. In

#### Default Rate Limit (per Header Value)

, enter the default limit for each unique header value.

d. (Optional) Select

#### Override Default Limit for Specific Headers

, and then add up to 50 header-value

overrides.

### 10.  Click

#### Save

.

### What to do next

(Optional) Verify the configured values in in the unified endpoint details page. The  Endpoint Access widget displays the details If overrides are configured, click  View Details  in the  Overrides  row.

#### Editing a Unified Endpoint

Edit a unified endpoint.

### Before you begin

Create a unified endpoint. For more information, see

Creating a Unified Endpoint using Local Endpoints

on

page 253.

### About this task

To edit inference endpoints, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Unified Endpoints

From the left navigation pane, select

.

The

#### Unified Endpoints

page displays all the endpoints that you created earlier.

3. 

Select a unified endpoint.

4. 

#### Actions

#### Edit

Select

>

.

The

#### Basics

tab of the

#### Create a Unified Endpoint

dialog box is displayed.

5. 

From the

#### API Spec

dropdown menu, select a value based on the required input and output.

6. 

In the

#### Models

section, click

#### Add Model

.

The

#### Add Model

dialog box is displayed.

7. 

Select a local endpoint running on the same cluster.

a. Select the

#### Type

as

#### Local

.

b. From the

#### Endpoint

dropdown menu, select an endpoint you created earlier.

The

#### Model Instance Name

field is auto populated.

c. Click

#### Add Model

.

#### Models

The model is displayed in the

section.

8. 
9. 

Select the

#### Type

as

#### Provider

.

10. (Optional) To add multiple models, repeat either step 7 or step 8.

### 11.  (Optional) To edit a model, click the

#### edit

icon.

#### delete

### 12.  (Optional) To delete a model, click the

icon.

#### Next

### 13.  Click

.

The

#### Configuration

tab is displayed.

»

#### Load Balance

»

#### Fallback Order

#### 14.  (Optional) To select the load balance strategy, follow these steps:

a. Select

#### Load Balance

.

The models are displayed.

b. In the

#### Load

column, enter the load for each model.

#### 15.  (Optional) To select the fallback order strategy, follow these steps:

a. Select

#### Fallback Order

.

### 16.  From the

#### API Client Keys

dropdown menu, select an API Key that you created for this unified endpoint.

### 17.  Click

#### Next

.

#### Summary

The

tab is displayed.

18. Review the information.

### 19.  Click

#### Update

.

#### Unified Endpoints

The system creates the unified endpoint, which is displayed on the

page.

#### Deleting a Unified Endpoint

Delete a unified endpoint.

### Before you begin

Create a unified endpoint. For more information, see

Creating a Unified Endpoint using Local Endpoints

on

page 253.

### About this task

To delete an inference endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Unified Endpoints

.

#### Unified Endpoints

The

page displays all the endpoints that you created earlier.

3. Select a unified endpoint.

### 4.  Select

#### Actions

>

#### Delete

.

### 5.  In the confirmation box, follow these steps:

a. Type

#### delete

.

b. Click

#### Delete

The unified endpoint is deleted.

Accessing a Unified Endpoint using Open AI Compatible Clients

Access a unified endpoint created in Nutanix Enterprise AI ( NAI) using any OpenAI compatible clients.

### Before you begin

Ensure that the following requirements are met before you access a unified endpoint created in NAI using any OpenAI compatible clients.

- 

An unified endpoint is created in NAI . For more information, see

Creating a Unified Endpoint using Local

Endpoints

on page  253.

- 

Creating an API

An API key is created in NAI  and attached it to the unified endpoints. For more information, see

Key

.

- 

The API key created in Nutanix Enterprise AI is made available for your use.

- 

The URL for unified endpoints has a gateway prefix. For example,

```bash
/enterpriseai/gateway/v1/chat/
completions
```

- 

(Optional) If per-client rate limiting is configured using

#### Request Header Value

, include the configured request

header in client requests.

### About this task

Nutanix Enterprise AI supports the following OpenAI compatible unified endpoints:

- 

/v1/images/generations

- 

/v1/chat/completions

- 

v1/embeddings

### Procedure

- 

For information on how to access a unified endpoint using an OpenAI-compatible client, see the client-specific documentation.
