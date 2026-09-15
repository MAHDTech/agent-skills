# Nutanix Enterprise AI Manual: Model and Endpoint Security Scanning

Inline model scanning architecture, scan settings and credentials, automated and manual scanning of models and endpoints, re-triggering scans for blocked models, security dashboard, and vulnerability remediation.

---

## SECURING AI MODELS AND ENDPOINTS

Configure model and endpoint security scanning in Nutanix Enterprise AI (NAI) and monitor security outcomes.

### Before you begin

- 

Review the following considerations:

- 

Integrating with Palo Alto Networks Prisma AIRS provides security scan results that help you identify potential risks and recommended actions. However, controls or enforcement capabilities available in Palo Alto Networks Prisma AIRS are not automatically applied in NAI based on these scan results. You must manually configure and apply the appropriate controls in NAI.

- 

Integrating with Palo Alto Networks Prisma AIRS, NAI sends data outside of the NAI environment. Your use of Palo Alto Networks Prisma AIRS is subject to Palo Alto Networks terms and conditions.

- 

Ensure that you meet the following requirements:

- 

Palo Alto

Configure identity and access in Palo Alto Networks Prisma AIRS. For more information, see

documentation

.

### About this task

To secure models and endpoints, follow these high-level steps:

### Procedure

1. Add Palo Alto Networks Prisma AIRS credentials and configure scan settings.

For detailed steps, see

Adding Credentials and Configuring Scan Settings

on page 273.

2. View model scan status and review scan details for models that are scanned automatically.

Viewing the Scan Status of Models

For more information, see

on page  275.

3. (Optional) Re-trigger security scans for validated Hugging Face models if required.

For more information, see

Re-triggering a Manual Security Scan for Blocked Models

on page  274.

4. Run and manage security scans for local endpoints.

Running a Manual Security Scan for Local Endpoints

Viewing

For more information, see

on page  276,

the Scan Status of Local Endpoints

on page 277, and

Aborting Security Scan for Local Endpoints

on

page 278.

5. Review the security dashboard to track model and endpoint risk.

Viewing the Security Dashboard

For more information, see

on page 279.

#### Adding Credentials and Configuring Scan Settings

Add Palo Alto Networks Prisma AIRS credentials in Nutanix Enterprise AI and configure scan settings.

### Before you begin

Configure identity and access in Palo Alto Networks Prisma AIRS . For more information, see

Palo Alto

Networks Prisma AIRS documentation

.

### About this task

To add Palo Alto Networks Prisma AIRS credentials and configure scan settings, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Settings

.

3. 

Click the

#### Security Monitoring

tab.

4. 

Click

#### Add Credentials and Configure Scan Settings

.

The

#### Add Credentials and Configure Scan Settings

dialog box is displayed.

5. 

In the

#### Tenant Service Group ID

field, enter the value that you generated in Palo Alto Networks Prisma

AIRS.

6. 

In the

#### Client ID

field, enter the value that you generated in Palo Alto Networks Prisma AIRS.

7. 

In the

#### Client Secret Key

field, enter the value that you generated in Palo Alto Networks Prisma AIRS.

8. 

In the

#### Security Group ID

field, enter the group ID that you generated in Palo Alto Networks Prisma AIRS.

9. 

(Optional) If your NAI instance is not internet-accessible or is not signed by a third-party certificate, create a network channel on your Kubernetes cluster and in the

#### Network Channel ID

field, enter the network channel

ID:

a. Create a network channel on your Kubernetes cluster.

For more information on generating a network channel, see

Creating a network channel

.

b. Copy the network channel ID that Palo Alto Networks Prisma AIRS generates.
c. In the

#### Network Channel ID

field, enter the network channel ID.

### 10.  Click

#### Add

.

#### Security Monitoring

The credentials and model scan configuration are displayed in the

tab. NAI

automatically runs a security scan for every Hugging Face model that you import from the Hugging Face model catalog or with a model URL.

### What to do next

- 

View the status of the scan for every Hugging Face model that you import. For more information, see

Viewing

the Scan Status of Models

on page  275.

- 

