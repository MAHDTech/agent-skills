+++
title = "requirements-and-sizing"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Requirements, Sizing, and Hardware Guidelines

Prerequisites, supported Kubernetes versions, supported NVIDIA GPUs (L40S, H100, H100-NVL, A100, B300, H200, RTX PRO 6000), RWX/RWO storage classes, MetalLB networking, compute and memory sizing matrices, profile-based deployments (c1k_k200, c5k_k1k), component compute and storage requirements, Agent Gateway sizing, and platform limitations.

---

#### Figure 2: Nutanix Enterprise AI Workflow

#### Nutanix Enterprise AI Deployment Types

This section describes the types of Nutanix Enterprise AI deployments. The deployment types that you subscribe to determines which Nutanix Enterprise AI features are available to you.

Nutanix Enterprise AI supports distinct deployment methods to ensure the solution seamlessly integrates with your infrastructure and operational goals:

Nutanix Enterprise AI for GPT-in-a-Box 2.0

This deployment involves deploying Nutanix Enterprise AI Pro on a Nutanix validated solution stack, including Nutanix Cloud Infrastructure (NCI), Nutanix Kubernetes Platform (NKP), Nutanix Database Service (NDB), and Nutanix Unified Storage with 50TiB of Files, on your choice of compatible servers and hardware.

Nutanix Enterprise AI for Bare Metal

This deployment involves deploying Nutanix Enterprise AI Pro on bare metal servers, including hardware and GPU AI accelerators running on Nutanix Kubernetes Platform.

Nutanix Enterprise AI Standalone

This deployment involves deploying Nutanix Enterprise AI Pro on clusters running CNCF (Cloud Native Computing Foundation) compliant Kubernetes on bare metal servers, on-prem VMs, and public clouds such as Amazon EKS, Azure AKS, or Google GKE.

Nutanix Enterprise AI - Private Inference and Agent Gateway Requirements

This section describes the requirements for deploying Nutanix Enterprise AI- private inference and agent gateway.

### General Requirements

- 

Nutanix

For minimum software requirements to deploy Nutanix Enterprise AI in your environment, see

Enterprise AI - Private Inference and Agent Gateway Software Requirements

in  Nutanix Enterprise AI

Release Notes .

- 

Ensure that you deploy a Kubernetes cluster with version 1.35.

You can deploy Nutanix Enterprise AI only on the following Cloud Native Computing Foundation (CNCF) - certified Kubernetes distributions:

- 

Nutanix Kubernetes Platform (NKP)

- 

You can install NAI 2.8 only on NKP 2.18.

- 

Nutanix Enterprise AI supports air-gapped installation only on NKP.

- 

Amazon Elastic Kubernetes Service (EKS)

- 

Azure Kubernetes Service (AKS)

- 

Google Kubernetes Engine (GKE)

- 

Ensure that you configure a worker node pool with the necessary resources required to host an inference endpoint based on the inference engine and the required number of GPUs. When you create an endpoint, the system displays an error message if the worker node pool does not have the required number of GPUs available.

For information on the Kubernetes platforms supported by the NVIDIA GPUs, see  Supported Operating Systems and Kubernetes Platforms  in  NVIDIA GPU Operator documentation .

For information on how to configure a worker node pool on an NKP cluster, see

Configuring Node Pools

in the

Nutanix Kubernetes Platform Guide . For information on the minimum resources required for a worker node pool to host an inference endpoint, see

Sizing Requirements

on page  13.

- 

After you configure a worker node pool with NVIDIA GPUs, ensure that you taint the worker nodes with the key nvidia.com/gpu and the effect NoSchedule, to enable inference pods with matching toleration to be scheduled on the tainted worker node. Any pods that do not require GPUs are not scheduled on the tainted worker node.

- 

Ensure that you deploy the necessary GPUs within a node pool in the Kubernetes cluster that hosts Nutanix Enterprise AI.

Adding GPU Node Pool to a Nutanix Cluster

For information on how to deploy GPUs on an NKP cluster, see

in the Nutanix Kubernetes Platform Guide.

- 

Ensure that you install the NVIDIA GPU Operator to deploy LLMs using NVIDIA GPUs.

Configuring GPU for

For information on how to install the NVIDIA GPU Operator on an NKP cluster, see

Kommander Clusters

in the  Nutanix Kubernetes Platform Guide . For information on how to install the NVIDIA

GPU Operator on a supported public cloud Kubernetes platform, see the  CSP Configurations  section in the

NVIDIA GPU Operator

documentation.

Nutanix recommends that you have one or more AI compatible GPUs deployed in your cluster. Nutanix Enterprise AI supports the following GPUs:

