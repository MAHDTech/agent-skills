+++
title = "local-endpoints"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Local Endpoints and Inference Management

Local endpoint lifecycle, attributes and performance widgets, deploying endpoints with validated models, non-validated Hugging Face models, non-catalog NVIDIA NIMs, air-gapped NIM deployments, experimental endpoints and runtime parameters, hibernation and resumption, and OpenAI-compatible API invocation.

---

#### LOCAL ENDPOINTS IN NUTANIX ENTERPRISE AI

Nutanix Enterprise AI allows you to create AI inference endpoints.

After you successfully create an endpoint, you can deploy and use a large language model (LLM) in real-time applications. AI inference endpoints allow an application to send input data to the LLM and receive predictions or responses. The endpoint handles requests in real time, making it possible to use the LLM for tasks like generating text, answering questions, or making predictions. You can share an endpoint with developers to incorporate into their AI applications, such as chatbots or document summarizations.

> [!NOTE]
> Important:   By default, Nutanix Enterprise AI does not limit or modify the maximum number of inference requests that an endpoint can handle simultaneously.

Perform the following actions to manage a local endpoint:

- 

Create an endpoint for a validated model. For more information, see

Creating a Local Endpoint using a

Validated Model

on page 221.

- 

Creating a Local Endpoint using a

Create an endpoint for a non-validated model. For more information, see

non-validated Hugging Face Model

on page 226.

- 

Editing a Local Endpoint

Edit an endpoint. For more information, see

on page  241.

- 

Delete an endpoint

Delete an endpoint. For more information, see

.

- 

Test an endpoint

Test an endpoint. For more information, see

.

- 

Hibernate an endpoint. For more information, see

Hibernating an Endpoint

on page  243.

- 

Resume an endpoint. For more information, see

Resuming an Endpoint

on page  243.

- 

Managing

Manage runtime parameters for a hibernated experimental endpoint. For more information, see

Runtime Parameters for a Hibernated Experimental Endpoint

on page 240.

- 

View the logs of an endpoint. For more information, see

Viewing the Logs of an Endpoint

on page  220.

- 

Downloading the Logs of an Endpoint

Download the logs of an endpoint. For more information, see

on

page 221.

#### Viewing Local Endpoints in Nutanix Enterprise AI

The  Endpoints  page displays all the endpoints that you created in Nutanix Enterprise AI.

### About this task

To view the endpoints, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The

#### List

page opens, displaying a summary of all the endpoints. For information on the endpoints attributes

Endpoints Attributes

displayed in the page, see

on page 215.

#### 3.  (Optional) To filter endpoints, follow these steps:

a. Click

#### Modify Filters

.

The

#### Filters

pane opens.

b. Select one of the following filters:

»

Hibernated

»

Non-Hibernated

#### Endpoints Attributes

#### List

The following table describes the endpoint attributes that appear on the

page.

**Table 50: Endpoints Attributes - Description**

| Attributes | Description | Values |
| --- | --- | --- |
| Endpoint Name | Displays the user-provided name when creating the endpoint. | Endpoint name |
| Model Instance Name | Displays the model name you specified when you imported the model. | Model name |
| Capabilities | Displays the capabilities of the deployed model. | Embedding, Reranker, Tool Calling, Text To Text |
| Accelerator | Displays the accelerator type and count. |  |

- 

Displays the number and name of the GPUs per instance if the accelerator is GPU.

- 

Displays INTEL AMX vCPU if the accelerator is an AMX enabled intel CPU.

- 

Displays CPU if the processor does not support AMX.

Number of Instances

Displays the number of Kubernetes pods configured to serve the endpoint.

Number of Instances

Created By

Displays the name of the user who created the endpoint.

User name

Status

Displays the current status of the endpoint.

Active, Failed, Hibernated, Pending, Processing

Security Status

Displays the status of the security scan for endpoints.

- 

You have configured security scan.

- 

Scans

#### Viewing the Local Endpoint Details

View detailed information about endpoints.

### About this task

To view the detailed information about an endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### Local Endpoints

### 2.  From the left navigation pane, select

.

#### List

The

page displays a summary of all the endpoints that you created.

3. Click an endpoint.

The details page opens, displaying detailed information about the endpoint in widgets.

### 4.  To view the sample code, click

#### Access Endpoint

.

Viewing Sample API Code

For more information, see

on page  220.

### 5.  Click

#### Metrics

#### Usage

>

.

**Table 52: Metrics tab Widgets - Description**

| For information on the widgets displayed in the page, see | on |
| --- | --- |
| page 219. |  |

### 6.  To view the logs of an endpoint, click

#### Logs

.

Viewing the Logs of an Endpoint

For more information, see

on page 220.

#### Widgets in the Overview tab of an Endpoint Details Page

The following table describes the widgets that appear on the endpoint details page.

**Table 51: Widgets in the Overview tab**

| Widget Name | Information Provided |
| --- | --- |
| Details | Provides the following: |

- 

#### Endpoint Name

: Displays the user-provided name

of the endpoint.

- 

#### Description

: Displays the user-provided description

of the endpoint.

- 

#### Status

: Displays the current status of the endpoint.

- 

#### Model Instance Name

: Displays the user-provided

name of the large language model (LLM) on which the endpoint is deployed.

- 

#### Model Type

: Displays the type of imported LLM

model on which the endpoint is deployed.

- 

#### Inference Engine

: Displays the type of inference

engine running the LLM model.

- 

#### Accelerator

: Displays the name of the GPU card

used to run the model.

Endpoint Access

Provides the following:

- 

#### Endpoint URL

: Displays the URL of the endpoint.

- 

#### API Keys

: Displays the API key required to access

the endpoint. The key is masked.

Hover over the tooltip to go to the

#### API Keys

page.

You can create new API keys and manage existing keys from this page. For more information, see

Viewing API Keys in Nutanix Enterprise AI

on

page 282.

- 

#### View Sample Request

: Click this option to

view the sample code that contains the URL and API key required to access the endpoint. For more information, see

Viewing Sample API Code

on

page 220.

#### Widget Name

#### Information Provided

Number of Compute Instances

Displays the:

- 

Maximum number of instances configured to serve the endpoint.

- 

Number of instances actively running to serve the endpoint.

- 

Node allocation mode for each endpoint instance (single-node or multi-node), when configured.

- 

Total accelerators assigned across all nodes for an instance, when accelerator details are available.

- 

Number of vCPUs and the memory usage per instance actively in use to serve the endpoint. When KV cache offloading is enabled, the displayed memory usage includes the configured offloading memory.

Requests Summary

Displays the total number of API requests received by the endpoint.

Token Usage

Displays the number of tokens received and generated by this endpoint.

### Widgets in the Metrics tab

#### Metrics

The following table describes the widgets that appear in the

tab on the endpoint details page. You can select

### Usage

or

#### Performance

from the drop-down menu to view the required metrics

- 

Metrics are available on the dashboard based on the capability of the Inference Engine.

- 

Metrics may display the following state:

- 

Not applicable when a model using this endpoint does not support a metric.

- 

No Data is available when there is no data for a specific interval.

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

**Table 52: Metrics tab Widgets - Description**

| Option | Widget Name | Information Provided |
| --- | --- | --- |
| Usage metrics | Requests Trend |  |

- 

Displays the total number of API requests received by the endpoint. The API requests for all API keys are displayed by default. You can filter the metrics for an API key using the drop-down menu.

- 

> [!NOTE]
> Note:   The trends chart shows the end time of the last closing window instead of the current time.

