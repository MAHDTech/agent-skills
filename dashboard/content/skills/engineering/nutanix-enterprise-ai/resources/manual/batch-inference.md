+++
title = "batch-inference"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Batch Inference Processing

Batch inference architecture, data sources (S3/NFS buckets), creating dedicated batch local endpoints, creating, monitoring, pausing, resuming, and deleting batch inference jobs.

---

## RUNNING BATCH INFERENCE

Run a batch inference job in Nutanix Enterprise AI (NAI).

### About this task

To run a batch inference job, follow these high-level steps:

### Procedure

1. Create a data source exclusively for batch inference.

Adding a Data Source for Batch Inference

For more information, see

on page  245.

2. Create a local endpoint exclusively for batch inference.

For more information, see

Creating a Local Endpoint for Batch Inference

on page 246.

3. Start the batch inference job.

Creating a Batch Inference Job

For more information, see

on page  250.

#### Adding a Data Source for Batch Inference

Add an NFS-based data source containing a JSONL dataset exclusively for batch inference in Nutanix Enterprise AI(NAI).

### Before you begin

Ensure that you meet the following requirements:

- 

The dataset file must be in JSONL format because NAI only accepts JSONL files.

- 

The content of the datasource file must be in one of the following formats because NAI adds the dataset only if the content follows any of these formats:

- 

Batch-chat request format

```bash
{"custom_id": 1, "method" :"POST", "url": "/v1/chat/completions", "model":
"llama-3.2-1b", "body": {"messages": [{"role": "user", "content": "What color is
the sky?"}, {"role": "assistant", "content": "It is blue."}]}}
{"custom_id": 2, "method" :"POST", "url": "/v1/chat/completions", "model":
"llama-3.2-1b", "body": {"messages": [{"role": "system", "content": "You are
a helpful assistant"}, {"role": "user", "content": "What color is the sky?"},
{"role": "assistant", "content": "It is blue."}]}}
```

- 

Batch-embedding request

```bash
{"custom_id": 1, "method" :"POST", "url": "/v1/embeddings", "model": "granite",
"body": {"input": "What color is the sky?"}}
{"custom_id": 2, "method" :"POST", "url": "/v1/embeddings", "model": "granite",
"body": {"input": ["What color is the sky?", "What is your name"]}}
```

- 

NIM embedding models/endpoints

```bash
{"custom_id": 1, "method" :"POST", "url": "/v1/embeddings", "model": "granite",
"body": {"input": "What color is the sky?", "extra_body": {"input_type":
"passage"}}}
{"custom_id": 2, "method" :"POST", "url": "/v1/embeddings", "model": "granite",
"body": {"input": ["What color is the sky?", "What is your name"], "extra_body":
{"input_type": "passage"}}}
```

- 

You can add datasets only from Network File System (NFS) for batch inferencing.

- 

Data sources must be 200 MB or smaller and contain a maximum of 50,000 lines. For embedding data sources, a single line in the

field can contain multiple requests. The total number of requests across all lines must not

```bash
input
```

exceed 50,000.

- 

The

field in the requests for datasource must exactly match the name of the endpoint for batch inference to

```bash
model
```

work.

- 

Nutanix Enterprise AI supports the following OpenAI compatible endpoints for batch inference:

- 

/v1/chat/completions

for text-to-text batch endpoints

- 

v1/embeddings

for embeddings batch endpoints

Therefore, enter only one of the supported values in the data source

field.

```bash
url
```

### About this task

To add a data source, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Data Sources

.

#### Data Sources

The

page is displayed.

### 3.  Click

#### Add Data Source

.

### 4.  In the

#### Name

field, enter a name.

### 5.  From the

#### Purpose

#### Batch Inference

dropdown menu, select

.

#### 6.  Add a dataset from File Share. To add a dataset from File Share, follow these steps:

a. From the

#### Source

dropdown menu, select

#### File Share

.

b. In the

#### File Server Address

field, enter the fully qualified domain name (FQDN) or an IP address.

c. In the

#### NFS Export Path

field, enter the path.

d. In the

#### JSONL File Path

field, enter the path.

e. In the

#### Size

field, enter a size equal to or greater than the actual file size displayed in the file share to ensure

that files are copied successfully to file shares within the NAI cluster. Enter the size in MB.

### 7.  Click

#### Add Data Source

.

The data set is displayed in the

#### Data Sources

page.

### What to do next

Create a local endpoint for batch inference. For more information, see

Creating a Local Endpoint for Batch

Inference

on page 246.

#### Creating a Local Endpoint for Batch Inference