- 

NVIDIA Ada Lovelace L40S

- 

NVIDIA Hopper H100

- 

NVIDIA Hopper H100-NVL

- 

NVIDIA Ampere A100

- 

NVIDIA Blackwell Ultra B300

- 

NVIDIA Hopper H200

- 

NVIDIA RTX PRO 6000 Blackwell Server Edition

> [!NOTE]
> Note:

Nutanix Enterprise AI does not block the deployment of GPU models that are not listed above. However, automatic resource calculation for validated model and GPU combinations is available only for the supported GPU models listed above. Other GPU models are not officially supported and have not been validated by Nutanix. However, if you use an unlisted GPU model, the following limitations apply:

- 

Nutanix does not provide official support for the GPU model.

- 

Nutanix Enterprise AI does not automatically calculate the resource requirements for the model and GPU combination. You must size and validate the deployment manually.

For information on the Kubernetes platforms supported by the NVIDIA GPUs, see  Supported Operating Systems and Kubernetes Platforms  in  NVIDIA GPU Operator documentation .

- 

Ensure that you create a ReadWriteMany (RWX) storage class that enables static or dynamic provisioning of NFS shares for persistent volumes. Configure the RWX storage class with

set to

.

VolumeBindingMode

Immediate

Creating a Storage Class

For more information on how to create a storage class using Nutanix CSI driver, see

for Dynamic NFS Shares

in the  CSI Volume Driver Guide .

- 

Ensure that you configure the  LoadBalancer  service based on your Kubernetes cluster.

For example, if you have an NKP cluster, ensure that you configure the MetalLB service in the cluster. For more information, see

MetalLB

in the  Nutanix Kubernetes Platform Guide .

The  LoadBalancer  service assigns an external IP address to the Envoy Ingress Gateway service in Nutanix Enterprise AI that external clients can use to connect to Nutanix Enterprise AI.

- 

Ensure that you configure a fully qualified domain name (FQDN) on the DNS domain that is accessible to your Kubernetes cluster using the external IP address of the Envoy Ingress Gateway service in Nutanix Enterprise AI.

The FQDN is necessary to connect to Nutanix Enterprise AI.

- 

Ensure that you have a certificate authority (CA) signed TLS certificate for the configured FQDN.

The TLS certificate is necessary to update the Envoy Ingress gateway after you install Nutanix Enterprise AI in the cluster.

> [!NOTE]
> Note:   Nutanix Enterprise AI only supports HTTPS based connections with a valid TLS certificate.

### Sizing Requirements

To deploy Nutanix Enterprise AI on a Kubernetes cluster in your environment, ensure that the cluster meets the minimum requirements. Additional worker nodes are required for your environment and vary based on additional cluster workloads and high-availability requirements. In addition to scaling worker node capacity, you must create a dedicated GPU node pool.

The following table outlines the minimum compute and local storage recommendation validated for a baseline deployment of Nutanix Enterprise AI - private inference and agent gateway.

> [!NOTE]
> Note:   To determine the cloud VM instance type based on the requirements mentioned in the following table, see the respective cloud service provider documentation.

**Table 2: Minimum Compute and Storage Requirements for Deploying Nutanix Enterprise AI-private inference and agent gateway**

| Node Type Description Number | Property 2 | of Nodes (VM) | vCPU per Node | Memory per Node | Storage per Node | Total vCPU | Total Memory |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Control Plane | These control plane nodes are used for running the Kubernetes Control plane. | 3 | 4 | 16 GB | 150 GB | 12 | 48 GB |
| Worker | These worker nodes are for running the NAI Control Plane. | 3 | 10 | 20 GB | 150 GB | 30 | 60 GB |

The node sizing assumes the following environmental conditions:

- 

The NKP workload cluster is dedicated exclusively to NAI and its dependencies.

- 

Worker nodes host only the NAI management plane.

- 

Inference-related resources are excluded from these requirements.

The above setup is validated for 100 API Keys and 15 concurrent users. For power usage, scale up the worker nodes to the appropriate sizes in the table below, then redeploy NAI using the profile name. To deploy on:

- 

NKP:

Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform

on page 46

- 

Deploying Nutanix Enterprise AI on Amazon Elastic Kubernetes Service

EKS:

on page 72

- 

Deploying Nutanix Enterprise AI on Azure Kubernetes Service

AKS :

on page 82

- 

Deploying Nutanix Enterprise AI on Google Kubernetes Engine

GKE:

on page 90

**Table 3: Compute and Storage Requirements for Profile-based deployment of Nutanix Enterprise AI-private inference and agent gateway**