Token Usage

Displays the number of tokens processed and generated by this endpoint.

> [!NOTE]
> Note:   The trends chart shows the end time of the last closing window instead of the current time.

Cached Token Usage

Displays the graph of the number of cached tokens. Cached Token Usage is displayed only if the selected endpoint's inference engine is vLLM.

Performance metrics

Latency

Displays the graph of the time taken by an endpoint to respond to an API request.

Number of Requests waiting at the Inference Engine

Displays the graph of the requests that have reached the endpoint but are in queue.

Number of Requests running at the Inference Engine

Displays the graph of requests that are currently being processed by that endpoint.

Time to First Token (TTFT)

Displays the graph of the time interval between when a request reaches the inference endpoint and when the first generated token is streamed back to the client.

Time per Output Token (TPOT)

Displays the graph of the average time that the endpoint takes to generate each output token after sending the first one

Output Tokens per Second

Displays the graph of the token throughput of the inference endpoint.

#### Viewing Sample API Code

The inference endpoints in Nutanix Enterprise AI can be accessed using a URL and an API Key. Share the sample code with the developers to access and integrate endpoints into their applications.

### About this task

To view the sample code that contains the URL and API key required to access the endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### Local Endpoints

### 2.  From the left navigation pane, select

.

#### List

The

page displays a summary of all the endpoints that you created.

3. Click an endpoint.

The details page opens, displaying detailed information about the endpoint in widgets.

### 4.  To view the sample code, click

#### Access Endpoint

.

The

#### Access Endpoint

dialog box displays the sample code. The sample code contains the URL required to

access the endpoint.

#### 5.  (Optional) To perform tasks that require the endpoint to interact with an external tool, select the

#### Include Tool

#### Calling

checkbox.

For example, enable tool calling to check the current weather at a location.

### 6.  (Optional) To copy the sample code, click

#### Copy Code

.

You can share the code with the developers to access the endpoints and integrate them into their applications.

#### Viewing the Logs of an Endpoint

View the logs of an endpoint. Logs provide insight into the interactions between the client and the inference endpoint and help debug failures.

### About this task

To view the logs of an endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The

#### List

page displays a summary of all the endpoints that you created.

3. Click an endpoint.

The details page opens, displaying detailed information about the endpoint in widgets.

### 4.  Click the

#### Logs

tab.

### 5.  From the

#### Instance

dropdown menu, select an instance.

### 6.  Click

View Logs.

The logs of the selected pod or instance are displayed.

### 7.  (Optional) To view the latest logs, click

#### Refresh Logs

.

The latest logs are displayed. A maximum of 500 lines is displayed.

### What to do next

(Optional) To download the logs, see

Downloading the Logs of an Endpoint

on page 221.

#### Downloading the Logs of an Endpoint

Download the logs of an endpoint. Logs provide insight into the interactions between the client and the inference endpoint and help debug failures.

### About this task

To download the logs of an endpoint, follow these steps:

### Procedure

### 1.  Complete the steps in

Viewing the Logs of an Endpoint

on page 220.

### 2.  From the

#### View Logs

#### Download Logs

dropdown menu, select

.

### 3.  Click the

#### Download

icon.

Logs are saved in the Kubernetes pod as per the Kubernetes log rotation policy. The logs available as per the Kubernetes log rotation policy are downloaded. You can download a maximum of 10 MB of log lines.

#### Creating a Local Endpoint using a Validated Model

Create an inference endpoint in Nutanix Enterprise AI and share it with developers to incorporate into their AI applications. The inference endpoint accepts requests and sends back responses.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Import at least one validated model to Nutanix Enterprise AI and ensure that the import status is

#### Active

. For more

information, see:

- 

Importing a Large Language Model from Hugging Face

on page 193

- 

Importing NVIDIA NIMs from NVIDIA NGC Catalog

on page  199

- 

Importing a Large Language Model Manually from Hugging Face

on page 196

### About this task

To create an endpoint from an LLM, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page displays a summary of all the endpoints that you created.

3. 

#### Create Endpoint

Click

.

The

#### Basics

tab of the

#### Create an Endpoint

dialog box is displayed.

4. 

#### Endpoint Name

In the

field, enter a name for the endpoint.

Nutanix recommends that you provide a name that is meaningful and identifiable. Ensure that the name meets the following complexity requirements:

- 

Contains only lowercase letters, numbers, or special characters (-).

- 

Start with a lowercase letter.

5. 

#### Description

(Optional) In the

field, enter a description for the endpoint.

6. 

#### Purpose

#### Real Time

From the

dropdown menu, select

.

7. 

#### Model Capabilities

In the

field, enter the capabilities required for the endpoint.

#### Model Instance Name

The

field is filtered to display models that support the specified capabilities.

8. 

#### Model Instance Name

From the

dropdown menu, select an active imported validated model.

9. 

Select the acceleration type. Do any of the following:

- 

Enable GPU acceleration. Complete the following steps:

#### 1.  From the

#### Acceleration Type

#### GPU Passthrough

dropdown menu, select

.

The

#### GPU Passthrough

option is displayed only if the cluster has GPU nodes and the model you

Step 7

selected in

supports GPU execution.

#### 2.  From the

#### GPU Card

dropdown menu, select the required GPU card.

For information on the GPU cards supported for a model, see GPU Requirements in

Nutanix Enterprise

AI Requirements

.

- 

Enable CPU acceleration. Complete the following steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

## CPU

.

## CPU

Step 7

The

option is displayed only if the model you selected in

supports CPU execution.

2. (Optional) If you have Intel Xeon 4th-gen or newer CPUs, select

#### Optimise endpoint to run on

#### Intel® AMX enabled CPUs

.

The built-in accelerator available in Intel® Xeon® 4th-gen (or newer) CPUs is enabled.

The default configuration of this checkbox is as follows:

- 

This checkbox is selected by default and grayed out when all CPUs in the cluster support Intel® Advanced Matrix Extensions (AMX).

- 

This checkbox is grayed out by default when none of the CPUs support AMX.

- 

If your Kubernetes cluster is configured with Multi-Instance GPU (MIG), select MIG. To select MIG, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

## NVIDIA MIG

.

#### Accelerator Details

#### MIG GPU Profile

#### 2.  From the

dropdown menu, select

.

Only Mixed Strategy MIGs are supported in NAI. A single cluster node can be configured for either MIG or GPU Passthrough. A single node cannot utilize both configurations simultaneously. An NAI cluster fully supports a mixed environment. You can seamlessly deploy a combination of MIG-enabled nodes and GPU Passthrough nodes within the same cluster.

- 

If your Kubernetes cluster is configured with vGPU, select vGPU. To select vGPU, follow these steps:

#### 1.  From the

#### Acceleration Type

#### vGPU

dropdown menu, select

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select the required vGPU.

#### 10.  Specify the API key to access endpoints. Do one of the following:

»

#### Create a New API Key

If you did not create an API key to access the endpoint, click

.

For more information, see

Creating an API Key

on page  283.

»

If you already created an API key, select an active API key from the

#### API Keys

dropdown menu.

### 11.  Click

#### Next

.

#### Configuration

The

tab is displayed.

### 12.  From the

#### Inference Engine

dropdown menu, select the type of inference engine required to deploy the

endpoint.

### 13.  Expand

#### Show Advanced Engine Configuration

.

### 14.  In

#### Configure Engine Arguments

#### Only Platform-Provided

, select

.

### 15.  From the

