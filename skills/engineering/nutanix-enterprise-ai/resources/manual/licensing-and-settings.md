# Nutanix Enterprise AI Manual: Licensing, Credentials, and System Settings

NAI licensing management, license key generation, GPU/CPU/Agent Gateway licenses, switching/upgrading licenses, third-party credentials (Hugging Face tokens, NVIDIA NGC keys), Nutanix Pulse telemetry, and remote Syslog server integration.

---

4. (Optional) Change your password if you are logging in for the first time.
a. In the

#### Type password

field, enter a new password.

b. In the

#### Retype password

field, enter a new password.

c. Click

#### Submit

.

After you successfully change the password, the system synchronizes the new password across the database.

Upon successful login, the system displays the

#### Dashboard

.

#### Nutanix Enterprise AI Licensing

Nutanix Enterprise AI is available with the Nutanix Enterprise AI Pro license.

deployment types

The Nutanix Enterprise AI  Pro license can be applied to all the

of  Nutanix Enterprise AI, and

includes all the features. For more information on the list of features, see

Nutanix Enterprise AI Overview

on

page 7.

Nutanix Enterprise AI includes a 2-day trial period. After the trial ends, you must purchase a valid license tier to continue using the service.

The licensing framework supports various deployment scenarios, including on-premises, hybrid, and cloud environments.

After your license expires, you have a 30-day grace period to renew or upgrade the license.

Requesting a

(Optional) Request a Proof of Concept License which is valid for 30 days. For more information, see

Proof of Concept License

.

### General Considerations

- 

The Nutanix Enterprise AI license consists of a vCPU and a GPU GB license. You must have either a vCPU license or GPU GB license to create an inference endpoint. A GPU GB license is optional and only required for GPU-based endpoints. Generate the licenses based on your specific requirements.

- 

During the trial period, if you add a valid license to your Nutanix Enterprise AI instance, the Nutanix Enterprise AI instance converts to a licensed version based on the resources defined in the applied license.

- 

You cannot create an inference endpoint if both licenses have expired. The system hibernates all endpoints after both licenses expire.

#### License Management

You can add, view, and upgrade licenses in Nutanix Enterprise AI (NAI).

You can perform the following tasks:

- 

After you install NAI for the first time, you can add any of the following licenses:

- 

GPU license: Add a GPU license to use features that are available for both CPU and GPU. For more information, see

Adding a GPU License

on page 172.

- 

CPU license: Add a CPU license to use features that are available only for CPU. For more information, see

Adding a CPU License

on page  173.

- 

Agent gateway license: Add an agent gateway license to meter gateway usage by access keys that are associated with unified endpoints and MCP connectors. For more information, see

Adding an Agent

Gateway License

on page 174.

- 

#### Licensing

Viewing the

After you add a license, you can view your license in the

tab. For more information, see

Applied License

on page  171.

- 

You can upgrade or switch your NAI license based on the license that is currently applied to your deployment. For more information, see

Upgrading or Switching Your License

on page 175.

Generating NAI License Keys Before using Nutanix Enterprise AI (NAI), you must first generate license keys.

### Before you begin

- 

Ensure that you have an active My Nutanix account with valid credentials.

- 

Your network must allow outbound traffic to portal.nutanix.com:443.

- 

You might need to turn off dialog box blockers in your browser.

- 

For more information on and considerations for NAI licenses, see

Nutanix Enterprise AI Licensing

on

page 170.

### Procedure

1. 

Nutanix Support Portal

Log on to the

.

2. 

menu

#### Licenses

In the upper-left corner, click the

icon, then click

.

3. 

#### Manage Licenses

#### Manage NAI/NKP License

Click

>

.

The  Manage Licenses with Keys  dialog box appears.

4. 

## NAI

#### Next

Select

and click

.

5. 

#### NAI Cluster Name

#### Next

Enter the name of the cluster in

, then click

.

#### +

In addition to the cluster name, you can enter the UUID of the cluster. To add more clusters, click

.

