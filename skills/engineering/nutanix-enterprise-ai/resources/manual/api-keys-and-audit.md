# Nutanix Enterprise AI Manual: API Client Keys, Audit Events, and Infrastructure Health

API client key lifecycle (creation, activation, deactivation, expiration), audit event logging and event filtering, cluster-level and node-level infrastructure metrics, and Kubernetes cluster health monitoring.

---

## API CLIENT KEYS

API keys are required to access the endpoints to integrate with the application.

Perform the following actions to manage an API key:

- 

Create an API key

- 

Update an API key

- 

Delete an API key

- 

Activate or deactivate an API key

#### Viewing API Keys in Nutanix Enterprise AI

The  API Keys  page displays all the API keys that you created in Nutanix Enterprise AI.

### About this task

To view the API keys, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### API Client Keys

.

#### API Client Keys

The

page opens, displaying a summary of all the API keys that you created. For information on

the attributes of the API keys displayed in the page, see

API Keys Attributes

on page 282.

### 3.  To view only the endpoint keys, click the

#### Endpoint Keys

tab.

### 4.  To view only the MCP Client keys, click the

#### MCP Client Keys

tab.

#### API Keys Attributes

The following table describes the API key attributes that appear on the

#### API Keys

page.

**Table 53: API Keys Attributes - Description**

| Property 1 | Field | Description | Values |
| --- | --- | --- | --- |
|  | Key Name | Displays the user-provided name when creating the key. | Key name |
|  | Key Value | Displays the API key created by the user. The key is masked. | Key |
|  | Endpoints | Displays the name of the endpoint associated with the API key. | Endpoint name |
|  | Created By | Displays the name of the user who created the key. | User name |
|  | Status | Displays the current status of the API key. | Active, Inactive |
| Creating an API Key |  |  |  |

Create an API key in Nutanix Enterprise AI to allow an application to access an inference endpoint.

### About this task

To create an API key, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### API Client Keys

### 2.  From the left navigation pane, select

.

#### API Client Keys

The

page opens, displaying a summary of all the API keys that you created. For information on

the attributes of the API keys displayed in the page, see

API Keys Attributes

on page 282.

### 3.  Click

#### Create a New Key

.

#### Create API Key

The

dialog box opens.

### 4.  In the

#### Key Name

field, enter a name for the API key.

Nutanix recommends that you provide a name that is meaningful and identifiable to you.

### 5.  To assign the key to an endpoint, from the

#### Unified Endpoints

or  the

#### Local Endpoints

dropdown menu,

select the endpoint.

### 6.  Click

#### Create

.

#### API Key Details

The system creates the API key and opens the

dialog box that displays the details of the API

key.

### 7.  Click

#### Copy Key

.

8. Store the key securely.

You cannot view the key again after you close the

#### API Key Details

dialog box. If you lose the key, you must

generate a new one.

#### Activating or Deactivating an API Key

Activate or deactivate an API key that you created in Nutanix Enterprise AI.

### About this task

By default, an API key is activated when you create it.

To activate or deactivate an API key, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### API Client Keys

### 2.  From the left navigation pane, select

.

#### API Client Keys

The

page opens, displaying a summary of all the API keys that you created. For information on

the attributes of the API keys displayed in the page, see

API Keys Attributes

on page 282.

### 3.  Perform one of the following:

»

#### Actions

To activate an inactive API key, select the checkbox associated with the key, and from the

dropdown

menu, click

#### Activate

.

»

To deactivate an active API key, select the checkbox associated with the key, and from the

#### Actions

dropdown

#### Deactivate

menu, click

.

The system prompts you to confirm the action.

### 4.  Click

#### Confirm

.

The system displays the new status of the key.

#### Updating an API Key

Update an existing API key that you created in Nutanix Enterprise AI.

### About this task

To update an API key, follow these steps:

You can perform the following API key configuration:

- 

Assign the same API key to multiple endpoints.

- 

Assign the same API key to multiple unified endpoints.

- 

Remove an endpoint to which the API key is assigned.

- 

Remove a unified endpoint to which the API key is assigned.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### API Client Keys

.

#### API Client Keys

The

page opens, displaying a summary of all the API keys that you created. For information on

the attributes of the API keys displayed in the page, see

API Keys Attributes

on page 282.

#### 3.  Select the checkbox associated with a key, and from the

#### Actions

dropdown menu, click

#### Update

.

#### Update API Key

The

dialog box opens.

### 4.  Perform one of the following actions:

»

#### Endpoints

To assign the API key to an endpoint, from the

dropdown menu, select the checkbox associated

with the endpoint.

»

#### Unified Endpoints

To assign the API key to a unified endpoint, from the

