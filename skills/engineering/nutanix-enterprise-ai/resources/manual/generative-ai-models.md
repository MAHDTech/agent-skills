# Nutanix Enterprise AI Manual: Generative AI Models and Fine-Tuning

Pre-validated generative AI models catalog, model access control, importing models from Hugging Face (catalog, URL/ID, manual air-gap), importing NVIDIA NIMs (catalog, URL/ID, air-gap), model size calculation, forward proxy configuration, import troubleshooting, model fine-tuning workflows, data sources, training metrics, and importing fine-tuned models.

---

#### GENERATIVE AI MODELS IN NUTANIX ENTERPRISE AI

You can select and import text-based generative LLMs from Hugging Face or NVIDIA NGC Catalog to Nutanix Enterprise AI. These LLMs are displayed in the  Models  page.

Nutanix Enterprise AI supports importing an LLM in the following ways:

Import a pre-validated LLM from Hugging Face

You can directly import a pre-validated LLM from Hugging Face. For more information, see

Importing a Large Language Model from Hugging Face

on page 193. For information on the LLMs

that can be imported directly, see

Pre-validated Models

on page 184.

Import a pre-validated NIM from NVIDIA NGC Catalog

You can directly import a NIM from NVIDIA NGC Catalog. For more information, see

Importing

NVIDIA NIMs from NVIDIA NGC Catalog

on page  199. For information on the pre-validated NIMs

that can be imported directly, see

Pre-validated Models

on page 184.

Import an LLM from Hugging Face using Model URL

You can import an LLM not validated by Nutanix directly from Hugging Face by adding the URL of the model in Nutanix Enterprise AI. For more information, see

Importing a Large Language Model

from Hugging Face using Model URL or ID

on page 195.

Import a pre-validated LLM from Hugging Face manually

If Nutanix Enterprise AI is deployed in a dark site, you can download a pre-validated LLM from Hugging Face to your local storage, Network File System (NFS) file share, or an S3 compatible Object bucket and then import it to Nutanix Enterprise AI. Dark sites are primarily on-premises installations that do not have access to the internet.

Importing a Large Language Model Manually

For information on how to manually import an LLM, see

from Hugging Face

on page  196.

Import an NVIDIA NIM manually

If Nutanix Enterprise AI is deployed in an airgapped environment or you cannot use the NVIDIA NGC Catalog, you can place the NIM artifacts on an NFS file share or an S3-compatible object bucket and then import the NIM to Nutanix Enterprise AI.

Importing a Large Language Model Manually

For information on how to manually import a NIM, see

from Hugging Face

on page  196.

Import a custom LLM

A user who has advanced knowledge about LLM operations can import a custom LLM from local storage, Network File System (NFS) file share, or an S3 compatible Object bucket to Nutanix Enterprise AI. A customized LLM might be a fine-tuned version or the latest version of an existing LLM and might resemble the

pre-validated

LLMs in its architecture. However, Nutanix does not

validate a custom LLM when you import it to Nutanix Enterprise AI.

For information on how to import a custom LLM, see

Importing a Large Language Model Manually from

Hugging Face

on page 196.

After a model is imported, a user with permissions can share access to that model with other users by using authorization policy scope.

> [!NOTE]
> Note:   Model resources are subject to change across releases based on a variety of factors.

#### Pre-validated Models

This section describes the models in Hugging Face and NVIDIA NIM format, which Nutanix tested and validated to run successfully on Nutanix Enterprise AI. The CPU and memory requirements for these models are automatically populated when deployed as an endpoint.

The following table lists the pre-validated models and the size of each model.

> [!NOTE]
> Note:   In addition to the pre-validated models listed in the following table, Nutanix Enterprise AI also supports importing unvalidated NVIDIA NIMs or custom LLM models. The architecture of a custom model might resemble the architecture of a listed pre-validated model. However, Nutanix does not validate these models when you import them to Nutanix Enterprise AI. To import an unvalidated NIM or a custom model, see

Importing a Large Language

Model Manually from Hugging Face

on page  196.

**Table 49: Pre-validated Models**

| Model Hub | Provider | Model | Model Type | Model Size (GiB) |
| --- | --- | --- | --- | --- |
| Hugging Face | AI21 Labs | ai21labs/AI21- Jamba-1.5-Mini | Text Generation | 110 |
|  | AllenAI | allenai/Olmo-3-7B- Instruct |  | 20 |

- 

Tool Calling

- 

Text to text

allenai/ Olmo-3-32B-Think

70

- 

Reasoning

- 

Text to text

allenai/Olmo-3-7B- Think

20

- 

Reasoning

- 

Text to text

Cross-Encoder

cross-encoder/ms- marco-MiniLM-L6- v2

Reranker

4

Facebook

facebook/deit- base-distilled- patch16-224

Image Classification

4

Google

google/ gemma-2-2b-it

Text Generation

10

google/ gemma-2-9b-it

Text Generation

20

google/vit-base- patch16-224

Image Classification

4

google/ gemma-3-270m-it

Text Generation

10

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

google/gemma-4- E2B-it

20

- 

Image to text

- 

Reasoning

- 

Text to text

- 

Tool Calling

google/ gemma-4-26B- A4B-it

60

- 

Image to text

- 

Reasoning

- 

Text to text

- 

Tool Calling

google/ gemma-4-31B-it

70

- 

Image to text

- 

Reasoning

- 

Text to text

- 

Tool Calling

IBM

ibm-granite/granite- embedding-107m- multilingual

Embedding

2

Meta

meta-llama/ Llama-2-13b-chat- hf

Text Generation

60

meta-llama/ Llama-3.2-3b- Instruct

Text Generation

20

meta-llama/ Llama-3.2-1B- Instruct

Text Generation

10

meta-llama/ Llama-3.3-70B- Instruct

Text Generation

290

meta-llama/Meta- Llama-3.1-8B- Instruct

Text Generation

40

meta-llama/Meta- Llama-3.1-70B- Instruct

Text Generation

290

meta-llama/ CodeLlama-7b- Instruct-hf

Text Generation

30

meta-llama/ CodeLlama-13b- Instruct-hf

Text Generation

60

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

meta-llama/ CodeLlama-34b- Instruct-hf

Text Generation

140

meta-llama/ CodeLlama-70b- Instruct-hf

Text Generation

280

meta-llama/ Llama-3.2-11B- Vision-Instruct

Vision

55

meta-llama/ Llama-3.2-90B- Vision-Instruct

Vision

320

meta-llama/ Llama-4- Scout-17B-16E- Instruct

Text Generation

250

meta-llama/Llama- Guard-3-8B

Safety

17

Mistral AI

mistralai/Mistral-7B- Instruct-v0.3

Text Generation

30

mistralai/ Mixtral-8x7B- Instruct-v0.1

Text Generation

200

mistralai/ Mixtral-8x22B- Instruct-v0.1

Text Generation

290

mistralai/Mistral- Nemo-Instruct-2407

Text Generation

50

mistralai/Magistral- Small-2506

Text Generation

100

mistralai/Devstral- Small-2507

Text Generation

100

ministral-3-14B- Reasoning-2512

60

- 

Reasoning