6. 

#### Apply to Cluster

In the pane of a license, click

.

The  Selecting NAI  dialog box appears.

7. 

#### Next

Select one or more clusters, then click

.

The clusters that appear in the list are the same clusters that you specified in Step 5.

8. 

Select the number of GB or vCPU to apply to each cluster, then click

#### Save

.

The  Selecting NAI  dialog box closes.

9. 

Repeat Step 5 through Step 8 for any other licenses, then click

#### Next

.

### 10.  Review the summary, then click

#### Next

.

To view the associated license number (such as LIC-08220516), click the

chevron down

icon next to each

licensed cluster in the summary.

After the page finishes loading, the

#### + Generate license keys button

appears.

### 11.  Click

#### + Generate license keys

.

### 12.  Click

#### Print all UUIDs and Keys

or

#### Download a CSV

.

### What to do next

Adding a GPU License

Adding a CPU

Add the license keys to NAI. For more information, see

on page  172 or

License

on page 173.

Viewing the Applied License View the applied license and current license consumption in Nutanix Enterprise AI (NAI).

### About this task

To view the applied license, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed.

### 3.  Click the

#### Licensing

tab.

#### Licensing

The

tab displays the following:

- 

The

#### Current License Consumption

section displays the following:

- 

#### Product Edition

: Displays the edition of the active license in NAI.

- 

#### Total CPUs used

: Displays the number of vCPU cores used by the inference endpoints.

- 

#### Total GPU memory used

: Displays the GPU memory used by the inference endpoints.

- 

#### Total access keys used

: Displays the count of distinct active access keys that are associated with

unified endpoints and MCP connectors.

- 

The

#### Applied Licenses

section displays the following:

- 

#### License Name

: Displays the name of the license.

- 

#### Tier

: Displays the license tier, which can be CPU, GPU, or agent gateway.

- 

#### Quantity and Meter

: Displays the quantity that is available in the license, based on meter type (vCPUs,

GPU memory, or access keys).

- 

#### Expiration Date

: Displays the date on which the license expires.

```bash
Adding a GPU License Add a GPU license in Nutanix Enterprise AI (NAI).
```

### Before you begin

- 

You can add a GPU license only when installing NAI for the first time. You cannot add a GPU license when upgrading NAI from a previous version.

- 

Generating NAI License Keys

Generate license keys for NAI . For more information, see

on page  171.

### About this task

To add a GPU license, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed.

### 3.  Click the

#### Licensing

tab.

#### Licensing

The

tab displays the applied license information and the current consumption.

### 4.  Click

#### Add License

.

The

#### Add License

dialog box is displayed.

### 5.  Select

#### GPU GB license

.

### 6.  In the

#### License Key for GPU GB

field, enter the license key.

#### 7.  (Optional) If you generated the license key using a different NAI cluster, follow these steps:

a. Select the

#### Keys generated using a different cluster ID

checkbox.

The

#### Kubernetes Cluster ID

field is displayed.

b. In the

#### Kubernetes Cluster ID

field, enter the cluster ID of the NAI cluster that you used to generate the

license key in the Nutanix Licensing Portal.

### 8.  Click

#### Add

.

#### Licensing

The license is displayed in the

tab.

### What to do next

- 

View the license. For more information, see

Viewing the Applied License

on page 171.

- 

(Optional) Upgrade or switch your NAI license. For more information, see

Upgrading or Switching Your

License

on page 175.

Adding a CPU License Add a CPU license in Nutanix Enterprise AI(NAI).

### Before you begin

- 

You can add a CPU license only if you are installing NAI for the first time and not if you are upgrading from a previous version.

- 

Generate NAI license keys. For more information, see

Generating NAI License Keys

on page  171.

### About this task

To add a CPU license, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed.

### 3.  Click the

#### Licensing

tab.

The

#### Licensing

tab displays the applied license information and the current consumption.

### 4.  Click

#### Add License

.

#### Add License

The

dialog box is displayed.

### 5.  Select