| API Keys Concurrent | Requests | Profile name | Node Type | vCPU per Node | Memory per Node | Storage per Node | Total vCPU | Total Memory |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 200 | 1000 | c1k_k200 | Worker | 60 | 90 GB | 150 GB | 180 | 270 GB |
| 1000 | 5000 | c5k_k1k | Worker | 90 | 150 GB | 150 GB | 270 | 450 GB |

The following table lists the minimum compute requirements for each platform component.

The following table specifies the minimum vCPU resources required for the Model Controller component in millicores. 500m means 500 millicores, which is equivalent to 0.5 vCPU cores.

**Table 4: Minimum Compute Requirements By Component**

| Component | App Name | Minimum vCPU Resources | Minimum Memory Resources | Purpose |
| --- | --- | --- | --- | --- |
| ClickHouse Server | chi-nai-clickhouse- server | 4 | 8 GiB | Observability data store |
| ClickHouse Server | chk-nai-clickhouse- keeper | 100m | 1 GiB | Observability data store |
| IAM | iam-database- bootstrap | 200m | 0.12 GiB | User Management |
| IAM | iam-proxy | 100m | 0.12 GiB | User Management |
| IAM | iam-proxy-control- plane | 100m | 0.0625 GiB | User Management |
| IAM | iam-themis | 200m | 0.0625GiB | User Management |
| IAM | iam-themis- bootstrap | 100m | 0.031 GiB | User Management |
| IAM | iam-ui | 150m | 0.0625 GiB | User Management |
| IAM | iam-user-authn | 100m | 0.015 GiB | User Management |
| API | nai-api | 5.1 | 7.128 GiB | API Endpoint |
| NAI DB Migrator | nai-api-db-migrate | 1 | 1.00 GiB | Upgrade NAI DB |
| DB | nai-db-iep | 2 | 2.00 GiB | Application Data per instance |
| Model Controller | nai-iep-model- controller | 500m | 0.49 GiB | LLM Model Downloader Operator |
| NAI Labs | nai-labs | 500m | 2.00 GiB | Chat & Talk to My Data apps |
| NAI Labs | nai-agent | 1 | 2.00 GiB | Sample Agent App |
| Oauth2-Proxy | nai-oauth2-proxy | 100m | 0.3 GiB | User Management |
| OIDC Client Registration | nai-oidc-client- registration | 1 | 1.00 GiB | User Management |
| Component | App Name | Minimum vCPU Resources | Minimum Memory Resources | Purpose |
| ClickHouse Operator | nai-operators-nai- clickhouse-operator | 500m | 0.25 GiB | Observability data store |
| OTEL Collector | nai-otel-collector- collector | 4 | 2.00 GiB | Metrics exporter from Nodes to ClickHouse |
| Frontend UI | nai-ui | 2 | 2.00 GiB | Frontend |
| IAM & NAI Gateway Cache | nai-valkey | 100 m | 0.0625 GiB | Cache store per instance |
| IAM & NAI Gateway Cache | nai-valkey-sentinel | 25 m | 0.03 GiB | Cache store per instance |
| NAI Gateway | ai-gateway- controller | 200m | 0.25 GiB | Control Plane for AI Gateway Resources |
| NAI Gateway | envoy-gateway | 100m | 0.25 GiB | Control Plane for Envoy-Proxy |
| NAI Gateway | envoy-nai-system- nai-ingress- gateway | 5.1 | 10.32 GiB | Data Plane Proxy |
| NAI Gateway | envoy-ratelimit | 100m | 0.5 GiB | Ratelimit Service |
| Security | nai-securityscan- manager | 200m | 128 MiB | Model and Endpoint scanning |
| NAI | nai-go-processor | 210m | 266 MiB | Batch Inference processor per job |
| NAI | nai-audit-logs- rsyslog-otel- collector | 3 | 1 GiB | Audit Log Exporter |

The following table lists the minimum storage requirements for each platform component.

**Table 5: Minimum Storage Requirements By Component**

| Component | App Name | Minimum Persistent Storage | Access Mode | Purpose |
| --- | --- | --- | --- | --- |
| Database | nai-db-iep-1 | 40 GiB | RWO | Application data stored in PostgreSQL database |
| Model | nai-api | 20 GiB | RWO | Model custom resource used to initialize model storage |
| ClickHouse | chi-nai-clickhouse- server | 50 GiB | RWO | Observability Metrics store |
| Component | App Name | Minimum Persistent Storage | Access Mode | Purpose |
| Valkey | nai-valkey | 8 GiB | RWO | IAM and NAI Gateway cache storage per instance |
| NAI | nai-audit-logs- rsyslog-otel- collector | 1 GiB | RWO | Audit Logs exporter |
| ClickHouse Keeper chk-nai-clickhouse- |  | 10 GiB | RWO | Observability Metrics store |
|  | keeper-chkeeper |  |  |  |
| NAI Labs | nai-labs | 20 GiB | RWO | NAI Labs data |