- 

Tool Calling

- 

Image to text

- 

Text to text

mistralai/ Ministral-3-8B- Instruct-2512

30

- 

Tool Calling

- 

Image to text

- 

Text to text

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

mistralai/ Ministral-3-8B- Reasoning-2512

40

- 

Reasoning

- 

Tool Calling

- 

Image to text

- 

Text to text

mistralai/ Ministral-3-3B- Instruct-2512

10

- 

Tool Calling

- 

Image to text

- 

Text to text

mistralai/ Ministral-3-3B- Reasoning-2512

20

- 

Reasoning

- 

Tool Calling

- 

Image to text

- 

Text to text

mistralai/ Ministral-3-14B- Instruct-2512

40

- 

Tool Calling

- 

Image to text

- 

Text to text

mistralai/Mistral- Large-3-675B- Instruct-2512

690

- 

Image to Text

- 

Tool Calling

- 

Text to text

mistralai/Mistral- Small-4-119B-2603

250

- 

Image to Text

- 

Reasoning

- 

Tool Calling

- 

Text to text

NVIDIA

nvidia/NVIDIA- Nemotron-3- Nano-30B-A3B- FP8

40

- 

Reasoning

- 

Tool Calling

- 

Text to text

nvidia/NVIDIA- Nemotron-3- Nano-30B-A3B- BF16

70

- 

Reasoning

- 

Tool Calling

- 

Text to text

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

nvidia/NVIDIA- Nemotron-3- Super-120B-A12B- BF16

250

- 

Reasoning

- 

Tool Calling

- 

Text to text

nvidia/NVIDIA- Nemotron-3- Ultra-550B-A55B- BF16

1130

- 

Reasoning

- 

Tool Calling

- 

Text to text

OpenAI

openai/gpt-oss-20b Text Generation

50

openai/gpt- oss-120b

Text Generation

200

openai/gpt-oss- safeguard-20b

Content Safety

20

openai/gpt-oss- safeguard-120b

Content Safety

70

Stability AI

stable-diffusion- v1-5/stable- diffusion-v1-5

Image Generation

40

Unsloth

unsloth/ Llama-3.3-70B- Instruct-bnb-4bit

Text Generation

50

NVIDIA NGC Catalog

NVIDIA

llama-3.1-8b- instruct

Text Generation

50

llama-3.1-70b- instruct

Text Generation

160

llama-3.1- nemoguard-8b- content-safety

Safety

50

llama-3.2-nv- embedqa-1b-v2

Embedding

5

llama-3.2-nv- rerankqa-1b-v2

Reranker

5

llama-3.3-70b- instruct

Text Generation

160

llama-3.3- nemotron- super-49b-v1

Text Generation

120

llama-3.1- swallow-8b-instruct- v0

Text Generation

50

llama-3.1- nemoguard-8b- topic-control

Safety

50

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

mixtral-8x7b- instruct-v01

Text Generation

110

mistral-7b-instruct- v0

Text Generation

50

phi-3-mini-4k- instruct

Text Generation

10

black-forest-labs/ flux.1-dev

Image Generation

40

To use the model, you must have a valid Hugging Face token added to Nutanix Enterprise AI, and that token must have access permissions for the model on Hugging Face.

Mistral-nemo-12b- instruct

Text Generation

80

llama-nemotron- embed-vl-1b-v2

Embedding

40

Llama-3.2-90b- vision-instruct

Vision

200

Llama-3.1-70b- instruct-pb24h2

Text Generation

160

Llama-3.1- swallow-8b-instruct- v0.1

Text Generation

50

Llama-3.1- nemotron-70b- instruct

Text Generation

160

Llama-3.1-8b- instruct-pb24h2

Text Generation

50

Mistral-7b-instruct- v0.3

Text Generation

50

Mixtral-8x7B- Instruct-v0.1

Text Generation

110

gpt-oss-20b

Text Generation

60

gpt-oss-120b

Text Generation

210

nemoretriever- graphic-elements- v1

Object Detection

2

nemoretriever- parse

Object Detection

16

#### Model Hub

#### Provider

#### Model

#### Model Type

#### Model Size (GiB)

nemoretriever- table-structure-v1

Object Detection

2

nemoretriever-ocr- v1

Object Detection

6

nemoretriever- page-elements-v2

Object Detection

2

openai/whisper- large-v3

#### Configuring Access to Models

Configure ML User (user) access to models.

### Before you begin

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page  155.

### About this task

The default configuration to view and access models is as follows:

- 

If you upgraded Nutanix Enterprise AI, users can view all the models from prior versions. New models are disabled.

- 

If you installed Nutanix Enterprise AI for the first time, all the models are disabled by default.

You can either use the default configuration or configure permissions to view and download models.

To restrict user access to models in the catalog, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

#### Model Access Control

>

.

#### Model Access Control

#### Allow direct download using model

The

page opens. The default configuration of

## URL

and

#### Allow Manual Upload

are as follows:

- 

Both are enabled if:

- 

You upgraded Nutanix Enterprise AI

- 

You have uploaded models.

- 

Both are disabled if :

- 

You installed Nutanix Enterprise AI

- 

You have not uploaded models.

#### 3.  (Optional) To restrict the models that users can download, follow these steps:

a. In

#### Access Control for Catalogs

, select

#### Modify Allowed List

for the required catalog.

b. (Optional) To allow all the validated models, select

#### Allow all validated models

.

c. (Optional) To select only specific models, follow these steps:

#### 1.  Select

#### Allow Specific models

.

2. (Optional) To filter and view the LLMs based on model capabilities, select the required model capabilities

#### Filter by Capabilities

from the

dropdown menu.

Only the models with all the capabilities you specified in the

#### Model Capabilities

field are displayed.

3. Select the required models.
d. Click

#### Save

.

After you save,

- 

users cannot import restricted models.

- 

users can continue to use the models they downloaded before you applied this restriction. You can see a notification to delete the restricted models in the

#### Models List

screen and the

#### Endpoints List

screen.

- 

users cannot create new endpoints on previously imported and now disabled models.

e. To meet regulations, inform users to manually delete the restricted models and endpoints.

#### 4.  (Optional) To allow users to download Hugging Face Model hub models that are not in the catalog, enable

#### Import Model using model URL

.

#### 5.  (Optional) To prevent users from downloading models Hugging Face Model hub that are not in the catalog disable

#### Import Model using model URL

.

#### 6.  (Optional) To allow users to upload a model from a file share or bucket, enable

#### Allow Manual Upload

.

### 7.  (Optional) To allow only users with

#### cluster_updateConfigs

permission to upload a model, follow these steps:

a. Disable

#### Allow Manual Upload

.

b. Click

#### Disable

.

#### cluster_updateConfigs

After you disable, only users with

permission can upload models or catalog models.

Users cannot upload models or catalog models.

Users with pernissions can import disabled models. However, a warning that

```bash
Some models are not
```

is displayed in the

#### Models List

page.

```bash
compliant with the current organization policy.
```

### What to do next

- 

Inform users that they cannot download the restricted models in future.

- 

Inform users to stop using the restricted models that they downloaded before you applied this restriction. To stop using restricted models,