#### CPU License

.

### 6.  In the

#### License Key for CPUs

field, enter the license key.

#### 7.  (Optional) If you generated the license key using a different NAI cluster, follow these steps:

a. Select the

#### Keys generated using a different cluster ID

checkbox.

#### Kubernetes Cluster ID

The

field is displayed.

b. In the

#### Kubernetes Cluster ID

field, enter the cluster ID of the NAI cluster that you used to generate the

license key in the Nutanix Licensing Portal.

### 8.  Click

#### Add

.

The license is displayed in the

#### Licensing

tab.

### What to do next

- 

Viewing the Applied License

View the license. For more information, see

on page 171.

- 

(Optional) Upgrade or switch your NAI license. For more information, see

Upgrading or Switching Your

License

on page 175.

Adding an Agent Gateway License Add an agent gateway license to meter gateway usage in Nutanix Enterprise AI (NAI).

### Before you begin

- 

Generating NAI License Keys

Generate NAI license keys. For more information, see

on page  171.

- 

Hibernate all CPU or GPU inference endpoints that are active before applying an agent gateway license.

### About this task

To add an agent gateway license, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed.

### 3.  Click the

#### Licensing

tab.

#### Licensing

The

tab displays the applied license information and the current consumption.

### 4.  Click

#### Add License

.

The

#### Add License

dialog box is displayed.

### 5.  Select

#### Agent Gateway License

.

### 6.  In the

#### License Key

field, enter the license key.

#### 7.  (Optional) If you generated the license key using a different NAI cluster, follow these steps:

a. Select the

#### Keys generated using a different cluster ID

checkbox.

#### Kubernetes Cluster ID

The

field is displayed.

b. In the

#### Kubernetes Cluster ID

field, enter the cluster ID of the NAI cluster that you used to generate the

license key in the Nutanix Licensing Portal.

### 8.  Click

#### Add

.

#### Licensing

The license is displayed in the

tab. If an agent gateway license is already added, the existing agent

gateway license entry is updated with the new license information.

### What to do next

- 

Viewing the Applied License

View the license. For more information, see

on page 171.

- 

Upgrading or Switching Your

(Optional) Upgrade or switch your NAI license. For more information, see

License

on page 175.

Upgrading or Switching Your License Upgrade or switch the license that is applied to your Nutanix Enterprise AI (NAI) deployment.

### Before you begin

Generating NAI License Keys

Generate NAI license keys. For more information, see

on page 171.

You can upgrade or switch your NAI license based on the license that is currently applied to your deployment. The following table describes the supported license changes and the requirements that you must meet before applying a new license:

**Table 47: License Upgrade and Switching Options**

| Current License | Available License Change | Requirement |
| --- | --- | --- |
| CPU | Upgrade to a higher-capacity CPU license | None |
|  | Upgrade to a GPU license | None |
|  | Switch to an agent gateway license | Hibernate all CPU inference endpoints that are active before applying the license. |
| GPU | Switch to an agent gateway license | Hibernate all GPU inference endpoints that are active before applying the license. |
| Agent Gateway | Switch to a new agent gateway license | None |
|  | Upgrade to a CPU license | None |
|  | Upgrade to a GPU license | None |
| About this task |  |  |
| To upgrade or switch your license, follow these steps: |  |  |

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page is displayed.

### 3.  Click the

#### Licensing

tab.

#### Licensing

The

tab displays the applied license information and the current consumption.

### 4.  Click

#### Update License

.

The

#### Update License

dialog box is displayed.

5. Select the license to apply.

### 6.  In the

#### License Key

field, enter the license key.

#### 7.  (Optional) If you generated the license key using a different NAI cluster, follow these steps:

a. Select the

#### Keys generated using a different cluster ID

checkbox.

#### Kubernetes Cluster ID

The

field is displayed.

b. In the

#### Kubernetes Cluster ID

field, enter the cluster ID of the NAI cluster that you used to generate the

license key in the Nutanix Licensing Portal.