dropdown menu, select the

checkbox associated with the unified endpoint.

»

To unassign the API key from an endpoint, from the

#### Endpoints

dropdown menu, clear the checkbox

associated with the endpoint.

»

To unassign the API key from a unified endpoint, from the

#### Unified Endpoints

dropdown menu, clear the

checkbox associated with the unified endpoint.

You cannot change the key name or the key.

### 5.  Click

#### Update

.

#### Deleting an API Key

Delete an existing API key that you created in Nutanix Enterprise AI.

### About this task

To delete an API key, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### API Client Keys

### 2.  From the left navigation pane, select

.

#### API Client Keys

The

page opens, displaying a summary of all the API keys that you created. For information on

the attributes of the API keys displayed in the page, see

API Keys Attributes

on page 282.

#### 3.  Select the checkbox associated with an API key, and from the

#### Actions

dropdown menu, click

#### Delete

.

The system prompts you to confirm the delete action.

### 4.  In the field provided, type

#### delete

and click

#### Delete API Key

.

#### API Keys

The API key is deleted from Nutanix Enterprise AI and is no longer displayed on the

page.

#### VIEWING AUDIT EVENTS IN NUTANIX ENTERPRISE AI

You can view a list of all the event messages audited and captured as part of the audit logs on the  Audit Events  page of Nutanix Enterprise AI. By default, no retention period exists for the event messages.

### About this task

Event messages describe actions such as importing a model, resetting the password, a user logging in and logging out, updating or deleting an endpoint, or creating an endpoint, API key, or user. Unlike alerts, event messages are simply informational without the need to acknowledge or resolve.

To view the event messages, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Audit Events

.

The

#### Audit Events

page opens, displaying a list of all the event messages that are audited and captured as part of

Audit Events Summary

the audit logs. For information on the fields that appear on the page, see

on page 286.

#### Audit Events Summary

View the list of event messages in Nutanix Enterprise AI.

#### Audit Events

The following table describes the fields that appear on the

page.

**Table 54: Audit Events Summary - Field Description**

| Field | Description | Property 3 | Values |
| --- | --- | --- | --- |
| Description | Displays the name of the event. |  | Event name |
| Action | Displays the type of operation that took place. The possible operation types depend on the entity type. |  | Create, Delete, Log In, Log Out, Reset Password, Update, Expire, Pulse Upload Event |
| Entity Type | Displays the entity type, such as an endpoint, cluster, model, and so on. |  | API Key, Cluster Config, Endpoint, Hugging Face Token, Model, NVIDIA NGC API Key, License, User |
| User | Displays the name of the user who performed the event. |  | User name |
| Time | Displays the date and time when the event occurred. |  | Date and time |
| Audit Events Filters |  |  |  |
|  |  | Audit Events |  |

The following table describes the filters that are available on the

page. For information on how to

filter the list of event messages, see

Filtering Audit Events

on page  287.

**Table 55: Audit Events Filters - Field Description**

| Property 1 | Filter | Description | Property 4 | Property 5 | Values |
| --- | --- | --- | --- | --- | --- |
|  | Entity Type | Select the checkboxes of one or more entities to filter for actions on those entity types. |  |  | API Key, Cluster Config, Endpoint, Hugging Face Token, Model, NVIDIA NGC API Key, License, Support Telemetry, User |
|  | Action | Select the checkboxes of one or more actions to filter for those actions on an entity. |  |  | Create, Delete, Log In, Log Out, Reset Password, Update, Expire, Pulse Upload Event |
|  | User | Enter a user name and press Enter to filter for actions requested by that user. |  |  | User name |
|  |  | Click | + Add Rule | to create a new |  |
|  |  | rule. |  |  |  |
| Filtering Audit Events |  |  |  |  |  |
|  | Filter the list of event messages in Nutanix Enterprise AI. |  |  |  |  |
|  | About this task |  |  |  |  |
|  | To filter the event messages, follow these steps: |  |  |  |  |

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Audit Events

.

The

#### Audit Events

page opens, displaying a list of all the event messages that are audited and captured as part of

the audit logs.

### 3.  Click

#### Modify Filters

.

The

#### Filters

pane opens, displaying the available filters. For information on the available filters, see

**Table 55:**

| Audit Events Filters - Field Description | on page  287. |
| --- | --- |

4. To view the event messages based on a filter, select the checkbox associated with the filter.

You can apply multiple filters together. For example, if you want to filter for endpoint creation, you can select the

#### Endpoint

checkbox in

## ENTITY TYPE

and then select the

#### Create

checkbox in

#### Action

.

#### 5.  To export the filtered event messages, follow these steps:

a. Click

#### Export

.

b. Select

#### All entities

or

#### Only Filtered Entities

.

c. Click

#### Export

.

## VIEWING INFRASTRUCTURE USAGE METRICS

View the infrastructure usage metrics of the Kubernetes cluster that hosts Nutanix Enterprise AI and of the individual nodes in the cluster.

### About this task

To view the infrastructure usage metrics, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### 2.  From the left navigation pane, do one of the following:

- 

#### Dashboard

Select

.

The system displays the dashboard with the widgets.

From the

#### Infrastructure Summary

widget, click

#### View Details

.

- 

#### Infrastructure

#### Usage

Select

>

.

The system displays the

#### Usage

tab in the

#### Infrastructure

page.

Cluster Usage Metrics

For information on the cluster usage metrics, see

on page 288 and for node usage

statistics, see

Node Usage Metrics

on page 289.

#### Cluster Usage Metrics

Reference for the cluster usage and health metrics on the  Infrastructure  page when you select  Usage and then  Clusters .

#### Infrastructure

#### Usage

The following table describes the metrics that appear in the

page when you select the

tab and

then select

#### Clusters

from the drop-down menu on the left.

- 

The graphs display No Data when there is no data for a specific interval.

- 

The graph is displayed for a specified

you select from the dropdown menu on the right of the widget.

interval

#### Last 15 minutes

#### Last 1 hour

#### Last 24 hours

The

involves

,

, and

.

interval

**Table 56: Cluster Usage and Health - Metrics Description**

| Property 1 | Metrics | Description | Property 4 | Property 5 | Property 6 |
| --- | --- | --- | --- | --- | --- |
|  | Memory Usage | Displays the graph of memory capacity currently being used by the Kubernetes cluster that hosts Nutanix Enterprise AI. |  |  |  |
|  | CPU Usage | Displays the graph of CPU capacity currently being used by the Kubernetes cluster that Nutanix Enterprise AI. |  |  |  |
|  | Metrics | Description |  |  |  |
|  | Nodes | Displays a summary of all the nodes in the cluster that hosts Nutanix Enterprise AI, along with their names, number of GPUs, CPU capacity, and memory capacity currently in use. |  |  |  |
| Node Usage Metrics |  |  |  |  |  |
|  | The following table describes the metrics that appear in the | Infrastructure | page when you select the | Usage | tab and |
|  | then select a node from the drop-down menu on the left. |  |  |  |  |

- 

The graphs display No Data when there is no data for a specific interval.

- 

The graph is displayed for a specified

you select from the dropdown menu on the right of the widget.

interval

The

involves

#### Last 15 minutes

,

#### Last 1 hour

, and

#### Last 24 hours

.

interval

**Table 57: Infrastructure Usage and Health - Node Metrics Description**

| Property 1 | Widget | Description |
| --- | --- | --- |
|  | Memory Usage | Displays the graph of memory capacity used by the node. |
|  | CPU Usage | Displays the graph of CPU capacity used by the node. |
|  | GPU Utilization | Displays the graph of GPU capacity used by the node. |
|  | GPU Memory Usage | Displays the percentage of GPU memory capacity used by the node. |
| Viewing Kubernetes Cluster Resources and Health Metrics |  |  |

View the resources available in the Kubernetes cluster that hosts Nutanix Enterprise AI.

### About this task

To view the Kubernetes cluster resources, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Infrastructure

.

The

#### Overview

tab in the

#### Infrastructure

page opens, displaying a summary of the resources available in the

Kubernetes Cluster Summary

Kubernetes cluster. For information on the fields that appear on the page, see

on

page 289.

For information on the health and infrastructure usage statistics of the cluster and the nodes, see

Viewing

Infrastructure Usage Metrics

on page  288.

#### Kubernetes Cluster Summary

#### Kubernetes Cluster

The following table describes the fields that appear on the

page.

**Table 58: Kubernetes Cluster Summary - Field Description**

| Field | Description |
| --- | --- |
| Name |  |

- 

Displays the name of the node in the cluster

- 

Displays the health of the node.

- 

Green dot indicates node is online

- 

Grey dot indicates node is offline.

Version

Displays the Kubernetes version running on the node pool.

vCPU

Displays the number of vCPUs assigned to the node pool.

Memory

Displays the memory available to the node.

Disk

Displays the storage space available to the node.

Accelerators

Displays the name and number of GPUs assigned to this node when you hover over the hover info icon.

GPU Memory

Displays the total GPU memory available on the node.

The number of GPUs displayed might be incorrect if the GPU operator pods in your Kubernetes cluster are unhealthy. To fix these unhealthy pods, contact NVIDIA support.