Nutanix Enterprise AI uses various types of storage for the following purposes:

- 

To save the application and metrics data required to support the Nutanix Enterprise AI platform components.

- 

To download and store pre-validated LLMs required to optimize bootstrapping of LLM inferencing pods during deployment, scaling, and upgrades.

- 

To share LLM files, which enables high availability across multiple instances or replicas of LLM inferencing endpoints.

- 

To manually import downloaded pre-validated or custom LLM Models from existing NFS v4 Share or S3 Compatible Storage.

The following table lists the minimum storage sizing recommendation.

**Table 6: Minimum Storage Sizing Recommendations**

| Component | Minimum Capacity | Storage Technology | Purpose |
| --- | --- | --- | --- |
| RWO (Block - RWO) | 150 GiB | Block | Backend (ClickHouse, PostgreSQL, Valkey, Observability) |
| RWX (NFS - RWX) | 2 TiB | NFS | Pre-validated or custom LLM models |
| S3 Compatible API | 1 TiB | Object | Pre-validated or custom LLM models |

> [!NOTE]
> Note:   If you are upgrading NAI, make sure to expand the Block storage to at least 125#GiB. NAI Agent Gateway only deployments do not require RWS and S3 based storage.

### GPU Requirements

The following table lists the pre-validated LLMs and the minimum number of supported GPUs required to deploy these LLMs.

> [!NOTE]
> Note:   The requirements mentioned in this table are applicable for entry-level deployments. Ensure that you plan your deployment based on your scale and traffic requirements.

**Table 7: Minimum GPU Requirements**