#### Engine Source

dropdown menu, any of the images provided with NAI.

### 16.  Select the

#### KV Cache Aware Routing

checkbox.

#### KV Cache Aware Routing

The

checkbox is displayed only if did all of the following

- 

Selected the

#### Inference Engine

in

#### vLLM

- 

This is available as a tech preview feature only.

- 

Only for the following model capabilities Text2Text, Reasoning, Content Safety, Tool Calling

> [!NOTE]
> Note:   A local endpoint with KV cache-aware routing enabled does not use KV cache-aware routing when configured as a local endpoint within a unified endpoint.

#### 17.  (Optional) To enable speculative token generation, follow these steps:

a. Select the

#### Speculative Token Generation

checkbox.

#### Speculative Token Generation

The

checkbox is displayed only if you did the following:

- 

Selected the

#### Engine Source

as

## NAI

- 

Selected the

#### Inference Engine

in

#### vLLM

- 

#### Acceleration Type

## GPU

Enabled the

as

.

.

b. In the

#### Speculation Length

field, enter the number of tokens

c. In the

#### Maximum Prompt Lookup

field, enter the number of tokens

d. In the

#### Speculation Length

field, enter the number of tokens

### 18.  In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

#### 19.  (Optional) To enable single node allocation, follow these steps::

a. Select

#### Node Allocation

#### Single Node

as

.

b. In the

#### Number of Accelerators (Per Instance)

field, enter the number of accelerators required for each

instance.

The minimum value is 1. The available values depend on the selected accelerator and cluster inventory.

#### 20.  (Optional) To enable multi node, follow these steps:

Multi-Node is enabled only if if you met the following conditions:

- 

Multi-node is a tech preview feature.

- 

Select the model capabilty as image-to-text, text-to-text, content-safety, tool-calling, or reasoning

- 

#### Inference Engine

#### vLLM

select

as

- 

#### Acceleration Type

#### GPU Passthrough

select

as

.

- 

Disable speculative decoding and KV Cache offloading.

- 

Sufficient free accelerators are available on each node.

a. Select

#### Node Allocation

#### Multi-Node

as

.

b. elect

#### Nodes Per Instance

to define how many nodes serve one endpoint instance. This option is shown

only when the selected model, runtime, and cluster resources support multi-node allocation.

c. In the

#### Number of Accelerators (Per Node)

field, enter the number of accelerators required for each

node.

#### 21.  (Optional) To enable KV Cache Offloading, follow these steps:

KV cache offloading defines the tier to offload the KV Cache. The current default and supported tier is CPU. You can enable KV cache offloading only if you met the following conditions:

- 

KV cache offloading is a tech preview feature.

- 

#### Node Allocation

#### Multi-Node

You cannot enable KV cache offloading if you select

as

allocation.

- 

#### Inference Engine

#### vLLM

You can enable KV cache offloading only when you select

as

- 

You can enable KV cache offloading only when you select

#### Acceleration Type

as

#### GPU Passthrough

.

a. Enable

#### KV Cache Offloading

.

b. In the

#### Memory Per Accelerator (GiB)

, enter the memory to store the KV Cache per accelerator. The

default memory is 1.2x the accelerator value. Enter a memory that is greater than 1.2x the accelerator value.

### 22.  (Optional) Enable

#### Speculative Decoding

if it is available for your selected configuration.

#### Speculative Decoding

#### Multi-Node

cannot be used with

allocation.

### 23.  (Optional) Select the

#### Use Custom Configuration for Instances

checkbox to manually configure the

inference engine type, number of vCPUs, and memory.

The checkbox is selected in case of a non-validated model. In other cases, this checkbox remains clear because the system automatically configures the inference engine type, number of vCPUs, and memory according to the model you select in

Step 7

and your cluster configuration, and displays the values in the respective fields.

### 24.  (Optional) In the

#### vCPUs (Per Instance)

field, enter the number of Kubernetes node pool vCPUs to assign to

the endpoint.

Ensure that you assign the vCPUs according to your cluster configuration. If you assign a value that exceeds the number of vCPUs configured in the cluster, the system displays an error.

### 25.  (Optional) In the

#### Memory (Per Instance)

field, enter the amount of memory to assign to the endpoint. This

field is auto-populated based on the selected LLM and displays the recommended memory as per your cluster configuration.

Ensure that you assign the memory according to your cluster configuration. If you assign a value that exceeds the memory configured in the cluster, the system displays an error.

### 26.  In the

#### Context Length

field, enter the maximum number of tokens that can be processed based on the selected

pre-validated Hugging Face model, the CPU or GPU card, and the number of GPUs.

- 

This limit includes both input and output tokens. You can also manually configure this field based on your available system resources, including the Hugging Face model, the CPU or GPU card, and the number of GPUs.

- 

The context length determines the combined length of the input text and the generated output.

- 

Step 7

#### Inference Engine

#### vLLM

This field appears only if you select a Hugging Face model in

and the

is

.

- 

The system displays a default value only for pre-validated models.

- 

The maximum context length for a custom model with GPUs enabled is 1048576, and without GPUs, it is 4096. If you enter a higher value, the system displays an error.

- 

To override the recommended value, select

#### Customize Context Length

. If you clear this option, the

system resets to the recommended context length.

### 27.  (Optional) Click

#### Back

.

The

#### Basics

tab is displayed. The values you specify in the

#### Number of Accelerators (Per Instance)

,

#### vCPUs (Per Instance)

#### Memory (Per Instance)

#### Context Length

,

, and the

field are not saved. Therefore,

complete the following steps:

a. Click

#### Next

.

b. Update the

#### Number of Accelerators (Per Instance)

field.

c. Update the

#### vCPUs (Per Instance)

field.

d. Update the

#### Memory (Per Instance)

field.

e. Update the

#### Context Length

field.

### 28.  Click

#### Next

.

#### Summary

The

tab is displayed.

29. Review the information.

#### 30.  (Optional) To review the data you specified for

#### Advanced Configuration

, click the

#### View Details

link.

### 31.  Click

#### Create

.

#### List

The system creates the inference endpoint, which is displayed on the

page.

### What to do next

View the status of the endpoint on the

#### Endpoints

page. The system displays one of the following states for the

operation:

- 

#### Active

: The endpoint is created and ready to use. An inference endpoint accepts requests and sends back

#### Active

responses only if the status of the endpoint displays

.

- 

#### Failed

: Endpoint creation failed due to unauthorized NVIDIA NGC Personal Key, or  NGC Catalog  services

access restriction for the personal key. If the system displays an error message other than the ones listed, run the following command, and contact Nutanix Support with the endpoint specifications provided in the output.

```bash
$ kubectl describe isvc  endpoint_name  -n nai-admin
```

#### List

Replace

with the name of the endpoint on the

page.

endpoint_name

- 

#### Pending

: The system is waiting for the resources required to create the endpoint.

For example, if the required number of GPUs are not available, Nutanix Enterprise AI maintains a

#### Pending

status until the GPUs becomes available.

- 

#### Processing

: The system is creating the endpoint.

After you successfully create an endpoint, you can test it to ensure that it is configured correctly. For more information, see

Testing a Local Endpoint

on page  242.

Creating a Local Endpoint using a non-validated Hugging Face Model

Create an inference endpoint in Nutanix Enterprise AI and share it with developers to incorporate into their AI applications. The inference endpoint accepts requests and sends back responses.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

#### Active

Import at least one non-validated model to Nutanix Enterprise AI and ensure that the import status is