#### 1.  Delete the endpoints. For more information, see

Deleting a Local Endpoint

on page  241.

#### 2.  Delete the restricted models. For more information, see

Deleting a Large Language Model

on page  203.

#### Viewing Imported Large Language Models

The  Models  page displays all the large language models (LLMs) that are imported to Nutanix Enterprise AI.

### About this task

To view your imported LLMs, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

The

#### Models

page opens displaying a summary of all the LLMs imported to Nutanix Enterprise AI.

- 

#### Model Instance Name

: Displays the model name that you provide while importing the model.

- 

#### Model

: Displays the LLM name and the link to the source page

- 

#### Model Capabilities

: Displays the capabilities of the model.

- 

#### Developer

: Displays the developer of the model

- 

Import Mode: Displays the method used to import the model.

- 

Imported By: Displays the name of the user who imported the model.

- 

Status:

- 

For Hugging Face models, the status is displayed as follows:

- 

For NVIDIA NIMs, the status is displayed as follows:

- 
- 

#### Status

: Displays the current status of the model. The status may be any of the following:

- 

#### Ready

: The NIM is imported and ready to use.

- 

#### Failed

: The import failed due to deactivated NVIDIA NGC Personal Key, or incorrect key value

added in Nutanix Enterprise AI.

You can view the logs for up to 24 hours after import fails. To view the logs, select the model and click

#### Actions

#### Download Logs

>

.

- 

#### Pending

: The system is waiting for the resources required to save the NIM.

For example, if the required storage space is not available, Nutanix Enterprise AI maintains a

#### Pending

status until the storage space becomes available.

- 

#### Processing

: The system is importing the NIM from NVIDIA NGC Catalog.

- 

#### Security Status

: Displays the status of the security scan. This field is displayed only if

- 

The model is a validated Hugging Face model.

- 

You have configured security scan.

For more information, see

Viewing the Scan Status of Models

on page  275.

### 3.  Click a link in the

#### Model Instance Name

column.

#### Model Details

The

page displays the following:

- 

#### Model Instance Name

: Displays the model name that you provide while importing the model.

- 

#### Model

: Displays the name of the model

- 

#### Model Capabilities

: Displays the capabilities of the model.

- 

#### Developer

: Displays the developer of the model

- 

#### Repo version

: Displays the version of the model

- 

#### Import Mode

: Displays the mode and source from where the model was imported.

#### Storage Provider

If you manually uploaded the model, hovering over the hover info icon displays the

,

#### Server IP

,

#### NFS Export Path

, and the

#### Directory Path

for the model.

- 

#### Model Size

: Displays the size of the model

- 

#### Imported By

: Displays the user who imported the model

- 

#### Imported On

: Displays the date when the model was imported.

- 

#### Status

: Displays the status.

### 4.  Click a link in the

#### Model

column.

The the model in Hugging Face Hub or NVIDIA NGC are displayed.

#### Importing a Large Language Model from Hugging Face

Import a

pre-validated

LLM from Hugging Face to Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you accept the usability terms and licenses of the LLM and agree to share your contact information (email address and username) with the repository authors of the model in Hugging Face to access the repository and import the LLM to Nutanix Enterprise AI.

- 

Create an access token in Hugging Face and add it to Nutanix Enterprise AI . For more information, see

Adding

a Hugging Face Token

on page  179. If you create a fine-grained access token in Hugging Face, provide

read access permission to the repository of the model hosted in Hugging Face to ensure that the model can be imported to Nutanix Enterprise AI. For more information, see  User access tokens best practices  in

Hugging Face

documentation

.

### About this task

To import an LLM from Hugging Face, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

#### Models

### 2.  From the left navigation pane, select

.

#### Models

The system displays the

page.

### 3.  Do one of the following:

»

#### Models

#### Import Model

#### View Validated

If you are logging in for the first time, from the

page, click

>

#### Models

.

»

Click

#### Import Models

>

#### From Hugging Face Model Hub

.

#### Import Model - Hugging Face Model Hub

The

page opens, displaying all the LLMs validated to run on

Nutanix Enterprise AI.

### 4.  (Optional) In the

#### Search by Model Name

field, type the LLM name to search for an LLM.

The system lists the matching model names as you type.

#### 5.  (Optional) To filter and view the LLMs based on the model type, select the model type from the

#### All Models

dropdown menu.

#### 6.  (Optional) To filter and view the LLMs based on model capabilities, select the required model capabilities from

#### Filter by Capabilities

the

dropdown menu.

#### Model Instance Name

Only the models with all the capabilities you specified in the

field are displayed.

### 7.  Select an LLM and click

#### Import

.

- 

To go to an LLM's Hugging Face repository, click the name of the LLM.

- 

If you did not add the Hugging Face access token to Nutanix Enterprise AI, the system prompts you to do so. Add the access token to import the LLM. For more information, see

Adding a Hugging Face Token

on

page 179.

The

#### Import Model

dialog box opens.

### 8.  In the

#### Model Instance Name

field, enter a name for the LLM.

Nutanix recommends that you use the actual name of the LLM, suffixed with an identifier that is meaningful to you.

### 9.  Click

#### Import

.

The imported LLM is displayed on the

#### Models

page.

### What to do next

#### Models

After you initiate an import, you can view the status of the import on the

page. The system displays one of

the following states for the import operation:

- 

#### Ready

: The LLM is imported and ready to use. You can create an endpoint from an LLM only if the status

#### Ready

displays

.

- 

#### Failed

: The import failed due to insufficient storage, unauthorized Hugging Face token, or LLM repository read

access restriction for the access token.

#### Actions

You can view the logs for up to 24 hours after import fails. To view the logs, select the model and click

#### Download Logs

>

.

- 

#### Pending

: The system is waiting for the resources required to save the LLM.

#### Pending

For example, if the required storage space is not available, Nutanix Enterprise AI maintains a

status

until the storage space becomes available.

- 

#### Processing

: The system is downloading the LLM from Hugging Face.

After you successfully import an LLM to Nutanix Enterprise AI, you can deploy the LLM to an AI endpoint. Complete the following steps:

1. Select the model.

### 2.  Click

#### Actions

>

#### Create Endpoint

.

#### Create Endpoint

The

action is enabled only for active models and the models you import. You cannot deploy

models created by other users.

### 3.  The

#### Create Endpoint

screen is displayed and the

#### Model Instance Name

field displays the imported model.

4. Create the endpoint.

For more information, see

Creating a Local Endpoint using a non-validated Hugging Face Model

on

page 226.

Importing a Large Language Model from Hugging Face using Model URL or ID

Import an LLM not validated by Nutanix directly from Hugging Face, by adding the URL or ID of the model in Nutanix Enterprise AI.

### Before you begin

- 

Adding

Create an access token in Hugging Face and add it to Nutanix Enterprise AI . For more information, see

a Hugging Face Token

on page  179. If you create a fine-grained access token in Hugging Face, provide

read access permission to the repository of the model hosted in Hugging Face to ensure that the model can be imported to Nutanix Enterprise AI. For more information, see  User access tokens best practices  in

Hugging Face

documentation

.

- 