| LLM Provider | LLM Name | GPU Models NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Ai2 | allenai/ Olmo-3-32B- Think | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | allenai/ Olmo-3-7B- Instruct | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | allenai/ Olmo-3-7B- Think | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| AI21 Labs | ai21labs/AI21- Jamba-1.5-Mini | 4 | 2 | 2 | 2 | 1 | 2 | 1 |
| Cross- Encoder | cross-encoder/ ms-marco- MiniLM-L6-v2 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Facebook facebook/deit- |  | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | base-distilled- patch16-224 |  |  |  |  |  |  |  |
| Google | google/ gemma-2-2b-it | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/ gemma-2-9b-it | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/ gemma-3-270m- it | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/ gemma-4-E2B- it | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/ gemma-4-26B- A4B-it | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/ gemma-4-31B-it | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | google/vit-base- patch16-224 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| IBM | ibm-granite/ granite- embedding-107m- multilingual | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| LLM Provider | LLM Name | GPU Models |  |  |  |  |  |  |
|  |  | NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
| Meta | meta-llama/ CodeLlama-7b- Instruct-hf | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ CodeLlama-13b- Instruct-hf | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ CodeLlama-34b- Instruct-hf | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ CodeLlama-70b- Instruct-hf | 4 | 2 | 2 | 2 | 2 | 2 | 1 |
|  | meta-llama/ Llama-2-13b- chat-hf | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ Llama-3.2-11B- Vision-Instruct | 1 | 1 | 1 | 1 | 1 | 1 | Not supported |
|  | meta-llama/ Llama-3.2-1B- Instruct | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ Llama-3.2-3b- Instruct | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta-llama/ Llama-3.2-90B- Vision-Instruct | 4 | 4 | 4 | 4 | 2 | 2 | Not supported |
|  | meta-llama/ Llama-3.3-70B- Instruct | 4 | 2 | 2 | 2 | 2 | 2 | 1 |
|  | meta-llama/ Llama-4- Scout-17B-16E | Not supported | Not supported | 4 | 4 | 2 | 4 | 1 |
|  | meta-llama/ Llama- Guard-3-8B | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | meta- llama/Meta- Llama-3.1-70B- Instruct | 4 | 2 | 2 | 2 | 2 | 2 | 1 |
| LLM Provider | LLM Name | GPU Models |  |  |  |  |  |  |
|  |  | NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
|  | meta- llama/Meta- Llama-3.1-8B- Instruct | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Mistral AI | mistralai/ Devstral- Small-2507 | Not supported | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Magistral- Small-2506 | Not supported | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-14B- Instruct-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-14B- Reasoning-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-3B- Instruct-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-3B- Reasoning-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-8B- Instruct-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Ministral-3-8B- Reasoning-2512 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Mistral-7B- Instruct-v0.3 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Mistral-Nemo- Instruct-2407 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | mistralai/ Mixtral-8x7B- Instruct-v0.1 | 4 | 2 | 2 | 2 | 1 | 2 | 1 |
|  | mistralai/ Mixtral-8x22B- Instruct-v0.1 | Not supported | 4 | 4 | 4 | 4 | 4 | 1 |
|  | mistralai/ Mistral- Small-4-119B-2603 | Not supported | Not supported | Not supported | Not supported | 1 | 2 | 1 |
| LLM Provider | LLM Name | GPU Models |  |  |  |  |  |  |
|  |  | NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
|  | mistralai/ Mistral- Large-3-675B- Instruct-2512 | Not supported | Not supported | Not supported | Not supported | 6 | 8 | 3 |
| NVIDIA | nvidia/NVIDIA- Nemotron-3- Nano-30B-A3B- FP8 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | nvidia/NVIDIA- Nemotron-3- Nano-30B-A3B- BF16 | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | NVIDIA- Nemotron-3- Super-120B- A12B-BF16 | Not supported | Not supported | Not supported | Not supported | 2 | 3 | 1 |
|  | NVIDIA- Nemotron-3- Ultra-550B- A55B-BF16 | Not supported | Not supported | Not supported | Not supported | 9 | 13 | 5 |
| NVIDIA | black-forest- labs/flux.1-dev | Not supported | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
|  | gpt-oss-120b | Not supported | Not supported | 2 | 1 | Not supported | Not supported | Not supported |
|  | gpt-oss-20b | Not supported | Not supported | 1 | 1 | Not supported | Not supported | Not supported |
|  | llama- nemotron- embed-vl-1b-v2 | 1 | 1 | 1 | 1 | 1 |  | Not supported |
|  | llama-3.1-70b- instruct | 4 | Not supported | Not supported | 2 | 1 | Not supported | Not supported |
|  | llama-3.1-8b- instruct | 1 | Not supported | Not supported | 1 | 1 | 1 | Not supported |
|  | llama-3.1- nemoguard-8b- content-safety | 1 | 1 | 1 | 1 | 1 | Not supported | Not supported |
|  | llama-3.1- nemoguard-8b- topic-control | 1 | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
|  | Llama-3.1- nemotron-70b- instruct | Not supported | Not supported | Not supported | 2 | Not supported | Not supported | Not supported |
| LLM Provider | LLM Name | GPU Models |  |  |  |  |  |  |
|  |  | NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
|  | llama-3.1- swallow-8b- instruct-v0.1 | 1 | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
|  | Llama-3.1-8b- instruct-pb24h2 | 1 | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
|  | Llama-3.1-70b- instruct-pb24h2 | Not supported | Not supported | Not supported | 2 | Not supported | Not supported | Not supported |
|  | llama-3.2-nv- embedqa-1b-v2 | 1 | 1 | 1 | 1 | 1 | Not supported | Not supported |
|  | llama-3.2-nv- rerankqa-1b-v2 | 1 | 1 | 1 | 1 | 1 | Not supported | Not supported |
|  | Llama-3.2-90b- vision-instruct | Not supported | Not supported | Not supported | 2 | 1 | Not supported | Not supported |
|  | llama-3.3- nemotron- super-49b-v1 | Not supported | Not supported | 2 | 2 | 1 | Not supported | Not supported |
|  | llama-3.3-70b- instruct | 4 | Not supported | 4 | 4 | 1 | Not supported | Not supported |
|  | Mistral- nemo-12b- instruct | 2 | Not supported | Not supported | 1 | Not supported | Not supported | Not supported |
|  | mistral-7b- instruct-v0.3 | 1 | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
|  | mixtral-8x7b- instruct-v0.1 | 4 | Not supported | Not supported | 2 | 1 | Not supported | Not supported |
|  | nemoretriever- graphic- elements-v1 | 1 | 1 | Not supported | 1 | 1 | Not supported | Not supported |
|  | nemoretriever- ocr-v1 | 1 | Not supported | 1 | 1 | 1 | Not supported | Not supported |
|  | nemoretriever- page-elements- v2 | 1 |  | 1 | 1 | 1 | Not supported | Not supported |
|  | nemoretriever- parse | 1 | Not supported | 1 | 1 | 1 | Not supported | Not supported |
|  | nemoretriever- table-structure- v1 | 1 | Not supported | 1 | 1 | 1 | Not supported | Not supported |
|  | openai/whisper- large-v3 | 1 | Not supported | 1 | 1 | 1 | Not supported | Not supported |
| LLM Provider | LLM Name | GPU Models |  |  |  |  |  |  |
|  |  | NVIDIA L40S-48G | NVIDIA A100-80G | NVIDIA H100-80G | NVIDIA H100 NVL-94G | NVIDIA H 200-141G | NVIDIA RTX PRO 6000-96G | NVIDIA B300-268G |
|  | phi-3-mini-4k- instruct | 1 | Not supported | Not supported | 1 | 1 | Not supported | Not supported |
| OpenAI | openai/gpt- oss-120b | Not supported | 1 | 1 | 1 | 1 | 1 | 1 |
|  | openai/gpt- oss-20b | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | openai/gpt-oss- safeguard-120b | 2 | 1 | 1 | 1 | 1 | 1 | 1 |
|  | openai/gpt-oss- safeguard-20b | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Stability AI | stable-diffusion- v1-5/stable- diffusion-v1-5 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| Unsloth | unsloth/ Llama-3.3-70B- Instruct-bnb-4bit | Not supported | 1 | 1 | 1 | 1 | Not supported | 1 |