(Optional) If you update the Palo Alto Networks Prisma AIRS security policy when the security scan for a model is

#### Blocked

, you can manually re-trigger model scans. For more information, see

Re-triggering a Manual

Security Scan for Blocked Models

on page 274.

- 

Running a Manual Security

(Optional) Run a manual security scan for endpoints. For more information, see

Scan for Local Endpoints

on page  276.

#### Re-triggering a Manual Security Scan for Blocked Models

If you update the Palo Alto Networks Prisma AIRS security policy after the security status is  Blocked , you can manually re-trigger the security scan for blocked models in Nutanix Enterprise AI.

### Before you begin

Ensure that you meet the following requirements:

- 

You must have at least one validated Hugging Face model.

For more information, see

- 

Importing a Large Language Model from Hugging Face

on page 193

- 

Importing a Large Language Model from Hugging Face using Model URL or ID

on page 195

- 

Importing a Large Language Model Manually from Hugging Face

on page 196

- 

Add Palo Alto Networks Prisma AIRS  credentials and configure scan settings. For more information, see

Adding

Credentials and Configuring Scan Settings

on page 273.

### About this task

To re-trigger the scan for blocked models, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

The

#### Models

page is displayed.

3. Select a validated Hugging Face model model.

### 4.  From the

#### Actions

#### Run Security Scan

dropdown menu, click

.

The system prompts you to confirm the scan action on the selected model.

### 5.  Click

#### Run Scan

.

The status is displayed in the

#### Security Status

column in the

#### Models

page.

### What to do next

View the status of the scan for every Hugging Face model that you import. For more information, see

Viewing the Scan Status of Models

on page 275.

#### Viewing the Scan Status of Models

View the security scan status of models in Nutanix Enterprise AI or the Palo Alto Networks Prisma AIRS dashboard.

### Before you begin

#### The  Security Status  column is displayed only when the model is a validated Hugging Face model and security scan is configured. Therefore, ensure that you meet the following requirements:

- 

You must have at least one validated Hugging Face model.

For more information, see

- 

Importing a Large Language Model from Hugging Face

on page 193

- 

Importing a Large Language Model from Hugging Face using Model URL or ID

on page 195

- 

Importing a Large Language Model Manually from Hugging Face

on page 196

- 

Adding

Add Palo Alto Networks Prisma AIRS  credentials and configure scan settings. For more information, see

Credentials and Configuring Scan Settings

on page 273.

### About this task

To view the model scan status, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

#### Models

#### Security Status

The

page displays a summary of all the LLMs imported to Nutanix Enterprise AI. The

column displays the status of the security scan.

### 3.  Review the

#### Security Status

column for each model:

- 

#### Unavailable

: No scan record exists for the model because it has never been scanned. This status is

#### Unavailable

for NVIDIA NIM and custom models. You can fine-tune these models or create endpoints using

these models.

- 

#### Pending

: Scan is pending and waiting to be processed. You cannot fine-tune or create endpoints using

models with

#### Pending

status. Wait until the status changes to

#### Allowed

.

- 

#### Scanning

: Scan is submitted to Palo Alto Networks Prisma AIRS.

- 

#### Completed

#### Outcome

: Scan is completed successfully, and the

field is populated.

- 

#### Failed

: Scan fails due to an error or a timeout.

- 

#### Allowed

: No threats are detected in the model. The model is safe to deploy.

- 

#### Blocked

: Threats are detected in the model. You cannot fine-tune or create endpoints using models with

#### Blocked

status.

- 

#### Error

: Model security scan fails.

#### 4.  (Optional) To view the scan details in the Palo Alto Networks Prisma AIRS dashboard for any allowed or blocked

models, follow these steps:

a. In the

#### Security Status

#### Allowed

#### Blocked

column, hover over

or

.

The

#### View Scan Details

link is displayed.

b. Click

#### View Scan Details

.

The Palo Alto Networks Prisma AIRS login screen is displayed.

c. Log in to Palo Alto Networks Prisma AIRS.

The Palo Alto Networks Prisma AIRS dashboard displays the scan details.