. For

more information, see

Importing a Large Language Model from Hugging Face using Model URL or ID

on

page 195.

### About this task

To create an endpoint from an LLM, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Local Endpoints

From the left navigation pane, select

.

The

#### List

page displays a summary of all the endpoints that you created.

3. 

Click

#### Create Endpoint

.

#### Basics

#### Create an Endpoint

The

tab of the

dialog box is displayed.

4. 

#### Endpoint Name

In the

field, enter a name for the endpoint.

Nutanix recommends that you provide a name that is meaningful and identifiable. Ensure that the name meets the following complexity requirements:

- 

Contains only lowercase letters, numbers, or special characters (-).

- 

Start with a lowercase letter.

5. 

#### Description

(Optional) In the

field, enter a description for the endpoint.

6. 

#### Purpose

#### Real Time

From the

dropdown menu, select

.

7. 

#### Model Capabilities

In the

field, enter the capabilities required for the endpoint.

#### Model Instance Name

The

field is filtered to display models that support the specified capabilities.

8. 

#### Model Instance Name

From the

dropdown menu, select an active imported non-validated model.

9. 

Select the acceleration type and configure the accelerator. Do any of the following:

- 

Enable GPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

#### GPU Passthrough

dropdown menu, select

.

The

#### GPU Passthrough

option is displayed only when the cluster has GPU nodes and the model you

Step 7

selected in

supports GPU execution.

#### 2.  From the

#### GPU Card

dropdown menu, select the required GPU card.

For information on the GPU cards supported for a model, see GPU Requirements in

Nutanix Enterprise

AI Requirements

.

- 

Enable CPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

## CPU

.

## CPU

Step 7

The

option is displayed only if the model you selected in

supports CPU execution.

2. (Optional) If you have Intel Xeon 4th-gen or newer CPUs, select

#### Optimise endpoint to run on

#### Intel® AMX enabled CPUs

.

The built-in accelerator available in Intel® Xeon® 4th-gen (or newer) CPUs is enabled.

The default configuration of this checkbox is as follows:

- 

This checkbox is selected by default and grayed out when all CPUs in the cluster support Intel® Advanced Matrix Extensions (AMX).

- 

This checkbox is grayed out by default when none of the CPUs support AMX.

- 

If your Kubernetes cluster is configured with Multi-Instance GPU (MIG), enable MIG. To enable MIG, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

## NVIDIA MIG

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select

#### MIG GPU Profile

.

Only Mixed Strategy MIGs are supported. A single cluster node can be configured for either MIG or GPU Passthrough. A single node cannot utilize both configurations simultaneously. An NAI cluster fully supports a mixed environment. You can seamlessly deploy a combination of MIG-enabled nodes and GPU Passthrough nodes within the same cluster.

- 

If your Kubernetes cluster is configured with GPU, enable vGPU. To enable GPU, follow these steps:

#### 1.  From the

#### Acceleration Type

#### vGPU

dropdown menu, select

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select the required vGPU.

#### 10.  Specify the API key to access endpoints. Do one of the following:

»

#### Create a New API Key

If you did not create an API key to access the endpoint, click

.

For more information, see

Creating an API Key

on page  283.

»

#### API Keys

If you already created an API key, select an active API key from the

dropdown menu.

#### Next

### 11.  Click

.

#### Configuration

The

tab is displayed.

### 12.  From the

#### Inference Engine

dropdown menu, select the type of inference engine required to deploy the

endpoint.

### 13.  Expand

#### Show Advanced Engine Configuration

.

### 14.  In

#### Configure Engine Parameters

#### Only Platform-Provided

#### Custom Arguments and

, select

or

#### Environment Variables

.

### 15.  To add a CLI argument, select

#### Custom Arguments and Environment Variables

.

### 16.  In the

#### Arguments

field, ensure that you follow the following formats:

- 

Enter each argument on a new line in the

format.

```bash
--key value
```

- 

If the argument contains multiple words, ensure that the argument is wrapped in double quotes in the

```bash
--key
```

format.

```bash
"val1 val2"
```

### 17.  In the

#### Environment Variables

field, enter each environment variable on a new line in the

format.

```bash
key=value
```

### 18.  To review the parsed key-value pairs,

#### 1.  Click

#### View Parsed Parameters

.

#### 2.  Review the parsed key-value pairs

The arguments and environment variables that are parsed correctly are displayed.

3. Correct the format of the arguments and environment variables that are not displayed.
4. To review the arguments and environment variables ,click

#### View Parsed Parameters

.

5. Ensure that all the arguments and environment variables that you entered are displayed.

### 19.  Click

#### Update

.

The number of parameters you added are displayed.

### 20.  From the

#### Engine Source

dropdown menu, select a source.

You can select any of the following:

- 

Select any of the images provided with NAI.

- 

#### Import from community vLLM registry

: Select this to add images maintained by the vLLM

community. After you select this option the container registry URL is displayed based on whether you selected CPU or GPU in step 8.

- 

## GPU

If you selected

in step 8, a Docker URL is displayed. You cannot edit this URL. You can add

images from the Docker Hub registry.

In the

#### Engine Tag

field, enter the tag which you can copy from the Docker Hub registry.

- 

## CPU

If you selected

in step 8, an AWS URL is displayed. You cannot edit this URL. You can add

images from the AWS registry.

In the

#### Engine Tag

field, enter the tag which you can copy from the AWS registry.

- 

#### Import from other registry

: Select this option to add your own images. In the

#### Engine Image URL

field, paste the equivalent docker image URL of the tag which you can copy from your image registry.

### 21.  Select the

#### KV Cache Aware Routing

checkbox.

The

#### KV Cache Aware Routing

checkbox is displayed only if did all of the following

- 

#### Inference Engine

#### vLLM

Selected the

in

- 

This is available as a tech preview feature only.

- 

Only for the following model capabilities Text2Text, Reasoning, Content Safety, Tool Calling

> [!NOTE]
> Note:   A local endpoint with KV cache-aware routing enabled does not use KV cache-aware routing when configured as a local endpoint within a unified endpoint.

### 22.  In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

#### 23.  (Optional) To enable single node allocation, follow these steps::

a. Select

#### Node Allocation

as

#### Single Node

.

b. In the

#### Number of Accelerators (Per Instance)

field, enter the number of accelerators required for each

instance.

The minimum value is 1. The available values depend on the selected accelerator and cluster inventory.

#### 24.  (Optional) To enable multi node, follow these steps:

Multi-Node is enabled only if you met all of the following conditions:

- 

Multi-node is a tech preview feature.

- 

Select the model capability as image-to-text, text-to-text, content-safety, tool-calling, or reasoning

- 

#### Inference Engine

#### vLLM

select

as

- 

select

#### Acceleration Type

as

#### GPU Passthrough

.

- 

Disable speculative decoding and KV Cache offloading.

a. Select

#### Node Allocation

as

#### Multi-Node

.

b. elect

#### Nodes Per Instance

to define how many nodes serve one endpoint instance. This option is shown

only when the selected model, runtime, and cluster resources support multi-node allocation.

c. In the

#### Number of Accelerators (Per Node)

field, enter the number of accelerators required for each

node.

#### 25.  (Optional) To enable KV Cache Offloading, follow these steps:

KV cache offloading defines the tier to offload the KV Cache. The current default and supported tier is CPU. You can enable KV cache offloading only if you met the following conditions:

- 

KV cache offloading is a tech preview feature.

- 

#### Node Allocation

#### Multi-Node