### 8.  Click

#### Update

.

The updated license information is displayed in the

#### Licensing

tab.

## ADDING A HUGGING FACE TOKEN

To download a large language model (LLM) from Hugging Face to Nutanix Enterprise AI, you must add a Hugging Face access token to Nutanix Enterprise AI.

### Before you begin

- 

Hugging Face documentation

Create a Hugging Face account. For more information, see

.

- 

Create an access token in your Hugging Face account. For more information, see  Generating a User Access Token  in

Hugging Face documentation

. If you create a fine-grained access token in Hugging Face, provide

read access permission to the repository of the model hosted in Hugging Face to ensure that the model can be imported to Nutanix Enterprise AI. For more information, see  User access tokens best practises  in

Hugging Face

documentation

.

### About this task

To add a new Hugging Face token to Nutanix Enterprise AI, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Settings

page opens.

3. Select Third Party Credentials.

### 4.  Click the

#### Add Credential

.

The

#### Add Credential

dialog box opens.

5. In the Credential Name field, enter a name for the credential.
6. From the Platform dropdown menu, select Hugging Face.

### 7.  In the

#### Token

field, enter the token key obtained from Hugging Face.

### 8.  Click

#### Add Credential

.

The Hugging Face token is added to Nutanix Enterprise AI.

#### Replacing a Hugging Face Token

You can add a new Hugging Face token to replace the existing token in Nutanix Enterprise AI.

### Before you begin

- 

Hugging Face documentation

Create a Hugging Face account. For more information, see

.

- 

Create an access token in your Hugging Face account. For more information, see  Generating a User Access Token in

Hugging Face documentation

.

If you create a fine-grained access token in Hugging Face, provide read access permission to the repository of the model hosted in Hugging Face to ensure that the model can be imported to Nutanix Enterprise AI. For more information, see  User access tokens best practices  in

Hugging Face documentation

.

### About this task

To replace the existing token in Nutanix Enterprise AI, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation bar, click

#### Settings

.

#### Third Party Credentials

The

page opens.

### 3.  From the

#### Hugging Face Model Hub Token

#### Update

section, click

.

The

#### Manage Hugging Face Model Hub Token

dialog box opens.

### 4.  In the

#### Token Name

field, enter the new token name.

### 5.  In the

#### Access Token

field, enter the new token obtained from Hugging Face.

### 6.  Click

#### Update

.

The new Hugging Face token is added to Nutanix Enterprise AI.

## ADDING AN NVIDIA NGC PERSONAL KEY

To download NVIDIA NIMs from NVIDIA NGC Catalog to Nutanix Enterprise AI, you must add an NVIDIA NGC Personal Key to Nutanix Enterprise AI.

### Before you begin

- 

NVIDIA

Ensure that you have an NGC account with an active subscription. For more information, see

Documentation Hub

.

- 

Generate an NVIDIA NGC Personal Key. For more information, see the  Personal API Key  section in the  NGC User Guide  listed in

NVIDIA Documentation Hub

.

After you generate an NVIDIA NGC Personal Key, add  NGC Catalog  services to the personal key to ensure that the NIM can be imported to Nutanix Enterprise AI. For more information, see the  Personal API Key  section in the NGC User Guide  listed in

NVIDIA Documentation Hub

.

### About this task

To add a new NGC Personal Key to Nutanix Enterprise AI, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

The

#### Settings

page opens.

3. Select Third Party Credentials.

### 4.  Click the

#### Add Credential

.

The

#### Add Credential

dialog box opens.

5. In the Credential Name field, enter a name for the credential.

#### 6.  In the Platform dropdown menu, select NVIDIA NGC

### 7.  In the

#### API Key

field, enter the API key obtained from NGC.

### 8.  Click

#### Add Credential

.

The personal key is added to Nutanix Enterprise AI.

#### Replacing an NVIDIA NGC Personal Key

You can add a new NVIDIA NGC Personal Key to replace the existing key in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you have an NGC account with an active subscription. For more information, see