The following table lists the minimum resources required to host an inference endpoint based on the inference engine and the number of GPUs.

- 

The minimum worker node pool size must be defined based on the sum of the CPU and memory resources required for all running inference workloads.

- 

The requirements mentioned in this table are suggestive. Deploying larger LLMs requires more resources.

- 

Ensure that you consider the additional resources required by DaemonSets, the Container Storage Interface (CSI) driver, the Container Network Interface (CNI) plugin, and so on.

**Table 8: Minimum GPU Endpoint Requirements**

| Type of Inference Engine | Resource Requirement | Number of GPUs 1 | 2 | 4 | 8 |
| --- | --- | --- | --- | --- | --- |
| NVIDIA NIM | CPU (number of cores) | 12 | 12 | 12 | 12 |
|  | Memory (GiB) | 32 | 32 | 32 | 32 |
| vLLM | CPU (number of cores) | 8 | 8 | 8 | 8 |
|  | Memory (GiB) | 16 | 32 | 64 | 128 |

The following table lists the virtual machine types supported by the GPU model necessary to deploy Nutanix Enterprise AI in public cloud Kubernetes platforms.

> [!NOTE]
> Note:   For the latest virtual machine types supported by a GPU model, see the respective cloud service provider documentation.

**Table 9: GPU Supported Virtual Machine Type**

| Property 1 | Cloud Service Provider Platform Name | Property 3 | GPU Supported Virtual Machine Type | GPU Model |
| --- | --- | --- | --- | --- |
|  | AWS | Amazon Elastic Kubernetes Service (EKS) | EC2 G6e | NVIDIA Ada Lovelace L40S |
|  |  |  | EC2 P5 | NVIDIA Hopper H100 |
|  |  |  | EC2 P4 | NVIDIA Ampere A100 |
|  |  |  | EC2 P5e | NVIDIA Hopper H 200 |
|  |  |  | EC2 p6-b300 | NVIDIA Blackwell B300 |
|  | GCP | Google Kubernetes Engine (GKE) | A3 VM | NVIDIA Hopper H100 |
|  |  |  | A2 VM | NVIDIA Ampere A100 |
|  | Azure | Azure Kubernetes Service (AKS) | NCads_H100_v5-series | NVIDIA Hopper H100 |
|  |  |  | NCCads_H100_v5- series |  |
|  |  |  | NC_A100_v4-series | NVIDIA Ampere A100 |
| Nutanix Enterprise AI - Agent Gateway Requirements |  |  |  |  |

This section describes the requirements for deploying Nutanix Enterprise AI- agent gateway.

### General Requirements

- 

For minimum software requirements to deploy Nutanix Enterprise AI in your environment, see

Nutanix

Enterprise AI - Agent Gateway Software Requirements

in  Nutanix Enterprise AI Release Notes .

- 

Ensure that you deploy a Kubernetes cluster with version 1.35.

You can deploy Nutanix Enterprise AI only on the following Cloud Native Computing Foundation (CNCF) - certified Kubernetes distributions:

- 

Nutanix Kubernetes Platform (NKP)

- 

You can install NAI 2.8 only on NKP 2.18.

- 

Nutanix Enterprise AI supports air-gapped installation only on NKP.

- 

Amazon Elastic Kubernetes Service (EKS)

- 

Azure Kubernetes Service (AKS)

- 

Google Kubernetes Engine (GKE)

- 

For information on how to configure a worker node pool on an NKP cluster, see