You cannot enable KV cache offloading if you select

as

allocation.

- 

You can enable KV cache offloading only when you select

#### Inference Engine

as

#### vLLM

- 

You can enable KV cache offloading only when you select

#### Acceleration Type

as

#### GPU Passthrough

.

a. Enable

#### KV Cache Offloading

.

b. In the

#### Memory Per Accelerator (GiB)

, enter the memory to store the KV Cache per accelerator. The

default memory is 1.2x the accelerator value. Enter a memory that is greater than 1.2x the accelerator value.

### 26.  (Optional) Enable

#### Speculative Decoding

if it is available for your selected configuration.

#### Speculative Decoding

cannot be used with

#### Multi-Node

allocation.

### 27.  (Optional) Select the

#### Use Custom Configuration for Instances

checkbox to manually configure the

inference engine type, number of vCPUs, and memory.

The checkbox is selected in case of a non-validated model. In other cases, this checkbox remains clear because the system automatically configures the inference engine type, number of vCPUs, and memory according to the model you select and your cluster configuration, and displays the values in the respective fields.

### 28.  (Optional) In the

#### vCPUs (Per Instance)

field, enter the number of Kubernetes node pool vCPUs to assign to

the endpoint.

Ensure that you assign the vCPUs according to your cluster configuration. If you assign a value that exceeds the number of vCPUs configured in the cluster, the system displays an error.

### 29.  (Optional) In the

#### Memory (Per Instance)

field, enter the amount of memory to assign to the endpoint. This

field is auto-populated based on the selected LLM and displays the recommended memory as per your cluster configuration.

Ensure that you assign the memory according to your cluster configuration. If you assign a value that exceeds the memory configured in the cluster, the system displays an error.

### 30.  (Optional) Click

#### Back

.

#### Basics

#### Number of Accelerators (Per Instance)

The

tab is displayed. The values you specify in the

,

#### vCPUs (Per Instance)

,

#### Memory (Per Instance)

, and the

#### Context Length

field are not saved. Therefore,

complete the following steps:

a. Click

#### Next

.

b. Update the

#### Number of Accelerators (Per Instance)

field.

c. Update the

#### vCPUs (Per Instance)

field.

d. Update the

#### Memory (Per Instance)

field.

e. Update the

#### Context Length

field.

### 31.  Click

#### Next

.

#### Summary

The

tab is displays the data.

32. Review the information.

#### 33.  (Optional) To review the data you specified for

#### Advanced Configuration

, click the

#### View Details

link.

### 34.  Click

#### Create

.

#### List

The system creates the inference endpoint, which is displayed on the

page.

### What to do next

View the status of the endpoint on the

#### Endpoints

page. The system displays one of the following states for the

operation:

- 

#### Active

: The endpoint is created and ready to use. An inference endpoint accepts requests and sends back

responses only if the status of the endpoint displays

#### Active

.

- 

#### Failed

: Endpoint creation failed due to unauthorized NVIDIA NGC Personal Key, or  NGC Catalog  services

access restriction for the personal key. If the system displays an error message other than the ones listed, run the following command, and contact Nutanix Support with the endpoint specifications provided in the output.

```bash
$ kubectl describe isvc  endpoint_name  -n nai-admin
```

#### List

Replace

with the name of the endpoint on the

page.

endpoint_name

- 

#### Pending

: The system is waiting for the resources required to create the endpoint.

For example, if the required number of GPUs are not available, Nutanix Enterprise AI maintains a

#### Pending

status until the GPUs becomes available.

- 

#### Processing

: The system is creating the endpoint.

After you successfully create an endpoint, you can test it to ensure that it is configured correctly. For more information, see

Testing a Local Endpoint

on page  242.

- 

#### Creating a Local Endpoint using a non-catalog NVIDIA NIM

Create an inference endpoint in Nutanix Enterprise AI and share it with developers to incorporate into their AI applications. The inference endpoint accepts requests and sends back responses.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Import at least one non-catalog NVIDIA NIM to Nutanix Enterprise AI and ensure that the import status is

#### Active

. For more information, see

Importing NVIDIA NIMs using Model URL or ID

on page 200

.

### About this task

To create an endpoint, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Local Endpoints

From the left navigation pane, select

.

The

#### List

page displays a summary of all the endpoints that you created.

3. 

Click

#### Create Endpoint

.

#### Basics

#### Create an Endpoint

The

tab of the

dialog box is displayed.

4. 

#### Endpoint Name

In the

field, enter a name for the endpoint.

Nutanix recommends that you provide a name that is meaningful and identifiable. Ensure that the name meets the following complexity requirements:

- 

Contains only lowercase letters, numbers, or special characters (-).

- 

Start with a lowercase letter.

5. 

#### Description

(Optional) In the

field, enter a description for the endpoint.

6. 

#### Purpose

#### Real Time

From the

dropdown menu, select

.

7. 

#### Model Capabilities

In the

field, enter the capabilities required for the endpoint.

#### Model Instance Name

The

field is filtered to display models that support the specified capabilities.

8. 

#### Model Instance Name

From the

dropdown menu, select an active imported non-validated model.

9. 

Select the acceleration type and configure the accelerator. Do any of the following:

- 

Enable GPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

#### GPU Passthrough

dropdown menu, select

.

The

#### GPU Passthrough

option is displayed only when the cluster has GPU nodes and the model you

selected supports GPU execution.

#### 2.  From the

#### GPU Card

dropdown menu, select the required GPU card.

Nutanix Enterprise

For information on the GPU cards supported for a model, see GPU Requirements in

AI Requirements

.

- 

Enable CPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

## CPU

dropdown menu, select

.

The

## CPU

option is displayed only if the model you selected supports CPU execution.

2. (Optional) If you have Intel Xeon 4th-gen or newer CPUs, select

#### Optimise endpoint to run on

#### Intel® AMX enabled CPUs

.

The built-in accelerator available in Intel® Xeon® 4th-gen (or newer) CPUs is enabled.

The default configuration of this checkbox is as follows:

- 

This checkbox is selected by default and grayed out when all CPUs in the cluster support Intel® Advanced Matrix Extensions (AMX).

- 

This checkbox is grayed out by default when none of the CPUs support AMX.

- 

If your Kubernetes cluster is configured with Multi-Instance GPU (MIG), enable MIG. To enable MIG, follow these steps:

#### 1.  From the

#### Acceleration Type

## NVIDIA MIG

dropdown menu, select

.

#### 2.  From the

#### Accelerator Details

#### MIG GPU Profile

dropdown menu, select

.

Only Mixed Strategy MIGs are supported. A single cluster node can be configured for either MIG or GPU Passthrough. A single node cannot utilize both configurations simultaneously. An NAI cluster fully supports a mixed environment. You can seamlessly deploy a combination of MIG-enabled nodes and GPU Passthrough nodes within the same cluster.

- 

If your Kubernetes cluster is configured with GPU, enable vGPU. To enable GPU, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

#### vGPU

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select the required vGPU.

#### 10.  Specify the API key to access endpoints. Do one of the following:

»

If you did not create an API key to access the endpoint, click

#### Create a New API Key

.

Creating an API Key

For more information, see

on page  283.

»

If you already created an API key, select an active API key from the

#### API Keys

dropdown menu.

### 11.  Click

#### Next

.

#### Configuration

The

tab is displayed.

#### Configure Engine Parameters

#### Only Platform-Provided

#### Custom Arguments and

### 12.  In

, select

or

#### Environment Variables

.