Ensure that you accept the usability terms and licenses of the LLM and agree to share your contact information (email address and username) with the repository authors of the model in Hugging Face to access the repository and import the LLM to Nutanix Enterprise AI.

### About this task

manual import method

Unlike the

, this method does not require you to manually provision storage, such as a

Network File System (NFS) file share or an S3-compatible Object bucket. This import method avoids downloading and storing the LLM locally, making it efficient for environments with limited storage space. However, Nutanix does not test and validate an LLM when you import it using the model URL.

To import an LLM from Hugging Face using model URL, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Models

From the left navigation pane, select

.

The system displays the

#### Models

page.

3. 

Click

#### Import Models

>

#### From Hugging Face Model Hub

.

#### Import Model - Hugging Face Model Hub

The

page opens, displaying all the LLMs validated to run on

Nutanix Enterprise AI.

4. 

Click

#### Import using Model URL

.

#### Import Model using Hugging Face Model URL

The system displays the

dialog box.

5. 

#### Model URL

In the

field, enter a valid Hugging Face URL or ID of the LLM.

huggingface.com/

hf.co/

huggingface.co/

You can specify the prefix for the URL,as

,

, or

.

6. 

#### Model Instance Name

In the

field, enter a name for the LLM.

Nutanix recommends that you use the actual name of the LLM, suffixed with an identifier that is meaningful to you.

7. 

#### Developer (Optional)

(Optional) In the

field, enter the name of the LLM developer.

8. 

#### Model Capabilities

In the

field, select the required model capabilities.

9. 

#### Model Type

From the

dropdown menu, select the LLM type.

### 10.  Click

#### Import

.

The system prompts you to confirm the import action.

### 11.  In the field provided, type

#### confirm

#### Import

and click

.

The system displays the imported LLM on the

#### Models

page.

### What to do next

#### Models

After you initiate an import, you can view the status of the import on the

page. The system displays one of

the following states for the import operation:

- 

#### Ready

: The LLM is imported and ready to use. You can create an endpoint from an LLM only if the status

#### Ready

displays

.

- 

#### Failed

: The import failed due to insufficient storage, unauthorized Hugging Face token, or LLM repository read

access restriction for the access token.

You can view the logs for up to 24 hours after import fails. To view the logs, select the model and click

#### Actions

#### Download Logs

>

.

- 

#### Pending

: The system is waiting for the resources required to save the LLM.

For example, if the required storage space is not available, Nutanix Enterprise AI maintains a

#### Pending

status

until the storage space becomes available.

- 

#### Processing

: The system is downloading the LLM from Hugging Face.

After you successfully import an LLM to Nutanix Enterprise AI, you can deploy the LLM to an AI endpoint. To deploy the LLM to an AI endpoint,

follow these steps:

1. Select the model.

### 2.  Click

#### Actions

#### Create Endpoint

>

.

The

#### Create Endpoint

action is enabled only for active models and the models you import. You cannot deploy

models created by other users.

#### Create Endpoint

#### Model Instance Name

### 3.  The

screen is displayed and the

field displays the imported model.

4. Create the endpoint.

Creating a Local Endpoint using a non-validated Hugging Face Model

For more information, see

on

page 226.

#### Importing a Large Language Model Manually from Hugging Face

Manually import a pre-validated LLM from Hugging Face or a custom LLM to Nutanix Enterprise AI.

### Before you begin

Ensure that you download the

pre-validated

LLM from Hugging Face in the original file format and save

it to your local storage, Network File System (NFS) file share, or an S3 compatible Object bucket. For information on how to download an LLM from Hugging Face, see

in

Hugging Face

Downloading models

documentation

.

### About this task

To manually import an LLM from your local storage, NFS file share, or an S3 compatible Object bucket, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Models

.

#### Models

The system displays the

page.

3. 

#### Import Models

#### Using Manual Import

Click

>

.

#### Import Model

#### Import Manually

#### Models

If you are logging in for the first time, click

>

from the

page.

#### Manual Upload

The

page opens.

4. 

#### Choose a Model Type

From the

section, choose one of the following:

»

#### Pre-validated Model

: Select this option to upload a

pre-validated

LLM in the original format as

downloaded from Hugging Face.

»

#### Custom Model

: Select this option to upload a custom LLM.

Nutanix does not validate a custom LLM when you import it to Nutanix Enterprise AI.

5. 

From the

#### Model

dropdown menu, select the pre-validated LLM.

This field appears only if you select

#### Pre-validated Model

in

Step 4

.

6. 

In the

#### Model Instance Name

field, enter a name for the LLM.

Nutanix recommends that you use the actual name of the LLM, suffixed with an identifier that is meaningful to you.

7. 

#### Model Capabilities

In the

field, select the model capabilities.

#### Custom Model

The Model Capabilities field is displayed only if you select

in Step 4.

8. 

#### Model Type

From the

dropdown menu, select the LLM type.

9. 

#### Model Size

In the

field, enter the storage size required to store the downloaded files.

#### Custom Model

Step 4

This field appears only if you select

in

.

### 10.  (Optional) In the

#### Developer (Optional)

field, enter the name of the LLM developer.

#### Custom Model

Step 4

This field appears only if you select

in

.

#### Location

### 11.  From the

dropdown menu, select the appropriate option:

»

#### File Share

: If the LLM is saved in an NFS file share.

»

#### Bucket

: If the LLM is saved in an S3 compatible Object bucket.

### 12.  If you select

#### File Share

Step 10

in

, enter the following details:

- 

#### File Server Address

: Enter the fully qualified domain name (FQDN) or the IP address of the Nutanix

Files server.

- 

#### NFS Export Path

: Enter the share path to the NFS export.

- 

#### Directory Path for the Model

: Enter the directory path to the NFS export where you have saved the

LLM.

Nutanix does not validate the NFS configuration in your cluster. An incorrect NFS configuration results in an LLM import failure.

### 13.  If you select

#### Bucket

Step 10

in

, enter the following details:

- 

#### Service Host

: Enter the complete URL or the IP address of the endpoint used to tier the objects.

For example,

.

example.buckets.company.com

- 

#### Bucket Name

: Enter the name of the S3-compatible Object bucket within the service host to which the

objects must tier out.

- 

#### Model Path

: Enter the prefix of the S3-compatible Object bucket key name.

- 

#### Access Key

: Enter the access key of the S3-compatible Object bucket owner.

- 

#### Secret Key

: Enter the secret key of the S3-compatible Object bucket owner.

### 14.  Click

#### Upload

.

The system displays the imported LLM on the

#### Models

page.

### What to do next

#### Models

After you initiate an import, you can view the status of the import on the

page. The system displays one of

the following states for the import operation:

- 

#### Ready

: The LLM is imported and ready to use. You can create an endpoint from an LLM only if the status

#### Ready

displays

.

- 

#### Failed

: The import failed due to insufficient storage, unauthorized Hugging Face token, or LLM repository read

access restriction for the access token.

#### Actions

You can view the logs for up to 24 hours after import fails. To view the logs, select the model and click

>

#### Download Logs

.

- 

#### Pending

: The system is waiting for the resources required to save the LLM.

#### Pending