NVIDIA

Documentation Hub

.

- 

Generate an NVIDIA NGC Personal Key. For more information, see the  Personal API Key  section in the  NGC User Guide  listed in

NVIDIA Documentation Hub

. After you generate an NVIDIA NGC Personal Key, add

NGC Catalog  services to the personal key to ensure that the NIM can be imported to Nutanix Enterprise AI. For more information, see the  Personal API Key  section in the  NGC User Guide  listed in

NVIDIA Documentation

Hub

.

### About this task

To replace the existing NVIDIA NGC Personal Key in Nutanix Enterprise AI, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation bar, click

#### Settings

.

#### Third Party Credentials

The

page opens.

### 3.  From the

#### NVIDIA NGC Personal Key

#### Update

section, click

.

The

#### Manage NVIDIA NGC Personal Key

dialog box opens.

### 4.  In the

#### Key Name

field, enter the new personal key name.

### 5.  In the

#### Key Value

field, enter the new personal key obtained from NGC.

### 6.  Click

#### Update

.

The new key is added to Nutanix Enterprise AI.

## SETTINGS

Settings in NAI allow you to perform the following actions:

- 

#### Language Settings

Change the language in the

tab.

- 

Adding a third party credentials to enable Nutanix AI users to securely connect and authenticate with external platforms by storing and managing API keys or credentials. .

- 

View third party credentials for Hugging Face, NVIDIA NGC,

- 

View third party credentials for external AI providers such as Anthropic, AWS Bedrock, Azure Open AI, Cohere, GCP Vertex AI, Google Gemini, Mistral, and Open AI

- 

Configure and view syslog server settings that allow you to forward logs from Nutanix Enterprise AI to an external syslog server

- 

Configure and view model and endpoint scanning for enhanced security.

Adding Credentials and Configuring Scan Settings

For more information, see

on page  273.

- 

Add and view licences.

- 

Add and view HTTP Proxy

- 

Add and view Syslog server.

#### Viewing Third Party Credentials

View third party credentials in Nutanix Enterprise AI.

### Before you begin

You must have atleast one third party credential. For more information on adding credentials, see

- 

Adding a Third Party Credential for Amazon Web Services

on page  258

- 

Adding a Third Party Credential for Anthropic

on page 257

- 

Adding a Third Party Credential for Google Cloud Platform

on page  263

- 

Adding a Third Party Credential for Hugging Face

on page 303

- 

Adding a Third Party Credential for Microsoft Azure

on page 260

- 

Adding a Third Party Credential for NVIDIA NGC

on page 303

- 

Adding a Third Party Credential for Open AI

on page  268

- 

Adding a Third-Party Credential for a Remote MCP Server

on page  291

- 

Adding a Third Party Credential for Remote NAI

on page  267

### About this task

To view third party credentials, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page displays the credentials

#### Adding a Third Party Credential for Hugging Face

Add a third party credential for Hugging Face in Nutanix Enterprise AI.

### About this task

To add a credential for Hugging Face, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter a name for the credential.

### 5.  From the

#### Platform

#### Hugging Face

dropdown menu, select

.

### 6.  In the

#### Token

field, enter a token obtained from Hugging Face.

### 7.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential for Hugging Face is displayed in the

page.

#### Adding a Third Party Credential for NVIDIA NGC

Add a third party credential for NVIDIA NGC in Nutanix Enterprise AI.

### About this task

To add a credential for NVIDIA NGC, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page is displayed.

### 3.  Click

#### Add Credential

.

#### Add Credential

The

dialog box is displayed.

### 4.  In the

#### Credential Name

field, enter a name for the credential.

### 5.  From the

#### Platform

## NVIDIA NGC

dropdown menu, select

.

### 6.  In the

#### API Key

field, enter the API Key generated from NVIDIA NGC.

### 7.  Click

#### Add Credential

.

#### Third Party Credentials

The third party credential for NVIDIA NGC is displayed in the

page.