#### 13.  (Optional) Configure custom arguments and environment variables. The procedure varies for the following:

»

NVIDIA NIM imported using model URL or model ID

»

NVIDIA NIMs imported manually

#### 14.  To configure custom arguments and environment variables for NVIDIA NIM imported using model URL or

model ID, follow these steps:

a. Select

#### Environment Variables

.

The

#### Engine Parameters

field is displayed.

b. In the

#### Environment Variables

field, enter each environment variable on a new line in the

```bash
key=value
```

format.

c. Click

#### Update

.

### 15.  In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

#### 16.  Enable single node allocation, follow these steps::

a. Select

#### Node Allocation

as

#### Single Node

.

b. In the

#### Number of Accelerators (Per Instance)

field, enter the number of accelerators required for each

instance.

The minimum value is 1. The available values depend on the selected accelerator and cluster inventory.

### 17.  (Optional) Select the

#### Use Custom Configuration for Instances

checkbox to manually configure the

number of vCPUs, and memory.

The checkbox is selected in case of a non-validated model. In other cases, this checkbox remains clear because the system automatically configures the number of vCPUs, and memory according to the model you select and your cluster configuration, and displays the values in the respective fields.

#### vCPUs (Per Instance)

### 18.  (Optional) In the

field, enter the number of Kubernetes node pool vCPUs to assign to

the endpoint.

Ensure that you assign the vCPUs according to your cluster configuration. If you assign a value that exceeds the number of vCPUs configured in the cluster, the system displays an error.

### 19.  (Optional) In the

#### Memory (Per Instance)

field, enter the amount of memory to assign to the endpoint. This

field is auto-populated based on the selected LLM and displays the recommended memory as per your cluster configuration.

Ensure that you assign the memory according to your cluster configuration. If you assign a value that exceeds the memory configured in the cluster, the system displays an error.

### 20.  (Optional) Click

#### Back

.

The

#### Basics

tab is displayed. The values you specify in the

#### Number of Accelerators (Per Instance)

,

#### vCPUs (Per Instance)

#### Memory (Per Instance)

, and

field are not saved. Therefore, complete the following

steps:

a. Click

#### Next

.

b. Update the

#### Number of Accelerators (Per Instance)

field.

c. Update the

#### vCPUs (Per Instance)

field.

d. Update the

#### Memory (Per Instance)

field.

#### Next

### 21.  Click

.

#### Summary

The

tab is displays the data.

22. Review the information.

#### 23.  (Optional) To review the data you specified for

#### Advanced Configuration

, click the

#### View Details

link.

### 24.  Click

#### Create

.

#### List

The system creates the inference endpoint, which is displayed on the

page.

### What to do next

View the status of the endpoint on the

#### Endpoints

page. The system displays one of the following states for the

operation:

- 

#### Active

: The endpoint is created and ready to use. An inference endpoint accepts requests and sends back

responses only if the status of the endpoint displays

#### Active

.

- 

#### Failed

: Endpoint creation failed due to unauthorized NVIDIA NGC Personal Key, or  NGC Catalog  services

access restriction for the personal key. If the system displays an error message other than the ones listed, run the following command, and contact Nutanix Support with the endpoint specifications provided in the output.

```bash
$ kubectl describe isvc  endpoint_name  -n nai-admin
```

#### List

Replace

with the name of the endpoint on the

page.

endpoint_name

- 

#### Pending

: The system is waiting for the resources required to create the endpoint.

For example, if the required number of GPUs are not available, Nutanix Enterprise AI maintains a

#### Pending

status until the GPUs becomes available.

- 

#### Processing

: The system is creating the endpoint.

After you successfully create an endpoint, you can test it to ensure that it is configured correctly. For more information, see

Testing a Local Endpoint

on page  242.

#### Deploying a NIM Endpoint in an Air-Gapped Environment

Deploy a local endpoint for a manually imported NVIDIA NIM by using a container image available in your on-premises registry.

### Before you begin

- 

Ensure that you are assigned the permissions required to perform this operation. For more information, see

Authorization Permissions

on page 155.

- 

Ensure that the NVIDIA NIM container image is available in an on-premises container registry that is reachable from your deployment.

- 

#### Ready

Import at least one NVIDIA NIM manually to Nutanix Enterprise AI and ensure that the import status is

.

For more information, see

Importing NVIDIA NIMs Manually in Air-Gapped Environments

on page  202.

### About this task

To deploy an NVIDIA NIM endpoint in an air-gapped environment, complete model import and then create an endpoint that uses your on-premises NIM container image.

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page displays a summary of all the endpoints that you created.

3. 

#### Create Endpoint

Click

.

The

#### Basics

tab of the

#### Create an Endpoint

dialog box is displayed.

4. 

In the

#### Endpoint Name

field, enter a name for the endpoint.

Nutanix recommends that you provide a name that is meaningful and identifiable. Ensure that the name meets the following complexity requirements:

- 

Contains only lowercase letters, numbers, or special characters (-).

- 

Start with a lowercase letter.

5. 

(Optional) In the

#### Description

field, enter a description for the endpoint.

6. 

From the

#### Purpose

dropdown menu, select

#### Real Time

.

7. 

In the

#### Model Capabilities

field, enter the capabilities required for the endpoint.

The

#### Model Instance Name

field is filtered to display models that support the specified capabilities.

8. 

From the

#### Model Instance Name

dropdown menu, select an NVIDIA NIM that you imported manually and

#### Ready

whose status is

.

9. 

Select the acceleration type and configure the accelerator for the selected model.

#### 10.  Specify the API key to access endpoints. Do one of the following:

»

If you did not create an API key to access the endpoint, click

#### Create a New API Key

.

Creating an API Key

For more information, see

on page  283.

»

#### API Keys

If you already created an API key, select an active API key from the

dropdown menu.

### 11.  Click

#### Next

.

The

#### Configuration

tab is displayed.

### 12.  In

#### Configure Engine Parameters

, select

#### Custom Arguments and Environment Variables

.

### 13.  In the

#### Environment Variables

field, enter each environment variable on a new line in the key=value format.

### 14.  Click

#### Update

.

### 15.  From the

#### Engine Source

options, select

#### Import from other registry

, and then enter the complete image

#### Engine Image URL

URL in

.

Use the image URL for the NIM container that you uploaded to your on-premises registry.

### 16.  Click

#### Update

.

### 17.  Click

#### Next

.

The

#### Summary

tab is displayed.

18. Review the information.

#### 19.  (Optional) To review the data you specified for

#### Advanced Configuration

#### View Details

, click the

link.

### 20.  Click

#### Create

.

The system creates the inference endpoint, which is displayed on the

#### List

page.

### What to do next

#### Endpoints

#### Active

View the endpoint status on the

page. The endpoint is ready to serve requests when the status is

.

#### Creating Experimental Endpoints from Hugging Face models

Create experimental endpoints from Hugging Face models.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Import at least one non-validated model to Nutanix Enterprise AI and ensure that the import status is

#### Active

. For

Importing a Large Language Model from Hugging Face using Model URL or ID

more information, see

on

page 195.

### About this task

To create experimental endpoints from Hugging Face models, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Local Endpoints

From the left navigation pane, select

.

The

#### List

page displays a summary of all the endpoints that you created.

3. 

Click

#### Create Endpoint

.

#### Basics

#### Create an Endpoint

The

tab of the

dialog box is displayed.

4. 

#### Endpoint Name

In the

field, enter a name for the endpoint.