#### Running a Manual Security Scan for Local Endpoints

Run a manual security scan for local endpoints in Nutanix Enterprise AI.

### Before you begin

Ensure that you meet the following requirements:

- 

Adding

Add Palo Alto Networks Prisma AIRS credentials and configure scan settings. For more information, see

Credentials and Configuring Scan Settings

on page 273.

- 

You can scan only local endpoints that have the status Active and the model capabilities Text To Text and Content-Safety. Therefore, you must have at least one local endpoint with the status Active and the model capabilities Text To Text and Content-Safety.

### About this task

To run a manual security scan for endpoints, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page opens, displaying a summary of all the endpoints that you created.

3. Select an endpoint.

### 4.  Click

#### Actions

>

#### Run Security Scan

.

#### Run Security Scan

The

dialog box is displayed.

### 5.  In the

#### Endpoint API Key

field, enter the API key for the endpoint.

### 6.  Click

#### Run Scan

.

The status is displayed in the

#### Security Status

column in the

#### Endpoints

page.

### What to do next

View the scan status of local endpoints. For more information, see

Viewing the Scan Status of Local

Endpoints

on page 277.

#### Viewing the Scan Status of Local Endpoints

View the security scan status of local endpoints in Nutanix Enterprise AI or the Palo Alto Networks Prisma AIRS dashboard.

### Before you begin

#### The  Security Status  column is displayed only when local endpoint security scan is configured. Therefore, ensure that you meet the following requirements:

- 

You must have at least one local endpoint.

- 

Adding

Add Palo Alto Networks Prisma AIRS  credentials and configure scan settings. For more information, see

Credentials and Configuring Scan Settings

on page 273.

### About this task

To view the endpoint scan status, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

#### List

#### Security Status

The

page opens, displaying a summary of all the endpoints that you created. The

column

displays the status of the security scan.

### 3.  Review the

#### Security Status

column for each local endpoint:

- 
- 

#### Unavailable

: No scan record exists for the endpoint or the endpoint was never scanned. The status is

#### Unavailable

. Run a manual security scan. For more information, see

Running a Manual Security Scan

for Local Endpoints

on page 276.

- 

#### Low

: Palo Alto Networks Prisma AIRS assigned a lower score. Rescan the endpoint if required. For more

information, see

Running a Manual Security Scan for Local Endpoints

on page 276.

- 

#### Pending

: Scan is pending and waiting to be processed.

- 

#### Scanning

: Scan is submitted to Palo Alto Networks Prisma AIRS.

- 

#### Completed

#### Outcome

: Scan is completed successfully, and the

field is populated.

- 

#### Failed

: Scan fails due to an error or a timeout.

- 

#### Allowed

: No threats are detected in the endpoint.

- 

#### Blocked

: Threats are detected in the endpoint. Review and remediate the endpoint before you use the

endpoint in production.

- 

#### Error

: Endpoint security scan fails.

#### 4.  (Optional) To view the scan details in the Palo Alto Networks Prisma AIRS dashboard for any allowed or blocked

local endpoints, follow these steps:

a. In the

#### Security Status

column, hover over

#### Allowed

or

#### Blocked

.

#### View Scan Details

The

link is displayed.

b. Click

#### View Scan Details

.

The Palo Alto Networks Prisma AIRS login screen is displayed.

c. Log in to Palo Alto Networks Prisma AIRS.

The Palo Alto Networks Prisma AIRS dashboard displays the scan details.

#### Aborting Security Scan for Local Endpoints

Abort a running security scan for a local endpoint in Nutanix Enterprise AI.

### Before you begin

Ensure that you meet the following requirements:

- 

Add Palo Alto Networks Prisma AIRS  credentials and configure scan settings. For more information, see

Adding

Credentials and Configuring Scan Settings

on page 273.

- 

You must have at least one local endpoint with security scan status

#### Pending

or

#### Scanning

.

### About this task

To abort a security scan for a local endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page opens, displaying a summary of all the endpoints that you created.

3. Select an endpoint.

### 4.  Click

#### Actions

>

#### Abort Security Scan

.