Create a local endpoint exclusively for processing batch inference jobs in Nutanix Enterprise AI. Batch endpoints require a separate data source and specific configuration settings.

### About this task

To create a local endpoint for batch inference, follow these steps:

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

#### Batch

.

7. 

In the

#### Model Capabilities

dropdown menu, select the following model capabilities:

- 

#### Text To Text

- 

#### Embedding

The models displayed in the

#### Model Instance Name

field are filtered based on these model capabilities.

8. 

From the

#### Model Instance Name

dropdown menu, select an active imported model required to deploy the

endpoint.

The dropdown menu displays only the models with all the capabilities that you specified in the

#### Model

#### Capabilities

field.

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

#### Acceleration Type

dropdown menu displays

#### GPU Passthrough

only if the cluster has GPU

Step

nodes and the model you selected in

supports GPU execution.

- 

Enable CPU acceleration. Complete the following steps:

#### 1.  From the

#### Acceleration Type

## CPU

dropdown menu, select

.

The

#### Acceleration Type

dropdown menu displays

## CPU

only if the model you selected in

Step 6

supports CPU execution.

2. (Optional) If you have Intel Xeon 4th-gen or newer CPUs, select the

#### Optimise endpoint to run on

#### Intel® AMX enabled CPUs

checkbox.

The built-in accelerator available in Intel® Xeon® 4th-gen (or newer) CPUs is enabled.

The default configuration of this checkbox is as follows:

- 

This checkbox is selected by default if all the CPUs in the cluster are Intel® Advanced Matrix Extensions (AMX) enabled CPUs.

- 

This checkbox is grayed out by default if none of the CPUs in the cluster are Intel® AMX enabled CPUs.

- 

If your Kubernetes cluster is configured with Multi-Instance GPU (MIG), follow these steps:

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

Only Mixed Strategy MIGs are supported in NAI. A single cluster node can be configured for either MIG or GPU Passthrough. A single node cannot utilize both configurations simultaneously. An NAI cluster fully supports a mixed environment. You can seamlessly deploy a combination of MIG-enabled nodes and GPU Passthrough nodes within the same cluster.

- 

If your Kubernetes cluster is configured with vGPU, follow these steps:

#### 1.  From the

#### Acceleration Type

dropdown menu, select

#### vGPU

.

#### 2.  From the

#### Accelerator Details

dropdown menu, select the required vGPU.

### 10.  Click

#### Next

.

The

#### Configuration

tab is displayed.

### 11.  From the

#### Inference Engine

dropdown menu, select the type of inference engine required to deploy the

endpoint.

### 12.  Expand

#### Show Advanced Engine Configuration

.

### 13.  In

#### Configure Engine Arguments

, select

#### Only Platform-Provided

.

### 14.  From the

#### Engine Source

dropdown menu, select any of the images provided with NAI.

### 15.  Select the

#### Enable KV Cache Aware Routing

checkbox.

#### Enable KV Cache Aware Routing

The

checkbox is displayed only when you meet all the following

conditions:

- 

You selected the

#### Inference Engine

as

#### vLLM

.

- 

KV cache aware routing is only available as a tech preview feature.

- 

You selected the following model capabilities:

#### Text2Text

,

#### Reasoning

,

#### Content Safety

, and

#### Tool

#### Calling

.

### 16.  In the

#### Context Length

field, enter the maximum number of tokens that can be processed.

Consider the following information when configuring the context length:

- 

The context length is based on the selected pre-validated Hugging Face model, the CPU or GPU card, and the number of GPUs.

- 

For better performance, it is recommended to reduce the context length of batch purpose endpoints from the maximum length to a value suitable for the datasource .

- 

This limit includes both input and output tokens.

- 

You can manually configure the context length based on your available system resources, including the Hugging Face model, the CPU or GPU card, and the number of GPUs.

- 

The context length determines the combined length of the input text and the generated output.

- 

The system displays a default value only for pre-validated models.

- 

For a custom model, consider the following:

- 

The maximum context length is 1048576 if GPUs are enabled.

- 

The maximum context length is 4096 when GPUs are not enabled.

If you enter a value higher than the maximum suppored value, an error is displayed.

This field appears is displayed only when you meet the following conditions:

- 

Step 6

You selected a Hugging Face model in

.

- 

You selected the

#### Inference Engine

as

#### vLLM

.

### 17.  In the

#### Number of GPUs (Per Instance)

field, enter the number of GPUs per instance required to run the

model.

#### Acceleration Type

#### GPU Passthrough

This field is displayed only if you have selected the

as

. The

minimum number of GPUs is 1. The supported numbers are 1, 2, 4, and 8.

### 18.  In the

#### Number of Instances