For example, if the required storage space is not available, Nutanix Enterprise AI maintains a

status

until the storage space becomes available.

- 

#### Processing

: The system is downloading the LLM from Hugging Face.

After you successfully import an LLM to Nutanix Enterprise AI, you can deploy the LLM to an AI endpoint. Complete the following steps:

1. Select the model.

### 2.  Click

#### Actions

>

#### Create Endpoint

.

#### Create Endpoint

The

action is enabled only for ready models and the models you import. You cannot deploy

models created by other users.

### 3.  The

#### Create Endpoint

#### Model Instance Name

screen is displayed and the

field displays the imported model.

4. Create the endpoint.

Creating a Local Endpoint using a non-validated Hugging Face Model

For more information, see

on

page 226.

#### Importing NVIDIA NIMs from NVIDIA NGC Catalog

Import NVIDIA NIMs from NVIDIA NGC Catalog to Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you have an NGC account with an active subscription. For more information, see

NVIDIA

Documentation Hub

.

- 

Certain NIM models are exclusively available with NVIDIA AI Enterprise subscription only. To import these models, ensure that you have an active NVIDIA AI Enterprise subscription. For more information, see

NVIDIA

Documentation Hub

.

- 

Adding

Generate an NVIDIA NGC Personal Key and add it to Nutanix Enterprise AI. For more information, see

an NVIDIA NGC Personal Key

on page  181. After you generate an NVIDIA NGC Personal Key, add  NGC

Catalog  services to the personal key to ensure that the NIM can be imported to Nutanix Enterprise AI. For more information, see the  Personal API Key  section in the  NGC User Guide  listed in

NVIDIA Documentation Hub

.

### About this task

To import a NIM from the NGC catalog, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

The system displays the

#### Models

page.

### 3.  Click

#### Import Models

>

#### From NVIDIA NGC Catalog

.

#### Import Model - NVIDIA NGC Catalog

The

page opens, displaying all the NIMs that can run on Nutanix

Enterprise AI. The NIMs validated by Nutanix display a green checkmark. For more information, see

Pre-

validated Models

on page 184.

### 4.  (Optional) In the

#### Search by Model Name

field, type the LLM name to search for an LLM.

The system lists the matching model names as you type.

#### 5.  (Optional) To filter and view the NIM models based on the model type, select the model type from the

#### All

#### Models

dropdown menu.

### 6.  (Optional) To filter and view only the

pre-validated

NIM models, enable the

#### Show only Pre-validated

#### Models

toggle.

#### Import

### 7.  Select a NIM and click

.

To go to the NVIDIA repository of the NIM, click the name of the NIM. If you did not add the NVIDIA NGC Personal Key to Nutanix Enterprise AI, the system prompts you to do so. Add the personal key to import the NIM. For more information, see

Adding an NVIDIA NGC Personal Key

on page  181.

The

#### Import Model

dialog box opens.

### 8.  In the

#### Model Instance Name

field, enter a name for the NIM.

Nutanix recommends that you use the actual name of the NIM, suffixed with an identifier that is meaningful to you.

### 9.  Click

#### Import

.

The imported NIM is displayed on the

#### Models

page.

### What to do next

#### Models

After you initiate an import, you can view the status of the import on the

page. The system displays one of

the following states for the import operation:

- 

#### Ready

: The NIM is imported and ready to use.

- 

#### Failed

: The import failed due to deactivated NVIDIA NGC Personal Key, or incorrect key value added in

Nutanix Enterprise AI.

#### Actions

You can view the logs for up to 24 hours after import fails. To view the logs, select the model and click

>

#### Download Logs

.

- 

#### Pending

: The system is waiting for the resources required to save the NIM.

#### Pending

For example, if the required storage space is not available, Nutanix Enterprise AI maintains a

status

until the storage space becomes available.

- 

#### Processing

: The system is importing the NIM from NVIDIA NGC Catalog.

After you import a NIM to Nutanix Enterprise AI, you can deploy the NIM to an AI inference endpoint. For more information, see

Creating a Local Endpoint using a Validated Model

on page  221. Complete the following

steps:

1. Select the model.

### 2.  Click

#### Actions

#### Create Endpoint

>

.

The

#### Create Endpoint

action is enabled only for active models and the models you import. You cannot deploy

models created by other users.

### 3.  The

#### Create Endpoint

#### Model Instance Name

screen is displayed and the

field displays the imported model.

4. Create the endpoint.

Creating a Local Endpoint using a non-validated Hugging Face Model

For more information, see

on

page 226.

#### Importing NVIDIA NIMs using Model URL or ID

Import an NVIDIA NIM not validated by Nutanix directly by adding the URL or ID of the model in Nutanix Enterprise AI.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Ensure that you have the the NIM URL of the NIM model. To get the NIM URL, follow these steps:

#### 1.  Go to

NGC Catalog

.

2. Log in to an NVIDIA account and accept the terms of use or license agreement for the model
3. Search for the name of the model and select the correct model. The selected model is displayed.
4. Click Get Container to get the image path which is the NIM URL

### About this task

manual import method

Unlike the

, this method does not require you to manually provision storage, such as a

Network File System (NFS) file share or an S3-compatible Object bucket. This import method avoids downloading

and storing the LLM locally, making it efficient for environments with limited storage space. However, Nutanix does not test and validate an LLM when you import it using the model URL.

To import an LLM from Hugging Face using model URL, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation pane, select

#### Models

.

#### Models

The system displays the

page.

3. 

#### Import Models

#### From NVIDIA NGC Catalog

Click

>

.

The

#### Import Model - NVIDIA NGC Catalog

page displays all the NIMs that can run on Nutanix Enterprise

Pre-validated

AI . The NIMs validated by Nutanix display a green checkmark. For more information, see

Models

on page 184.

4. 

Click

#### Import using Model URL

.

#### Import Model using NVIDIA NGC Catalog URL

The

dialog box is displayed.

## NIM URL

5. 

In the

field, enter the NVCR container name of the NIM container.

Only images from nvcr.io/nim registry are allowed in this field.

6. 

#### Model Instance Name

In the

field, enter a name for the NIM.

Nutanix recommends that you use the actual name of the NIM, suffixed with an identifier that is meaningful to you.

7. 

In the

#### Model Capabilities

field, select the required model capabilities.

8. 

In the

#### Model Size

field, enter the storage size required to store the downloaded files.

For more information about the correct size, see

Supported Models for NVIDIA NIM for LLMs

.

9. 

Click

#### Import

.

The system prompts you to confirm the import action.

### 10.  In the field provided, type

#### confirm

and click

#### Import

.

#### Models

The system displays the imported LLM on the

page.

### What to do next

### 1.  View the status of the import on the

#### Models

page. For more information, see

Viewing Imported Large

Language Models

on page  191.

#### 2.  After you successfully import an LLM to Nutanix Enterprise AI, you can deploy the LLM to an AI endpoint. To

deploy the LLM to an AI endpoint, follow these steps:

1. Select the model.

#### 2.  Click

#### Actions

#### Create Endpoint

>

.

The

#### Create Endpoint

action is enabled only for active models and the models you import. You cannot

deploy models created by other users.