#### Abort Security Scan

The

dialog box is displayed.

#### Endpoint API Key

### 5.  In the

field, enter the API key for the endpoint.

### 6.  Click

#### Abort Scan

.

### 7.  In the

#### List

#### Security Status

page, review the endpoint scan status in the

column .

Viewing the Scan Status of Local Endpoints

For more information, see

on page 277.

#### Aborted

The status is displayed as

.

#### Viewing the Security Dashboard

View the  Security  dashboard in Nutanix Enterprise AI.

### Before you begin

Ensure that you meet the following requirements:

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Adding

Add Palo Alto Networks Prisma AIRS  credentials and configure scan settings. For more information, see

Credentials and Configuring Scan Settings

on page 273.

- 

You must have at least one model to view model security summary.

- 

You must have at least one endpoint to view the red teaming security summary.

### About this task

To view the

#### Security

dashboard, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Dashboard

.

### 3.  Select the

#### Security

tab.

The following widgets are displayed:

- 

#### Model Security Summary

: This widget displays the total number of models and the scan status of models.

- 

#### Red Teaming Security Summary

: This widget displays the total number of endpoints and the scan status

of endpoints.

- 

#### Blocked Models

: This widget displays models with blocked status. You can review and remediate those

models.

- 

#### High Risk Endpoints

: This widget displays endpoints with high or critical risk. You can take corrective

actions for these endpoints.

4. Review these widgets to prioritize remediation and re-scan actions for models and endpoints.

#### Editing Credentials and Scan Settings

Edit Palo Alto Networks Prisma AIRS credentials and scan settings in Nutanix Enterprise AI.

### About this task

To edit Palo Alto Networks Prisma AIRS credentials and scan settings, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Settings

.

3. 

Click the

#### Security Monitoring

tab.

4. 

Click

#### Edit Configuration

.

5. 

In the

#### Client Secret Key

field, enter the value that you generated in Palo Alto Networks Prisma AIRS.

6. 

In the

#### Tenant Service Group ID

field, enter the value that you generated in Palo Alto Networks Prisma

AIRS.

7. 

In the

#### Client ID

field, enter the value from Palo Alto Networks Prisma AIRS.

8. 

In the

#### Security Group ID

field, enter the group ID that you generated in Palo Alto Networks Prisma AIRS.

9. 

(Optional) If your deployment is not internet-accessible or does not use a third-party signed certificate, in the

#### Network Channel ID

field, update the network channel ID:.

a. Create a network channel on your Kubernetes cluster.

Creating a network channel

For more information on generating a network channel, see

.

b. Copy the network channel ID that Palo Alto Networks Prisma AIRS generates.
c. In the

#### Network Channel ID

field, enter the network channel ID.

### 10.  Click

#### Save

.

#### Security

The credentials with masked client secret key and model scan configuration are displayed in the

#### Monitoring

tab. NAI automatically runs a security scan for every Hugging Face model that you import.

### What to do next

- 

Viewing

View the status of the scan for every Hugging Face model that you import. For more information, see

the Scan Status of Models

on page  275.

- 

(Optional) If you made changes to the Palo Alto Networks Prisma AIRS security policy because the security scan for a model is

#### Blocked

Re-triggering a Manual

, you can manually re-trigger scans. For more information, see

Security Scan for Blocked Models

on page 274.

- 

(Optional) Run a manual security scan for endpoints. For more information, see

Running a Manual Security

Scan for Local Endpoints

on page  276.

#### Deleting Credentials and Scan Settings

Delete Palo Alto Networks Prisma AIRS credentials and scan settings in Nutanix Enterprise AI.

### About this task

To delete Palo Alto Networks Prisma AIRS credentials and scan settings, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Settings

.

### 3.  Click the

#### Security Monitoring

tab.

### 4.  Click

#### Delete Configuration

.

### 5.  In the confirmation box, follow these steps:

a. Type delete.
b. Click

#### Delete

.

#### Run Security Scan

#### Models

#### List

The credentials and scan settings are deleted. The

option in the

>

page is

disabled. All existing scan status and results are retained.