Nutanix recommends that you provide a name that is meaningful and identifiable. Ensure that the name meets the following complexity requirements:

- 

Contains only lowercase letters, numbers, or special characters (-).

- 

Start with a lowercase letter.

5. 

#### Description

(Optional) In the

field, enter a description for the endpoint.

6. 

#### Purpose

#### Real Time

From the

dropdown menu, select

.

7. 

#### Model Capabilities

In the

field, enter the capabilities required for the endpoint.

#### Model Instance Name

The

field is filtered to display models that support the specified capabilities.

8. 

#### Model Instance Name

From the

dropdown menu, select an active imported non-validated model.

9. 

Select the acceleration type and configure the accelerator. Do any of the following:

- 

Enable GPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

#### GPU Passthrough

dropdown menu, select

.

The

#### GPU Passthrough

option is displayed only when the cluster has GPU nodes and the model you

selected supports GPU execution.

#### 2.  From the

#### GPU Card

dropdown menu, select the required GPU card.

Nutanix Enterprise

For information on the GPU cards supported for a model, see GPU Requirements in

AI Requirements

.

- 

Enable CPU acceleration. To enable, follow these steps:

#### 1.  From the

#### Acceleration Type

## CPU

dropdown menu, select

.

The

## CPU

option is displayed only if the model you selected supports CPU execution.

2. (Optional) If you have Intel Xeon 4th-gen or newer CPUs, select

#### Optimise endpoint to run on

#### Intel® AMX enabled CPUs

.

The built-in accelerator available in Intel® Xeon® 4th-gen (or newer) CPUs is enabled.

The default configuration of this checkbox is as follows:

- 

This checkbox is selected by default and grayed out when all CPUs in the cluster support Intel® Advanced Matrix Extensions (AMX).

- 

This checkbox is grayed out by default when none of the CPUs support AMX.

- 

If your Kubernetes cluster is configured with Multi-Instance GPU (MIG), enable MIG. To enable MIG, follow these steps:

#### 1.  From the

#### Acceleration Type

## NVIDIA MIG

dropdown menu, select

.

#### 2.  From the

#### Accelerator Details

#### MIG GPU Profile

dropdown menu, select

.

Only Mixed Strategy MIGs are supported. A single cluster node can be configured for either MIG or GPU Passthrough. A single node cannot utilize both configurations simultaneously. An NAI cluster fully supports a mixed environment. You can seamlessly deploy a combination of MIG-enabled nodes and GPU Passthrough nodes within the same cluster.

- 

If your Kubernetes cluster is configured with GPU, enable vGPU. To enable GPU, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

#### vGPU

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select the required vGPU.

#### 10.  Specify the API key to access endpoints. Do one of the following:

»

If you did not create an API key to access the endpoint, click

#### Create a New API Key

.

Creating an API Key

For more information, see

on page  283.

»

If you already created an API key, select an active API key from the

#### API Keys

dropdown menu.

### 11.  Click

#### Next

.

#### Configuration

The

tab is displayed.

#### Inference Engine

#### vLLM

### 12.  From the

dropdown menu, select

.

### 13.  Expand

#### Show Advanced Engine Configuration

.

### 14.  Select

#### Custom Arguments and Environment Variables

.

#### Engine Parameters

The

field is displayed.

### 15.  Click

#### Update

.

#### Update Engine Parameters

The

dialog box is displayed.

### 16.  In the

#### Arguments

field, ensure that you follow the following formats:

- 

Enter each argument on a new line in the

format.

```bash
--key value
```

- 

If the argument contains multiple words, ensure that the argument is wrapped in double quotes in the

```bash
"--
```

format.

```bash
key "val1 val2"
```

For a list of arguments, see

### 17.  In the

#### Environment Variables

field, enter each environment variable on a new line in the

format.

```bash
key=value
```

### 18.  Review the parsed key-value pairs, click

#### View Parsed Parameters

.

The arguments and environment variables that are parsed correctly are displayed.

19. Correct the format of the arguments and environment variables that are not displayed.
20. Review the parsed parameters.

### 21.  Click

#### Update

.

The number of parameters you added are displayed.

### 22.  Select the

#### KV Cache Aware Routing

checkbox.

The

#### KV Cache Aware Routing

checkbox is displayed only if did all of the following

- 

#### Inference Engine

#### vLLM

Selected the

in

- 

This is available as a tech preview feature only.

- 

Only for the following model capabilities Text2Text, Reasoning, Content Safety, Tool Calling

> [!NOTE]
> Note:   A local endpoint with KV cache-aware routing enabled does not use KV cache-aware routing when configured as a local endpoint within a unified endpoint.

### 23.  In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

#### 24.  (Optional) To enable single node allocation, follow these steps::

a. Select

#### Node Allocation

as

#### Single Node

.

b. In the

#### Number of Accelerators (Per Instance)

field, enter the number of accelerators required for each

instance.

The minimum value is 1. The available values depend on the selected accelerator and cluster inventory.

#### 25.  (Optional) To enable multi node, follow these steps:

Multi-Node is enabled only if if you met the following conditions:

- 

Multi-node is a tech preview feature.

- 

Select the model capabilty as image-to-text, text-to-text, content-safety, tool-calling, or reasoning

- 

#### Inference Engine

#### vLLM

select

as

- 

#### Acceleration Type

#### GPU Passthrough

select

as

.

- 

Disable speculative decoding and KV Cache offloading.

a. Select

#### Node Allocation

as

#### Multi-Node

.

b. elect

#### Nodes Per Instance

to define how many nodes serve one endpoint instance. This option is shown

only when the selected model, runtime, and cluster resources support multi-node allocation.

c. In the

#### Number of Accelerators (Per Node)

field, enter the number of accelerators required for each

node.

### 26.  (Optional) Enable

#### Speculative Decoding

if it is available for your selected configuration.

#### Speculative Decoding

#### Node Allocation

#### Multi-Node

You cannot enable

if you select

as

.

#### Number of Accelerators (Per Instance)

### 27.  In the

field, enter the number of accelerators required for each

instance.

#### Acceleration Type

#### GPU Passthrough

This field is displayed only if you have selected the

as

. The

minimum value is 1. The available values depend on the selected accelerator and cluster inventory.

### 28.  (Optional) Select the

#### Use Custom Configuration for Instances

checkbox to manually configure the

inference engine type, number of vCPUs, and memory.

The checkbox is selected in case of a non-validated model. In other cases, this checkbox remains clear because the system automatically configures the inference engine type, number of vCPUs, and memory according to the model you select and your cluster configuration, and displays the values in the respective fields.

### 29.  (Optional) In the

#### vCPUs (Per Instance)

field, enter the number of Kubernetes node pool vCPUs to assign to

the endpoint.

Ensure that you assign the vCPUs according to your cluster configuration. If you assign a value that exceeds the number of vCPUs configured in the cluster, the system displays an error.

### 30.  (Optional) In the

#### Memory (Per Instance)

field, enter the amount of memory to assign to the endpoint. This

field is auto-populated based on the selected LLM and displays the recommended memory as per your cluster configuration.

Ensure that you assign the memory according to your cluster configuration. If you assign a value that exceeds the memory configured in the cluster, the system displays an error.

### 31.  (Optional) Click

#### Back

.

#### Basics

#### Number of Accelerators (Per Instance)

The

tab is displayed. The values you specify in the

,

#### vCPUs (Per Instance)

,

#### Memory (Per Instance)

, and the

#### Context Length

field are not saved. Therefore,