#### Nutanix Enterprise AI Pulse Telemetry

Pulse telemetry collects data to improve and optimize Nutanix Enterprise AI.

When you enable Pulse, you share data about your Nutanix Enterprise AI application usage with Nutanix. The information collected helps Nutanix understand how you are using Nutanix Enterprise AI to help us improve the product. For more information on the data that Pulse gathers from Nutanix Enterprise AI , see

Nutanix Enterprise AI

Data Shared with Nutanix

on page  304.

Nutanix processes data that Pulse sends in a manner consistent with your agreement with Nutanix and, where applicable, by the Nutanix Privacy Statement. For more information, see

Nutanix Privacy Statement

.

You can review the Pulse settings in Nutanix Enterprise AI. For more information, see

Enabling Nutanix Enterprise

AI Pulse Telemetry

Disabling Nutanix Enterprise AI Pulse Telemetry

on page 306  or

on page 306.

#### Nutanix Enterprise AI Data Shared with Nutanix

This topic enlists the entities and the data shared from Nutanix Enterprise AI with Nutanix through Pulse telemetry.

Some of this information might be anonymized depending on your settings. This list is not exhaustive; for more details about the Pulse telemetry your clusters send to Nutanix, contact Nutanix Support.

**Table 59: Pulse Data**

| Property 1 | Entity | Data Collected |
| --- | --- | --- |
|  | Inference endpoint | Name of the inference endpoint |
|  |  | Total number of API requests sent to the inference endpoint |
|  |  | Total number of API keys assigned to the inference endpoint |
|  |  | Name of the imported large language models |
|  |  | Number of large language models used in inference endpoints |
|  |  | Number of GPUs and the type of GPU assigned to the inference endpoint |
|  |  | Details of the Kubernetes cluster that hosts Nutanix Enterprise AI |
|  |  | Details of the Kubernetes cluster on which Nutanix Enterprise AI is licensed |
|  |  | Current status of the endpoint |
|  |  | Details of the imported model used to create the endpoint |
|  | Kubernetes cluster | Kubernetes version name |
|  |  | Nutanix Enterprise AI version number |
|  |  | Total number of worker nodes present in the Kubernetes cluster |
|  |  | vCPUs assigned to the Kubernetes cluster |
|  |  | Disk size allotted to the Kubernetes cluster |
|  | Entity | Data Collected |
|  |  | Memory allotted to the Kubernetes cluster |
|  |  | Number of accelerators allotted to the Kubernetes cluster |
|  |  | Name of the accelerator allotted to the Kubernetes cluster |
|  |  | Actual memory available per accelerator |
|  |  | Details of the Kubernetes cluster that hosts Nutanix Enterprise AI |
|  |  | Details of the Kubernetes cluster on which Nutanix Enterprise AI is licensed |
|  |  | Details of the Nutanix Enterprise AI license key applied to the cluster |
|  |  | Names of the models imported to the Nutanix Enterprise AI instance |
|  | User | Total number of users |
|  |  | Total number of Hugging Face tokens, and NVIDIA NGC tokens created in the Nutanix Enterprise AI instance |
|  |  | Details of the Kubernetes cluster that hosts Nutanix Enterprise AI |
|  |  | Details of the Kubernetes cluster on which Nutanix Enterprise AI is licensed |
|  |  | Number of API keys available in the Nutanix Enterprise AI instance |
|  | Nutanix Enterprise AI features | Details of the Kubernetes cluster that hosts Nutanix Enterprise AI |
|  |  | Details of the Kubernetes cluster on which Nutanix Enterprise AI is licensed |
|  |  | Number of inference requests triggered for an endpoint |
|  |  | Number of API keys associated with an endpoint |
|  |  | Number of input and output tokens processed by an endpoint |
|  |  | Number of times an endpoint is created using a specific model type or an endpoint created using a specific model type is hibernated |
|  |  | Number of times a model is imported using a specific import method |
|  |  | Number of times a metric is queried |
|  |  | Number of times the access control settings were updated |
|  |  | Number of times an API for a resource is queried |
| Enabling Nutanix Enterprise AI Pulse Telemetry |  |  |