Configuring Node Pools

in the

Nutanix Kubernetes Platform Guide .

- 

Ensure that you configure the  LoadBalancer  service based on your Kubernetes cluster.

For example, if you have an NKP cluster, ensure that you configure the MetalLB service in the cluster. For more information, see

MetalLB

in the  Nutanix Kubernetes Platform Guide .

The  LoadBalancer  service assigns an external IP address to the Envoy Ingress Gateway service in Nutanix Enterprise AI that external clients can use to connect to Nutanix Enterprise AI.

- 

Ensure that you configure a fully qualified domain name (FQDN) on the DNS domain that is accessible to your Kubernetes cluster using the external IP address of the Envoy Ingress Gateway service in Nutanix Enterprise AI.

The FQDN is necessary to connect to Nutanix Enterprise AI.

- 

Ensure that you have a certificate authority (CA) signed TLS certificate for the configured FQDN.

The TLS certificate is necessary to update the Envoy Ingress gateway after you install Nutanix Enterprise AI in the cluster.

> [!NOTE]
> Note:   Nutanix Enterprise AI only supports HTTPS based connections with a valid TLS certificate.

### Sizing Requirements

To deploy Nutanix Enterprise AI on a Kubernetes cluster in your environment, ensure that the cluster meets the minimum requirements. Additional worker nodes are required for your environment and vary based on additional cluster workloads and high-availability requirements.

The following table outlines the minimum compute and local storage recommendation validated for a baseline deployment of Nutanix Enterprise AI - agent gateway.

> [!NOTE]
> Note:   To determine the cloud VM instance type based on the requirements mentioned in the following table, see the respective cloud service provider documentation.

**Table 10: Minimum Compute and Storage Requirements for Deploying Nutanix Enterprise AI - agent gateway**

| Node Type Description Number | Property 2 | of Nodes (VM) | vCPU per Node | Memory per Node | Storage per Node | Total vCPU | Total Memory |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Control Plane | These control plane nodes are used for running the Kubernetes Control plane. | 3 | 4 | 16 GB | 150 GB | 12 | 48 GB |
| Worker | These worker nodes are for running the NAI Control Plane. | 3 | 10 | 20 GB | 150 GB | 30 | 60 GB |

The node sizing assumes the following environmental conditions:

- 

The NKP workload cluster is dedicated exclusively to NAI and its dependencies.

- 

Worker nodes host only the NAI management plane.

- 

Inference-related resources are excluded from these requirements.

The above setup is validated for 100 API Keys and 15 concurrent users. For power usage, scale up the worker nodes to the appropriate sizes in the table below, then redeploy NAI using the profile name. To deploy on:

- 

NKP:

Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform

on page 46

- 

EKS:

Deploying Nutanix Enterprise AI on Amazon Elastic Kubernetes Service

on page 72

- 

Deploying Nutanix Enterprise AI on Azure Kubernetes Service

AKS :

on page 82

- 

Deploying Nutanix Enterprise AI on Google Kubernetes Engine

GKE:

on page 90

**Table 11: Compute and Storage Requirements for Profile-based deployment of Nutanix Enterprise AI - agent gateway**

| API Keys Concurrent | Requests | Profile name | Node Type | vCPU per Node | Memory per Node | Storage per Node | Total vCPU | Total Memory |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 200 | 1000 | c1k_k200 | Worker | 60 | 90 GB | 150 GB | 180 | 270 GB |
| 1000 | 5000 | c5k_k1k | Worker | 90 | 150 GB | 150 GB | 270 | 450 GB |

The following table lists the minimum compute requirements for each platform component.

The following table specifies the minimum vCPU resources required for the Model Controller component in millicores. 500m means 500 millicores, which is equivalent to 0.5 vCPU cores.

**Table 12: Minimum Compute Requirements By Component**