#### 3.  The

#### Create Endpoint

screen is displayed and the

#### Model Instance Name

field displays the imported

model.

4. Create the endpoint.

Creating a Local Endpoint using a non-validated Hugging Face Model

For more information, see

on

page 226.

#### Importing NVIDIA NIMs Manually in Air-Gapped Environments

Manually import an NVIDIA Inference Microservices (NIM) model into Nutanix Enterprise AI from a Network File System (NFS) file share or an S3-compatible object bucket. Use this method for custom models, for unvalidated models, or to import a NIM in an airgapped environment without access to the NVIDIA NGC Catalog.

### Before you begin

- 

Ensure that you are assigned the permissions required to perfrom this operation. For more information, see

Authorization Permissions

on page 155.

- 

Download the NIM artifacts on an NFS file share or an S3-compatible object bucket that Nutanix Enterprise AI can access.

### About this task

To manually import a NVIDIA NIM from an NFS file share or an S3-compatible object bucket, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

#### Models

From the left navigation pane, select

.

The system displays the

#### Models

page.

3. 

Click

#### Import Models

>

#### Using Manual Import

.

#### Manual Upload

The

page is displayed.

4. 

#### Choose a Model Type

#### Custom Model

From the

section, select

:

Nutanix does not validate a custom model when you import it to Nutanix Enterprise AI.

5. 

#### Model Format

#### NVIDIA Inference Microservices (NIM)

From the

dropdown menu, select

.

6. 

#### Model Instance Name

In the

field, enter a name for the NIM.

Nutanix recommends that you use the actual name of the NIM, suffixed with an identifier that is meaningful to you.

7. 

In the

#### Model Capabilities

field, select the model capabilities.

8. 

In the

#### Model Size

field, enter the storage size required to store the downloaded files.

9. 

#### Developer (Optional)

(Optional) In the

field, enter the name of the NIM developer.

### 10.  From the

#### Location

dropdown menu, select the appropriate option:

»

#### File Share

: If the NIM is saved in an NFS file share.

»

#### Bucket

: If the NIM is saved in an S3 compatible Object bucket.

### 11.  If you select

#### File Share

10

in step

on page 202, enter the following details:

- 

#### File Server Address

: Enter the fully qualified domain name (FQDN) or the IP address of the Nutanix

Files server.

- 

#### NFS Export Path

: Enter the share path to the NFS export.

- 

#### Directory Path for the Model

: Enter the directory path to the NFS export where you have saved the

LLM.

Nutanix does not validate the NFS configuration in your cluster. An incorrect NFS configuration results in an LLM import failure.

### 12.  If you select

#### Bucket

in , enter the following details:

- 

#### Service Host

: Enter the complete URL or the IP address of the endpoint used to tier the objects.

For example,

.

example.buckets.company.com

- 

#### Bucket Name

: Enter the name of the S3-compatible Object bucket within the service host to which the

objects must tier out.

- 

#### Model Path

: Enter the prefix of the S3-compatible Object bucket key name.

- 

#### Access Key

: Enter the access key of the S3-compatible Object bucket owner.

- 

#### Secret Key

: Enter the secret key of the S3-compatible Object bucket owner.

### 13.  Click

#### Upload

.

The system displays the imported NVIDIA NIM on the

#### Models

page.

### What to do next

### 1.  View the status of the import on the

#### Models

Viewing Imported Large

page. For more information, see

Language Models

on page  191.

### 2.  When the status is

#### Ready

, you can create an endpoint.

1. Select the model.

#### 2.  Click

#### Actions

#### Create Endpoint

>

.

The

#### Create Endpoint

action is enabled only for ready models and the models you import. You cannot

deploy models created by other users.

#### Create Endpoint

#### Model Instance Name

#### 3.  The

screen is displayed and the

field displays the imported

model.

4. Create the endpoint.

Creating a Local Endpoint using a non-validated Hugging Face Model

For more information, see

on

page 226.

#### Deleting a Large Language Model

Delete an LLM that you imported to Nutanix Enterprise AI.

### About this task

To delete an LLM, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

The system displays the

#### Models

page.

### 3.  Select an LLM, and from the

#### Actions

dropdown menu, click

#### Delete

.

The system prompts you to confirm the delete action.

### 4.  In the field provided, type

#### delete

and click

#### Delete Model

.

#### Models

The selected LLM is deleted from Nutanix Enterprise AI and is no longer displayed on the

page.

#### Calculating a Large Language Model Size

Calculate the size of a Hugging Face LLM imported to Nutanix Enterprise AI.

### About this task

Before you import an LLM using the

Importing a Large Language Model from Hugging Face using Model

URL or ID

on page 195 method, you must calculate the size of the LLM to automatically provision storage for

the LLM. Knowing the model size helps you avoid deployment failures if the cluster cannot meet the resource requirements.

### Procedure

1. Clone the LLM from the Hugging Face model hub to your local repository.

The system creates a folder for the LLM in your working directory.

2. Navigate to the folder that contains the LLM.
3. Check the size of the cloned folder.

The size of the cloned folder is the size of the LLM.

### What to do next

Use this value as the model size when you import a Hugging Face LLM to Nutanix Enterprise AI, provided that the file system on your local machine matches the network file system used for storage when installing Nutanix Enterprise AI.

#### Adding a Proxy Server

You can add a proxy server as an intermediary between Nutanix Enterprise AI and the internet to download models from the Hugging Face Model Hub.

### About this task

To add a proxy server in Nutanix Enterprise AI, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI.

2. 

From the left navigation bar, click

#### Settings

.

The

#### Third Party Credentials

tab is displayed.

3. 

Select the

#### HTTP Proxy

tab.

The

#### Add Proxy Server

dialog box is displayed.

4. 

In the

#### Name

field, enter the name of the proxy server.

5. 

#### Proxy Address

In the

field, enter the IP address of the proxy server.

6. 

#### Port

In the

field, enter the port number of the proxy server.

7. 

#### Username

(Optional) In the

field, enter the username to access the proxy.

8. 

#### Password

(Optional) In the

field, enter the password to access the proxy.

9. 

#### Protocols

In the

section, do one of the following:

- 

Select the

## HTTP

checkbox.

- 

Select the

## HTTPS

checkbox.

- 

Select both checkboxes.

### 10.  Click

#### Save

.

#### HTTP Proxy

The proxy server is displayed in the

tab.

#### Downloading the Logs when a Model Import Fails

You can download the logs of a model if the model import fails.

### About this task

To download the logs of a model, follow these steps:

> [!NOTE]
> Note:   Logs are not retained if the import is successful. If the import fails, the logs are available only for up to 24 hours after the failure.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

#### Models

The

page opens displaying a summary of all the LLMs imported into Nutanix Enterprise AI.

3. Select the model.

### 4.  Click

#### Actions

>

#### Download Logs

.

The logs that are available are downloaded to your machine. By default, a maximum of 10 MB of downloaded logs is retained before log rotation occurs.

#### Fine-Tuning

Fine-Tuning helps you adapt a compatible imported base model for task-specific behavior by using your data source and resource configuration.

#### Models

#### Fine-Tuning

The

page includes a