field, enter the number of instances required to serve the endpoint.

### 19.  (Optional) Select the

#### Use Custom Configuration for Instances

checkbox to manually configure the

inference engine type, number of vCPUs, and memory.

The checkbox is selected in case of a non-validated model. In other cases, this checkbox remains clear because the system automatically configures the inference engine type, number of vCPUs, and memory according to the model you select in

Step 6

and your cluster configuration, and displays the values in the respective fields.

### 20.  (Optional) In the

#### vCPUs (Per Instance)

field, enter the number of Kubernetes node pool vCPUs to assign to

the endpoint.

Ensure that you assign the vCPUs according to your cluster configuration. If you assign a value that exceeds the number of vCPUs configured in the cluster, an error appears.

### 21.  (Optional) In the

#### Memory (Per Instance)

field, enter the amount of memory to assign to the endpoint.

This field is auto-populated based on the selected LLM and displays the recommended memory as per your cluster configuration. Ensure that you assign the memory according to your cluster configuration. If you assign a value that exceeds the memory configured in the cluster, the system displays an error.

### 22.  Click

#### Next

.

The

#### Summary

tab is displays the information you entered.

23. Review the information.

#### 24.  (Optional) To view the data you specified for

#### Advanced Configuration

#### View Details

, click

.

### 25.  Click

#### Create

.

The local endpoint is displayed in the

#### List

page.

### What to do next

- 

Creating a Batch

Start a batch inference job using the endpoint you created. For more information, see

Inference Job

on page  250.

- 

(Optional) Edit the endpoint. For more information, see

Editing a Local Endpoint

on page 241.

- 

Delete the endpoint. For more information, see

Deleting a Local Endpoint

on page 241.

- 

Resuming an Endpoint

Resume the endpoint. For more information, see

on page 243.

#### Creating a Batch Inference Job

Start a batch inference job to process large volumes of inference requests from a JSONL file using a local endpoint in Nutanix Enterprise AI.

### Before you begin

Ensure you have created the following:

- 

A data source for batch inference. For more information, see

Adding a Data Source for Batch Inference

on

page 245.

- 

Creating a Local Endpoint for Batch Inference

A local endpoint for batch inference. For more information, see

on page 246.

### About this task

To start a batch inference, follow these steps:

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

Select the

#### Batch Inference

tab.

4. 

Click

#### Create Batch Inference Task

.

#### Create Batch Inference Task

The

dialog box is displayed.

5. 

#### Name

In the

field, enter the batch process name.

6. 

#### Data Source

From the

dropdown menu, select a data source you created earlier for batch inference.

7. 

#### Endpoint

From the

dropdown menu, select a local endpoint you created earlier for batch inference.

8. 

#### File Server Address

In the

field, enter the fully qualified domain name or IP address of the file server.

Ensure that the Network File System (NFS) has read/write access so that batch inference can write the output file to this NFS or path.

9. 

In the

#### NFS Export Path

field, enter the NFS export path to the shared directory.

### 10.  In the

#### JSONL File Path

field, enter the path to the JSONL input file.

### 11.  Click

#### Start

.

#### Batch Inference

The batch inference job starts. You can monitor the job status in the

tab.

### What to do next

- 

(Optional) Stop the batch inference job. For more information, see

Stopping a Batch Inference Job

on

page 251.

- 

(Optional) Delete the batch inference job. For more information, see

Deleting a Batch Inference

on page  251.

#### Stopping a Batch Inference Job

Stop a running batch inference job to cancel processing before completion in Nutanix Enterprise AI.

### About this task

To stop a batch inference, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### Local Endpoints

### 2.  From the left navigation pane, select

.

#### List

The

page displays a summary of all the endpoints that you created.

### 3.  Select the

#### Batch Inference

tab.

The list of batch inference jobs are displayed.

4. Select a job.

### 5.  Select

#### Actions

#### Stop

>

.

The batch inference job stops and the

#### Batch Inference

tab displays the updated status changes.

#### Deleting a Batch Inference

Delete a completed or stopped batch inference job to remove it from Nutanix Enterprise AI.

### About this task

To delete a batch inference, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Local Endpoints

.

The

#### List

page displays a summary of all the endpoints that you created.

### 3.  Select the

#### Batch Inference

tab.

The list of batch inference jobs are displayed.

4. Select a job.

### 5.  Select

#### Actions

#### Delete

>

.

A confirmation dialog box is displayed.

### 6.  In the field provided, type

#### delete

.

### 7.  Click

#### Delete

.

The batch inferencing job is deleted from Nutanix Enterprise AI and is no longer displayed on the

#### List

page.