complete the following steps:

a. Click

#### Next

.

b. Update the

#### Number of Accelerators (Per Instance)

field.

c. Update the

#### vCPUs (Per Instance)

field.

d. Update the

#### Memory (Per Instance)

field.

e. Update the

#### Context Length

field.

### 32.  Click

#### Next

.

#### Summary

The

tab is displays the data.

33. Review the information.

#### 34.  (Optional) To review the data you specified for

#### Advanced Configuration

, click the

#### View Details

link.

### 35.  Click

#### Create

.

#### List

The system creates the inference endpoint, which is displayed on the

page.

Managing Runtime Parameters for a Hibernated Experimental Endpoint

Update custom runtime parameters for an eligible local endpoint in Nutanix Enterprise AI.

### Before you begin

- 

#### Hibernated

This action is available only if you have at least one experimental endpoint in

state.

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

### About this task

To manage runtime parameters for a local endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The

#### List

page displays a summary of all the endpoints that you created.

### 3.  Do one of the following:

- 

Select an endpoint, and then click

#### Actions

>

#### Manage Parameters

.

- 

#### Actions

#### Manage Parameters

Click an endpoint name. In the endpint details page,click

>

.

The runtime parameters dialog box opens.

#### 4.  In the runtime parameters dialog box, enter the required values:

a. Enter each argument on a new line in the

format.

```bash
--key value
```

b. Enter each environment variable on a new line in the

format.

```bash
key=value
```

The system validates the format and displays an error if keys or values are invalid.

### 5.  (Optional) Click

#### View Parsed Parameters

to verify how the entered values are parsed.

### 6.  Click

#### Update

.

The endpoint configuration is updated with the runtime parameters.

### What to do next

Resume the endpoint to use the updated runtime parameters. For more information, see

Resuming an Endpoint

on

page 243.

#### Editing a Local Endpoint

Edit an inference endpoint in Nutanix Enterprise AI.

### About this task

To edit an endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The

#### List

page opens, displaying a summary of all the endpoints that you created.

3. Select an endpoint.

### 4.  Click

#### Actions

#### Edit

>

.

The system displays the

#### Edit Endpoint Configuration

dialog box.

### 5.  (Optional) In the

#### Description

field, update the description of the endpoint.

### 6.  (Optional) In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

### 7.  Click

#### Update

.

The description or number of instances is updated.

#### Deleting a Local Endpoint

Delete an endpoint in Nutanix Enterprise AI.

### About this task

To delete an endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page opens, displaying a summary of all the endpoints that you created.

#### 3.  Select the checkbox associated with an endpoint, and from the

#### Actions

#### Delete

dropdown menu, click

.

The system prompts you to confirm the delete action.

### 4.  In the field provided, type

#### delete

#### Delete Endpoint

and click

.

The endpoint is deleted from Nutanix Enterprise AI and is no longer displayed on the

#### List

page.

#### Testing a Local Endpoint

Test an endpoint created in Nutanix Enterprise AI using a text generation LLM to verify the endpoint configuration and to ensure that the API connection to an LLM is active.

### Before you begin

- 

To test an endpoint created using non-text generation LLMs, you must copy the sample code that contains the URL and API key required to access the endpoint and run it in your application. For more information, see

Viewing Sample API Code

on page  220.

- 

Create an inference endpoint, and ensure that the status of the endpoint is

#### Active

.

### About this task

To test an endpoint, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

#### List

The

page opens, displaying a summary of all the endpoints that you created.

3. Click the name of an endpoint to test.

The endpoint details page opens, displaying detailed information about the endpoint in widgets.

### 4.  Click

#### Test

.

Test functionality is not supported for Image Generation and Image Classification endpoints.

#### Test Endpoint

The system displays the

dialog box.

### 5.  Perform one of the following:

»

#### Sample Request

#### Test

Select

, choose a default question, and click

.

»

#### Custom Request

#### Test

Select

, enter a question in the field provided, and click

.

If the endpoint is configured correctly, the system displays the

#### Status

as

#### Succeeded

and the

#### Result

text box

displays a generated answer.

#### 6.  To close the dialog box and return to the endpoint details page, click

#### Done

.

#### Accessing an Endpoint using Open AI Compatible Clients

Access an endpoint created in Nutanix Enterprise AI using any OpenAI compatible clients.

### Before you begin

Ensure that the following requirements are met before you access an endpoint created in Nutanix Enterprise AI using any OpenAI compatible clients.

- 

Creating an Endpoint

Ensure that endpoints are created in Nutanix Enterprise AI. For more information, see

.

- 

Ensure that an API key is created in Nutanix Enterprise AI and attached it to the endpoints. For more information, see

Creating an API Key

.

- 

Ensure that the API key created in Nutanix Enterprise AI is made available for your use.

### About this task

Nutanix Enterprise AI supports the following OpenAI compatible endpoints:

- 

/v1/images/generations

- 

/v1/chat/completions

- 

/v1/models

- 

v1/embeddings

- 

v1/audio/transcriptions

- 

/v1/audio/translations

### Procedure

- 

For information on how to access an endpoint using an OpenAI-compatible client, see the client-specific documentation.

#### Hibernating an Endpoint

Hibernate an existing endpoint in Nutanix Enterprise AI.

### About this task

Hibernating an endpoint in Nutanix Enterprise AI pauses the activity of the endpoint and releases the associated compute resources without deleting the endpoint. Hibernating an endpoint optimizes resource usage while preserving endpoint configuration for future use. You can hibernate an endpoint when the cluster that hosts Nutanix Enterprise AI has limited available resources. You can resume a hibernated endpoint after you add more compute resources to the cluster or if the cluster already has enough resources.To hibernate an endpoint, follow these steps:

### Procedure

### 1.  Log in to Nutanix Enterprise AI

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The List page opens, displaying a summary of all the endpoints.

#### 3.  Select the checkbox associated with an endpoint, and from the

#### Actions

dropdown menu, click

#### Hibernate

.

The system prompts you to confirm the hibernation action.

### 4.  In the field provided, type

#### hibernate

and click

#### Hibernate

.

#### Hibernated

The system hibernates the endpoint and displays the endpoint status as

on the List page.

### What to do next

Resume the hibernated endpoint after you add more compute resources to the cluster or if the cluster already has enough resources. For more information, see

Resuming an Endpoint

on page 243

#### Resuming an Endpoint

Resume a hibernated endpoint in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that the cluster that hosts Nutanix Enterprise AI has enough resources to meet the endpoint requirements.

- 

If the cluster does not have enough resources, allow a five-minute gap between the Hibernate and Resume actions to ensure that Kubernetes fully releases the resources used by the hibernated endpoint.

- 

If the cluster has enough resources to meet the endpoint requirements, you can resume the endpoint immediately. However, the startup time might increase depending on the resource usage of the endpoint.

### About this task

To resume an endpoint, follow these steps:

### Procedure

### 1.  Log in to Nutanix Enterprise AI

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The List page opens, displaying a summary of all the endpoints.

#### 3.  Select the checkbox associated with an endpoint, and from the

#### Actions

#### Resume

dropdown menu, click

.

The system prompts you to confirm the resume action.

### 4.  Click

#### Resume

.

#### Resume

After you click

, the system provisions the compute resources required to run the endpoint. If the cluster

has the required resources, the system displays the endpoint status as

#### Ready

on the List page. If the cluster

#### Pending

does not have the required resources, the system displays the endpoint status as

until the resources are

available.

The system resumes the endpoint.