workflow where you can create, monitor, and manage fine-tuning jobs.

The 2 workflow supports these actions:

- 

Create a fine-tuning job from a compatible base model. For more information, see

Fine-Tuning a Model

on

page 206.

- 

View fine-tuning job details in

#### Overview

and

#### Metrics

tabs. For more information, see

Viewing Fine-Tuning

Job Overview and Metrics

on page 209.

- 

Pause or resume a fine-tuning job.

- 

Downloading the logs of a Fine-Tuned Model

Download fine-tuning logs. For more information, see

on

page 212.

- 

Deleting a Fine-Tuning Job

Delete a fine-tuning job. For more information, see

on page  213.

- 

After job completion, you can import the fine-tuned model and then host endpoints from the imported model.

Current behavior and limits:

- 

Only compatible imported base models are shown for job creation.

- 

#### Supervised Fine-Tuning Low-Rank Adaptation (SFT-LoRA)

Only

is available.

- 

#### Ready

Only data sources marked for fine-tuning and in

state are available.

- 

Only file share output storage is supported for fine-tuned model artifacts.

- 

GPU passthrough accelerators are required for fine-tuning jobs.

#### Fine-Tuning a Model

Fine-Tuning helps you adapt a compatible imported base model for task-specific behavior by using your data source and resource configuration.

### Before you begin

- 

Add a data source. For more information, see

Adding a Data Source for Fine-Tuning Models

on page  208.

- 

A GPU node with CUDA version 13 or later.

- 

GPU passthrough is required for acceleration.

- 

An active GPU license with sufficient capacity is required.

- 

#### Ready

You can fine-tune only compatible imported base models in

state.

- 

An NFS server is needed to save the output model. Ensure that the storage required for the NFS server is equivalent to the size of the base model.

- 

You can only fine-tune the following base models:

- 

meta-llama/Meta-Llama-3.1-8B-Instruct

- 

meta-llama/Llama-3.2-1B-Instruct

- 

meta-llama/Llama-3.2-3B-Instruct

- 

meta-llama/CodeLlama-7b-Instruct-hf

- 

mistralai/Mistral-7B-Instruct-v0.3

- 

google/gemma-2-9b-it

- 

google/gemma-2-2b-it

- 

meta-llama/Llama-Guard-3-8B

- 

allenai/Olmo-3-7B-Instruct

- 

allenai/Olmo-3-7B-Think

- 

mistralai/Ministral-3-3B-Reasoning-2512

- 

mistralai/Ministral-3-3B-Instruct-2512

- 

mistralai/Ministral-3-8B-Reasoning-2512

- 

mistralai/Ministral-3-8B-Instruct-2512

- 

google/gemma-4-E2B-it

- 

You can fine-tune custom models. However, NAI does not validate the resource requirements for custom models or whether the fine-tuning process will succeed.

### About this task

To fine-tune a model, follow these steps:

### Procedure

1. 

Log in to Nutanix Enterprise AI .

2. 

From the left navigation pane, select

#### Models

.

3. 

Do one of the following:

»

Select

#### Fine-Tuning

, then click

#### Fine-Tune Model

.

»

#### Models

#### Actions

#### Fine-Tune Model

From the

list, select a compatible model, then click

>

.

The

#### Basics

tab is displayed.

4. 

From the

#### Base Model

dropdown menu, select the base model you downloaded earlier.

The

#### Base Model

dropdown menu displays only the compatible models. The

#### Method

field is automatically

#### Supervised Fine-Tuning Low-Rank Adaptation (SFT-LoRA)

populated with

.

#### Data Source

5. 

From the

dropdown menu,select a data source that you added earlier.

Adding a Data Source

Data sources are displayed only after you add a data source. For more information, see

for Fine-Tuning Models

on page 208.

6. 

#### Fine-Tuned Model Folder Name

In the

field, enter the name of the folder where the fine-tuned model files

will be stored.

7. 

#### File Server Address

In the

field, enter the FQDN or IP address of the NFS server.

8. 

#### NFS Export Path

, enter the exported directory path on the NFS server where the fine-tuned model is stored.

9. 

#### Next

Click

#### Parameters

The

tab is displayed.

### 10.  In the

#### Random Seed

field, retain the recommended default value or enter a value to initialize the training

process for reproducible results.

### 11.  In the

#### Epochs

field, retain the recommended default value or enter the required value.

### 12.  In the

#### Test Split

field, specify the percentage of the dataset to reserve for model evaluation.

#### Learning Rate

### 13.  In the

field, retain the recommended default value or enter the required value.

#### Maximum Sequence Length

### 14.  In the

field, specify the maximum number of tokens processed in a single

training sequence.

### 15.  In the

#### Batch Size per Step

field, retain the recommended default value or enter the required value.

### 16.  In the

#### Gradient Accumulation Steps

ield, retain the recommended default value or enter the required value.

### 17.  In the

#### Rank

field, retain the recommended default value or enter the required value.

### 18.  In the

#### Alpha

field, retain the recommended default value or enter the required value.

### 19.  Click

#### Next

.

The

#### Resources

tab is displayed.

### 20.  From the

#### Accelerator

dropdown menu, select the GPU to use for fine-tuning.

### 21.  In the

#### Accelerator Count

field, , specify the number of GPUs to allocate.

### 22.  In the

#### vCPUs

field, specify the number of vCPUs to allocate.

### 23.  In the

#### Memory

, specify the memory allocation in GiB.

### 24.  Click

#### Next

.

The

#### Summary

tab is displayed.

25. Review the configuration.

### 26.  Click

#### Start Fine-Tuning

.

The fine-tuning job is created and appears in the

#### Fine-Tuning

list with its current status.

### What to do next

#### 1.  View the fine-tuning job details and metrics. For more information, see

Viewing Fine-Tuning Job Overview

and Metrics

on page 209 .

### 2.  After the fine-tuning job reaches the

#### Ready

state, import the fine-tuned model. For more information, see

Importing Fine-tuned Models

on page 210.

Adding a Data Source for Fine-Tuning Models Add a data source for fine-tuning Models.

### Before you begin

- 

Ensure that the dataset file is in JSONL format because NAI only accepts JSONL files at this time.

- 

Ensure that the content of the file follows the  chat template  format, as NAI adds the dataset only if the content follows the chat template format.

The following is an example of a chat template:

```json
{"messages": [{"role": "user", "content": "What color is the sky?"},
{"role": "assistant", "content": "It is blue."}]}
```

- 

You can add datasets only from Hugging Face or NFS.

For information about supported dataset formats, see

Dataset formats and types

.

- 

Only Data Sources <= 200 MB and <= 50,000 lines are accepted.

### About this task

To add a data source, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Agentic Tools and Data

#### Data Sources

>

.

The system displays the

#### Data Sources

page.

### 3.  Click

#### Add a Data Source

### 4.  In the

#### Name

field, enter a name.

#### 5.  (Optional) To add a dataset from Hugging Face, follow these steps:

a. From the Hugging Face website, copy the repository ID of the dataset.
b. In NAI, from the

#### Source

dropdown menu, select

#### Hugging Face

.

c. In the

#### Dataset URL