Pulse shares data about your Nutanix Enterprise AI application usage with Nutanix. This topic describes the steps to enable Nutanix Enterprise AI Pulse telemetry.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Ensure that your firewall allows the IP addresses of the Kubernetes cluster that hosts Nutanix Enterprise AI because Pulse data is sent from this cluster to insights.nutanix.com over port 443 using the HTTPS REST endpoint.

### About this task

To enable Nutanix Enterprise AI Pulse telemetry, follow these steps:

> [!NOTE]
> Note:   Nutanix does not collect any personally identifiable information (PII) through Pulse.

### Procedure

1. Log in to Nutanix Enterprise AI.

When you log in to Nutanix Enterprise AI for the first time after an installation or upgrade, the system checks whether Pulse is enabled. If it is not enabled, a screen appears recommending you to enable Pulse. To enable Pulse, select the

#### Enable Pulse

#### Save and Proceed

checkbox, and click

. To continue without enabling Pulse in

Nutanix Enterprise AI, clear the

#### Enable Pulse

checkbox and click

#### Save and Proceed

.

### 2.  From the left navigation bar, click

#### Settings

.

#### Third Party Credentials

The

page opens.

#### Pulse

### 3.  Click the

tab.

#### Enable Pulse

### 4.  Click

.

The system displays the

#### Enable Pulse

window.

### 5.  Click

#### Enable Pulse

.

The system enables Pulse and displays the message Pulse setting has been enabled successfully.

#### Disabling Nutanix Enterprise AI Pulse Telemetry

Pulse shares data about your Nutanix Enterprise AI application usage with Nutanix. This topic describes the steps to disable Nutanix Enterprise AI Pulse telemetry.

### Before you begin

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page  155.

### About this task

To disable Nutanix Enterprise AI Pulse telemetry, follow these steps:

Caution:   Nutanix recommends that you enable Pulse to allow Nutanix Support to receive cluster data and deliver proactive and context-aware support.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation bar, click

#### Settings

.

#### Third Party Credentials

The

page opens.

### 3.  Click the

#### Pulse

tab.

### 4.  Click

#### Disable Pulse

.

#### Disable Pulse

The system displays the

window.

### 5.  Click

#### Disable Pulse

.

The system enables Pulse and displays the message Pulse setting has been disabled successfully.

#### Syslog Server Tab

You can forward logs from Nutanix Enterprise AI to an external syslog server.

You can perform the following actions:

- 

Adding a Syslog Server

Add a syslog server to Nutanix Enterprise AI. For more information, see

on

page 307.

- 

Delete a syslog server from Nutanix Enterprise AI. For more information, see

Deleting a Syslog Server

on

page 308.

#### Adding a Syslog Server

Add a syslog server to Nutanix Enterprise AI.

### About this task

To add a syslog server, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Settings

.

The

#### Third Party Credentials

page opens.

3. 

Click the

#### Syslog Server

tab.

4. 

Click

#### Add Syslog Server

.

5. 

In the

#### Name

field, enter a name for the syslog server.

6. 

In the

#### IP Address

field, enter the IP address of the configured syslog server.

7. 

In the

#### Port

field, enter the port number of the configured syslog server.

8. 

From the

#### Transport Protocol

dropdown menu, select the protocol.

9. 

From the

#### Module Name

dropdown menu, select the module from which you need to forward the logs.

### 10.  Click

#### Add Server

.

The server is displayed in the

#### Syslog Server

tab.

#### Deleting a Syslog Server

Delete a syslog server from Nutanix Enterprise AI.

### About this task

To delete a syslog server, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

#### Third Party Credentials

The

page opens.

### 3.  Click the

#### Syslog Server

tab.

#### Delete Server

### 4.  Click

.

#### Delete Syslog Server

The

dialog box appears.

### 5.  In the confirmation box, click

#### Delete

.