| Component | App Name | Minimum vCPU Resources | Minimum Memory Resources | Purpose |
| --- | --- | --- | --- | --- |
| ClickHouse Server | chi-nai-clickhouse- server | 4 | 8 GiB | Observability data store |
| ClickHouse Server | chk-nai-clickhouse- keeper | 100m | 1 GiB | Observability data store |
| IAM | iam-database- bootstrap | 200m | 0.12 GiB | User Management |
| IAM | iam-proxy | 100m | 0.12 GiB | User Management |
| IAM | iam-proxy-control- plane | 100m | 0.0625 GiB | User Management |
| IAM | iam-themis | 200m | 0.0625GiB | User Management |
| IAM | iam-themis- bootstrap | 100m | 0.031 GiB | User Management |
| IAM | iam-ui | 150m | 0.0625 GiB | User Management |
| IAM | iam-user-authn | 100m | 0.015 GiB | User Management |
| Component | App Name | Minimum vCPU Resources | Minimum Memory Resources | Purpose |
| API | nai-api | 5.1 | 7.128 GiB | API Endpoint |
| NAI DB Migrator | nai-api-db-migrate | 1 | 1.00 GiB | Upgrade NAI DB |
| DB | nai-db-iep | 2 | 2.00 GiB | Application Data per instance |
| Model Controller | nai-iep-model- controller | 500m | 0.49 GiB | LLM Model Downloader Operator |
| NAI Labs | nai-labs | 500m | 2.00 GiB | Chat & Talk to My Data apps |
| NAI Labs | nai-agent | 1 | 2.00 GiB | Sample Agent App |
| Oauth2-Proxy | nai-oauth2-proxy | 100m | 0.3 GiB | User Management |
| OIDC Client Registration | nai-oidc-client- registration | 1 | 1.00 GiB | User Management |
| ClickHouse Operator | nai-operators-nai- clickhouse-operator | 500m | 0.25 GiB | Observability data store |
| OTEL Collector | nai-otel-collector- collector | 4 | 2.00 GiB | Metrics exporter from Nodes to ClickHouse |
| Frontend UI | nai-ui | 2 | 2.00 GiB | Frontend |
| IAM & NAI Gateway Cache | nai-valkey | 100 m | 0.0625 GiB | Cache store per instance |
| IAM & NAI Gateway Cache | nai-valkey-sentinel | 25 m | 0.03 GiB | Cache store per instance |
| NAI Gateway | ai-gateway- controller | 200m | 0.25 GiB | Control Plane for AI Gateway Resources |
| NAI Gateway | envoy-gateway | 100m | 0.25 GiB | Control Plane for Envoy-Proxy |
| NAI Gateway | envoy-nai-system- nai-ingress- gateway | 5.1 | 10.32 GiB | Data Plane Proxy |
| NAI Gateway | envoy-ratelimit | 100m | 0.5 GiB | Ratelimit Service |
| Security | nai-securityscan- manager | 200m | 128 MiB | Model and Endpoint scanning |
| NAI | nai-go-processor | 210m | 266 MiB | Batch Inference processor per job |
| NAI | nai-audit-logs- rsyslog-otel- collector | 3 | 1 GiB | Audit Log Exporter |

The following table lists the minimum storage requirements for each platform component.

**Table 13: Minimum Storage Requirements By Component**

| Component | App Name | Minimum Persistent Storage | Access Mode | Purpose |
| --- | --- | --- | --- | --- |
| Database | nai-db-iep-1 | 40 GiB | RWO | Application data stored in PostgreSQL database |
| ClickHouse | chi-nai-clickhouse- server | 50 GiB | RWO | Observability Metrics store |
| Valkey | nai-valkey | 8 GiB | RWO | IAM and NAI Gateway cache storage per instance |
| NAI | nai-audit-logs- rsyslog-otel- collector | 1 GiB | RWO | Audit Logs exporter |
| ClickHouse Keeper chk-nai-clickhouse- |  | 10 GiB | RWO | Observability Metrics store |
|  | keeper-chkeeper |  |  |  |
| NAI Labs | nai-labs | 20 GiB | RWO | NAI Labs data |

Nutanix Enterprise AI - agent gateway uses RWO storage to save the application and metrics data.

The following table lists the minimum storage sizing recommendation.

**Table 14: Minimum Storage Sizing Recommendations**

| Component | Minimum Capacity | Storage Technology | Purpose |
| --- | --- | --- | --- |
| RWO (Block - RWO) | 130 GiB | Block | Backend (ClickHouse, PostgreSQL, Valkey, Observability) |

> [!NOTE]
> Note:   If you are upgrading NAI, make sure to expand the Block storage to at least 105#GiB. NAI Agent Gateway only deployments do not require RWS and S3 based storage.

#### Nutanix Enterprise AI Limitations

The following limitations apply when you deploy Nutanix Enterprise AI at your site.

- 

Nutanix Enterprise AI supports Hugging Face and NVIDIA NIM formatted LLMs only.

- 

Nutanix Enterprise AI supports x86 architecture only.

#### Deploy Nutanix Enterprise AI

Deploy Nutanix Enterprise AI (NAI) on supported Kubernetes platforms and environments.

If you are installing or upgrading Nutanix Enterprise AI, do one of the following:

- 

Deploy NAI on Nutanix Kubernetes Platform (NKP)

Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform

For more information, see

on page  41.