field, paste the repository ID of the dataset.

#### 6.  (Optional) To add a dataset from File Share, follow these steps:

a. From the

#### Source

drop-down menu, select

#### File Share

.

b. In the

#### File Server Address

field, enter the FQDN or an IP address.

c. In the

#### NFS Export Path

field, enter the path.

d. In the

#### JSONL File Path

field, enter the path.

e. In the

#### Size

field, enter the size in MB.

Ensure that the file size is either equal to or more than the size displayed in File Share to ensure that files are copied successfully to file shares within the NAI cluster.

#### Add Data Source

### 7.  Click

.

#### Data Sources

The data set is displayed in the

page

### What to do next

After the data source is downloaded, you can select this data source to fine-tune a model. For more information, see

Fine-Tuning a Model

on page  206.

#### Viewing Fine-Tuning Job Overview and Metrics

View the status, configuration, and training metrics of a fine-tuning job.

### Before you begin

You must have created at least one fine-tuning job. For more information, see

Fine-Tuning a Model

on

page 206.

### About this task

Use the fine-tuning job details page to monitor progress and review training parameters. To view fine-tuning job overview and metrics, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

>

#### Fine-Tuning

.

3. Select a fine-tuning job.

The

#### Overview

tab of the fine-tuning job is displayed.

### 4.  In

#### Overview

, review job details and runtime information.

The

#### Overview

tab displays information such as the fine-tuning status, training progress, estimated time

remaining, base model, data source, output location, and resource configuration.

### 5.  Click

#### Metrics

.

The

#### Metrics

tab displays training charts, including Step Loss, Learning Rate, and Grad Normalization. Metrics

for a fine-tuning job are available for only six months from the time the job was created.

6. Review the training metrics to monitor the progress and performance of the fine-tuning job.

#### Importing Fine-tuned Models

Import fine-tuned models into Nutanix Enterprise AI.

### Before you begin

#### 1.  Complete a fine-tuning job. For more information, see

Fine-Tuning a Model

on page 206.

#### 2.  Ensure that the completed fine-tuning job is available in the

#### Fine-Tuning Task Name

list.

### About this task

Import a fine-tuned model from the NFS location where the fine-tuning job saved the model artifacts. To import a fine-tuned model, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

The

#### Models

page is displayed.

### 3.  Click

#### Import Models

>

#### From Fine-Tuning

.

#### Import Fine-Tuned Model

The

page is displayed.

### 4.  From the

#### Fine-Tuning Task Name

list, select a completed fine-tuning task.

When you select a task, the model details and storage location fields are auto-populated. You can review and modify these values before you import the model.

#### 5.  (Optional) To manually update the model and storage details, follow these steps:

a. Click

#### Import Models

>

#### From Fine-Tuning

#### Import Fine-Tuned Model

The

page is displayed.

b. In the

#### Model Instance Name

field, enter a name for the model.

Nutanix recommends using the original model name with a meaningful suffix to distinguish it from other model instances.

c. In the

#### Model Capabilities

field, select the required model capabilities.

d. In the

#### Model Size

field, enter the storage size required to store the downloaded files.

e. (Optional) In the

#### Developer (Optional)

field, enter the name of the model developer.

f. In the

#### File Server Address

field, enter the fully qualified domain name (FQDN) or the IP address of the

NFS server.

g. In the

#### File Server Address

field, enter the fully qualified domain name (FQDN) or the IP address of the

NFS server.

h. In the

#### NFS Export Path

field, enter the NFS export path.

i. In the

#### Directory Path for the Model

field, enter the directory that contains the fine-tuned model.

Nutanix does not validate the NFS configuration in your cluster. An incorrect NFS configuration results in an LLM import failure.

### 6.  Click

#### Upload

.

#### Models

The fine-tuned model is imported and is displayed on the

page.

### What to do next

### 1.  View the status of the import on the

#### Models

page. For more information, see

Viewing Imported Large

Language Models

on page  191.

### 2.  When the imported model status is

#### Ready

, you can create an endpoint.

1. Select the model.

#### 2.  Click

#### Actions

>

#### Create Endpoint

.

#### Create Endpoint

The

action is enabled only for ready models and the models you import. You cannot

deploy models created by other users.

#### 3.  The

#### Create Endpoint

screen is displayed and the

#### Model Instance Name

field displays the imported

model.

4. Create the endpoint.

For more information, see

Creating a Local Endpoint using a non-validated Hugging Face Model

on

Creating a Local Endpoint using a non-catalog NVIDIA NIM

page 226  or

on page  231.

#### Pausing a Fine-Tuning Job

Pause a running or pending fine-tuning job to temporarily stop the training process. You can resume the job later.

### Before you begin

- 

Pausing a fine-tuning job may result in some loss of progress because the job resumes from the last saved checkpoint.

- 

The fine-tuning job must be in tqhe Running or Pending state.

### About this task

To pause a fine-tuning job, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

>

#### Fine-Tuning

.

3. Select a fine-tuning job.

### 4.  Click

#### Actions

#### Pause

>

.

#### Pause

#### Running

#### Pending

The

action is available only for fine-tuning jobs in the

or

state.

### 5.  In the confirmation dialog box, click

#### Pause

.

The status of the fine-tuning job changes to

#### Paused

.

### What to do next

To continue the training process, resume the fine-tuning job. For more information, see

Resuming a Fine-

Tuning Job

on page  212.

#### Resuming a Fine-Tuning Job

Resume a paused fine-tuning job.

### Before you begin

The fine-tuning job must be in the  Paused  state.

### About this task

To resume a fine-tuning job, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

>

#### Fine-Tuning

.

3. Select a paused fine-tuning job.

### 4.  Click

#### Actions

#### Resume

>

.

#### Resume

#### Paused

The

action is available only for fine-tuning jobs in the

state.

### 5.  In the confirmation dialog box, click

#### Resume

.

The fine-tuning job status first changes to Pending and then

#### Running

, and the training process resumes.

### What to do next

Monitor the progress of the fine-tuning job. For more information, see

Viewing Fine-Tuning Job Overview

and Metrics

on page 209.

#### Downloading the logs of a Fine-Tuned Model

Download the logs for a fine-tuning job to troubleshoot training issues or review job execution.

### Before you begin

The fine-tuning job must be in the  Running  or  Failed  state.

### About this task

To download the logs of a fine-tuned model, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

.

3. Select a fine-tuning job.

### 4.  Click

#### Actions

#### Download Logs

>

.

- 

If logs are available, the system downloads the logs for the selected fine-tuning job.

- 

If logs are unavailable, the system displays an error message.

The system downloads available logs for the selected fine-tuning job.

5. Review the downloaded logs to identify errors, warnings, or other information about the fine-tuning job.

#### Deleting a Fine-Tuning Job

Delete a selected fine-tuning job.

### About this task

To delete a fine-tuning job, follow these steps:

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Models

>

#### Fine-Tuning

.

3. Select a fine-tuning job.

### 4.  Click

#### Actions

#### Delete

>

.

A confirmation dialog box is displayed.

### 5.  In the confirmation field, type

#### delete

#### Delete

, and then click

.

The selected fine-tuning job is deleted.