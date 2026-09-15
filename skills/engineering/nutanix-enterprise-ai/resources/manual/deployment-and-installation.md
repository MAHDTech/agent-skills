# Nutanix Enterprise AI Manual: Deployment and Installation Guide

Nutanix Docker Hub access tokens, Helm chart configuration parameters for nai-operators and nai-core, connected deployment on Nutanix Kubernetes Platform (NKP), air-gapped NKP deployment with private registries, deployment on Amazon EKS, Azure AKS, Google GKE, self-managed PostgreSQL configurations, dashboard IP access, and Docker registry credentials rotation.

---

- 

Deploy NAI on NKP in an air#gapped environment

Deploying Nutanix Enterprise AI on a Nutanix Kubernetes Platform Cluster in

For more information, see

Air-Gapped Environments

on page 50.

- 

Deploy NAI  on Amazon Elastic Kubernetes Service (EKS). For more information, see

Deploy Nutanix

Enterprise AI on Amazon Elastic Kubernetes Service

on page  67.

- 

Deploy NAI on Azure Kubernetes Service (AKS)

For more information, see

Deploying Nutanix Enterprise AI on Azure Kubernetes Service

on page 76.

- 

Deploy NAI on Google Kubernetes Engine (GKE)

For more information, see

Deploy Nutanix Enterprise AI on Google Kubernetes Engine

on page  85.

#### Generating Nutanix Docker Hub Access Tokens

Generate tokens that you can use to access the Nutanix Docker Hub private repository.

### Before you begin

If your environment is air-gapped, you cannot use this procedure. For more information on the procedure for an air-gapped environment, see

Downloading Product Files

.

### About this task

To generate Nutanix Docker Hub Access Tokens, follow these steps:

### Procedure

1. Log in to the Nutanix Support portal.

#### 2.  In the upper-left corner of the Nutanix Support Portal, click the

menu

#### Downloads

icon, then click

.

### 3.  Click

#### Nutanix Enterprise AI

.

### 4.  Click

#### Generate Access Token

.

You can generate a maximum of two tokens.

#### Manage Docker Hub Access Token

The

dialog box appears and generates a token. Once generated, this

token remains valid for all Nutanix Enterprise AI (NAI) releases. You only need to generate a replacement if the existing token is manually deleted or revoked.

### What to do next

For information on how to use your tokens to download and install Nutanix Enterprise AI, see

Deploy

Nutanix Enterprise AI

on page 27.

#### Nutanix Enterprise AI Configuration Parameters for the

#### Helm Chart

#### nai-operators

The following tables list the configurable parameters of the

```bash
helm chart and their default
nai-operators
```

values.

**Table 15: Global Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| imagePullSecrets | List of Secrets for Container Registry hosting NAI Images |  |

**Table 16: Global Storage Parameters**

| Key | Description | Property 3 | Default Value |
| --- | --- | --- | --- |
| global.storage.storageClassName Default RWO storage class used |  |  | nutanix-volume |
|  | by persistent components. For |  |  |
|  | nai-operators | , this is used by |  |
|  | CNPG cluster and Valkey PVCs when a cluster-specific storage class is empty. |  |  |

**Table 17: AI Gateway Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| ai-gateway- helm.extProc.image.repository | Repository for extProc image | docker.io/nutanix/nai-ai-gateway- extproc |
| ai-gateway- helm.extProc.image.tag | Tag for extProc image | bc729717 |
| ai-gateway- helm.controller.image.repository | Repository for controller image | docker.io/nutanix/nai-ai-gateway- controller |
| ai-gateway- helm.controller.image.tag | Tag for controller image | bc729717 |

**Table 18: Valkey Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| naiValkey.image.name | Valkey image name | docker.io/nutanix/nai-valkey |
| naiValkey.image.tag | Valkey image tag | 9.1.0 |
| naiValkey.image.pullPolicy | Image pull policy for Valkey | IfNotPresent |
| naiValkey.replicaCount | Number of Valkey data nodes | 1 |
| naiValkey.sentinel.replicaCount | Number of Valkey Sentinel instances | 1 |
| naiValkey.sentinel.resources.limits.cpu | Sentinel CPU limit | 50m |
| naiValkey.sentinel.resources.limits.memory | Sentinel memory limit | 64Mi |
| naiValkey.sentinel.resources.requests.cpu | Sentinel CPU request | 25m |
| naiValkey.sentinel.resources.requests.memory | Sentinel memory request | 32Mi |
| naiValkey.sentinel.topologySpread.whenUnsatisfiable | Sentinel topology spread scheduling policy | DoNotSchedule |
| naiValkey.sentinel.pdb.enabled | Enable Pod Disruption Budget for Sentinel pods | false |
| naiValkey.sentinel.pdb.minAvailableMinimum number of Sentinel pods |  |  |
|  | that must remain available |  |
| naiValkey.persistence.size | Persistent storage size for Valkey data | 4Gi |
| Key | Description | Default Value |
| naiValkey.persistence.storageClassStorage class for Valkey |  |  |
|  | persistent volumes. Empty falls back to global.storage.storageClassName, then the cluster default StorageClass. |  |
| naiValkey.topologySpread.whenUnsatisfiable | Valkey data pod topology spread scheduling policy | DoNotSchedule |
| naiValkey.pdb.enabled | Enable Pod Disruption Budget for Valkey data pods | false |
| naiValkey.pdb.minAvailable | Minimum number of Valkey data pods that must remain available |  |
| naiValkey.resources.limits.cpu | Valkey CPU limit | 200m |
| naiValkey.resources.limits.memory Valkey memory limit |  | 256Mi |
| naiValkey.resources.requests.cpu Valkey CPU request |  | 100m |
| naiValkey.resources.requests.memory | Valkey memory request | 64Mi |

**Table 19: Clickhouse Operators Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| nai-clickhouse- operator.operator.image.registry | Container image name or tag | docker.io |
| nai-clickhouse- operator.operator.image.repository | Container image name or tag | nutanix/nai-clickhouse-operator |
| nai-clickhouse- operator.operator.image.tag | Container image name or tag | 0.24.2 |
| nai-clickhouse- operator.metrics.image.registry | Container image name or tag | docker.io |
| nai-clickhouse- operator.metrics.image.repository | Container image name or tag | nutanix/nai-clickhouse-metrics- exporter |
| nai-clickhouse- operator.metrics.image.tag | Container image name or tag | 0.24.2 |

**Table 20: NAI Database Parameters**

| Key | Description | Property 3 | Property 4 | Property 5 | Property 6 | Default Value |
| --- | --- | --- | --- | --- | --- | --- |
| naiDatabase.external | Use an externally managed PostgreSQL database instead of provisioning CNPG clusters. The same value should be supplied to both |  |  |  |  | false |
|  |  |  | nai-operators | and | nai- |  |
|  | core | . |  |  |  |  |
| Key | Description |  |  |  |  | Default Value |
| naiDatabase.image | PostgreSQL operand image used by CloudNativePG |  |  |  |  | docker.io/nutanix/nai- postgresql:17.10-standard-trixie |
| naiDatabase.postgresql.parameters.shared_buffers | PostgreSQL shared buffer size |  |  |  |  | 1GB |
| naiDatabase.postgresql.parameters.work_mem | PostgreSQL per-operation work memory |  |  |  |  | 8MB |
| naiDatabase.postgresql.parameters.idle_in_transaction_session_timeout | Maximum time an idle transaction can remain open |  |  |  |  | 5min |
| naiDatabase.postgresql.parameters.idle_session_timeout | Maximum time an idle database session can remain inactive |  |  |  |  | 5min |
| naiDatabase.affinity.enablePodAntiAffinity | Enable PostgreSQL pod anti- affinity |  |  |  |  | true |
| naiDatabase.affinity.podAntiAffinityType | PostgreSQL pod anti-affinity policy |  |  |  |  | required |
| naiDatabase.affinity.topologyKey | Topology key used for PostgreSQL pod anti-affinity |  |  |  |  | kubernetes.io/hostname |
| naiDatabase.enablePDB | Enable the CNPG-managed Pod Disruption Budget |  |  |  |  | true |
| naiDatabase.clusters.iep.clusterName | Name of the CNPG cluster for the NAI application database |  |  |  |  | nai-db-iep |
| naiDatabase.clusters.iep.database Application database name |  |  |  |  |  | nai_iep |
| naiDatabase.clusters.iep.username Application database username |  |  |  |  |  | nai-api-user |
| naiDatabase.clusters.iep.password Application database password |  |  |  |  |  | nai-api-password |
| naiDatabase.clusters.iep.sslMode SSL mode for NAI client |  |  |  |  |  | disable |
|  | connections to PostgreSQL |  |  |  |  |  |
| naiDatabase.clusters.iep.sslRootCertName | Client CA certificate file name |  |  |  |  |  |
| naiDatabase.clusters.iep.sslClientCertName | Client certificate file name |  |  |  |  |  |
| naiDatabase.clusters.iep.sslClientKeyName | Client private key file name |  |  |  |  |  |
| naiDatabase.clusters.iep.host | PostgreSQL read-write host advertised to NAI clients |  |  |  |  | nai-db-iep-rw.nai-system |
| naiDatabase.clusters.iep.port | PostgreSQL port |  |  |  |  | 5432 |
| naiDatabase.clusters.iep.instances Number of PostgreSQL instances |  |  |  |  |  | 1 |
|  | in the CNPG cluster |  |  |  |  |  |
| naiDatabase.clusters.iep.maxConnections | Maximum PostgreSQL connections |  |  |  |  | 1000 |
| naiDatabase.clusters.iep.synchronous.enabled | Enable synchronous PostgreSQL replication |  |  |  |  | false |
| naiDatabase.clusters.iep.synchronous.method | Synchronous replication selection method |  |  |  |  | any |
| naiDatabase.clusters.iep.synchronous.number | Number of synchronous standbys required |  |  |  |  | 1 |
| naiDatabase.clusters.iep.synchronous.dataDurability | Synchronous replication durability policy |  |  |  |  | preferred |
| Key | Description |  |  |  |  | Default Value |
| naiDatabase.clusters.iep.resources.requests.cpu | PostgreSQL CPU request |  |  |  |  | 2 |
| naiDatabase.clusters.iep.resources.requests.memory | PostgreSQL memory request |  |  |  |  | 2Gi |
| naiDatabase.clusters.iep.resources.limits.cpu | PostgreSQL CPU limit |  |  |  |  | 4 |
| naiDatabase.clusters.iep.resources.limits.memory | PostgreSQL memory limit |  |  |  |  | 4Gi |
| naiDatabase.clusters.iep.storage.size | Persistent storage size for PostgreSQL |  |  |  |  | 20Gi |
| naiDatabase.clusters.iep.storage.storageClass | Storage class for PostgreSQL persistent volumes. Empty falls back to global.storage.storageClassName. |  |  |  |  |  |
| naiDatabase.clusters.iep.import.typeType of legacy PostgreSQL |  |  |  |  |  | monolith |
|  | import |  |  |  |  |  |
| naiDatabase.clusters.iep.import.sourceHost | Hostname of the legacy standalone PostgreSQL database used for import |  |  |  |  | nai-db |
| naiDatabase.clusters.iep.import.sourcePort | Port of the legacy PostgreSQL database |  |  |  |  | 5432 |
| naiDatabase.clusters.iep.import.databases | Databases imported from the legacy PostgreSQL instance |  |  |  |  | nai_iep, nai_iam |
| naiDatabase.clusters.iep.import.roles | PostgreSQL roles imported from the legacy instance |  |  |  |  | nai-api-user |
| naiDatabase.clusters.iep.import.sourceDatabase | Database used by the import session on the source PostgreSQL instance |  |  |  |  | nai_iep |
| naiDatabase.clusters.iam.inCluster CNPG cluster hosting the IAM |  |  |  |  |  | nai-db-iep |
|  | database |  |  |  |  |  |
| naiDatabase.clusters.iam.database IAM database name |  |  |  |  |  | nai_iam |
| naiDatabase.clusters.iam.usernameIAM database username |  |  |  |  |  | nai-api-user |
| naiDatabase.clusters.iam.passwordIAM database password |  |  |  |  |  | nai-api-password |
| naiDatabase.clusters.iam.sslMode SSL mode for IAM client |  |  |  |  |  | disable |
|  | connections to PostgreSQL |  |  |  |  |  |
| naiDatabase.clusters.iam.sslRootCertName | IAM client CA certificate file name |  |  |  |  |  |
| naiDatabase.clusters.iam.sslClientCertName | IAM client certificate file name |  |  |  |  |  |
| naiDatabase.clusters.iam.sslClientKeyName | IAM client private key file name |  |  |  |  |  |
| naiDatabase.clusters.iam.host | PostgreSQL read-write host used by IAM services |  |  |  |  | nai-db-iep-rw.nai-system |
| naiDatabase.clusters.iam.port | PostgreSQL port used by IAM services |  |  |  |  | 5432 |

**Table 21: NAI Jobs Parameters**

| Property 1 | Key | Description | Property 4 | Default Value | Property 6 |
| --- | --- | --- | --- | --- | --- |
|  | naiJobs.naiJobsImage.image | Job service image |  | docker.io/nutanix/nai-jobs |  |
|  | naiJobs.naiJobsImage.tag | Job service image tag |  | v2.8.0 |  |
|  | naiJobs.resources.limits.cpu | CPU limit for nai-jobs container |  | 1 |  |
|  | naiJobs.resources.limits.memory | Memory limit for nai-jobs container |  | 1Gi |  |
|  | naiJobs.resources.requests.cpu | CPU request for nai-jobs container |  | 100m |  |
|  | naiJobs.resources.requests.memoryMemory request for nai-jobs |  |  | 50Mi |  |
|  |  | container |  |  |  |
| Nutanix Enterprise AI Configuration Parameters for the |  |  | nai-core |  | Helm chart |

The following tables lists the configurable parameters of the

Helm chart and their default values.

```bash
nai-core
```

**Table 22: Global Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| imagePullSecrets | List of Secrets for Container Registry hosting NAI Images |  |

**Table 23: Clickhouse keeper**

| Key | Description | Default Value |
| --- | --- | --- |
| nai-clickhouse- keeper.clickhouseKeeper.image.registry | Clickhouse keeper registry | docker.io |
| nai-clickhouse- keeper.clickhouseKeeper.image.repository | Clickhouse keeper repository | nutanix/nai-clickhouse-keeper |
| nai-clickhouse- keeper.clickhouseKeeper.image.tag | Clickhouse keeper image tag | 25.8.17.37 |
| nai-clickhouse- keeper.clickhouseKeeper.storage.storageClass | Clickhouse keeper storage class | nutanix-volume |

**Table 24: Clickhouse Server**

| Key | Description | Default Value |
| --- | --- | --- |
| nai-clickhouse- server.clickhouse.image.registry | Container image name or tag | docker.io |
| nai-clickhouse- server.clickhouse.image.repository | Container image name or tag | nutanix/nai-clickhouse-server |
| Key | Description | Default Value |
| nai-clickhouse- server.clickhouse.image.tag | Container image name or tag | 25.8.17.37 |
| nai-clickhouse- server.clickhouse.initContainers.addUdf.image.registry | Container image name or tag | docker.io |
| nai-clickhouse- server.clickhouse.initContainers.addUdf.image.repository | Container image name or tag | nutanix/nai-clickhouse-udf |
| nai-clickhouse- server.clickhouse.initContainers.addUdf.image.tag | Container image name or tag | v2.8.0 |
| nai-clickhouse- server.clickhouse.initContainers.waitForKeeper.image.registry | Container image name or tag | docker.io |
| nai-clickhouse- server.clickhouse.initContainers.waitForKeeper.image.repository | Container image name or tag | nutanix/nai-jobs |
| nai-clickhouse- server.clickhouse.initContainers.waitForKeeper.image.tag | Container image name or tag | v2.8.0 |
| nai-clickhouse- server.clickhouse.resources.limits.cpu | CPU resource specification |  |
| nai-clickhouse- server.clickhouse.resources.limits.memory | Memory resource specification | 8Gi |
| nai-clickhouse- server.clickhouse.resources.requests.cpu | CPU resource specification |  |
| nai-clickhouse- server.clickhouse.resources.requests.memory | Memory resource specification | 8Gi |
| nai-clickhouse- server.clickhouse.storage.pvcStorage | Persistent volume claim size | 50Gi |
| nai-clickhouse- server.clickhouse.storage.storageClass | Storage class name | nutanix-volume |
| nai-clickhouse- server.clickhouse.users.admin.password | Password value | nai-clickhouse-password |
| nai-clickhouse- server.clickhouse.users.admin.username | Username value | nai-clickhouse-user |

**Table 25: Storage Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| defaultStorageClassName | Storage class name to be used by nai-db and ClickHouse server. |  |

**Table 26: NAI IEP Operator Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| naiIepOperator.iepOperatorImage.image | IEP operator image name | docker.io/nutanix/nai-iep-operator |
| Key | Description | Default Value |
| naiIepOperator.iepOperatorImage.tag | IEP operator image tag | v2.8.0 |
| naiIepOperator.iepOperatorResources.limits.cpu | IEP operator CPU limits | 500m |
| naiIepOperator.iepOperatorResources.limits.memory | IEP operator memory limits | 500Mi |
| naiIepOperator.iepOperatorResources.requests.cpu | IEP operator CPU requests | 100m |
| naiIepOperator.iepOperatorResources.requests.memory | IEP operator memory requests | 100Mi |
| naiIepOperator.modelProcessorImage.image | Model Processor image name | docker.io/nutanix/nai-python- processor |
| naiIepOperator.modelProcessorImage.tag | Model Processor image tag | v2.8.0 |
| naiIepOperator.modelProcessorResources.limits.cpu | Model Processor CPU limits |  |
| naiIepOperator.modelProcessorResources.limits.memory | Model Processor memory limits |  |
| naiIepOperator.modelProcessorResources.requests.cpu | Model Processor CPU requests |  |
| naiIepOperator.modelProcessorResources.requests.memory | Model Processor memory requests |  |
| naiIepOperator.retainModelProcessorJob | Determines whether the model processor job must be retained after completion. | false |
| naiIepOperator.dataSourceProcessorImage.image | Data Source Processor Image name | docker.io/nutanix/nai-python- processor |
| naiIepOperator.dataSourceProcessorImage.tag | Data source Processor image tag | v2.8.0 |
| naiIepOperator.dataSourceProcessorResources.limits.cpu / Data source Processor | CPU limits |  |
| naiIepOperator.dataSourceProcessorResources.limits.memory | Data source Processor memory limits |  |
| naiIepOperator.dataSourceProcessorResources.requests.cpu | Data source Processor CPU requests |  |
| naiIepOperator.dataSourceProcessorResources.requests.memory | Data source Processor memory requests |  |
| naiIepOperator.finetuneProcessorImage.image | Finetune Processor image name | docker.io/nutanix/nai-finetuning |
| naiIepOperator.finetuneProcessorImage.tag | Finetune Processor image tag | v2.8.0 |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.image | Batch Inference Processor image name | docker.io/nutanix/nai-go- processor |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.tag | Batch Inference Processor image tag | v2.8.0 |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.resources.limits.cpu | Batch Inference Processor  CPU limits | 200m |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.resources.limits.memory | Batch Inference Processor  memory limits | 256Mi |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.resources.requests.cpu | Batch Inference Processor  CPU limits | 200m |
| naiIepOperator.batchInferenceProcessorImage.containers.processor.resources.requests.memory | Batch Inference Processor  CPU memory | 256Mi |
| Key | Description | Default Value |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.image | Batch Inference Status Provider image name | docker.io/nutanix/nai-go- processor |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.tag | Batch Inference Status Provider image tag | v2.8.0 |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.resources.limits.cpu | Batch Inference Status Provider CPU limits | 200m |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.resources.limits.memory | Batch Inference Status Provider CPU memory | 256Mi |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.resources.requests.cpu | Batch Inference Status Provider CPU limits | 200m |
| naiIepOperator.batchInferenceProcessorImage.containers.statusProvider.resources.requests.memory | Batch Inference Status Provider CPU memory | 256Mi |

**Table 27: NAI Inference UI Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| naiInferenceUi.naiUiImage.image | The name of the NAI UI Docker image | docker.io/nutanix/nai-inference-ui |
| naiInferenceUi.naiUiImage.tag | The tag of the Docker image to be used | v2.8.0 |
| naiInferenceUi.resources.requests.cpu | NAI UI container CPU requests | 1 |
| naiInferenceUi.resources.requests.memory | NAI UI container memory requests | 1Gi |
| naiInferenceUi.resources.limits.cpu NAI UI container CPU limits |  | 2 |
| naiInferenceUi.resources.limits.memory | NAI UI container memory limits | 2Gi |

**Table 28: NAI API Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| naiApi.naiApiImage.image | The name of the NAI API Docker image | docker.io/nutanix/nai-api |
| naiApi.naiApiImage.tag | The tag of the Docker image to be used | v2.8.0 |
| naiApi.naiApiResources.requests.cpu | NAI API container CPU requests | 2 |
| naiApi.naiApiResources.requests.memory | NAI API container memory requests | 2Gi |
| naiApi.naiApiResources.limits.cpu NAI API container CPU limits |  | 8 |
| naiApi.naiApiResources.limits.memory | NAI API container memory limits | 4Gi |
| naiApi.naiMigrateInitContainerResources.requests.cpu | NAI API migrate init container CPU requests | 100m |
| Key | Description | Default Value |
| naiApi.naiMigrateInitContainerResources.requests.memory | NAI API migrate init container memory requests | 50Mi |
| naiApi.naiMigrateInitContainerResources.limits.cpu | NAI API migrate init container CPU limits | 1 |
| naiApi.naiMigrateInitContainerResources.limits.memory | NAI API migrate init container memory limits | 1Gi |
| naiApi.naiMigrateJobResources.requests.cpu | NAI API migrate job container CPU requests | 1 |
| naiApi.naiMigrateJobResources.requests.memory | NAI API migrate job container memory requests | 1Gi |
| naiApi.naiMigrateJobResources.limits.cpu | NAI API migrate job container CPU limits | 1 |
| naiApi.naiMigrateJobResources.limits.memory | NAI API migrate job container memory limits | 1Gi |
| naiApi.storageClassName | Storage class name to be used nai-iep for storing models | nai-nfs-storage |
| naiApi.supportedTGIImage | Supported TGI Runtime Image | docker.io/nutanix/nai-tgi |
| naiApi.supportedTGIImageTag | Supported TGI Runtime Image tag | 3.3.4-b2485c9 |
| naiApi.replicaCount | Number of instances of nai-api | 1 |

**Table 29: NAI Database Parameters**

| Key | Description | Property 3 | Property 4 | Property 5 | Property 6 | Property 7 | Property 8 | Default Value |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| naiDatabase.external | Use an externally managed PostgreSQL database. When true, |  |  |  |  |  |  | false |
|  |  | nai-operators |  |  |  | does not |  |  |
|  | provision CNPG clusters. Pass the same value to both charts. |  |  |  |  |  |  |  |
| naiDatabase.clientImage | PostgreSQL client image containing |  |  |  |  |  |  | docker.io/nutanix/nai- postgresql:17.10-standard-trixie |
|  |  |  | psql | and |  | pg_isready | , |  |
|  | used by the IAM database bootstrap job |  |  |  |  |  |  |  |
| naiDatabase.storageSpec.resources.requests.storage | Persistent storage size for the legacy standalone PostgreSQL data PVC ( |  |  |  |  |  |  | 4Gi |
|  |  |  | nai-db |  | ). The value |  |  |  |
|  | must match the existing PVC size during migration. |  |  |  |  |  |  |  |
| naiDatabase.clusters.iep.database Application database name used |  |  |  |  |  |  |  | nai_iep |
|  | by NAI core services |  |  |  |  |  |  |  |
| naiDatabase.clusters.iep.port | PostgreSQL port for the application database |  |  |  |  |  |  | 5432 |
| naiDatabase.clusters.iep.host | Read-write PostgreSQL host for the application database |  |  |  |  |  |  | nai-db-iep-rw.nai-system |
| Key | Description |  |  |  |  |  |  | Default Value |
| naiDatabase.clusters.iep.sslMode SSL mode for application |  |  |  |  |  |  |  | disable |
|  | database client connections |  |  |  |  |  |  |  |
| naiDatabase.clusters.iep.sslSecretName | Kubernetes Secret containing client TLS certificate material when required |  |  |  |  |  |  | nai-db-certs |
| naiDatabase.clusters.iep.sslRootCertName | Client CA certificate file name |  |  |  |  |  |  |  |
| naiDatabase.clusters.iep.sslClientCertName | Client certificate file name |  |  |  |  |  |  |  |
| naiDatabase.clusters.iep.sslClientKeyName | Client private key file name |  |  |  |  |  |  |  |
| naiDatabase.clusters.iam.database IAM database name used by IAM |  |  |  |  |  |  |  | nai_iam |
|  | services |  |  |  |  |  |  |  |
| naiDatabase.clusters.iam.port | PostgreSQL port for the IAM database |  |  |  |  |  |  | 5432 |
| naiDatabase.clusters.iam.host | Read-write PostgreSQL host for the IAM database |  |  |  |  |  |  | nai-db-iep-rw.nai-system |
| naiDatabase.clusters.iam.sslMode SSL mode for IAM database client |  |  |  |  |  |  |  | disable |
|  | connections |  |  |  |  |  |  |  |
| naiDatabase.clusters.iam.sslSecretName | Kubernetes Secret containing client TLS certificate material when required |  |  |  |  |  |  | nai-db-certs |
| naiDatabase.clusters.iam.sslRootCertName | IAM client CA certificate file name |  |  |  |  |  |  |  |
| naiDatabase.clusters.iam.sslClientCertName | IAM client certificate file name |  |  |  |  |  |  |  |
| naiDatabase.clusters.iam.sslClientKeyName | IAM client private key file name |  |  |  |  |  |  |  |

> [!NOTE]
> Note:

- 

The

chart owns the database credentials Secrets.

contains the non-secret

```bash
nai-operators
nai-core
```

connection topology used to render service configuration.

- 

For an internal CNPG deployment, the

and

databases are co-located in the

```bash
iep
iam
nai-db-iep
```

CNPG cluster by default.

- 

For an external PostgreSQL deployment, configure

consistently in both

```bash
naiDatabase.external
```

charts. Configure the corresponding database connection details in both charts as described earlier.

- 

The

chart automatically detects legacy PostgreSQL imports using Helm lookup. Use

```bash
nai-operators
```

the

setting only for advanced or break-glass scenarios.

```bash
importOverride
```

- 

Component-specific

values are intentionally empty by default so that they inherit

```bash
storageClass
```

.

```bash
global.storage.storageClassName
```

**Table 30: NAI Monitoring Parameters**

| Key | Description | Default Value |
| --- | --- | --- |
| naiMonitoring.opentelemetry.collectorImage | The name and tag of the Target Collector Docker image | docker.io/nutanix/nai- opentelemetry-collector- contrib:0.141.0 |
| naiMonitoring.opentelemetry.targetAllocator.image.repository | The name of the Target Allocator Docker image. | docker.io/nutanix/nai-target- allocator |
| naiMonitoring.opentelemetry.targetAllocator.image.tag | The tag of the Target Allocator Docker image | 0.141.0 |
| naiMonitoring.opentelemetry.targetAllocator.resources.requests.cpu | Target Allocator container CPU requests | 0.5 |
| naiMonitoring.opentelemetry.targetAllocator.resources.requests.memory | Target Allocator container memory request | 100Mi |
| naiMonitoring.opentelemetry.targetAllocator.resources.limits.cpu | Target Allocator container CPU limits | 1 |
| naiMonitoring.opentelemetry.targetAllocator.resources.limits.memory | Target Allocator container memory limits | 500Mi |
| naiMonitoring.opentelemetry.storageClassName | Storage class name to be used by Opentelemetry components | nai-nfs-storage |
| naiMonitoring.opentelemetry.common.resources.requests.cpu | Opentelemetry container CPU requests | 0.1 |
| naiMonitoring.opentelemetry.common.resources.requests.memory | Opentelemetry container memory requests | 500Mi |
| naiMonitoring.opentelemetry.common.resources.limits.cpu | Opentelemetry container CPU limits | 4 |
| naiMonitoring.opentelemetry.common.resources.limits.memory | Opentelemetry container memory limits | 2Gi |
| naiMonitoring.opentelemetry.common.storageSpec.resources.requests.storage | Storage spec for Opentelemetry data | 1Gi |
| naiMonitoring.opentelemetry.rsyslogAuditLogsExport.resources.requests.cpu | Rsyslog container CPU requests | 2 |
| naiMonitoring.opentelemetry.rsyslogAuditLogsExport.resources.requests.memory | Rsyslog container memory requests | 450Mi |
| naiMonitoring.opentelemetry.rsyslogAuditLogsExport.resources.limits.cpu | Rsyslog container CPU limits | 3 |
| naiMonitoring.opentelemetry.rsyslogAuditLogsExport.resources.limits.memory | Rsyslog container memory limits | 1Gi |
| naiMonitoring.opentelemetry.rsyslogAuditLogsExport.storageSpec.resources.requests.storage | Storage spec for RsysLog data | 1Gi |
| naiMonitoring.nodeExporter.serviceMonitor.enabled | Enable node exporter service monitor | false |
| naiMonitoring.nodeExporter.serviceMonitor.namespaceSelector.matchNames[0] | Namespace in which Kube- Prometheus-Stack is installed | prometheus |
| naiMonitoring.dcgmExporter.podLevelMetrics | Enable DCGM exporter pod-level metrics | false |
| naiMonitoring.dcgmExporter.serviceMonitor.enabled | Enable DCGM exporter service monitor | false |
| naiMonitoring.dcgmExporter.serviceMonitor.namespaceSelector.matchNames[0] | Namespace in which NVIDIA GPU Operator is installed | gpu-operator |

**Table 31: NAI Labs**

| Key | Description | Value |
| --- | --- | --- |
| naiLabs.labsImage.image | The name of the NAI Labs Docker image. | docker.io/nutanix/nai-rag-app |
| naiLabs.labsImage.tag | The tag of the Docker image to be used | v2.8.0 |
| naiLabs.resources.requests.memory | NAI Labs memory requests | 2Gi |
| naiLabs.resources.requests.cpu | NAI Labs container CPU requests 500m |  |
| naiLabs.resources.limits.memory | NAI Labs memory limits | 6Gi |
| naiLabs.resources.limits.cpu | NAI Labs container CPU limits | 1500m |
| naiLabs.storageSpec.resources.requests.storage | Storage spec for NAI Labs data | 20Gi |
| naiLabs.vectorDb.external | Set external: true, to use an external Milvus database instead of embedded ChromaDB | false |
| naiLabs.vectorDb.milvus.host | Milvus server hostname or IP address |  |
| naiLabs.vectorDb.milvus.port | Milvus server port (self hosted Milvus uses 19530, Zilliz Cloud uses 443) | 19530 |
| naiLabs.vectorDb.milvus.token | Authentication token (leave empty if not required) |  |
| naiLabs.vectorDb.milvus.milvusTLSEnabled | TLS Configuration (one-way TLS only). | false |
|  | Set to true if your Milvus server requires TLS/SSL. |  |

> [!NOTE]
> Note:   Only one-way TLS is supported (client verifies server certificate)

naiLabs.vectorDb.milvus.milvusTLSSecretName

K8s secret containing the CA certificate for SSL connection to be created in the nai-system namespac

nai-milvus-tls-certs

naiLabs.vectorDb.milvus.milvusTLSCACertName

CA certificate key name within the secret . For example, "ca.pem".

Leave empty if your Milvus uses a publicly trusted CA (e.g., Let's Encrypt, Zilliz Cloud)

naiLabs.vectorDb.milvus.milvusTLSServerName

Hostname for TLS certificate verification (optional).

If left blank, defaults to the host value above for hostname verification

Set this if the certificate's CN/SAN doesn't match the host value.

For example, host is an IP address (10.111.48.98) but certificate is issued for a hostname (milvus.example.com)

naiLabs.enabled

Flag to deploy Chat and Talk to Data app

false

**Table 32: AI Gateway**

| Key | Description | Default Value |
| --- | --- | --- |
| gateway.replicaCount | Number of instances of nai- ingress-gateway | 1 |

**Table 33: NAI Agent**

| Property 1 | naiAgent.enabled | Enable the sample NAI Agent app true | Property 4 |
| --- | --- | --- | --- |
|  | naiAgent.agentImage.image | NAI Agent app image name | docker.io/nutanix/nai-agent-app |
|  | naiAgent.agentImage.tag | NAI Agent app image tag | v2.8.0 |
|  | naiAgent.resources.requests.memory | NAI Agent app memory requests | 2Gi |
|  | naiAgent.resources.requests.cpu | NAI Agent app CPU requests | 1000m |
|  | naiAgent.resources.limits.memory NAI Agent app memory limits |  | 4Gi |
|  | naiAgent.resources.limits.cpu | NAI Agent app CPU limits | 1500m |
| Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform |  |  |  |

Install or upgrade Nutanix Enterprise AI (NAI) on Nutanix Kubernetes Platform (NKP).

To install or upgrade NAI on NKP, follow these high-level steps:

#### 1.  Set up an NKP cluster. For more information, see

Setting up a Nutanix Kubernetes Platform Cluster

on

page 41.

#### 2.  Perform preflight checks. For more information, see

Performing Preflight Checks Before Deploying Nutanix

Enterprise AI on Nutanix Kubernetes Platform

on page 42.

Installing Prerequisite Components on a Nutanix

#### 3.  Install prerequisite components. For more information, see

Kubernetes Platform Cluster

on page  43.

### 4.  Deploy NAI  on NKP. For more information, see

Deploying Nutanix Enterprise AI on Nutanix Kubernetes

Platform

on page 46.

Setting up a Nutanix Kubernetes Platform Cluster Set up a Nutanix Kubernetes Platform (NKP) cluster before you deploy Nutanix Enterprise AI (NAI).

### About this task

To set up your NKP cluster, follow these steps:

### Procedure

1. Create an NKP cluster.

Ensure that the Kubernetes version is 1.35and the nodes are Ubuntu based images. For more information, see the

Nutanix Kubernetes® Platform Guide

.

2. Add the NKP Pro or Ultimate license to your NKP Cluster.

Add an NKP License

For more information, see

in the  Nutanix Kubernetes® Platform Guide .

#### 3.  Ensure that the NKP cluster meets all the requirements listed in

Nutanix Enterprise AI - Private Inference and

Agent Gateway Requirements

on page  10.

4. Create a Nutanix Files server with sufficient storage capacity to save the models.

Creating a File Server

For more information, see

in the  Nutanix Files User Guide .

For information on the size of the models, see

**Table 49: Pre-validated Models**

| on page 184. |
| --- |

#### 5.  Create a Network File System (NFS v4) export in the Nutanix Files server to import a custom model or to import

a model manually.

Creating an NFS Export

For more information, see

in the  Nutanix Files User Guide .

#### 6.  (Optional) To override the default configuration with a custom CNI plugin, ensure that NetworkPolicy

enforcement is enabled.

By default, NKP)installs the Cilium add-on with the default configuration while creating a cluster.

### What to do next

Perform preflight checks. For more information, see

Performing Preflight Checks Before Deploying Nutanix

Enterprise AI on Nutanix Kubernetes Platform

on page 42.

Performing Preflight Checks Before Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform Perform preflight checks before installing or upgrading Nutanix Enterprise AI (NAI) on Nutanix Kubernetes Platform (NKP). Preflight checks ensure the Kubernetes environment meets all the technical requirements for a successful NAI installation.

### About this task

To perform preflight checks, follow these steps:

### Procedure

#### 1.  Verify that the Kubernetes version is 1.33 or 1.34:

```bash
kubectl get nodes \
--selector='!node-role.kubernetes.io/control-plane,!node-role.kubernetes.io/master'
\
-o custom-
columns=NODE:.metadata.name,KUBELET_VERSION:.status.nodeInfo.kubeletVersion
```

The expected output is that the Kubernetes version must be 1.33 or 1.34.

#### 2.  Verify if the number of CSI pods matches the number of worker nodes:

```bash
[ $(kubectl get nodes --no-headers | wc -l) -eq $(kubectl get pods -n ntnx-system --
no-headers | grep csi-node | wc -l) ] && echo "# CSI pods = node count" || echo "#
Mismatch: CSI pods != node count"
```

The expected output is CSI pods = node count.

### 3.  Verify that the

class binds volumes immediately:

```bash
nai-nfs-storage
kubectl get storageclass nai-nfs-storage -o jsonpath='{.volumeBindingMode}{"\n"}'
```

The expected output is

.

```bash
immediate
```

### 4.  Verify if the

storage class has ReadWriteMany access:

```bash
nai-nfs-storage
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
name: test-rwx-pvc
namespace: default
spec:
accessModes:
- ReadWriteMany
resources:
requests:
storage: 1Gi
storageClassName: nai-nfs-storage
EOF
kubectl get pvc test-rwx-pvc -n default
```

The expected output is that the storage class

has

set to

.

```bash
nai-nfs-storage
Access Modes
RWX
NAME           STATUS   VOLUME                                     CAPACITY   ACCESS
MODES   STORAGECLASS      VOLUMEATTRIBUTESCLASS   AGE
test-rwx-pvc            Bound    pvc-0df5145a-590c-4f28-9f9d-cefefc55266c   1Gi
RWX            nai-nfs-storage   <unset>                 5s
kubectl delete pvc test-rwx-pvc
```

Installing Prerequisite Components on a Nutanix Kubernetes Platform Cluster Install prerequisite components on a Nutanix Kubernetes Platform (NKP) cluster.

### Before you begin

Complete the following:

1. Set up an NKP cluster.

Setting up a Nutanix Kubernetes Platform Cluster

For more information, see

on page  41. cert-manager is

deployed when you set up an NKP cluster.

2. Perform preflight checks.

For more information, see

Performing Preflight Checks Before Deploying Nutanix Enterprise AI on Nutanix

Kubernetes Platform

on page 42.

### About this task

To install components on an NKP 2.18 cluster, follow these steps:

### Procedure

### 1.  Install or upgrade Envoy Gateway:

a. Install Envoy Gateway and Gateway API CRDs:
```bash
helm template eg oci://docker.io/envoyproxy/gateway-crds-helm --version v1.8.1 \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

b. Create

with the following configuration:

```yaml
envoy-gateway-config.yaml
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
container:
image: "docker.io/envoyproxy/ratelimit:1e50889b"
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
image: "docker.io/envoyproxy/ratelimit:1e50889b"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

> [!NOTE]
> Note:   The rate-limit backend uses Valkey Sentinel with master name

```bash
mymaster
```

and the

```bash
nai-valkey-
```

service in

.

```bash
sentinel
nai-system
```

c. Install or Upgrade Envoy Gateway:
```bash
helm upgrade --install eg oci://docker.io/envoyproxy/gateway-helm --version v1.8.1
\
-n envoy-gateway-system --create-namespace --skip-crds \
-f "./envoy-gateway-config.yaml"
```

### 2.  Install or Upgrade KServe:

The required version is KSERVE_VERSION=v0.19.0.

a. Install or upgrade the KServe CRDs:
```bash
helm upgrade --install kserve-crd oci://ghcr.io/kserve/charts/kserve-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

b. Install or upgrade the KServe resources:
```bash
helm upgrade --install kserve oci://ghcr.io/kserve/charts/kserve-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true
```

c. Install or upgrade the KServe LLMInferenceService CRD:
```bash
helm upgrade --install kserve-llmisvc-crd oci://ghcr.io/kserve/charts/kserve-
llmisvc-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

d. Install or upgrade the KServe LLMInferenceService resources:
```bash
helm upgrade --install kserve-llmisvc-resources oci://ghcr.io/kserve/charts/
kserve-llmisvc-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.createSharedResources=false \
--set kserve.llmisvc.createGIECRDs=false
```

3. Install CloudNativePG Operator from NKP platform applications.

### 4.  Install LeaderWorkerSet:

```bash
helm install lws oci://registry.k8s.io/lws/charts/lws \
--version 0.8.0 -n lws-system --create-namespace --wait
```

### 5.  Install or upgrade OpenTelemetry Operator

```bash
helm upgrade --install opentelemetry-operator opentelemetry-operator \
--repo https://open-telemetry.github.io/opentelemetry-helm-charts \
--version=0.114.1 -n opentelemetry --create-namespace --wait
```

#### 6.  Install Prometheus Monitoring from NKP platform applications:

To optimize resource utilization on the workload cluster, configure Prometheus Monitoring with the following minimum installation settings when you enable the application:

```bash
alertmanager:
enabled: false
grafana:
enabled: false
prometheus:
enabled: false
kubeStateMetrics:
enabled: false
kubernetesServiceMonitors:
enabled: false
prometheus-node-exporter.kubeRBACProxy:
kubeRBACProxy:
enabled: true
```

Pro: Enabling an Application Using the UI

For more information, see

.

7. Install NVIDIA GPU Operator from NKP platform applications.

If the GPU nodes do not have precompiled NVIDIA drivers installed, enable driver installation in the NVIDIA GPU Operator configuration. This setting ensures that the NVIDIA drivers are installed on the GPU nodes. When enabling the NVIDIA GPU Operator, add the following cluster override:

```bash
driver:
enabled: true
```

Pro: Enabling an Application Using the UI

For more information, see

.

### What to do next

1. Verify that the Envoy Gateway CRDs and controller are installed and ready.
2. Verify that the KServe CRDs and controller resources are ready.
3. Verify that the CloudNativePG operator is ready.
4. Verify that the LeaderWorkerSet controller is ready.
5. Verify that the OpenTelemetry Operator is ready.
6. Verify that Prometheus monitoring is ready.
7. Verify that the NVIDIA GPU Operator is ready.

Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform

on page  46

Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform Install or upgrade Nutanix Enterprise AI on Nutanix Kubernetes Platform (NKP).

### Before you begin

Ensure to complete the following:

- 

Installing Prerequisite Components

Install components on the Kubernetes cluster. For more information, see

on a Nutanix Kubernetes Platform Cluster

on page  43.

- 

Ensure that the required RWO/RWX storage classes exist.

- 

Ensure that the registry secret is available for NAI images.

- 

Upgrades from version 2.7.0 to 2.8.0 must be executed during planned downtime. User logins will be unavailable during the upgrade, and full functionality will resume automatically once the upgrade is complete.

### About this task

To install or upgrade Nutanix Enterprise AI on NKP, follow these steps:

### Procedure

### 1.  Choose a profile:

Profile-based deployment allows you to select a predefined configuration based on your environment and availability requirements. The default profile uses single replica for components and is intended for baseline deployment. The other profiles are for higher capacity usage.

**Table 34: Profile and Capacity**

| Profile | Capacity |
| --- | --- |
| Default | 300 concurrent requests and 100 API Keys |
| c1k_k200 | 1000 concurrent requests and 200 API Keys |
| c5k_k1k | 5000 concurrent requests and 1000 API Keys |

### 2.  Pull and untar both the 2.8.0 charts

```bash
helm pull ntnx-charts/nai-operators --version 2.8.0 --untar=true
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

The extracted charts contain profile files such as:

```yaml
./nai-operators/profiles/c1k_k200.yaml
./nai-operators/profiles/c5k_k1k.yaml
./nai-core/profiles/c1k_k200.yaml
./nai-core/profiles/c5k_k1k.yaml
```

### 3.  Deploy for c1k_k200 profile

a. Deploy NAI Operators for c1k_k200 profile
```bash
export NAI_DEFAULT_RWO_STORAGECLASS=<RWO storageclass>
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
-f ./nai-operators/profiles/c1k_k200.yaml
```

b. Deploy NAI Core for c1k_k200 profile
```bash
export NAI_API_RWX_STORAGECLASS=<NFS Storageclass i.e nai-nfs-storage>
export NAI_DEFAULT_RWO_STORAGECLASS=<RWO default storageclass>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/profiles/c1k_k200.yaml
```

4. Set up the NAI Helm repository.
a. Add and update the Nutanix Helm repository, which contains the

and

```bash
helm chart:
nai-core
nai-operators
helm repo add ntnx-charts https://nutanix.github.io/helm-releases && helm repo
update ntnx-charts
```

b. Search for the version of the

and

```bash
helm chart available for installation in the
nai-core
nai-operators
```

Nutanix helm repository:

```bash
helm search repo ntnx-charts/nai-operators --versions
helm search repo ntnx-charts/nai-core --versions
```

### 5.  Create Docker Registry Secrets:

Create the

namespace and the

secret in both

and

```bash
nai-system
docker-registry
nai-system
envoy-
```

namespaces.

```bash
gateway-system
```

The

is already present on the cluster.

```bash
envoy-gateway-system namespace
export REGISTRY_SECRET_NAME=nai-regcred
export DOCKER_SERVER=https://index.docker.io/v1/
export DOCKER_USERNAME=<docker-username>
export DOCKER_PASSWORD=<docker-password>
export DOCKER_EMAIL=<docker-email>
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-system create secret docker-registry ${REGISTRY_SECRET_NAME} \
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-gateway-system create secret docker-registry ${REGISTRY_SECRET_NAME}
\
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
```

### 6.  Deploy NAI Operators:

```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0  -n
nai-system --create-namespace --take-ownership --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}"
```

> [!NOTE]
> Note: Ensure the REGISTRY_SECRET_NAME environment variable is set before running this command.

### 7.  Deploy NAI Core on NKP:

```bash
# Set the environment variable
export NAI_API_RWX_STORAGECLASS=<NFS Storageclass i.e nai-nfs-storage>
export NAI_DEFAULT_RWO_STORAGECLASS=<default storageclass i.e nutanix-volume>
export NKP_WORKSPACE_NAMESPACE=kommander-default-workspace # update this env as per
your workspace
export REGISTRY_SECRET_NAME=<secret created>
helm upgrade --install nai-core ntnx-charts/nai-core --version 2.8.0 -n nai-system --
create-namespace --force-conflicts --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
--set
"naiMonitoring.nodeExporter.serviceMonitor.namespaceSelector.matchNames[0]=
${NKP_WORKSPACE_NAMESPACE}" \
--set
"naiMonitoring.dcgmExporter.serviceMonitor.namespaceSelector.matchNames[0]=
${NKP_WORKSPACE_NAMESPACE}"
```

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

TLS Encryption on Nutanix

using your own certificate or a cert-manager ClusterIssuer, see

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

8. Configure the TLS certificate.

For more information, see

TLS Encryption on Nutanix Enterprise AI

on page 119.

### What to do next

- 

Verify that the

and

Helm releases have

.

```bash
nai-operators
nai-core
STATUS=deployed
```

- 

Verify that all expected pods are Ready.

- 

Verify that persistent volumes are Bound and use the intended storage classes.

- 

Verify that the selected profile produced the expected replica counts for critical components.

- 

Verify that NAI services are reachable.

- 

#### Pending

#### Failed

If you had endpoints in

status before the upgrade displaying the status as

with the message

Unable to pull runtime image with provided credentials, hibernate and resume the endpoints.

- 

Access NAI Dashboard IP:

```bash
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" -
o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

For information on viewing the

#### Dashboard

, see

Log in to Nutanix Enterprise AI

on page 167.

Deploying Nutanix Enterprise AI on a Nutanix Kubernetes Platform Cluster in Air- Gapped Environments

Install Nutanix Enterprise AI (NAI) on a Nutanix Kubernetes Platform (NKP) cluster in an air-gapped environment.

### Before you begin

- 

NAI supports air-gapped installation only on NKP.

- 

Ensure that you meet requirements listed in

Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a

Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments

on page  52.

### About this task

To install NAI on an NKP cluster in an air-gapped environment, follow these high-level steps:

### Procedure

1. 

Download the NAI 2.8.0 release bundles from the Nutanix portal.

Downloading Nutanix Enterprise AI Air-Gap Release Bundles

For more information, see

on page  52.

2. 

Push Container Images to a private registry.

From this step onwards, execute the all commands from within your air-gapped environment typically a jumpbox or bastion host that has access to both your container registry and your Kubernetes cluster. Ensure that you have Docker and kubectl available on this machine before proceeding.

For more information, see

Pushing Container Images to Private Registry in Air-Gapped Environments

on

page 53

3. 

Extract the NAI Helm charts bundle to your working directory:

```bash
tar -xvf nai-helm-charts-2.8.0.tar
```

The extraction produces the following Helm chart archives:

```bash
gateway-crds-helm-v1.8.1.tgz
gateway-helm-v1.8.1.tgz
kserve-crd-v0.19.0.tgz
kserve-llmisvc-crd-v0.19.0.tgz
kserve-llmisvc-resources-v0.19.0.tgz
kserve-resources-v0.19.0.tgz
lws-0.8.0.tgz
nai-core-2.8.0.tgz
nai-operators-2.8.0.tgz
opentelemetry-operator-0.114.1.tgz
```

4. 

(Optional) Publish Helm Charts to Private Registry

To install Helm charts directly from your OCI-compatible registry instead of local files, push the charts using the following commands:

```bash
# Authenticate to your registry
helm registry login -u <username> -p <password> https://<registry>
# Push each chart to the registry
helm push gateway-crds-helm-v1.8.1.tgz oci://<registry>
helm push gateway-helm-v1.8.1.tgz oci://<registry>
helm push kserve-crd-v0.19.0.tgz oci://<registry>
helm push kserve-resources-v0.19.0.tgz oci://<registry>
helm push kserve-llmisvc-crd-v0.19.0.tgz oci://<registry>
helm push kserve-llmisvc-resources-v0.19.0.tgz oci://<registry>
helm push opentelemetry-operator-0.114.1.tgz oci://<registry>
helm push lws-0.8.0.tgz oci://<registry>
helm push nai-core-2.8.0.tgz oci://<registry>
helm push nai-operators-2.8.0.tgz oci://<registry>
```

The remaining steps assume installation from local Helm chart files.

5. 

Configure Registry Credentials

For more information, see

Configuring Docker Registry Credentials for Dependencies

on page  58.

6. 

Install Prometheus Monitoring from the Nutanix Kubernetes Platform (NKP) platform applications catalog. Prometheus Monitoring is not included in the Nutanix Enterprise AI air-gap image tar or the helm-charts tar. On an NKP cluster in an air-gapped environment, install Prometheus Monitoring from the NKP platform applications catalog before you install Nutanix Enterprise AI components. Enable the Prometheus Monitoring platform application on your NKP workload cluster.

To optimize resource utilization on the workload cluster, configure Prometheus Monitoring with the following minimum installation settings when you enable the application:

```bash
alertmanager:
enabled: false
grafana:
enabled: false
prometheus:
enabled: false
kubeStateMetrics:
enabled: false
kubernetesServiceMonitors:
enabled: false
prometheus-node-exporter.kubeRBACProxy:
kubeRBACProxy:
enabled: true
```

Pro: Enabling an Application Using the UI

For more information, see

.

7. 

Install NVIDIA GPU Operator from NKP platform applications.

If the GPU nodes do not have precompiled NVIDIA drivers installed, enable driver installation in the NVIDIA GPU Operator configuration. This setting ensures that the NVIDIA drivers are installed on the GPU nodes. When enabling the NVIDIA GPU Operator, add the following cluster override:

```bash
driver:
enabled: true
```

Pro: Enabling an Application Using the UI

For more information, see

.

8. 

Install CloudNativePG Operator:

```bash
helm install cnpg cloudnative-pg \
--repo https://cloudnative-pg.github.io/charts \
--version 0.28.0 -n cnpg-system --create-namespace --wait
```

9. 

Install LeaderWorkerSet:

```bash
helm install lws oci://registry.k8s.io/lws/charts/lws \
--version 0.8.0 -n lws-system --create-namespace --wait
```

10. Install Envoy Gateway.

For more information, see

Installing Envoy Gateway in an Air-gapped Environment

on page  59.

11. Install KServe.

Installing KServe

For more information, see

on page  61.

12. Deploy the OpenTelemetry Operator.

For more information, see

Deploying the OpenTelemetry Operator

on page 61.

13. Install NAI components.

Deploying Nutanix Enterprise AI Components in Air-Gapped Environments

For more information, see

on

page 62.

### What to do next

#### 1.  Verify NAI operators installation. Check the status of NAI components:

```bash
# Check all pods in nai-system namespace
kubectl get pods -n nai-system
# Check NAI Operators and Core Helm release
helm list -n nai-system
# Check persistent volume claims
kubectl get pvc -n nai-system
```

### 2.  Verify if NAI services are accessible:

```bash
# List all services in nai-system
kubectl get svc -n nai-system
# Check NAI API service
kubectl get svc -n nai-system nai-api
# Check NAI UI service
kubectl get svc -n nai-system nai-inference-ui
```

3. Troubleshoot common issues that can occur when you install NAI 2.8 in air-gapped environments.

For more information, see

Troubleshooting Deployment of Nutanix Enterprise AI 2.8.0 in Air-Gapped

Environments

on page 66.

```bash
Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments Before proceeding with the installation, ensure the following requirements are met:
```

- 

Nutanix Kubernetes Platform (NKP) cluster running Kubernetes 1.35.

- 
```bash
kubectl CLI v1.33+ configured with cluster access
```

- 

Helm CLI v4.0.5

- 

Docker or compatible container runtime (for loading images)

- 

Access to a private container registry

- 

Registry credentials with appropriate permissions

- 

Sufficient disk space for loading images (~100GB)

```bash
Downloading Nutanix Enterprise AI Air-Gap Release Bundles Download the required Nutanix Enterprise AI 2.8.0 release bundles from the Nutanix Portal.
```

### About this task

To download the required NAI 2.8.0 release bundles from the Nutanix Portal, follow these steps:

### Procedure

#### 1.  Navigate to the Nutanix Enterprise AI page on the Nutanix Support Portal

#### 2.  Select NAI Version 2.8.0 from the available releases

### 3.  Download the following two bundles:

- 

NAI Air-Gap Bundle (

)

```bash
nai-v2.8.0.tar
```

- 

NAI Helm Charts Bundle (

)

```bash
nai-helm-charts-2.8.0.tar
```

**Table 35: Air-Gap Download Bundles**

| Bundle | Description | Size |
| --- | --- | --- |
| NAI Air-Gap Bundle (nai- v2.8.0.tar) |  | ~70-80GB |

- 

Contains all NAI container images and dependencies

- 

Required for air-gapped deployments

NAI Helm Charts Bundle (nai- helm-charts-2.8.0.tar)

~5 MB

- 

Contains NAI Helm charts and all dependency charts

- 

Includes Envoy Gateway, KServe, LeaderWorkerSet and OpenTelemetry charts

#### 4.  Transfer both bundles to your air-gapped environment using approved methods such as USB drive, secure file

transfer, and so on.

Pushing Container Images to Private Registry in Air-Gapped Environments Before installing NAI, you must load all container images and push them to your private registry.

### Before you begin

Execute all the commands from within your air-gapped environment typically a jumpbox or bastion host that has access to both your container registry and your Kubernetes cluster. Ensure you have Docker and kubectl available on this machine before proceeding.

### About this task

To push container images to a private registry, follow these steps:

### Procedure

### 1.  Login to your private container registry:

```bash
docker login <registry-url>
```

For example,

```bash
docker login registry.example.com
```

The system displays a prompt to enter your registry credentials.

2. Enter your registry credentials when prompted.
3. Create the Image Push Script.

The Image Push Script pushes container images to your private registry.

a. Create a project/repository with the name

in your container registry, where all NAI images are

```bash
nutanix
```

stored. For example, in Harbor this would be a project named

, resulting in image paths like

```bash
nutanix
```

.

```bash
registry.example.com/nutanix/<image-name>:<tag>
```

b. Create a script file named

with the following content:

```bash
push-images-to-registry.sh
#!/bin/bash
#
# NAI Images - Load, Retag, and Push to Private Registry
#
# This script loads NAI container images from a tar bundle, retags them for your
# private registry, and pushes them to the registry.
#
# Prerequisites:
#   - Docker installed and running
#   - Docker logged into the target registry (docker login)
#   - NAI images tar bundle file
#
# Usage:
#   ./push-images-to-registry.sh <registry-url> <project> <tar-file>
#
# Example:
#   ./push-images-to-registry.sh registry.example.com nutanix nai-images-2.8.0.tar
#
set -uo pipefail
# ============================================================================
# Helper Functions
# ============================================================================
print_header() {
echo ""
echo "========================================"
echo "$1"
echo "========================================"
}
print_success() {
echo "# $1"
}
print_error() {
echo "# ERROR: $1" >&2
}
print_info() {
echo "# $1"
}
# ============================================================================
# Validate Arguments
# ============================================================================
if [ $# -ne 3 ]; then
echo "Usage: $0 <registry-url> <project> <tar-file>"
echo ""
echo "Arguments:"
echo "  registry-url    Your private registry URL (e.g.,
registry.example.com)"
echo "  project         Project/repository name in the registry (e.g.,
nutanix)"
echo "  tar-file        Path to the NAI images tar bundle"
echo ""
echo "Example:"
echo "  $0 registry.example.com nutanix nai-images-2.8.0.tar"
echo ""
exit 1
fi
REGISTRY="$1"
PROJECT="$2"
TAR_FILE="$3"
# Validate tar file exists
if [ ! -f "$TAR_FILE" ]; then
print_error "Tar file not found: $TAR_FILE"
exit 1
fi
# ============================================================================
# Configuration
# ============================================================================
print_header "NAI Images - Load, Retag & Push"
echo "Registry:  $REGISTRY"
echo "Project:   $PROJECT"
echo "Tar File:  $TAR_FILE"
echo "Date:      $(date)"
# Arrays to track images
LOADED_IMAGES=()
FAILED_IMAGES=()
# ============================================================================
# Step 1: Load Images from Tar Bundle
# ============================================================================
print_header "Step 1: Loading Images from Tar Bundle"
print_info "Loading images from $TAR_FILE..."
LOAD_OUTPUT=$(docker load -i "$TAR_FILE" 2>&1)
# Extract loaded image names
while IFS= read -r line; do
if [[ "$line" =~ Loaded\ image:\ (.+)$ ]]; then
LOADED_IMAGES+=("${BASH_REMATCH[1]}")
fi
done <<< "$LOAD_OUTPUT"
if [ ${#LOADED_IMAGES[@]} -eq 0 ]; then
print_error "No images were loaded from the tar file"
exit 1
fi
print_success "Loaded ${#LOADED_IMAGES[@]} images"
# ============================================================================
# Step 2: Retag and Push Images
# ============================================================================
print_header "Step 2: Retagging and Pushing Images"
PUSHED_COUNT=0
TOTAL_IMAGES=${#LOADED_IMAGES[@]}
for source_image in "${LOADED_IMAGES[@]}"; do
echo ""
print_info "[$((PUSHED_COUNT + 1))/$TOTAL_IMAGES] Processing: $source_image"
# Retag image for target registry
# Format: nutanix/nai-api:v2.8.0 # registry.example.com/<project>/nai-
api:v2.8.0
if [[ "$source_image" =~ ^nutanix/(.+)$ ]]; then
image_path="${BASH_REMATCH[1]}"
target_image="${REGISTRY}/${PROJECT}/${image_path}"
print_info "Tagging as: $target_image"
if ! docker tag "$source_image" "$target_image"; then
print_error "Failed to tag image"
FAILED_IMAGES+=("$source_image")
continue
fi
print_info "Pushing to registry..."
if docker push "$target_image"; then
print_success "Pushed successfully"
((PUSHED_COUNT++))
else
print_error "Failed to push image"
FAILED_IMAGES+=("$target_image")
fi
else
print_info "Skipping (not in nutanix/* format)"
fi
done
# ============================================================================
# Summary
# ============================================================================
print_header "Summary"
echo "Total images loaded:    $TOTAL_IMAGES"
echo "Successfully pushed:    $PUSHED_COUNT"
echo "Failed:                 ${#FAILED_IMAGES[@]}"
if [ ${#FAILED_IMAGES[@]} -gt 0 ]; then
echo ""
print_error "The following images failed:"
for img in "${FAILED_IMAGES[@]}"; do
echo "  - $img"
done
echo ""
exit 1
fi
echo ""
print_success "All images successfully pushed to $REGISTRY/$PROJECT"
echo ""
exit 0
```

c. Make the script executable:
```bash
chmod +x push-images-to-registry.sh
```

d. Execute the script to load, retag, and push all NAI images.
```yaml
./push-images-to-registry.sh <registry-url> <project> nai-v2.8.0.tar
```

Example:

```yaml
./push-images-to-registry.sh registry.example.com nutanix nai-v2.8.0.tar
```

Expected Output:

```bash
========================================
NAI Images - Load, Retag & Push
========================================
Registry:  registry.example.com
Project:   nutanix
Tar File:  ./nai-v2.8.0.tar
Date:      Tue Aug 18 05:16:33 PM UTC 2026
========================================
Step 1: Loading Images from Tar Bundle
========================================
# Loading images from ./nai-v2.8.0.tar...
# Loaded 41 images
========================================
Step 2: Retagging and Pushing Images
========================================
# [1/41] Processing: nutanix/nai-iam-proxy-control-plane:v2.8.0
# Tagging as: registry.example.com/nutanix/nai-iam-proxy-control-plane:v2.8.0
# Pushing to registry...
The push refers to repository [registry.example.com/nutanix/nai-iam-proxy-control-
plane]
054a97ddb80b: Pushed
5228eaa6af5b: Pushed
256f393e029f: Mounted from nutanix/nai-inference-ui
v2.8.0: digest:
sha256:587189a6559b7af653769a21f4755c66af14fe5135eaf45728c84a2eb2f3c808 size: 951
# Pushed successfully
# [2/41] Processing: nutanix/nai-iam-ui:v2.8.0
# Tagging as: registry.example.com/nutanix/nai-iam-ui:v2.8.0
# Pushing to registry...
The push refers to repository [registry.example.com/nutanix/nai-iam-ui]
7673a750ed47: Pushed
187de06a3fb0: Pushed
[... continues for all images ...]
========================================
Summary
========================================
Total images loaded:    41
Successfully pushed:    41
Failed:                 0
# All images successfully pushed to registry.example.com/nutanix
```

- 

The image push process typically takes 30-60 minutes depending on your network speed and registry performance.

- 

All images are retagged with your registry URL while preserving the original image path and tag

- 

Original format: nutanix/nai-api:v2.8.0

- 

Retagged format: <your-registry>/nutanix/nai-api:v2.8.0

- 

The script reports failures and continues processing the remaining images.

- 

You can safely re-run the script if it fails partway through.

Configuring Docker Registry Credentials for Dependencies Configure registry credentials.

### About this task

To configure registry credentials, follow these steps:

### Procedure

1. Set environment Variables.

Export the following environment variables with your private registry credentials:

```bash
export REGISTRY=<registry-url-without-https>
export REGISTRY_USERNAME='<registry-username>'
export REGISTRY_PASSWORD='<registry-password>'
export REGISTRY_EMAIL='<registry-email>'
export IMAGE_PULL_SECRET=nai-docker-regcred
export PROJECT=nutanix # set the registry project name
```

2. Replace the placeholder values with your actual registry information.

The

must not include the

protocol prefix.

```bash
REGISTRY
https://
```

3. Create Image Pull Secrets.

Create Kubernetes namespaces and docker-registry secrets for Envoy Gateway System :

```yaml
kubectl create namespace envoy-gateway-system --dry-run=client -o yaml | kubectl
apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n envoy-gateway-system \
--dry-run=client -o yaml | kubectl apply -f -
```

### 4.  Create Kubernetes namespaces and

secrets for KServe:

```bash
docker-registry
kubectl create namespace kserve --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n kserve \
--dry-run=client -o yaml | kubectl apply -f -
```

#### 5.  Create Kubernetes namespaces and docker-registry secrets for OpenTelemetry:

```yaml
kubectl create namespace opentelemetry --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n opentelemetry \
--dry-run=client -o yaml | kubectl apply -f -
```

#### 6.  Create Kubernetes namespaces and docker-registry secrets for LeaderWorkerSet:

```yaml
kubectl create namespace lws-system --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n lws-system \
--dry-run=client -o yaml | kubectl apply -f -
```

#### 7.  Create Kubernetes namespaces and docker-registry secrets for nai-system:

```yaml
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n nai-system \
--dry-run=client -o yaml | kubectl apply -f -
```

Installing Envoy Gateway in an Air-gapped Environment Install Envoy Gateway.

### Before you begin

Ensure that you meet requirements listed in

Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a

Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments

on page  52.

### About this task

To install Envoy Gateway, follow these steps:

### Procedure

#### 1.  Install the Envoy Gateway and Gateway API Custom Resource Definitions (CRDs):

```bash
helm template eg ./gateway-crds-helm-v1.8.1.tgz \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

This command also installs the necessary Gateway API CRDs required for Envoy Gateway.

### 2.  Create the configuration template file

:

```yaml
eg-config-for-gateway-mode.yaml.template
# This file configures Envoy Gateway for AI Gateway mode with rate limiting
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

3. Ensure the REGISTRY environment variable is configured.

### 4.  Deploy Envoy Gateway:

```bash
helm upgrade --install eg ./gateway-helm-v1.8.1.tgz \
-n envoy-gateway-system --create-namespace --wait \
--set global.images.envoyGateway.image=${REGISTRY}/${PROJECT}/nai-gateway:v1.8.1 \
--set global.images.ratelimit.image=${REGISTRY}/${PROJECT}/nai-ratelimit:1e50889b \
--set "global.imagePullSecrets[0].name=${IMAGE_PULL_SECRET}" \
-f ./eg-config-for-gateway-mode.yaml
```

The configuration file now uses your private registry for the

images through the

```bash
ratelimit
${REGISTRY}
```

variable substitution.

Installing KServe Install KServe.

### About this task

To install KServe, follow these steps:

### Procedure

#### 1.  Install the KServe Custom Resource Definitions (CRDs:)

```bash
helm upgrade --install kserve-crd ./kserve-crd-v0.19.0.tgz -n kserve --create-
namespace --wait
```

#### 2.  Deploy the KServe controller with RawDeployment mode:

```bash
helm upgrade --install kserve ./kserve-resources-v0.19.0.tgz \
-n kserve --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true \
--set kserve.controller.image=${REGISTRY}/${PROJECT}/nai-kserve-controller \
--set kserve.controller.rbacProxyImage=${REGISTRY}/${PROJECT}/nai-kube-rbac-
proxy:v0.18.0 \
--set "kserve.controller.imagePullSecrets[0].name=${IMAGE_PULL_SECRET}"
```

### 3.  Deploy the KServe llmisvc crds:

```bash
helm upgrade --install kserve-llmisvc-crd ./kserve-llmisvc-crd-v0.19.0.tgz -n kserve
--create-namespace --wait
```

### 4.  Deploy the KServe llmisvc controller:

```bash
helm upgrade --install kserve-llmisvc-resources ./kserve-llmisvc-resources-
v0.19.0.tgz \
-n kserve --create-namespace --wait --set kserve.createSharedResources=false --set
kserve.llmisvc.createGIECRDs=false \
--set kserve.llmisvc.controller.image=${REGISTRY}/${PROJECT}/nai-llmisvc-controller
\
--set "kserve.llmisvc.controller.imagePullSecrets[0]=${IMAGE_PULL_SECRET}"
```

Deploying the OpenTelemetry Operator Deploy the OpenTelemetry Operator for observability and telemetry collection.

### About this task

To deploy the OpenTelemetry Operator, run the following command:

### Procedure

Deploy the OpenTelemetry Operator for observability and telemetry collection:

```bash
helm upgrade --install opentelemetry-operator ./opentelemetry-operator-0.114.1.tgz \
-n opentelemetry --create-namespace --wait \
--set manager.image.repository=${REGISTRY}/${PROJECT}/nai-opentelemetry-operator \
--set manager.collectorImage.repository=${REGISTRY}/${PROJECT}/nai-opentelemetry-
collector-contrib \
--set "imagePullSecrets[0].name=${IMAGE_PULL_SECRET}"
```

Deploying Nutanix Enterprise AI Components in Air-Gapped Environments Deploy Nutanix Enterprise AI components in air-gapped environments.

### Before you begin

Upgrades from version 2.7.0 to 2.8.0 must be executed during planned downtime. User logins will be unavailable during the upgrade, and full functionality will resume automatically once the upgrade is complete.

### About this task

To deploy Nutanix Enterprise AI components in air-gapped environments, follow these steps:

### Procedure

using the

#### 1.  Create a values override file for NAI Operators named

```yaml
darksite-nai-operators.yaml.template
```

following template:

This file configures all operator images to use your private registry.

```bash
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
naiValkey:
image:
name: ${REGISTRY}/${PROJECT}/nai-valkey
naiJobs:
naiJobsImage:
image: ${REGISTRY}/${PROJECT}/nai-jobs
nai-clickhouse-operator:
operator:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-operator
metrics:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-metrics-exporter
ai-gateway-helm:
extProc:
image:
repository: ${REGISTRY}/${PROJECT}/nai-ai-gateway-extproc
controller:
image:
repository: ${REGISTRY}/${PROJECT}/nai-ai-gateway-controller
naiDatabase:
image: ${REGISTRY}/${PROJECT}/nai-postgresql:17.10-standard-trixie
```

### 2.  Generate the actual values file:

Use

to replace environment variables and create the final values file:

```bash
envsubst
envsubst < darksite-nai-operators.yaml.template > darksite-nai-operators.yaml
```

The

command substitutes

,

and

with the actual

```bash
envsubst
${REGISTRY}
${PROJECT}
${IMAGE_PULL_SECRET}
```

values from your environment variables.

### 3.  Install NAI Operators:

```bash
helm upgrade --install nai-operators ./nai-operators-2.8.0.tgz \
-n nai-system --create-namespace --wait --timeout 15m -f ./darksite-nai-
operators.yaml
```

### 4.  Prepare NAI Core Values Override file:

Create a values override file for NAI Core using the provided template. This configures all NAI core component images.

a. Create the template file named

with the following content:

```yaml
darksite-nai-core.yaml.template
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
storageClassNameRWX: ${NAI_API_RWX_STORAGECLASS}
gateway:
envoyDeployment:
container:
image: ${REGISTRY}/${PROJECT}/nai-envoy:distroless-v1.38.0
naiIepOperator:
iepOperatorImage:
image: ${REGISTRY}/${PROJECT}/nai-iep-operator
modelProcessorImage:
image: ${REGISTRY}/${PROJECT}/nai-python-processor
dataSourceProcessorImage:
image: ${REGISTRY}/${PROJECT}/nai-python-processor
batchInferenceProcessor:
containers:
processor:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
statusProvider:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
finetuneProcessor:
containers:
processor:
image: ${REGISTRY}/${PROJECT}/nai-finetuning
statusProvider:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
naiInferenceUi:
naiUiImage:
image: ${REGISTRY}/${PROJECT}/nai-inference-ui
naiJobs:
naiJobsImage:
image: ${REGISTRY}/${PROJECT}/nai-jobs
naiApi:
naiApiImage:
image: ${REGISTRY}/${PROJECT}/nai-api
supportedTGIImage: ${REGISTRY}/${PROJECT}/nai-tgi
supportedKserveRuntimeImage: ${REGISTRY}/${PROJECT}/nai-kserve-huggingfaceserver
eppImage: ${REGISTRY}/${PROJECT}/nai-epp-inference-scheduler
supportedVLLMImage: ${REGISTRY}/${PROJECT}/nai-vllm
supportedKserveCustomModelServerRuntimeImage: ${REGISTRY}/${PROJECT}/nai-kserve-
custom-model-server
naiDatabase:
clientImage: ${REGISTRY}/${PROJECT}/nai-postgresql:17.10-standard-trixie
naiIam:
iamProxy:
image: ${REGISTRY}/${PROJECT}/nai-iam-proxy
iamProxyControlPlane:
image: ${REGISTRY}/${PROJECT}/nai-iam-proxy-control-plane
iamUi:
image: ${REGISTRY}/${PROJECT}/nai-iam-ui
iamUserAuthn:
image: ${REGISTRY}/${PROJECT}/nai-iam-user-authn
iamThemis:
image: ${REGISTRY}/${PROJECT}/nai-iam-themis
iamThemisBootstrap:
image: ${REGISTRY}/${PROJECT}/nai-iam-bootstrap
naiAgent:
agentImage:
image: ${REGISTRY}/${PROJECT}/nai-agent-app
naiLabs:
labsImage:
image: ${REGISTRY}/${PROJECT}/nai-rag-app
nai-clickhouse-keeper:
clickhouseKeeper:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-keeper
oauth2-proxy:
image:
repository: ${REGISTRY}/${PROJECT}/nai-oauth2-proxy
nai-clickhouse-server:
clickhouse:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-server
initContainers:
addUdf:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-udf
waitForKeeper:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-jobs
nai-clickhouse-schemas:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-schemas
naiMonitoring:
opentelemetry:
collectorImage: ${REGISTRY}/${PROJECT}/nai-opentelemetry-collector-
contrib:0.152.0
targetAllocator:
image:
repository: ${REGISTRY}/${PROJECT}/nai-target-allocator
nodeExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- prometheus
- kommander
- kommander-default-workspace
- ${NKP_WORKSPACE_NAMESPACE}
dcgmExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- prometheus
- kommander
- kommander-default-workspace
- ${NKP_WORKSPACE_NAMESPACE}
```

b. Ensure that

,

,

,

```bash
REGISTRY
IMAGE_PULL_SECRET
NAI_API_RWX_STORAGECLASS
```

, and

environment variables are set before

```bash
NAI_DEFAULT_RWO_STORAGECLASS
NKP_WORKSPACE_NAMESPACE
```

running the next command.

c. Generate the actual values file:

Use

to replace environment variables and create the final values file:

```bash
envsubst
envsubst < darksite-nai-core.yaml.template > darksite-nai-core.yaml
```

### 5.  Install NAI Core:

```bash
helm upgrade --install nai-core ./nai-core-2.8.0.tgz -n nai-system --create-namespace
--wait --timeout 15m \
-f ./darksite-nai-core.yaml
```

All configuration including registry URLs, storage classes, and monitoring namespaces are in the darksite-nai- core.yaml values file, making the install command much simpler.

The NAI Core installation may take 10 to15 minutes depending on your cluster resources and network speed.

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

The Chat and Talk to My Data applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

using your own certificate or a cert-manager ClusterIssuer, see

TLS Encryption on Nutanix

Enterprise AI

.

- 

Scale out:  To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Database Connections:  To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default is 1000. Increase if you expect a higher number of concurrent clients.

6. Configure the TLS certificate.

TLS Encryption on Nutanix Enterprise AI

For more information, see

on page 119.

#### Troubleshooting Deployment of Nutanix Enterprise AI 2.8.0 in Air-Gapped Environments The following troubleshooting tips can help you resolve a few common issues that can occur when you are installing

Image Pull Errors

Verify that:

- 

Registry credentials are correct

- 

Image pull secrets exist in the correct namespaces

- 

Registry URL is accessible from the cluster

- 

Image paths match your registry structure

- 

All required images are present in your private registry

CRD Installation Failures

Ensure you have sufficient permissions to create cluster-scoped resources.

Helm Installation Timeouts

Increase the timeout value using --timeout flag (e.g., --timeout 20m).

Storage Class Issues

- 

Verify the storage class exists: kubectl get storageclass

- 

Ensure the storage class supports the required access mode (RWX for NAI API, RWO for others)

- 

Check PVC status: kubectl get pvc -n nai-system

Pod Startup Failures

- 

Check pod logs: kubectl logs -n nai-system <pod-name>

- 

Describe pod for events: kubectl describe pod -n nai-system <pod-name>

- 

Verify resource limits if running on resource-constrained clusters

Dependency Issues

Ensure all dependencies (Envoy Gateway, KServe, OpenTelemetry) are installed and running before installing NAI Core.

Deploy Nutanix Enterprise AI on Amazon Elastic Kubernetes Service

Install or upgrade Nutanix Enterprise AI (NAI) on Amazon Elastic Kubernetes Service (EKS).

To install or upgrade NAI on EKS, follow these high-level steps:

#### 1.  Set up an EKS cluster. For more information, see

Setting up an Amazon Elastic Kubernetes Service Cluster

on page 67.

#### 2.  Perform preflight checks. For more information, see

Performing Preflight Checks Before Installing NAI on

Amazon EKS

on page  68.

Installing Prerequisite Components on an EKS

#### 3.  Install prerequisite components. For more information, see

Cluster

on page 69.

### 4.  Deploy NAI on EKS. For more information, see

Deploying Nutanix Enterprise AI on Amazon Elastic

Kubernetes Service

on page 72.

Setting up an Amazon Elastic Kubernetes Service Cluster Set up your Amazon Elastic Kubernetes Service (EKS) cluster for Nutanix Enterprise AI. Do not follow this procedure for Nutanix Kubernetes Platform (NKP) managed or attached EKS clusters.

### Before you begin

Ensure that your cluster meets all the requirements mentioned in

Nutanix Enterprise AI - Private Inference

and Agent Gateway Requirements

on page  10.

### About this task

To set up your EKS cluster before you deploy Nutanix Enterprise AI, follow these steps:

### Procedure

1. 

Configure the AWS CLI.

AWS documentation

For more information, see

.

2. 

Configure an Amazon virtual private cloud (VPC) with public and private subnets for EKS using CloudFormation.

For more information, see

AWS documentation

.

3. 

Create an Amazon EKS cluster with Kubernetes version 1.33 or later.

For more information, see

AWS documentation

.

4. 

Create a kubeconfig file to connect kubectl to the EKS cluster.

AWS documentation

For more information, see

.

5. 

Create an EKS role for the EKS cluster.

For more information, see

AWS documentation

.

6. 

Create an Elastic Compute Cloud (EC2) role for the EKS cluster.

AWS documentation

For more information, see

.

7. 

Create a default node group. Make sure the nodes have accelerator AVX2 or newer.

The recommended instance type is from instance family C5.

8. 

Create a GPU node group on the EKS Cluster.

For more information, see

AWS documentation

.

9. 

Configure the Amazon EBS CSI driver and the RWO storageclass.

AWS documentation

For more information, see

.

10. Configure the Amazon EFS CSI driver and the RWX storageclass.

For more information, see

AWS documentation

.

11. Configure the Amazon VPC CNI plugin to enable network policy enforcement.

AWS documentation

For more information, see

.

Performing Preflight Checks Before Installing NAI on Amazon EKS Perform preflight checks before installing NAI on Amazon Elastic Kubernetes Service (EKS). Preflight checks ensure the Kubernetes environment meets all the technical requirements for a successful NAI installation.

### About this task

To perform preflight checks for installing NAI on EKS, follow these steps:

### Procedure

#### 1.  Make sure the Kubernetes version on your worker nodes is compatible:

```bash
kubectl get nodes --selector='!node-role.kubernetes.io/control-plane' \
-o custom-
columns=NAME:.metadata.name,KUBELET_VERSION:.status.nodeInfo.kubeletVersion \
--no-headers
```

The expected output is that the Kubernetes version must be 1.33 or later.

#### 2.  Verify if the number of EFS CSI pods match the number of worker nodes:

```bash
[ $(kubectl get nodes --no-headers | wc -l) -eq $(kubectl get pods -n kube-system --
no-headers | grep efs-csi-node | wc -l) ] && echo "# efs-csi-node pods = node count"
|| echo "# Mismatch: efs-csi-node pods != node count"
```

The expected output is

.

```bash
efs-csi-node pods= node count
```

### 3.  Confirm that the

class binds volumes immediately:

```bash
nai-nfs-storage
kubectl get storageclass nai-nfs-storage -o jsonpath='{.volumeBindingMode}{"\n"}'
```

The expected output is

.

```bash
immediate
```

### 4.  Check if the

storage class has

access:

```bash
nai-nfs-storage
ReadWriteMany
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
name: test-rwx-pvc
namespace: default
spec:
accessModes:
- ReadWriteMany
resources:
requests:
storage: 1Gi
storageClassName: nai-nfs-storage
EOF
kubectl get pvc test-rwx-pvc -n default
```

The expected output is that the storage class

has

set to

.

```bash
nai-nfs-storage
Access Modes
RWX
NAME           STATUS   VOLUME                                     CAPACITY   ACCESS
MODES   STORAGECLASS      VOLUMEATTRIBUTESCLASS   AGE
test-rwx-pvc            Bound    pvc-0df5145a-590c-4f28-9f9d-cefefc55266c   1Gi
RWX            nai-nfs-storage   <unset>                 5s
kubectl delete pvc test-rwx-pvc
```

Installing Prerequisite Components on an EKS Cluster Install components on an EKS cluster.

### About this task

To install components on an EKS cluster, follow these steps:

### Procedure

### 1.  Install cert-manager:

```bash
helm upgrade --install cert-manager cert-manager --repo https://charts.jetstack.io --
version v1.19.3 --set installCRDs=true -n cert-manager --create-namespace --wait
```

### 2.  Install or upgrade Envoy Gateway:

a. Install Envoy Gateway and Gateway API CRDs:
```bash
helm template eg oci://docker.io/envoyproxy/gateway-crds-helm --version v1.8.1 \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

b. Create

with the following configuration:

```yaml
envoy-gateway-config.yaml
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
container:
image: "docker.io/envoyproxy/ratelimit:1e50889b"
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
image: "docker.io/envoyproxy/ratelimit:1e50889b"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

> [!NOTE]
> Note:   The rate-limit backend uses Valkey Sentinel with master name

```bash
mymaster
```

and the

```bash
nai-valkey-
```

service in

.

```bash
sentinel
nai-system
```

c. Install or Upgrade Envoy Gateway:
```bash
helm upgrade --install eg oci://docker.io/envoyproxy/gateway-helm --version v1.8.1
\
-n envoy-gateway-system --create-namespace --skip-crds \
-f "./envoy-gateway-config.yaml"
```

### 3.  Install or Upgrade KServe:

The required version is KSERVE_VERSION=v0.19.0.

a. Install or upgrade the KServe CRDs:
```bash
helm upgrade --install kserve-crd oci://ghcr.io/kserve/charts/kserve-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

b. Install or upgrade the KServe resources:
```bash
helm upgrade --install kserve oci://ghcr.io/kserve/charts/kserve-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true
```

c. Install or upgrade the KServe LLMInferenceService CRD:
```bash
helm upgrade --install kserve-llmisvc-crd oci://ghcr.io/kserve/charts/kserve-
llmisvc-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

d. Install or upgrade the KServe LLMInferenceService resources:
```bash
helm upgrade --install kserve-llmisvc-resources oci://ghcr.io/kserve/charts/
kserve-llmisvc-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.createSharedResources=false \
--set kserve.llmisvc.createGIECRDs=false
```

### 4.  Install CloudNativePG Operator:

```bash
helm install cnpg cloudnative-pg \
--repo https://cloudnative-pg.github.io/charts \
--version 0.28.0 -n cnpg-system --create-namespace --wait
```

### 5.  Install LeaderWorkerSet:

```bash
helm install lws oci://registry.k8s.io/lws/charts/lws \
--version 0.8.0 -n lws-system --create-namespace --wait
```

### 6.  Install or upgrade OpenTelemetry Operator

```bash
helm upgrade --install opentelemetry-operator opentelemetry-operator \
--repo https://open-telemetry.github.io/opentelemetry-helm-charts \
--version=0.114.1 -n opentelemetry --create-namespace --wait
```

### 7.  Install or upgrade Prometheus Monitoring:

```bash
helm upgrade --install prometheus kube-prometheus-stack \
--repo https://prometheus-community.github.io/helm-charts \
--version=82.13.6 -n prometheus --create-namespace --wait \
--set grafana.enabled=false \
--set prometheus.enabled=false \
--set kubeStateMetrics.enabled=false \
--set alertManager.enabled=false \
--set kubernetesServiceMonitors.enabled=false \
--set prometheus-node-exporter.kubeRBACProxy.enabled=true
```

### 8.  Install or upgrade NVIDIA GPU Operator :

```bash
helm upgrade --install --wait gpu-operator gpu-operator \
--repo https://helm.ngc.nvidia.com/nvidia \
-n gpu-operator --create-namespace --version=v26.3.0
```

### What to do next

1. Verify that the Envoy Gateway CRDs and controller are installed and ready.
2. Verify that the KServe CRDs and controller resources are ready.
3. Verify that the CloudNativePG operator is ready.
4. Verify that the LeaderWorkerSet controller is ready.
5. Verify that the OpenTelemetry Operator is ready.
6. Verify that Prometheus monitoring is ready.
7. Verify that the NVIDIA GPU Operator is ready.
```bash
Deploying Nutanix Enterprise AI on Amazon Elastic Kubernetes Service Install or upgrade Nutanix Enterprise AI on Amazon Elastic Kubernetes Service (EKS).
```

### Before you begin

Ensure to complete the following:

- 

Installing Prerequisite Components

Install components on the Kubernetes cluster. For more information, see

on an EKS Cluster

on page 69.

- 

Upgrades from version 2.7.0 to 2.8.0 must be executed during planned downtime. User logins will be unavailable during the upgrade, and full functionality will resume automatically once the upgrade is complete.

### About this task

To install or upgrade Nutanix Enterprise AI on EKS, follow these steps:

### Procedure

### 1.  Choose a profile:

Profile-based deployment allows you to select a predefined configuration based on your environment and availability requirements. The default profile uses single replica for components and is intended for baseline deployment. The other profiles are for higher capacity usage.

**Table 36: Profile and Capacity**

| Profile | Capacity |
| --- | --- |
| Default | 300 concurrent requests and 100 API Keys |
| c1k_k200 | 1000 concurrent requests and 200 API Keys |
| c5k_k1k | 5000 concurrent requests and 1000 API Keys |

### 2.  Pull and untar both the 2.8.0 charts

```bash
helm pull ntnx-charts/nai-operators --version 2.8.0 --untar=true
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

The extracted charts contain profile files such as:

```yaml
./nai-operators/profiles/c1k_k200.yaml
./nai-operators/profiles/c5k_k1k.yaml
./nai-core/profiles/c1k_k200.yaml
./nai-core/profiles/c5k_k1k.yaml
```

### 3.  Deploy for c1k_k200 profile

a. Deploy NAI Operators for c1k_k200 profile
```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
-f ./nai-operators/profiles/c1k_k200.yaml \
-f ./nai-operators/eks-values.yaml
```

b. Deploy NAI Core for c1k_k200 profile
```bash
export NAI_API_RWX_STORAGECLASS=<RWX/NFS storageclass>
export NAI_DEFAULT_RWO_STORAGECLASS=<RWO default storageclass>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/profiles/c1k_k200.yaml \
-f ./nai-core/eks-values.yaml
```

4. Set up the NAI Helm repository.
a. Add and update the Nutanix Helm repository, which contains the

and

```bash
helm chart:
nai-core
nai-operators
helm repo add ntnx-charts https://nutanix.github.io/helm-releases && helm repo
update ntnx-charts
```

b. Search for the version of the

and

```bash
helm chart available for installation in the
nai-core
nai-operators
```

Nutanix helm repository:

```bash
helm search repo ntnx-charts/nai-operators --versions
helm search repo ntnx-charts/nai-core --versions
```

### 5.  Create Docker Registry Secrets:

Create the

namespace and the

secret in both

and

```bash
nai-system
docker-registry
nai-system
envoy-
```

namespaces.

```bash
gateway-system
```

The

is already present on the cluster.

```bash
envoy-gateway-system namespace
export REGISTRY_SECRET_NAME=nai-regcred
export DOCKER_SERVER=https://index.docker.io/v1/
export DOCKER_USERNAME=<docker-username>
export DOCKER_PASSWORD=<docker-password>
export DOCKER_EMAIL=<docker-email>
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-system create secret docker-registry ${REGISTRY_SECRET_NAME} \
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-gateway-system create secret docker-registry ${REGISTRY_SECRET_NAME}
\
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
```

### 6.  Deploy NAI Operators:

```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0  -n
nai-system --create-namespace --take-ownership --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}"
```

> [!NOTE]
> Note: Ensure the REGISTRY_SECRET_NAME environment variable is set before running this command.

```bash
helm chart and extract it to your local repository:
```

### 7.  Pull the

```bash
nai-core
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

### 8.  Install or upgrade NAI Core on EKS:

```bash
# Set the environment variable
NAI_API_RWX_STORAGECLASS=<NFS Storageclass i.e nai-nfs-storage>
NAI_DEFAULT_RWO_STORAGECLASS=<default storageclass>
REGISTRY_SECRET_NAME=<secret created>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 -n nai-system --
create-namespace --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/eks-values.yaml
```

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

TLS Encryption on Nutanix

using your own certificate or a cert-manager ClusterIssuer, see

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

9. Configure the TLS certificate.

For more information, see

TLS Encryption on Nutanix Enterprise AI

on page 119.

### What to do next

- 

Ensure that all the pods are successfully deployed in the

namespace and are in the ready state by:

```bash
nai-system
kubectl get pods -n nai-system
```

The following is a sample output of the command. Match the pod names by ignoring the auto-generated suffix added by Kubernetes and its status.

> [!NOTE]
> Note:

The number of

should be equal to the number of worker

```bash
nai-otel-collector-collector pods
```

nodes in the NAI Kubernetes cluster.

```bash
NAME                                                            READY   STATUS
RESTARTS   AGE
chi-nai-clickhouse-server-chcluster1-0-0-0                      1/1     Running     0
16h
chk-nai-clickhouse-keeper-chkeeper-0-0-0                        1/1     Running     0
16h
iam-database-bootstrap-b8etj-hk9rz                              0/1     Completed   0
16h
iam-proxy-68f9459885-zcwgs                                      1/1     Running     0
16h
iam-proxy-control-plane-6897669d64-rvglt                        1/1     Running     0
16h
iam-themis-749b7b56f8-pmclb                                     1/1     Running     0
16h
iam-themis-bootstrap-qgczx-p7bl6                                0/1     Completed   0
16h
iam-ui-6697d94478-fftl5                                         1/1     Running     0
16h
iam-user-authn-5b4dcfdfb7-jhllq                                 1/1     Running     0
16h
nai-api-784fb7b99-8tch9                                         1/1     Running     0
16h
nai-api-db-migrate-mgnba-cl7gw                                  0/1     Completed   0
16h
nai-clickhouse-schema-job-1771350985-bd4sq                      0/1     Completed   0
16h
nai-db-0                                                        1/1     Running     0
16h
nai-iep-model-controller-5cd8bcd5f-9h9cl                        1/1     Running     0
16h
nai-labs-86589cc95d-gk87q                                       1/1     Running     0
16h
nai-oauth2-proxy-5746ccc8b7-65jmf                               1/1     Running     0
16h
nai-oidc-client-registration-lvrrv-pbdsn                        0/1     Completed   0
16h
nai-otel-collector-collector-4pxtl                              1/1     Running     0
16h
nai-otel-collector-collector-8p4w8                              1/1     Running     0
16h
nai-otel-collector-collector-bwstg                              1/1     Running     0
16h
nai-otel-collector-collector-drddv                              1/1     Running     0
16h
nai-otel-collector-collector-k457n                              1/1     Running     0
16h
nai-otel-collector-collector-nmwjw                              1/1     Running     0
16h
nai-otel-collector-targetallocator-c9dcc6544-5wbrx              1/1     Running     0
16h
nai-pulse-job-29522885-6w7pn                                    0/1     Completed   0
10h
nai-ui-6d9cc89b87-b4npf                                         1/1     Running     0
16h
nutanix-ai-operators-nai-clickhouse-operator-7f9965dbdd-trwp5   2/2     Running     0
16h
redis-standalone-6df56bc96d-hp86q                               2/2     Running     0
16h
```

- 

Access NAI Dashboard IP:

```bash
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" -
o jsonpath='{.items[0].status.loadBalancer.ingress[0].hostname}'
```

Log in to Nutanix Enterprise AI

For information on viewing the Dashboard, see

on page  167.

- 

If you had endpoints in

#### Pending

status before the upgrade displaying the status as

#### Failed

with the message

Unable to pull runtime image with provided credentials, hibernate and resume the endpoints.

#### Deploying Nutanix Enterprise AI on Azure Kubernetes Service

Install or upgrade Nutanix Enterprise AI (NAI) on Azure Kubernetes Service (AKS).

To install or upgrade NAI on AKS, follow these step:

#### 1.  Set up an AKS cluster. For more information, see

Setting up an Azure Kubernetes Service Cluster

on

page 76.

#### 2.  Perform preflight checks. For more information, see

Performing Preflight Checks Before Installing Nutanix

Enterprise AI on Azure AKS

on page  77.

#### 3.  Install prerequisite components. For more information, see

Deploying Nutanix Enterprise AI on Azure

Kubernetes Service

on page 82.

### 4.  Deploy NAI on AKS. For more information, see

Deploying Nutanix Enterprise AI on Azure Kubernetes

Service

on page  82.

Setting up an Azure Kubernetes Service Cluster Set up your Azure Kubernetes Service (AKS) cluster for Nutanix Enterprise AI (NAI). This procedure is not applicable for Nutanix Kubernetes Platform (NKP) managed or attached AKS clusters.

### Before you begin

Ensure that your cluster meets all the requirements mentioned in

Nutanix Enterprise AI - Private Inference

and Agent Gateway Requirements

on page  10.

### About this task

To set up your AKS cluster before you deploy NAI, follow these high-level steps:

### Procedure

1. Configure Azure CLI.

For more information, see

Azure documentation

.

2. Create an Azure resource group.

Azure documentation

For more information, see

.

3. Create an AKS cluster with Kubernetes version 1.33 or 1.34.

Azure documentation

For more information, see

.

4. Configure the AKS cluster with a supported network policy engine enabled for enforcement.

For more information, see

Azure documentation

.

5. Configure the AKS preview extension.

Azure documentation

For more information, see

.

6. Create a default node group. Make sure the nodes have accelerator AVX2 or newer.
7. Add a GPU node pool to the AKS cluster using Azure CLI.

Ensure that you skip installing the GPU driver when adding the GPU node pool to the cluster. For more information, see

Azure documentation

Azure documentation

. For more information, see

.

Performing Preflight Checks Before Installing Nutanix Enterprise AI on Azure AKS Perform preflight checks before installing Nutanix Enterprise AI on Azure AKS. Preflight checks ensure the Kubernetes environment meets all the technical requirements for a successful NAI installation.

### About this task

To perform preflight checks for installing Nutanix Enterprise AI on Azure AKS, follow these steps:

### Procedure

### 1.  Choose a profile:

Profile-based deployment allows you to select a predefined configuration based on your environment and availability requirements. The default profile uses single replica for components and is intended for baseline deployment. The other profiles are for higher capacity usage.

**Table 37: Profile and Capacity**

| Profile | Capacity |
| --- | --- |
| Default | 300 concurrent requests and 100 API Keys |
| c1k_k200 | 1000 concurrent requests and 200 API Keys |
| c5k_k1k | 5000 concurrent requests and 1000 API Keys |

### 2.  Pull and untar both the 2.8.0 charts

```bash
helm pull ntnx-charts/nai-operators --version 2.8.0 --untar=true
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

The extracted charts contain profile files such as:

```yaml
./nai-operators/profiles/c1k_k200.yaml
./nai-operators/profiles/c5k_k1k.yaml
./nai-core/profiles/c1k_k200.yaml
./nai-core/profiles/c5k_k1k.yaml
```

### 3.  Deploy for small profile:

a. Deploy NAI Operators for small profile
```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
-f ./nai-operators/profiles/small.yaml \
-f ./nai-operators/aks-values.yaml
```

b. Deploy NAI Core for small profile
```bash
export NAI_API_RWX_STORAGECLASS=<RWX/NFS storageclass>
export NAI_DEFAULT_RWO_STORAGECLASS=<RWO default storageclass>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/profiles/small.yaml \
-f ./nai-core/aks-values.yaml
```

#### 4.  Ensure your Kubernetes version is 1.33 or 1.34:

```bash
kubectl get nodes --selector='!node-role.kubernetes.io/control-plane' \
-o custom-
columns=NAME:.metadata.name,KUBELET_VERSION:.status.nodeInfo.kubeletVersion \
--no-headers
```

The expected output is that the Kubernetes version must be 1.33 or 1.34.

### 5.  Verify if the number of Azure Disk CSI pods (

) match the number of worker nodes:

```bash
csi-azuredisk-node
[ $(kubectl get nodes --no-headers | wc -l) -eq $(kubectl get pods -n kube-system --
no-headers | grep csi-azuredisk-node | wc -l) ] && echo "# csi-azuredisk-node pods =
node count" || echo "# Mismatch: csi-azuredisk-node pods != node count"
```

The expected output is

.

```bash
csi-azuredisk-node pods= node count
```

### 6.  Confirm that the

storage class binds volumes immediately:

```bash
azurefile-csi
kubectl get sc azurefile-csi -o jsonpath='{.metadata.name}: {.volumeBindingMode}
{"\n"}'
```

The expected output is

.

```bash
immediate
```

### 7.  Check if the storage class has

access:

```bash
ReadWriteMany
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
name: test-rwx-pvc
namespace: default
spec:
accessModes:
- ReadWriteMany
resources:
requests:
storage: 1Gi
storageClassName: azurefile-csi
EOF
kubectl get pvc test-rwx-pvc -n default
```

The expected output is that the storage class

has

set to

.

```bash
azurefile-csi
Access Modes
RWX
NAME           STATUS   VOLUME                                     CAPACITY   ACCESS
MODES   STORAGECLASS    VOLUMEATTRIBUTESCLASS   AGE
test-rwx-pvc   Bound    pvc-4811d039-5cf4-4925-9181-2b2a3689ea47   1Gi        RWX
azurefile-csi   <unset>                 4s
kubectl delete pvc test-rwx-pvc
```

### 8.  Verify if a CNI is enabled on the cluster:

```bash
az aks show --resource-group <resource-group-name> --name <aks-cluster-name> --query
"networkProfile.networkPolicy"
```

The expected output is the network policy engine selected during cluster setup. If the output is

, network policy is not enabled for the cluster.

```bash
"networkPolicy": "none"
```

Installing Prerequisite Components on an AKS Cluster Install components on an AKS cluster.

### About this task

To install components on an AKS cluster, follow these steps:

### Procedure

### 1.  Install cert-manager:

```bash
helm upgrade --install cert-manager cert-manager --repo https://charts.jetstack.io --
version v1.19.3 --set installCRDs=true -n cert-manager --create-namespace --wait
```

### 2.  Install or upgrade Envoy Gateway:

a. Install Envoy Gateway and Gateway API CRDs:
```bash
helm template eg oci://docker.io/envoyproxy/gateway-crds-helm --version v1.8.1 \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

b. Create

with the following configuration:

```yaml
envoy-gateway-config.yaml
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
container:
image: "docker.io/envoyproxy/ratelimit:1e50889b"
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
image: "docker.io/envoyproxy/ratelimit:1e50889b"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

> [!NOTE]
> Note:   The rate-limit backend uses Valkey Sentinel with master name

and the

```bash
mymaster
nai-valkey-
```

service in

.

```bash
sentinel
nai-system
```

c. Install or Upgrade Envoy Gateway:
```bash
helm upgrade --install eg oci://docker.io/envoyproxy/gateway-helm --version v1.8.1
\
-n envoy-gateway-system --create-namespace --skip-crds \
-f "./envoy-gateway-config.yaml"
```

### 3.  Install or Upgrade KServe:

The required version is KSERVE_VERSION=v0.19.0.

a. Install or upgrade the KServe CRDs:
```bash
helm upgrade --install kserve-crd oci://ghcr.io/kserve/charts/kserve-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

b. Install or upgrade the KServe resources:
```bash
helm upgrade --install kserve oci://ghcr.io/kserve/charts/kserve-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true
```

c. Install or upgrade the KServe LLMInferenceService CRD:
```bash
helm upgrade --install kserve-llmisvc-crd oci://ghcr.io/kserve/charts/kserve-
llmisvc-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

d. Install or upgrade the KServe LLMInferenceService resources:
```bash
helm upgrade --install kserve-llmisvc-resources oci://ghcr.io/kserve/charts/
kserve-llmisvc-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.createSharedResources=false \
--set kserve.llmisvc.createGIECRDs=false
```

### 4.  Install CloudNativePG Operator:

```bash
helm install cnpg cloudnative-pg \
--repo https://cloudnative-pg.github.io/charts \
--version 0.28.0 -n cnpg-system --create-namespace --wait
```

### 5.  Install LeaderWorkerSet:

```bash
helm install lws oci://registry.k8s.io/lws/charts/lws \
--version 0.8.0 -n lws-system --create-namespace --wait
```

### 6.  Install or upgrade OpenTelemetry Operator

```bash
helm upgrade --install opentelemetry-operator opentelemetry-operator \
--repo https://open-telemetry.github.io/opentelemetry-helm-charts \
--version=0.114.1 -n opentelemetry --create-namespace --wait
```

### 7.  Install or upgrade Prometheus Monitoring:

```bash
helm upgrade --install prometheus kube-prometheus-stack \
--repo https://prometheus-community.github.io/helm-charts \
--version=82.13.6 -n prometheus --create-namespace --wait \
--set grafana.enabled=false \
--set prometheus.enabled=false \
--set kubeStateMetrics.enabled=false \
--set alertManager.enabled=false \
--set kubernetesServiceMonitors.enabled=false \
--set prometheus-node-exporter.kubeRBACProxy.enabled=true
```

### 8.  Install or upgrade NVIDIA GPU Operator :

```bash
helm upgrade --install --wait gpu-operator gpu-operator \
--repo https://helm.ngc.nvidia.com/nvidia \
-n gpu-operator --create-namespace --version=v26.3.0
```

### What to do next

1. Verify that the Envoy Gateway CRDs and controller are installed and ready.
2. Verify that the KServe CRDs and controller resources are ready.
3. Verify that the CloudNativePG operator is ready.
4. Verify that the LeaderWorkerSet controller is ready.
5. Verify that the OpenTelemetry Operator is ready.
6. Verify that Prometheus monitoring is ready.
7. Verify that the NVIDIA GPU Operator is ready.
```bash
Deploying Nutanix Enterprise AI on Azure Kubernetes Service Install or upgrade Nutanix Enterprise AI on Azure Kubernetes Service (AKS).
```

### Before you begin

Ensure to complete the following:

- 

Perform preflight checks and obtain the expected output. For more information on preflight checks and the expected output, see

Performing Preflight Checks Before Installing Nutanix Enterprise AI on Azure AKS

on

page 77.

- 

Installing Prerequisite Components on an

Install components on the AKS cluster. For more information, see

AKS Cluster

on page 79.

- 

Upgrades from version 2.7.0 to 2.8.0 must be executed during planned downtime. User logins will be unavailable during the upgrade, and full functionality will resume automatically once the upgrade is complete.

### About this task

To install or upgrade Nutanix Enterprise AI on AKS, follow these steps:

### Procedure

### 1.  Setup NAI Helm repository

a. Add and update the Nutanix Helm repository, which contains the

Helm chart:

```bash
nai-core
helm repo add ntnx-charts https://nutanix.github.io/helm-releases && helm repo
update ntnx-charts
```

b. Search for the version of the

and

```bash
helm chart available for installation in the
nai-operators
nai-core
```

Nutanix Helm repository:

```bash
helm search repo ntnx-charts/nai-operators --versions
helm search repo ntnx-charts/nai-core --versions
```

### 2.  Create Docker Registry Secrets:

Create the

namespace and the

secret in both

and

```bash
nai-system
docker-registry
nai-system
envoy-
```

namespaces.

```bash
gateway-system
```

The

is already present on the cluster.

```bash
envoy-gateway-system namespace
export REGISTRY_SECRET_NAME=nai-regcred
export DOCKER_SERVER=https://index.docker.io/v1/
export DOCKER_USERNAME=<docker-username>
export DOCKER_PASSWORD=<docker-password>
export DOCKER_EMAIL=<docker-email>
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-system create secret docker-registry ${REGISTRY_SECRET_NAME} \
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-gateway-system create secret docker-registry ${REGISTRY_SECRET_NAME}
\
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
```

### 3.  Deploy NAI Operators:

```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0  -n
nai-system --create-namespace --take-ownership --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}"
```

> [!NOTE]
> Note: Ensure the REGISTRY_SECRET_NAME environment variable is set before running this command.

### 4.  Pull the

```bash
helm chart and extract it to your local repository:
nai-core
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

### 5.  Install or upgrade NAI Core on AKS:

```bash
# Set the environment variable
NAI_API_RWX_STORAGECLASS=<NFS Storageclass i.e nai-nfs-storage>
NAI_DEFAULT_RWO_STORAGECLASS=<default storageclass>
REGISTRY_SECRET_NAME=<secret created>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 -n nai-system --
create-namespace --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/aks-values.yaml
```

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

using your own certificate or a cert-manager ClusterIssuer, see

TLS Encryption on Nutanix

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

6. Configure the TLS certificate.

TLS Encryption on Nutanix Enterprise AI

For more information, see

on page 119.

### What to do next

- 

Ensure that all the pods are successfully deployed in the

namespace and are in the

state by:

```bash
nai-system
Ready
kubectl get pods -n nai-system
```

The following is a sample output of the command. Match the pod names by ignoring the auto-generated suffix added by Kubernetes and its status.

> [!NOTE]
> Note:

The number of

must be equal to the number of worker

```bash
nai-otel-collector-collector pods
```

nodes in the NAI Kubernetes cluster.

```bash
NAME                                                     READY   STATUS
ai-gateway-controller-6644f54c64-z2cwm                   1/1     Running
chi-nai-clickhouse-server-chcluster1-0-0-0               1/1     Running
chk-nai-clickhouse-keeper-chkeeper-0-0-0                 1/1     Running
iam-database-bootstrap-mlgpu-hk5zz                       0/1     Completed
iam-proxy-68d975978d-r7llr                               1/1     Running
iam-proxy-control-plane-d876b77dd-7swx2                  1/1     Running
iam-themis-679b9bbff8-nfknh                              1/1     Running
iam-themis-bootstrap-e1fhx-kglkm                         0/1     Completed
iam-ui-6cb6d49fcc-fhnhm                                  1/1     Running
iam-user-authn-5dbbdcbfcc-9j6jr                          1/1     Running
nai-agent-648c7b8c8d-rzq28                               1/1     Running
nai-api-85b8694cfc-5l5j8                                 1/1     Running
nai-api-db-migrate-lweo1-w8zfx                           0/1     Completed
nai-clickhouse-schema-job-1778237791-fs7p8               0/1     Completed
nai-db-0                                                 1/1     Running
nai-iep-model-controller-b5b8bf9-jxfrk                   1/1     Running
nai-labs-689dcb4644-l6f4p                                1/1     Running
nai-oauth2-proxy-579c5b4d9f-mn72l                        1/1     Running
nai-operators-nai-clickhouse-operator-858fdb9b94-zs8gx   2/2     Running
nai-otel-collector-collector-49bsx                       1/1     Running
nai-otel-collector-collector-6tgh5                       1/1     Running
nai-otel-collector-collector-8qp9x                       1/1     Running
nai-otel-collector-targetallocator-748856644d-5lnff      1/1     Running
nai-securityscan-manager-678ffd75ff-9t95c                1/1     Running
nai-ui-76d74f55bb-c28rw                                  1/1     Running
redis-standalone-67ccd5cc8f-6hp55                        2/2     Running
```

- 

Verify that the Envoy Gateway, ingress gateway, and rate limit pods in the envoy-gateway-system namespace are in the

state:

```bash
Running
kubectl get pods -n envoy-gateway-system
NAME                                                            READY    STATUS
envoy-gateway-6b987d469d-5l2w9                                   1/1     Running
envoy-nai-system-nai-ingress-gateway-ff52ba1f-7fd4897bd4-7smgg   2/2     Running
envoy-ratelimit-85b55c877c-2n2bf                                 1/1     Running
```

- 

#### Pending

#### Failed

If you had endpoints in

status before the upgrade displaying the status as

with the message

Unable to pull runtime image with provided credentials, hibernate and resume the endpoints.

- 

Access NAI Dashboard IP:

```bash
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" -
o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

#### Dashboard

Log in to Nutanix Enterprise AI

For information on viewing the

, see

on page 167.

#### Deploy Nutanix Enterprise AI on Google Kubernetes Engine

Install or upgrade Nutanix Enterprise AI (NAI) on Google Kubernetes Engine (GKE).

To install or upgrade NAI, follow these steps:

#### 1.  Set up a GKE cluster. For more information, see

Setting up a Google Kubernetes Engine Cluster

on

page 85.

#### 2.  Perform preflight checks. For more information, see

Performing Preflight Checks Before Installing Nutanix

Enterprise AI on Google Kubernetes Engine

on page 85.

#### 3.  Install prerequisite components. For more information, see

Installing Prerequisite Components on a Google

Kubernetes Engine Cluster

on page  86.

### 4.  Deploy NAI  on GKE. For more information, see

Deploying Nutanix Enterprise AI on Google Kubernetes

Engine

on page 90.

Setting up a Google Kubernetes Engine Cluster Set up your Google Kubernetes Engine (GKE) cluster for Nutanix Enterprise AI. This procedure is not applicable for Nutanix Kubernetes Platform (NKP) managed or attached GKE clusters.

### Before you begin

Ensure that your cluster meets all the requirements mentioned in

Nutanix Enterprise AI - Private Inference

and Agent Gateway Requirements

on page  10.

### About this task

To set up your GKE cluster before you deploy Nutanix Enterprise AI, follow these steps:

### Procedure

1. Configure and authorize the Google Cloud CLI.

For more information, see

Google Cloud documentation

.

### 2.  Enable

.

Required Services

This is required to enable Google files CSI driver and default RWX storage classes. For more information, see

Google Cloud documentation

.

#### 3.  Create a GKE cluster with Kubernetes version 1.33 or 1.34 based on the

GKE GPU Operator NVIDIA Driver Manager

workflow.

For more information, see

Google Cloud documentation

.

#### 4.  Configure the GKE cluster to enable network policy enforcement. Network policy enforcement is built into GKE

Dataplane V2 but needs to be enabled when Dataplane V2 is disabled.

Google Cloud documentation

For more information, see

.

5. Create a default node group. Make sure the nodes have accelerator AVX2 or newer.
6. Create a GKE node pool and add it to the cluster.

Ensure that you skip installing the Google GPU driver when adding the GKE node pool to the cluster. For more information, see

Google Cloud documentation

.

7. Configure the node pool to create the GPU operator namespace and deploy the GPU operator resource quota.

For more information, see

Google Cloud documentation

.

Performing Preflight Checks Before Installing Nutanix Enterprise AI on Google Kubernetes Engine Perform preflight checks before installing NAI on Google Kubernetes Engine (GKE). Preflight checks ensure the Kubernetes environment meets all the technical requirements for a successful NAI installation.

### About this task

To perform preflight checks for installing NAI on GKE, follow these steps:

### Procedure

#### 1.  Make sure your Kubernetes version is 1.33 or 1.34:

```bash
kubectl get nodes --selector='!node-role.kubernetes.io/control-plane' \
-o custom-
columns=NAME:.metadata.name,KUBELET_VERSION:.status.nodeInfo.kubeletVersion \
--no-headers
```

The expected output is that the Kubernetes version must be 1.33 or 1.34.

### 2.  Verify if the number of

CSI pods match the number of worker nodes:

```bash
filestore-node
[ $(kubectl get nodes --no-headers | wc -l) -eq $(kubectl get pods -n kube-system
--no-headers | grep filestore-node | wc -l) ] && echo "# filestore-node pods = node
count" || echo "# Mismatch: filestore-node pods != node count"
```

The expected output is

.

```bash
filestore-node pods= node count
```

### 3.  Confirm that the

class binds volumes immediately:

```bash
nai-nfs-storage
kubectl get storageclass nai-nfs-storage -o jsonpath='{.volumeBindingMode}{"\n"}'
```

The expected output is

.

```bash
immediate
```

### 4.  Check if the

storage class has

access:

```bash
nai-nfs-storage
ReadWriteMany
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
name: test-rwx-pvc
namespace: default
spec:
accessModes:
- ReadWriteMany
resources:
requests:
storage: 1Gi
storageClassName: nai-nfs-storage
EOF
kubectl get pvc test-rwx-pvc -n default
```

The expected output is that the storage class

has

set to

.

```bash
nai-nfs-storage
Access Modes
RWX
NAME           STATUS   VOLUME                                     CAPACITY   ACCESS
MODES   STORAGECLASS      VOLUMEATTRIBUTESCLASS   AGE
test-rwx-pvc            Bound    pvc-0df5145a-590c-4f28-9f9d-cefefc55266c   1Gi
RWX            nai-nfs-storage   <unset>                 5s
kubectl delete pvc test-rwx-pvc
```

Installing Prerequisite Components on a Google Kubernetes Engine Cluster Install components on a Google Kubernetes Engine (GKE) cluster.

### About this task

To install components on a GKE cluster, follow these steps:

### Procedure

### 1.  Install cert-manager:

```bash
helm upgrade --install cert-manager cert-manager --repo https://charts.jetstack.io --
version v1.19.3 --set installCRDs=true -n cert-manager --create-namespace --wait
```

### 2.  Install or upgrade Envoy Gateway:

a. Install Envoy Gateway and Gateway API CRDs:
```bash
helm template eg oci://docker.io/envoyproxy/gateway-crds-helm --version v1.8.1 \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

b. Create

with the following configuration:

```yaml
envoy-gateway-config.yaml
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
container:
image: "docker.io/envoyproxy/ratelimit:1e50889b"
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
image: "docker.io/envoyproxy/ratelimit:1e50889b"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

> [!NOTE]
> Note:   The rate-limit backend uses Valkey Sentinel with master name

```bash
mymaster
```

and the

```bash
nai-valkey-
```

service in

.

```bash
sentinel
nai-system
```

c. Install or Upgrade Envoy Gateway:
```bash
helm upgrade --install eg oci://docker.io/envoyproxy/gateway-helm --version v1.8.1
\
-n envoy-gateway-system --create-namespace --skip-crds \
-f "./envoy-gateway-config.yaml"
```

### 3.  Install or Upgrade KServe:

The required version is KSERVE_VERSION=v0.19.0.

a. Install or upgrade the KServe CRDs:
```bash
helm upgrade --install kserve-crd oci://ghcr.io/kserve/charts/kserve-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

b. Install or upgrade the KServe resources:
```bash
helm upgrade --install kserve oci://ghcr.io/kserve/charts/kserve-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true
```

c. Install or upgrade the KServe LLMInferenceService CRD:
```bash
helm upgrade --install kserve-llmisvc-crd oci://ghcr.io/kserve/charts/kserve-
llmisvc-crd \
--version $KSERVE_VERSION -n kserve --create-namespace --wait
```

d. Install or upgrade the KServe LLMInferenceService resources:
```bash
helm upgrade --install kserve-llmisvc-resources oci://ghcr.io/kserve/charts/
kserve-llmisvc-resources \
--version $KSERVE_VERSION -n kserve --create-namespace --wait \
--set kserve.createSharedResources=false \
--set kserve.llmisvc.createGIECRDs=false
```

### 4.  Install CloudNativePG Operator:

```bash
helm install cnpg cloudnative-pg \
--repo https://cloudnative-pg.github.io/charts \
--version 0.28.0 -n cnpg-system --create-namespace --wait
```

### 5.  Install LeaderWorkerSet:

```bash
helm install lws oci://registry.k8s.io/lws/charts/lws \
--version 0.8.0 -n lws-system --create-namespace --wait
```

### 6.  Install or upgrade OpenTelemetry Operator

```bash
helm upgrade --install opentelemetry-operator opentelemetry-operator \
--repo https://open-telemetry.github.io/opentelemetry-helm-charts \
--version=0.114.1 -n opentelemetry --create-namespace --wait
```

### 7.  Install or upgrade Prometheus Monitoring:

```bash
helm upgrade --install prometheus kube-prometheus-stack \
--repo https://prometheus-community.github.io/helm-charts \
--version=82.13.6 -n prometheus --create-namespace --wait \
--set grafana.enabled=false \
--set prometheus.enabled=false \
--set kubeStateMetrics.enabled=false \
--set alertManager.enabled=false \
--set kubernetesServiceMonitors.enabled=false \
--set prometheus-node-exporter.kubeRBACProxy.enabled=true
```

### 8.  Install or upgrade NVIDIA GPU Operator :

```bash
helm upgrade --install --wait gpu-operator gpu-operator \
--repo https://helm.ngc.nvidia.com/nvidia \
-n gpu-operator --create-namespace --version=v26.3.0
```

### 9.  Apply a resource quota:

```yaml
kubectl create namespace gpu-operator --dry-run=client -o yaml | kubectl apply -f -
kubectl apply -n gpu-operator -f - << EOF
apiVersion: v1
kind: ResourceQuota
metadata:
name: gpu-operator-quota
spec:
hard:
pods: 100
scopeSelector:
matchExpressions:
- operator: In
scopeName: PriorityClass
values:
- system-node-critical
- system-cluster-critical
EOF
```

### What to do next

1. Verify that the Envoy Gateway CRDs and controller are installed and ready.
2. Verify that the KServe CRDs and controller resources are ready.
3. Verify that the CloudNativePG operator is ready.
4. Verify that the LeaderWorkerSet controller is ready.
5. Verify that the OpenTelemetry Operator is ready. 6.  Verify that Prometheus monitoring is ready.
7. Verify that the NVIDIA GPU Operator is ready.

Deploying Nutanix Enterprise AI on Google Kubernetes Engine Install Nutanix Enterprise AI on Google Kubernetes Engine (GKE).

### Before you begin

Ensure to complete the following:

- 

Installing Prerequisite Components

Install components on the Kubernetes cluster. For more information, see

on a Google Kubernetes Engine Cluster

on page  86.

- 

Upgrades from version 2.7.0 to 2.8.0 must be executed during planned downtime. User logins will be unavailable during the upgrade, and full functionality will resume automatically once the upgrade is complete.

### About this task

To install Nutanix Enterprise AI on GKE, follow these steps:

### Procedure

### 1.  Choose a profile:

Profile-based deployment allows you to select a predefined configuration based on your environment and availability requirements. The default profile uses single replica for components and is intended for baseline deployment. The other profiles are for higher capacity usage.

**Table 38: Profile and Capacity**

| Profile | Capacity |
| --- | --- |
| Default | 300 concurrent requests and 100 API Keys |
| c1k_k200 | 1000 concurrent requests and 200 API Keys |
| c5k_k1k | 5000 concurrent requests and 1000 API Keys |

### 2.  Pull and untar both the 2.8.0 charts

```bash
helm pull ntnx-charts/nai-operators --version 2.8.0 --untar=true
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

The extracted charts contain profile files such as:

```yaml
./nai-operators/profiles/c1k_k200.yaml
./nai-operators/profiles/c5k_k1k.yaml
./nai-core/profiles/c1k_k200.yaml
./nai-core/profiles/c5k_k1k.yaml
```

### 3.  Deploy for small profile

a. Deploy NAI Operators for c1k_k200 profile
```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
-f ./nai-operators/profiles/c1k_k200.yaml \
-f ./nai-operators/gke-values.yaml
```

b. Deploy NAI Core for c1k_k200 profile
```bash
export NAI_API_RWX_STORAGECLASS=<RWX/NFS storageclass>
export NAI_DEFAULT_RWO_STORAGECLASS=<RWO default storageclass>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/profiles/c1k_k200.yaml \
-f ./nai-core/gke-values.yaml
```

### 4.  Setup NAI Helm repository

a. Add and update the Nutanix Helm repository, which contains the

Helm chart:

```bash
nai-core
helm repo add ntnx-charts https://nutanix.github.io/helm-releases && helm repo
update ntnx-charts
```

b. Search for the version of the

and

Helm chart available for installation in the

```bash
nai-operators
nai-core
```

Nutanix helm repository:

```bash
helm search repo ntnx-charts/nai-operators --versions
helm search repo ntnx-charts/nai-core --versions
```

### 5.  Create Docker Registry Secrets:

Create the

namespace and the

secret in both

and

```bash
nai-system
docker-registry
nai-system
envoy-
```

namespaces.

```bash
gateway-system
```

The

is already present on the cluster.

```bash
envoy-gateway-system namespace
export REGISTRY_SECRET_NAME=nai-regcred
export DOCKER_SERVER=https://index.docker.io/v1/
export DOCKER_USERNAME=<docker-username>
export DOCKER_PASSWORD=<docker-password>
export DOCKER_EMAIL=<docker-email>
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-system create secret docker-registry ${REGISTRY_SECRET_NAME} \
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-gateway-system create secret docker-registry ${REGISTRY_SECRET_NAME}
\
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
```

### 6.  Deploy NAI Operators:

```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0  -n
nai-system --create-namespace --take-ownership --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}"
```

> [!NOTE]
> Note: Ensure the REGISTRY_SECRET_NAME environment variable is set before running this command.

### 7.  Pull the

```bash
helm chart and extract it to your local repository:
nai-core
helm pull ntnx-charts/nai-core --version 2.8.0 --untar=true
```

### 8.  Install or upgrade NAI Core on GKE:

```bash
# Set the environment variable
NAI_API_RWX_STORAGECLASS=<NFS Storageclass i.e nai-nfs-storage>
NAI_DEFAULT_RWO_STORAGECLASS=<default storageclass>
REGISTRY_SECRET_NAME=<secret created>
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 -n nai-system --
create-namespace --wait \
--set "global.imagePullSecrets[0].name=${REGISTRY_SECRET_NAME}" \
--set "global.storage.storageClassNameRWX=${NAI_API_RWX_STORAGECLASS}" \
--set "global.storage.storageClassName=${NAI_DEFAULT_RWO_STORAGECLASS}" \
-f ./nai-core/gke-values.yaml
```

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

using your own certificate or a cert-manager ClusterIssuer, see

TLS Encryption on Nutanix

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

9. Configure the TLS certificate.

TLS Encryption on Nutanix Enterprise AI

For more information, see

on page 119.

### What to do next

- 

Ensure that all the pods are successfully deployed in the

namespace and are in the

state:

```bash
nai-system
Ready
kubectl get pods -n nai-system
```

The following is a sample output of the command:

```bash
NAME                                                     READY   STATUS
ai-gateway-controller-6644f54c64-z2cwm                   1/1     Running
chi-nai-clickhouse-server-chcluster1-0-0-0               1/1     Running
chk-nai-clickhouse-keeper-chkeeper-0-0-0                 1/1     Running
iam-database-bootstrap-mlgpu-hk5zz                       0/1     Completed
iam-proxy-68d975978d-r7llr                               1/1     Running
iam-proxy-control-plane-d876b77dd-7swx2                  1/1     Running
iam-themis-679b9bbff8-nfknh                              1/1     Running
iam-themis-bootstrap-e1fhx-kglkm                         0/1     Completed
iam-ui-6cb6d49fcc-fhnhm                                  1/1     Running
iam-user-authn-5dbbdcbfcc-9j6jr                          1/1     Running
nai-agent-648c7b8c8d-rzq28                               1/1     Running
nai-api-85b8694cfc-5l5j8                                 1/1     Running
nai-api-db-migrate-lweo1-w8zfx                           0/1     Completed
nai-clickhouse-schema-job-1778237791-fs7p8               0/1     Completed
nai-db-0                                                 1/1     Running
nai-iep-model-controller-b5b8bf9-jxfrk                   1/1     Running
nai-labs-689dcb4644-l6f4p                                1/1     Running
nai-oauth2-proxy-579c5b4d9f-mn72l                        1/1     Running
nai-operators-nai-clickhouse-operator-858fdb9b94-zs8gx   2/2     Running
nai-otel-collector-collector-49bsx                       1/1     Running
nai-otel-collector-collector-6tgh5                       1/1     Running
nai-otel-collector-collector-8qp9x                       1/1     Running
nai-otel-collector-targetallocator-748856644d-5lnff      1/1     Running
nai-securityscan-manager-678ffd75ff-9t95c                1/1     Running
nai-ui-76d74f55bb-c28rw                                  1/1     Running
redis-standalone-67ccd5cc8f-6hp55                        2/2     Running
```

- 

Verify that the Envoy Gateway, ingress gateway, and rate limit pods in the envoy-gateway-system namespace are in the

state:

```bash
Running
kubectl get pods -n envoy-gateway-system
NAME                                                            READY    STATUS
envoy-gateway-6b987d469d-5l2w9                                   1/1     Running
envoy-nai-system-nai-ingress-gateway-ff52ba1f-7fd4897bd4-7smgg   2/2     Running
envoy-ratelimit-85b55c877c-2n2bf                                 1/1     Running
```

- 

Access NAI Dashboard IP:

```bash
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" -
o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

For information on viewing the

#### Dashboard

, see

Log in to Nutanix Enterprise AI

on page 167.

- 

#### Pending

#### Failed

If you had endpoints in

status before the upgrade displaying the status as

with the message

Unable to pull runtime image with provided credentials, hibernate and resume the endpoints.

#### Deploy Nutanix Enterprise AI with Self-managed PostgreSQL

Install or upgrade Nutanix Enterprise AI on a Kubernetes cluster with a self-hosted or managed PostgreSQL database server.

Performing Preflight

Before install or upgrade, you must first perform prelight checks. For more information, see

Checks Before Deploying Nutanix Enterprise AI on Nutanix Kubernetes Platform

on page 42. After

performing preflight checks, you can do any of the following:

- 

Deploy in air-gapped environment. For more information, see

Deploying Nutanix Enterprise AI Components

on a Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments with PostgreSQL

on

page 95.

- 

Deploy on connected clusters. For more information, see

Deploying Nutanix Enterprise AI for a Connected

NKP Cluster

on page 113.

Performing Preflight Checks Before Deploying Nutanix Enterprise AI with Self-managed PostgreSQL Perform preflight checks before installing or upgrading Nutanix Enterprise AI (NAI) on Nutanix Kubernetes Platform (NKP). Preflight checks ensure the Kubernetes environment meets all the technical requirements for a successful NAI installation.

### About this task

To perform preflight checks for installing or upgrading NAI on NKP, follow these steps:

### Procedure

#### 1.  Verify that the Kubernetes version is 1.33 or 1.34:

```bash
kubectl get nodes \
--selector='!node-role.kubernetes.io/control-plane,!node-role.kubernetes.io/master'
\
-o custom-
columns=NODE:.metadata.name,KUBELET_VERSION:.status.nodeInfo.kubeletVersion
```

The expected output is that the Kubernetes version must be 1.33 or 1.34.

#### 2.  Verify if the number of CSI pods matches the number of worker nodes:

```bash
[ $(kubectl get nodes --no-headers | wc -l) -eq $(kubectl get pods -n ntnx-system --
no-headers | grep csi-node | wc -l) ] && echo "# CSI pods = node count" || echo "#
Mismatch: CSI pods != node count"
```

The expected output is CSI pods = node count.

### 3.  Verify that the

class binds volumes immediately:

```bash
nai-nfs-storage
kubectl get storageclass nai-nfs-storage -o jsonpath='{.volumeBindingMode}{"\n"}'
```

The expected output is

.

```bash
immediate
```

### 4.  Verify if the

storage class has ReadWriteMany access:

```bash
nai-nfs-storage
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
name: test-rwx-pvc
namespace: default
spec:
accessModes:
- ReadWriteMany
resources:
requests:
storage: 1Gi
storageClassName: nai-nfs-storage
EOF
kubectl get pvc test-rwx-pvc -n default
```

The expected output is that the storage class

has

set to

.

```bash
nai-nfs-storage
Access Modes
RWX
NAME           STATUS   VOLUME                                     CAPACITY   ACCESS
MODES   STORAGECLASS      VOLUMEATTRIBUTESCLASS   AGE
test-rwx-pvc            Bound    pvc-0df5145a-590c-4f28-9f9d-cefefc55266c   1Gi
RWX            nai-nfs-storage   <unset>                 5s
kubectl delete pvc test-rwx-pvc
```

Deploying Nutanix Enterprise AI Components on a Nutanix Kubernetes® Platform Cluster in Air- Gapped Environments with PostgreSQL Deploy Nutanix Enterprise AI(NAI) components on a Nutanix Kubernetes® Platform (NKP) cluster in an air- gapped environment with a PostgreSQL database.

### Before you begin

- 

Nutanix Enterprise AI supports air-gapped installation only on NKP.

- 

Ensure that you meet requirements listed in

Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a

Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments with PostgreSQL

on page  97.

### About this task

To deploy Nutanix Enterprise AI components on an NKP cluster in an air-gapped environment with PostgreSQL, follow these steps:

### Procedure

1. 

Download the required NAI 2.8.0 release bundles from the Nutanix portal.

Downloading Nutanix Enterprise AI Air-Gap Release Bundles in Air-Gapped

For more information, see

Environments with PostgreSQL

on page 97.

2. 

Push Container Images to a private registry.

From this step onwards, execute the all commands from within your air-gapped environment typically a jumpbox or bastion host that has access to both your container registry and your Kubernetes cluster. Ensure you have Docker and kubectl available on this machine before proceeding.

Pushing Container Images to Private Registry in Air-Gapped Environments

For more information, see

with PostgreSQL

on page  98.

3. 

Extract the NAI Helm charts bundle to your working directory:

```bash
tar -xvf nai-helm-charts-2.8.0.tar
```

The extraction produces the following Helm chart archives:

```bash
gateway-crds-helm-v1.6.3.tgz
gateway-helm-v1.6.3.tgz
kserve-crd-v0.15.0.tgz
kserve-v0.15.0.tgz
nai-core-2.8.0.tgz
nai-operators-2.8.0.tgz
opentelemetry-operator-0.102.0.tgz
```

4. 

(Optional) Publish Helm Charts to Private Registry

To install Helm charts directly from your OCI-compatible registry instead of local files, push the charts using the following commands:

```bash
# Authenticate to your registry
helm registry login -u <username> -p <password> https://<registry>
# Push each chart to the registry
helm push gateway-crds-helm-v1.7.0.tgz oci://<registry>
helm push gateway-helm-v1.7.0.tgz oci://<registry>
helm push kserve-crd-v0.15.0.tgz oci://<registry>
helm push kserve-v0.15.0.tgz oci://<registry>
helm push opentelemetry-operator-0.102.0.tgz oci://<registry>
helm push nai-core-2.8.0.tgz oci://<registry>
helm push nai-operators-2.8.0.tgz oci://<registry>
```

The remaining steps assume installation from local Helm chart files.

5. 

Configure Registry Credentials

Configuring Docker Registry Credentials for Dependencies in an Air-gapped

For more information, see

Environment with PostgreSQL

on page  103.

6. 

Install Prometheus Monitoring from the Nutanix Kubernetes Platform (NKP) platform applications catalog. Prometheus Monitoring is not included in the Nutanix Enterprise AI air-gap image tar or the helm-charts tar. On an NKP cluster in an air-gapped environment, install Prometheus Monitoring from the NKP platform applications catalog before you install Nutanix Enterprise AI components. Enable the Prometheus Monitoring platform application on your NKP workload cluster.

To optimize resource utilization on the workload cluster, configure Prometheus Monitoring with the following minimum installation settings when you enable the application:

```bash
alertmanager:
enabled: false
grafana:
enabled: false
prometheus:
enabled: false
kubeStateMetrics:
enabled: false
kubernetesServiceMonitors:
enabled: false
prometheus-node-exporter.kubeRBACProxy:
kubeRBACProxy:
enabled: true
```

For more information, see

Pro: Enabling an Application Using the UI

.

7. 

Install NVIDIA GPU Operator from NKP platform applications.

If the GPU nodes do not have precompiled NVIDIA drivers installed, enable driver installation in the NVIDIA GPU Operator configuration. This setting ensures that the NVIDIA drivers are installed on the GPU nodes. When enabling the NVIDIA GPU Operator, add the following cluster override:

```bash
driver:
enabled: true
```

For more information, see

Pro: Enabling an Application Using the UI

.

8. 

Install CloudNativePG Operator from NKP platform applications.

9. 

Deploy the LeaderWorkerSet controller for multi-host inference workloads:

```bash
helm upgrade --install lws ./lws-0.8.0.tgz -n lws-system --create-namespace --wait
\
--set "imagePullSecrets[0].name=${IMAGE_PULL_SECRET}" \
--set image.manager.repository=${REGISTRY}/${PROJECT}/nai-lws
```

10. Install Envoy Gateway.

For more information, see

Installing Envoy Gateway in an Air-gapped Environment with PostgreSQL

on

page 104.

11. Install KServe.

Installing KServe in Air-Gapped Environments with PostgreSQL

For more information, see

on

page 106.

12. Deploy the OpenTelemetry Operator.

For more information, see

Deploying the OpenTelemetry Operator in Air-Gapped Environments with

PostgreSQL

on page  106.

13. Install NAI components.

For more information, see

Deploying Nutanix Enterprise AI Components in Air-Gapped Environments

with PostgreSQL

on page  106.

### What to do next

#### 1.  Verify NAI operators installation. Check the status of NAI components:

```bash
# Check all pods in nai-system namespace
kubectl get pods -n nai-system
# Check NAI Operators and Core Helm release
helm list -n nai-system
# Check persistent volume claims
kubectl get pvc -n nai-system
```

### 2.  Verify if NAI services are accessible:

```bash
# List all services in nai-system
kubectl get svc -n nai-system
# Check NAI API service
kubectl get svc -n nai-system nai-api
# Check NAI UI service
kubectl get svc -n nai-system nai-inference-ui
```

3. Troubleshoot common issues that can occur when you are installing NAI 2.8 in air-gapped environments.

For more information, see

Troubleshooting Deployment of Nutanix Enterprise AI 2.8.0 in Air-Gapped

Environments with PostgreSQL

on page  113.

#### Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments with PostgreSQL

Before proceeding with the installation, ensure the following requirements are met:

- 

Nutanix Kubernetes Platform (NKP) cluster running Kubernetes 1.35.

- 
```bash
kubectl CLI v1.33+ configured with cluster access
```

- 

Helm CLI v4.0.5

- 

Docker or compatible container runtime (for loading images)

- 

Access to a private container registry

- 

Registry credentials with appropriate permissions

- 

Sufficient disk space for loading images (~100GB)

#### Downloading Nutanix Enterprise AI Air-Gap Release Bundles in Air-Gapped Environments with PostgreSQL

Download the required Nutanix Enterprise AI 2.8.0 release bundles from the Nutanix Portal.

### About this task

To download the required NAI 2.8.0 release bundles from the Nutanix Portal, follow these steps:

### Procedure

#### 1.  Navigate to the Nutanix Enterprise AI page on the Nutanix Support Portal

#### 2.  Select NAI Version 2.8.0 from the available releases

### 3.  Download the following two bundles:

- 

NAI Air-Gap Bundle (

)

```bash
nai-v2.8.0.tar
```

- 

NAI Helm Charts Bundle (

)

```bash
nai-helm-charts-2.8.0.tar
```

**Table 39: Air-Gap Download Bundles**

| Bundle | Description | Size |
| --- | --- | --- |
| NAI Air-Gap Bundle (nai- v2.8.0.tar) |  | ~70-80GB |

- 

Contains all NAI container images and dependencies

- 

Required for air-gapped deployments

NAI Helm Charts Bundle (nai- helm-charts-2.8.0.tar)

~5 MB

- 

Contains NAI Helm charts and all dependency charts

- 

Includes Envoy Gateway, KServe, LeaderWorkerSet and OpenTelemetry charts

#### 4.  Transfer both bundles to your air-gapped environment using approved methods such as USB drive, secure file

transfer, and so on.

#### Pushing Container Images to Private Registry in Air-Gapped Environments with PostgreSQL

Before installing NAI with PostgreSQL, you must load all container images and push them to your private registry.

### Before you begin

Execute all the commands from within your air-gapped environment typically a jumpbox or bastion host that has access to both your container registry and your Kubernetes cluster. Ensure you have Docker and kubectl available on this machine before proceeding.

### About this task

To push container images to a private registry, follow these steps:

### Procedure

### 1.  Login to your private container registry:

```bash
docker login <registry-url>
```

For example,

```bash
docker login registry.example.com
```

The system displays a prompt to enter your registry credentials.

2. Enter your registry credentials when prompted.
3. Create the Image Push Script.

The Image Push Script pushes container images to your private registry.

a. Create a project/repository with the name

in your container registry, where all NAI images are

```bash
nutanix
```

stored. For example, in Harbor this would be a project named

, resulting in image paths like

```bash
nutanix
```

.

```bash
registry.example.com/nutanix/<image-name>:<tag>
```

b. Create a script file named

with the following content:

```bash
push-images-to-registry.sh
#!/bin/bash
#
# NAI Images - Load, Retag, and Push to Private Registry
#
# This script loads NAI container images from a tar bundle, retags them for your
# private registry, and pushes them to the registry.
#
# Prerequisites:
#   - Docker installed and running
#   - Docker logged into the target registry (docker login)
#   - NAI images tar bundle file
#
# Usage:
#   ./push-images-to-registry.sh <registry-url> <project> <tar-file>
#
# Example:
#   ./push-images-to-registry.sh registry.example.com nutanix nai-images-2.8.0.tar
#
set -uo pipefail
# ============================================================================
# Helper Functions
# ============================================================================
print_header() {
echo ""
echo "========================================"
echo "$1"
echo "========================================"
}
print_success() {
echo "# $1"
}
print_error() {
echo "# ERROR: $1" >&2
}
print_info() {
echo "# $1"
}
# ============================================================================
# Validate Arguments
# ============================================================================
if [ $# -ne 3 ]; then
echo "Usage: $0 <registry-url> <project> <tar-file>"
echo ""
echo "Arguments:"
echo "  registry-url    Your private registry URL (e.g.,
registry.example.com)"
echo "  project         Project/repository name in the registry (e.g.,
nutanix)"
echo "  tar-file        Path to the NAI images tar bundle"
echo ""
echo "Example:"
echo "  $0 registry.example.com nutanix nai-images-2.8.0.tar"
echo ""
exit 1
fi
REGISTRY="$1"
PROJECT="$2"
TAR_FILE="$3"
# Validate tar file exists
if [ ! -f "$TAR_FILE" ]; then
print_error "Tar file not found: $TAR_FILE"
exit 1
fi
# ============================================================================
# Configuration
# ============================================================================
print_header "NAI Images - Load, Retag & Push"
echo "Registry:  $REGISTRY"
echo "Project:   $PROJECT"
echo "Tar File:  $TAR_FILE"
echo "Date:      $(date)"
# Arrays to track images
LOADED_IMAGES=()
FAILED_IMAGES=()
# ============================================================================
# Step 1: Load Images from Tar Bundle
# ============================================================================
print_header "Step 1: Loading Images from Tar Bundle"
print_info "Loading images from $TAR_FILE..."
LOAD_OUTPUT=$(docker load -i "$TAR_FILE" 2>&1)
# Extract loaded image names
while IFS= read -r line; do
if [[ "$line" =~ Loaded\ image:\ (.+)$ ]]; then
LOADED_IMAGES+=("${BASH_REMATCH[1]}")
fi
done <<< "$LOAD_OUTPUT"
if [ ${#LOADED_IMAGES[@]} -eq 0 ]; then
print_error "No images were loaded from the tar file"
exit 1
fi
print_success "Loaded ${#LOADED_IMAGES[@]} images"
# ============================================================================
# Step 2: Retag and Push Images
# ============================================================================
print_header "Step 2: Retagging and Pushing Images"
PUSHED_COUNT=0
TOTAL_IMAGES=${#LOADED_IMAGES[@]}
for source_image in "${LOADED_IMAGES[@]}"; do
echo ""
print_info "[$((PUSHED_COUNT + 1))/$TOTAL_IMAGES] Processing: $source_image"
# Retag image for target registry
# Format: nutanix/nai-api:v2.8.0 # registry.example.com/<project>/nai-
api:v2.8.0
if [[ "$source_image" =~ ^nutanix/(.+)$ ]]; then
image_path="${BASH_REMATCH[1]}"
target_image="${REGISTRY}/${PROJECT}/${image_path}"
print_info "Tagging as: $target_image"
if ! docker tag "$source_image" "$target_image"; then
print_error "Failed to tag image"
FAILED_IMAGES+=("$source_image")
continue
fi
print_info "Pushing to registry..."
if docker push "$target_image"; then
print_success "Pushed successfully"
((PUSHED_COUNT++))
else
print_error "Failed to push image"
FAILED_IMAGES+=("$target_image")
fi
else
print_info "Skipping (not in nutanix/* format)"
fi
done
# ============================================================================
# Summary
# ============================================================================
print_header "Summary"
echo "Total images loaded:    $TOTAL_IMAGES"
echo "Successfully pushed:    $PUSHED_COUNT"
echo "Failed:                 ${#FAILED_IMAGES[@]}"
if [ ${#FAILED_IMAGES[@]} -gt 0 ]; then
echo ""
print_error "The following images failed:"
for img in "${FAILED_IMAGES[@]}"; do
echo "  - $img"
done
echo ""
exit 1
fi
echo ""
print_success "All images successfully pushed to $REGISTRY/$PROJECT"
echo ""
exit 0
```

c. Make the script executable:
```bash
chmod +x push-images-to-registry.sh
```

d. Execute the script to load, retag, and push all NAI images.
```yaml
./push-images-to-registry.sh <registry-url> <project> nai-v2.8.0.tar
```

Example:

```yaml
./push-images-to-registry.sh registry.example.com nutanix nai-v2.8.0.tar
```

Expected Output:

```bash
========================================
NAI Images - Load, Retag & Push
========================================
Registry:  registry.example.com
Project:   nutanix
Tar File:  ./nai-v2.8.0.tar
Date:      Tue Aug 18 05:16:33 PM UTC 2026
========================================
Step 1: Loading Images from Tar Bundle
========================================
# Loading images from ./nai-v2.8.0.tar...
# Loaded 41 images
========================================
Step 2: Retagging and Pushing Images
========================================
# [1/41] Processing: nutanix/nai-iam-proxy-control-plane:v2.8.0
# Tagging as: registry.example.com/nutanix/nai-iam-proxy-control-plane:v2.8.0
# Pushing to registry...
The push refers to repository [registry.example.com/nutanix/nai-iam-proxy-control-
plane]
054a97ddb80b: Pushed
5228eaa6af5b: Pushed
256f393e029f: Mounted from nutanix/nai-inference-ui
v2.8.0: digest:
sha256:587189a6559b7af653769a21f4755c66af14fe5135eaf45728c84a2eb2f3c808 size: 951
# Pushed successfully
# [2/41] Processing: nutanix/nai-iam-ui:v2.8.0
# Tagging as: registry.example.com/nutanix/nai-iam-ui:v2.8.0
# Pushing to registry...
The push refers to repository [registry.example.com/nutanix/nai-iam-ui]
7673a750ed47: Pushed
187de06a3fb0: Pushed
[... continues for all images ...]
========================================
Summary
========================================
Total images loaded:    41
Successfully pushed:    41
Failed:                 0
# All images successfully pushed to registry.example.com/nutanix
```

- 

The image push process typically takes 30-60 minutes depending on your network speed and registry performance.

- 

All images are retagged with your registry URL while preserving the original image path and tag

- 

Original format: nutanix/nai-api:v2.8.0

- 

Retagged format: <your-registry>/nutanix/nai-api:v2.8.0

- 

The script reports failures and continues processing the remaining images.

- 

You can safely re-run the script if it fails partway through.

#### Configuring Docker Registry Credentials for Dependencies in an Air-gapped Environment with PostgreSQL

Configure registry credentials.

### About this task

To configure registry credentials, follow these steps:

### Procedure

1. Set environment Variables.

Export the following environment variables with your private registry credentials:

```bash
export REGISTRY=<registry-url-without-https>
export REGISTRY_USERNAME='<registry-username>'
export REGISTRY_PASSWORD='<registry-password>'
export REGISTRY_EMAIL='<registry-email>'
export IMAGE_PULL_SECRET=nai-docker-regcred
export PROJECT=nutanix # set the registry project name
```

2. Replace the placeholder values with your actual registry information.

The

must not include the

protocol prefix.

```bash
REGISTRY
https://
```

3. Create Image Pull Secrets.

Create Kubernetes namespaces and docker-registry secrets for Envoy Gateway System :

```yaml
kubectl create namespace envoy-gateway-system --dry-run=client -o yaml | kubectl
apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n envoy-gateway-system \
--dry-run=client -o yaml | kubectl apply -f -
```

### 4.  Create Kubernetes namespaces and

secrets for KServe:

```bash
docker-registry
kubectl create namespace kserve --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n kserve \
--dry-run=client -o yaml | kubectl apply -f -
```

#### 5.  Create Kubernetes namespaces and docker-registry secrets for OpenTelemetry:

```yaml
kubectl create namespace opentelemetry --dry-run=client -o yaml | kubectl apply -f -
kubectl create secret docker-registry ${IMAGE_PULL_SECRET} \
--docker-server=${REGISTRY} \
--docker-username=${REGISTRY_USERNAME} \
--docker-password=${REGISTRY_PASSWORD} \
--docker-email=${REGISTRY_EMAIL} \
-n opentelemetry \
--dry-run=client -o yaml | kubectl apply -f -
```

#### Installing Envoy Gateway in an Air-gapped Environment with PostgreSQL

Install Envoy Gateway.

### Before you begin

Ensure that you meet requirements listed in

Prerequisites for Deploying Nutanix Enterprise AI 2.8.0 on a

Nutanix Kubernetes® Platform Cluster in Air-Gapped Environments

on page  52.

### About this task

To install Envoy Gateway, follow these steps:

### Procedure

#### 1.  Install the Envoy Gateway and Gateway API Custom Resource Definitions (CRDs):

```bash
helm template eg ./gateway-crds-helm-v1.8.1.tgz \
--set crds.gatewayAPI.enabled=true \
--set crds.envoyGateway.enabled=true \
| kubectl apply --server-side --force-conflicts -f -
```

This command also installs the necessary Gateway API CRDs required for Envoy Gateway.

### 2.  Create the configuration template file

:

```yaml
eg-config-for-gateway-mode.yaml.template
# This file configures Envoy Gateway for AI Gateway mode with rate limiting
config:
envoyGateway:
gateway:
controllerName: "gateway.envoyproxy.io/gatewayclass-controller"
logging:
level:
default: "info"
provider:
kubernetes:
rateLimitDeployment:
patch:
type: "StrategicMerge"
value:
spec:
template:
spec:
containers:
- imagePullPolicy: "IfNotPresent"
name: "envoy-ratelimit"
env:
- name: REDIS_TYPE
value: "sentinel"
- name: REDIS_PIPELINE_WINDOW
value: "150us"
type: "Kubernetes"
extensionApis:
enableEnvoyPatchPolicy: true
enableBackend: true
extensionManager:
maxMessageSize: 11Mi
backendResources:
- group: inference.networking.k8s.io
kind: InferencePool
version: v1
hooks:
xdsTranslator:
translation:
listener:
includeAll: true
route:
includeAll: true
cluster:
includeAll: true
secret:
includeAll: true
post:
- "Translation"
- "Cluster"
- "Route"
service:
fqdn:
hostname: "ai-gateway-controller.nai-system.svc.cluster.local"
port: 1063
rateLimit:
backend:
type: "Redis"
redis:
url: "mymaster,nai-valkey-sentinel.nai-system.svc.cluster.local:26379"
```

3. Ensure the REGISTRY environment variable is configured.

### 4.  Generate the actual configuration file using

:

```bash
envsubst
envsubst < eg-config-for-gateway-mode.yaml.template > eg-config-for-gateway-mode.yaml
```

### 5.  Deploy Envoy Gateway:

```bash
helm upgrade --install eg ./gateway-helm-v1.8.1.tgz \
-n envoy-gateway-system --create-namespace --wait \
--set global.images.envoyGateway.image=${REGISTRY}/${PROJECT}/nai-gateway:v1.8.1 \
--set global.images.ratelimit.image=${REGISTRY}/${PROJECT}/nai-ratelimit:1e50889b \
--set "global.imagePullSecrets[0].name=${IMAGE_PULL_SECRET}" \
-f ./eg-config-for-gateway-mode.yaml
```

The configuration file now uses your private registry for the

images through the

```bash
ratelimit
${REGISTRY}
```

variable substitution.

#### Installing KServe in Air-Gapped Environments with PostgreSQL

Install KServe.

### About this task

To install KServe, follow these steps:

### Procedure

#### 1.  Install the KServe Custom Resource Definitions (CRDs:)

```bash
helm upgrade --install kserve-crd ./kserve-crd-v0.19.0.tgz -n kserve --create-
namespace --wait
```

#### 2.  Deploy the KServe controller with RawDeployment mode:

```bash
helm upgrade --install kserve ./kserve-resources-v0.19.0.tgz \
-n kserve --wait \
--set kserve.controller.deploymentMode=RawDeployment \
--set kserve.controller.gateway.disableIngressCreation=true \
--set kserve.controller.image=${REGISTRY}/${PROJECT}/nai-kserve-controller \
--set kserve.controller.rbacProxyImage=${REGISTRY}/${PROJECT}/nai-kube-rbac-
proxy:v0.18.0 \
--set "kserve.controller.imagePullSecrets[0].name=${IMAGE_PULL_SECRET}"
```

#### Deploying the OpenTelemetry Operator in Air-Gapped Environments with PostgreSQL

Deploy the OpenTelemetry Operator for observability and telemetry collection.

### About this task

To deploy the OpenTelemetry Operator, run the following command:

### Procedure

Deploy the OpenTelemetry Operator for observability and telemetry collection:

```bash
helm upgrade --install opentelemetry-operator ./opentelemetry-operator-0.114.1.tgz \
-n opentelemetry --create-namespace --wait \
--set manager.image.repository=${REGISTRY}/${PROJECT}/nai-opentelemetry-operator \
--set manager.collectorImage.repository=${REGISTRY}/${PROJECT}/nai-opentelemetry-
collector-contrib \
--set "imagePullSecrets[0].name=${IMAGE_PULL_SECRET}"
```

#### Deploying Nutanix Enterprise AI Components in Air-Gapped Environments with PostgreSQL

Deploy Nutanix Enterprise AI components in air-gapped environments.

### Before you begin

If you are upgrading from NAI 2.7 to 2.8, use the following mapping to migrate your environment variables from the NAI 2.7 deployment

#### NAI 2.8 Environment Variable

#### NAI 2.7Environment Variable

#### Notes

IAM_DB_HOST_URL

POSTGRES_HOST_URL

IAM_DB_NAME

nai_iam

Hardcoded default value

IAM_DB_USERNAME

POSTGRES_USERNAME

#### NAI 2.8 Environment Variable

#### NAI 2.7Environment Variable

#### Notes

IAM_DB_PASSWORD

POSTGRES_PASSWORD

IAM_DB_PORT

Not Applicable

Check with your DB provider, defaults to

if

```bash
5432
```

left empty

IAM_DB_SSLMODE

SSL_MODE

Check with your Postgres DB provider, defaults to

if left empty

```bash
disable
```

IEP_DB_HOST_URL

POSTGRES_HOST_URL

IEP_DB_NAME

CONFIGURED_DB_NAME

IEP_DB_USERNAME

POSTGRES_USERNAME

IEP_DB_PASSWORD

POSTGRES_PASSWORD

IEP_DB_PORT

Not Applicable

Check with your Postgres DB provider, defaults to

if left empty

```bash
5432
```

IEP_DB_SSLMODE

SSL_MODE

Check with your DB provider, defaults to

```bash
disable
```

if left empty

### About this task

To deploy Nutanix Enterprise AI components in air-gapped environments, follow these steps:

### Procedure

### 1.  Create the

namespace and configure the image pull secret:

```bash
nai-system
# Set environment variables (if not already set from Step 3)
export REGISTRY=<registry-url-without-https>
export REGISTRY_USERNAME='<registry-username>'
export REGISTRY_PASSWORD='<registry-password>'
export REGISTRY_EMAIL='<registry-email>'
export IMAGE_PULL_SECRET=nai-docker-regcred
export PROJECT=nutanix # set the registry project name
# Managed Postgres DB Details
# There are 2 DBs used by NAI, you can choose to use single instance or different as
per usecase
# IAM DB details
export IAM_DB_HOST_URL=<hostname>
export IAM_DB_NAME=<DB name>
export IAM_DB_USERNAME=<DB username>
export IAM_DB_PASSWORD=<DB password>
export IAM_DB_PORT=5432 # Update as per your DB port
export IAM_DB_SSLMODE=disable # Supported values: "disable", "require", "verify-ca",
"verify-full"
# Below fields to be used for "verify-full" and "verify-ca" mode
export IAM_SSL_SECRET_NAME=nai-db-certs # This secret to be precreated
export IAM_SSL_ROOT_CERT_NAME=<root cert name>
export IAM_SSL_CLIENT_CERT_NAME="" # client cert name
export IAM_SSL_CLIENT_KEY_NAME="" # client key name
# IEP DB details
export IEP_DB_HOST_URL=<hostname>
export IEP_DB_NAME=<DB name>
export IEP_DB_USERNAME=<DB username>
export IEP_DB_PASSWORD=<DB password>
export IEP_DB_PORT=5432 # Update as per your DB port
export IEP_DB_SSLMODE=disable # Supported values: "disable", "require", "verify-ca",
"verify-full"
# Below fields to be used for "verify-full" and "verify-ca" mode
export IEP_SSL_SECRET_NAME=nai-db-certs # This secret to be precreated
export IEP_SSL_ROOT_CERT_NAME=<root cert name>
export IEP_SSL_CLIENT_CERT_NAME="" # client cert name
export IEP_SSL_CLIENT_KEY_NAME="" # client key name
# Storage class for ReadWriteMany (RWX) volumes - used by NAI API
export NAI_API_RWX_STORAGECLASS=<your-rwx-storage-class>
# Storage class for ReadWriteOnce (RWO) volumes - default storage class
export NAI_DEFAULT_RWO_STORAGECLASS=<your-rwo-storage-class>
# NKP workspace namespace for monitoring (if using Nutanix Kubernetes Platform)
export NKP_WORKSPACE_NAMESPACE=<workspace-namespace>
```

Storage Class Examples:

- 

For NFS-based storage: Use your NFS storage class name (e.g., nai-nfs-storage)

- 

For Nutanix Volumes: Use nutanix-volumes or your configured storage class

- 

For RWO: Common options include local-path, nutanix-volumes, or your default storage class

Verification: Check available storage classes:

```bash
kubectl get storageclass
```

#### 2.  Create a values override file for NAI Operators using the provided template. This file configures all operator

images to use your private registry.

a. Create a file named

with the following content:

```yaml
darksite-nai-operators.yaml.template
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
naiValkey:
image:
name: ${REGISTRY}/${PROJECT}/nai-valkey
naiJobs:
naiJobsImage:
image: ${REGISTRY}/${PROJECT}/nai-jobs
nai-clickhouse-operator:
operator:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-operator
metrics:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-metrics-exporter
ai-gateway-helm:
extProc:
image:
repository: ${REGISTRY}/${PROJECT}/nai-ai-gateway-extproc
controller:
image:
repository: ${REGISTRY}/${PROJECT}/nai-ai-gateway-controller
naiDatabase:
external: true
image: ${REGISTRY}/${PROJECT}/nai-postgresql:17.10-standard-trixie
clusters:
iep:
host: ${IEP_DB_HOST_URL}
database: ${IEP_DB_NAME}
username: ${IEP_DB_USERNAME}
password: ${IEP_DB_PASSWORD}
port: ${IEP_DB_PORT}
sslMode: ${IEP_DB_SSLMODE}
iam:
host: ${IAM_DB_HOST_URL}
database: ${IAM_DB_NAME}
username: ${IAM_DB_USERNAME}
password: ${IAM_DB_PASSWORD}
port: ${IAM_DB_PORT}
sslMode: ${IAM_DB_SSLMODE}
```

### 3.  Generate the actual values file

Use

to replace environment variables and create the final values file:

```bash
envsubst
envsubst < darksite-nai-operators.yaml.template > darksite-nai-operators.yaml
```

The envsubst command substitutes

and

with the actual values from

```json
${REGISTRY}
${IMAGE_PULL_SECRET}
```

your environment variables.

### 4.  Install NAI Operators

```bash
helm upgrade --install nai-operators ./nai-operators-2.8.0.tgz \
-n nai-system --create-namespace --wait --timeout 15m -f ./darksite-nai-
operators.yaml
```

### 5.  Prepare NAI Core Values Override File

Create a values override file for NAI Core using the provided template. This configures all NAI core component images.

a. Create the template file named

with the following content:

```yaml
darksite-nai-core.yaml.template
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
storageClassNameRWX: ${NAI_API_RWX_STORAGECLASS}
gateway:
envoyDeployment:
container:
image: ${REGISTRY}/${PROJECT}/nai-envoy:distroless-v1.38.0
naiIepOperator:
iepOperatorImage:
image: ${REGISTRY}/${PROJECT}/nai-iep-operator
modelProcessorImage:
image: ${REGISTRY}/${PROJECT}/nai-python-processor
dataSourceProcessorImage:
image: ${REGISTRY}/${PROJECT}/nai-python-processor
batchInferenceProcessor:
containers:
processor:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
statusProvider:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
finetuneProcessor:
containers:
processor:
image: ${REGISTRY}/${PROJECT}/nai-finetuning
statusProvider:
image: ${REGISTRY}/${PROJECT}/nai-go-processor
naiInferenceUi:
naiUiImage:
image: ${REGISTRY}/${PROJECT}/nai-inference-ui
naiJobs:
naiJobsImage:
image: ${REGISTRY}/${PROJECT}/nai-jobs
naiApi:
naiApiImage:
image: ${REGISTRY}/${PROJECT}/nai-api
supportedTGIImage: ${REGISTRY}/${PROJECT}/nai-tgi
supportedKserveRuntimeImage: ${REGISTRY}/${PROJECT}/nai-kserve-huggingfaceserver
eppImage: ${REGISTRY}/${PROJECT}/nai-epp-inference-scheduler
supportedVLLMImage: ${REGISTRY}/${PROJECT}/nai-vllm
supportedKserveCustomModelServerRuntimeImage: ${REGISTRY}/${PROJECT}/nai-kserve-
custom-model-server
naiDatabase:
external: true
clientImage: ${REGISTRY}/${PROJECT}/nai-postgresql:17.10-standard-trixie
clusters:
iep:
host: ${IEP_DB_HOST_URL}
database: ${IEP_DB_NAME}
port: ${IEP_DB_PORT}
sslMode: ${IEP_DB_SSLMODE}
sslSecretName: ${IEP_SSL_SECRET_NAME}
sslRootCertName: {IEP_SSL_ROOT_CERT_NAME}
sslClientCertName: {IEP_SSL_CLIENT_CERT_NAME}
sslClientKeyName: {IEP_SSL_CLIENT_KEY_NAME}
iam:
host: ${IAM_DB_HOST_URL}
database: ${IAM_DB_NAME}
port: ${IAM_DB_PORT}
sslMode: ${IAM_DB_SSLMODE}
sslSecretName: ${IAM_SSL_SECRET_NAME}
sslRootCertName: {IAM_SSL_ROOT_CERT_NAME}
sslClientCertName: {IAM_SSL_CLIENT_CERT_NAME}
sslClientKeyName: {IAM_SSL_CLIENT_KEY_NAME}
naiIam:
iamProxy:
image: ${REGISTRY}/${PROJECT}/nai-iam-proxy
iamProxyControlPlane:
image: ${REGISTRY}/${PROJECT}/nai-iam-proxy-control-plane
iamUi:
image: ${REGISTRY}/${PROJECT}/nai-iam-ui
iamUserAuthn:
image: ${REGISTRY}/${PROJECT}/nai-iam-user-authn
iamThemis:
image: ${REGISTRY}/${PROJECT}/nai-iam-themis
iamThemisBootstrap:
image: ${REGISTRY}/${PROJECT}/nai-iam-bootstrap
naiAgent:
agentImage:
image: ${REGISTRY}/${PROJECT}/nai-agent-app
naiLabs:
labsImage:
image: ${REGISTRY}/${PROJECT}/nai-rag-app
nai-clickhouse-keeper:
clickhouseKeeper:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-keeper
oauth2-proxy:
image:
repository: ${REGISTRY}/${PROJECT}/nai-oauth2-proxy
nai-clickhouse-server:
clickhouse:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-server
initContainers:
addUdf:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-udf
waitForKeeper:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-jobs
nai-clickhouse-schemas:
image:
registry: ${REGISTRY}
repository: ${PROJECT}/nai-clickhouse-schemas
naiMonitoring:
opentelemetry:
collectorImage: ${REGISTRY}/${PROJECT}/nai-opentelemetry-collector-
contrib:0.152.0
targetAllocator:
image:
repository: ${REGISTRY}/${PROJECT}/nai-target-allocator
nodeExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- prometheus
- kommander
- kommander-default-workspace
- ${NKP_WORKSPACE_NAMESPACE}
dcgmExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- prometheus
- kommander
- kommander-default-workspace
- ${NKP_WORKSPACE_NAMESPACE}
```

b. Ensure that REGISTRY, IMAGE_PULL_SECRET, NAI_API_RWX_STORAGECLASS,

NAI_DEFAULT_RWO_STORAGECLASS, and NKP_WORKSPACE_NAMESPACE environment variables are set before running the next command.

c. Generate the actual values file

Use

to replace environment variables and create the final values file:

```bash
envsubst
envsubst < darksite-nai-core.yaml.template > darksite-nai-core.yaml
```

### 6.  Install NAI Core

```bash
helm upgrade --install nai-core ./nai-core-2.8.0.tgz -n nai-system --create-namespace
--wait --timeout 15m \
-f ./darksite-nai-core.yaml
```

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

using your own certificate or a cert-manager ClusterIssuer, see

TLS Encryption on Nutanix

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

7. Configure the TLS certificate.

For more information, see

TLS Encryption on Nutanix Enterprise AI

on page 119.

#### Troubleshooting Deployment of Nutanix Enterprise AI 2.8.0 in Air-Gapped Environments with PostgreSQL

The following troubleshooting tips can help you resolve a few common issues that can occur when you are installing

Image Pull Errors

Verify that:

- 

Registry credentials are correct

- 

Image pull secrets exist in the correct namespaces

- 

Registry URL is accessible from the cluster

- 

Image paths match your registry structure

- 

All required images are present in your private registry

CRD Installation Failures

Ensure you have sufficient permissions to create cluster-scoped resources.

Helm Installation Timeouts

Increase the timeout value using --timeout flag (e.g., --timeout 20m).

Storage Class Issues

- 

Verify the storage class exists: kubectl get storageclass

- 

Ensure the storage class supports the required access mode (RWX for NAI API, RWO for others)

- 

Check PVC status: kubectl get pvc -n nai-system

Pod Startup Failures

- 

Check pod logs: kubectl logs -n nai-system <pod-name>

- 

Describe pod for events: kubectl describe pod -n nai-system <pod-name>

- 

Verify resource limits if running on resource-constrained clusters

Dependency Issues

Ensure all dependencies (Envoy Gateway, KServe, OpenTelemetry) are installed and running before installing NAI Core.

Deploying Nutanix Enterprise AI for a Connected NKP Cluster Install or upgrade Nutanix Enterprise AI for a connected NKP cluster.

### Before you begin

- 

Installing Prerequisite Components

Install components on the Kubernetes cluster. For more information, see

on a Nutanix Kubernetes Platform Cluster

on page  43.

- 

If you are upgrading from NAI 2.7 to 2.8, use the following mapping to migrate your environment variables from the NAI 2.7 deployment

#### NAI 2.8 Environment Variable

#### NAI 2.7Environment Variable

#### Notes

IAM_DB_HOST_URL

POSTGRES_HOST_URL

IAM_DB_NAME

nai_iam

Hardcoded default value

IAM_DB_USERNAME

POSTGRES_USERNAME

IAM_DB_PASSWORD

POSTGRES_PASSWORD

IAM_DB_PORT

Not Applicable

Check with your DB provider, defaults to

if

```bash
5432
```

left empty

IAM_DB_SSLMODE

SSL_MODE

Check with your Postgres DB provider, defaults to

if left empty

```bash
disable
```

IEP_DB_HOST_URL

POSTGRES_HOST_URL

IEP_DB_NAME

CONFIGURED_DB_NAME

IEP_DB_USERNAME

POSTGRES_USERNAME

IEP_DB_PASSWORD

POSTGRES_PASSWORD

IEP_DB_PORT

Not Applicable

Check with your Postgres DB provider, defaults to

if left empty

```bash
5432
```

IEP_DB_SSLMODE

SSL_MODE

Check with your DB provider, defaults to

```bash
disable
```

if left empty

### About this task

To install or upgrade Nutanix Enterprise AI for a connected NKP cluster, follow these steps:

### Procedure

1. Set up the Nutanix Enterprise AI Helm repository.
a. Add and update the Nutanix Helm repository, which contains the

Helm chart:

```bash
nai-core
helm repo add ntnx-charts https://nutanix.github.io/helm-releases && helm repo
update ntnx-charts
```

b. Search for the version of the

and

Helm chart available for installation in the

```bash
nai-operators
nai-core
```

Nutanix Helm repository:

```bash
helm search repo ntnx-charts/nai-operators --versions
helm search repo ntnx-charts/nai-core --versions
```

### 2.  Install or upgrade NAI Core on NKP:

```bash
export REGISTRY_SECRET_NAME=nai-regcred
export DOCKER_SERVER=https://index.docker.io/v1/
export DOCKER_USERNAME=<docker-username>
export DOCKER_PASSWORD=<docker-password>
export DOCKER_EMAIL=<docker-email>
# Managed Postgres DB Details
# There are 2 DBs used by NAI, you can choose to use single instance or different as
per usecase
# IAM DB details
export IAM_DB_HOST_URL=<hostname>
export IAM_DB_NAME=<DB name>
export IAM_DB_USERNAME=<DB username>
export IAM_DB_PASSWORD=<DB password>
export IAM_DB_PORT=5432 # Update as per your DB port
export IAM_DB_SSLMODE=disable # Supported values: "disable", "require", "verify-ca",
"verify-full"
# Below fields to be used for "verify-full" and "verify-ca" mode
export IAM_SSL_SECRET_NAME=nai-db-certs # This secret to be precreated
export IAM_SSL_ROOT_CERT_NAME=<root cert name>
export IAM_SSL_CLIENT_CERT_NAME="" # client cert name
export IAM_SSL_CLIENT_KEY_NAME="" # client key name
# IEP DB details
export IEP_DB_HOST_URL=<hostname>
export IEP_DB_NAME=<DB name>
export IEP_DB_USERNAME=<DB username>
export IEP_DB_PASSWORD=<DB password>
export IEP_DB_PORT=5432 # Update as per your DB port
export IEP_DB_SSLMODE=disable # Supported values: "disable", "require", "verify-ca",
"verify-full"
# Below fields to be used for "verify-full" and "verify-ca" mode
export IEP_SSL_SECRET_NAME=nai-db-certs # This secret to be precreated
export IEP_SSL_ROOT_CERT_NAME=<root cert name>
export IEP_SSL_CLIENT_CERT_NAME="" # client cert name
export IEP_SSL_CLIENT_KEY_NAME="" # client key name
# Storage class for ReadWriteMany (RWX) volumes - used by NAI API
export NAI_API_RWX_STORAGECLASS=<your-rwx-storage-class>
# Storage class for ReadWriteOnce (RWO) volumes - default storage class
export NAI_DEFAULT_RWO_STORAGECLASS=<your-rwo-storage-class>
# NKP workspace namespace for monitoring (if using Nutanix Kubernetes Platform)
export NKP_WORKSPACE_NAMESPACE=<workspace-namespace>
```

> [!NOTE]
> Note:   Ensure that the configured PostgreSQL user credentials possess database creation privileges on the target PostgreSQL host.

> [!NOTE]
> Note:   You can append optional Helm overrides to the nai-core installation command to customize the deployment

- 

Enable the Chat and Talk to My Data application. These applications are disabled by default. To enable it, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiLabs.enabled=true"
```

- 

Enable HTTPS with a self-signed certificate. By default, NAI does not provision a TLS certificate for the ingress gateway. To quickly enable HTTPS with a self-signed certificate (recommended for dev/test environments), add the following flag to the

Helm install command:

```bash
nai-core
--set "gateway.certManager.selfSigned=true"
```

This requires

to be installed on the cluster. For production TLS options, including

```bash
cert-manager
```

using your own certificate or a cert-manager ClusterIssuer, see

TLS Encryption on Nutanix

Enterprise AI

.

- 

Scale out the ingress gateway and NAI API. To increase replicas for the ingress gateway and the NAI API, add the following flags to the

Helm install command:

```bash
nai-core
--set "gateway.replicaCount=<Number_of_replicas>"
--set "naiApi.replicaCount=<Number_of_replicas>"
```

The default is 1 replica each. Increase based on your scale requirements.

- 

Configure PostgreSQL database connections. To adjust the maximum number of concurrent PostgreSQL connections, add the following flag to the

Helm install command:

```bash
nai-core
--set "naiDatabase.postgresConfig.maxConnections=<Number_of_Connections>"
```

The default value is 1000. Increase this value if you expect a higher number of concurrent clients.

### 3.  Create Docker Registry Secrets

Create the

and the docker-registry secret in both

and

```bash
nai-system namespace
nai-system
envoy-gateway-
```

namespaces. The

namespace is already present on the cluster.

```bash
system
envoy-gateway-system
kubectl create namespace nai-system --dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-system create secret docker-registry ${REGISTRY_SECRET_NAME} \
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-gateway-system create secret docker-registry ${REGISTRY_SECRET_NAME}
\
--docker-server=${DOCKER_SERVER} \
--docker-username=${DOCKER_USERNAME} \
--docker-password=${DOCKER_PASSWORD} \
--docker-email=${DOCKER_EMAIL} \
--dry-run=client -o yaml | kubectl apply -f -
```

### 4.  Create a values override file

for NAI Operators using the

```yaml
nai-operators-external-db.yaml.template
```

provided template.

```bash
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
naiDatabase:
external: true
clusters:
iep:
host: ${IEP_DB_HOST_URL}
database: ${IEP_DB_NAME}
username: ${IEP_DB_USERNAME}
password: ${IEP_DB_PASSWORD}
port: ${IEP_DB_PORT}
sslMode: ${IEP_DB_SSLMODE}
iam:
host: ${IAM_DB_HOST_URL}
database: ${IAM_DB_NAME}
username: ${IAM_DB_USERNAME}
password: ${IAM_DB_PASSWORD}
port: ${IAM_DB_PORT}
sslMode: ${IAM_DB_SSLMODE}
```

Generate the actual values file

```yaml
envsubst < nai-operators-external-db.yaml.template > nai-operators-external-db.yaml
```

### 5.  Install NAI Operators:

```bash
helm upgrade --install nai-operators ntnx-charts/nai-operators --version 2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
-f ./nai-operators-external-db.yaml
```

#### 6.  Create a values override file(`nai-core-external-db.yaml.template`) for NAI Operators using the provided

template.

```bash
global:
imagePullSecrets:
- name: ${IMAGE_PULL_SECRET}
storage:
storageClassName: ${NAI_DEFAULT_RWO_STORAGECLASS}
storageClassNameRWX: ${NAI_API_RWX_STORAGECLASS}
naiDatabase:
external: true
clusters:
iep:
host: ${IEP_DB_HOST_URL}
database: ${IEP_DB_NAME}
port: ${IEP_DB_PORT}
sslMode: ${IEP_DB_SSLMODE}
sslSecretName: ${IEP_SSL_SECRET_NAME}
sslRootCertName: {IEP_SSL_ROOT_CERT_NAME}
sslClientCertName: {IEP_SSL_CLIENT_CERT_NAME}
sslClientKeyName: {IEP_SSL_CLIENT_KEY_NAME}
iam:
host: ${IAM_DB_HOST_URL}
database: ${IAM_DB_NAME}
port: ${IAM_DB_PORT}
sslMode: ${IAM_DB_SSLMODE}
sslSecretName: ${IAM_SSL_SECRET_NAME}
sslRootCertName: {IAM_SSL_ROOT_CERT_NAME}
sslClientCertName: {IAM_SSL_CLIENT_CERT_NAME}
sslClientKeyName: {IAM_SSL_CLIENT_KEY_NAME}
naiMonitoring:
nodeExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- ${NKP_WORKSPACE_NAMESPACE}
dcgmExporter:
serviceMonitor:
namespaceSelector:
matchNames:
- ${NKP_WORKSPACE_NAMESPACE}
```

Generate the actual values file

```yaml
envsubst < nai-core-external-db.yaml.template > nai-core-external-db.yaml
```

### 7.  Install NAI Core

```bash
helm upgrade --install nai-core ntnx-charts/nai-core --version=2.8.0 \
-n nai-system --create-namespace --wait --timeout 15m \
-f ./nai-core-external-db.yaml
```

8. Configure the TLS certificate.

TLS Encryption on Nutanix Enterprise AI

For more information, see

on page 119.

### What to do next

- 

Ensure that all the pods are successfully deployed in the

namespace and are in the

state by

```bash
nai-system
Ready
```

running:

```bash
kubectl get pods -n nai-system
```

The following is a sample output of the command. Match the pod names by ignoring the auto-generated suffix added by Kubernetes and their status.

> [!NOTE]
> Note:

The number of

should be equal to the number of worker

```bash
nai-otel-collector-collector pods
```

nodes in the NAI Kubernetes cluster.

```bash
NAME                                                            READY   STATUS
RESTARTS   AGE
chi-nai-clickhouse-server-chcluster1-0-0-0                      1/1     Running     0
16h
chk-nai-clickhouse-keeper-chkeeper-0-0-0                        1/1     Running     0
16h
iam-database-bootstrap-b8etj-hk9rz                              0/1     Completed   0
16h
iam-proxy-68f9459885-zcwgs                                      1/1     Running     0
16h
iam-proxy-control-plane-6897669d64-rvglt                        1/1     Running     0
16h
iam-themis-749b7b56f8-pmclb                                     1/1     Running     0
16h
iam-themis-bootstrap-qgczx-p7bl6                                0/1     Completed   0
16h
iam-ui-6697d94478-fftl5                                         1/1     Running     0
16h
iam-user-authn-5b4dcfdfb7-jhllq                                 1/1     Running     0
16h
nai-api-784fb7b99-8tch9                                         1/1     Running     0
16h
nai-api-db-migrate-mgnba-cl7gw                                  0/1     Completed   0
16h
nai-clickhouse-schema-job-1771350985-bd4sq                      0/1     Completed   0
16h
nai-db-0                                                        1/1     Running     0
16h
nai-iep-model-controller-5cd8bcd5f-9h9cl                        1/1     Running     0
16h
nai-labs-86589cc95d-gk87q                                       1/1     Running     0
16h
nai-oauth2-proxy-5746ccc8b7-65jmf                               1/1     Running     0
16h
nai-oidc-client-registration-lvrrv-pbdsn                        0/1     Completed   0
16h
nai-otel-collector-collector-4pxtl                              1/1     Running     0
16h
nai-otel-collector-collector-8p4w8                              1/1     Running     0
16h
nai-otel-collector-collector-bwstg                              1/1     Running     0
16h
nai-otel-collector-collector-drddv                              1/1     Running     0
16h
nai-otel-collector-collector-k457n                              1/1     Running     0
16h
```

### Gateway TLS-Related Helm Values

**Table 43: Gateway TLS Helm values**

| Property 1 | Value | Type | Default | Description | Property 6 |
| --- | --- | --- | --- | --- | --- |
|  | gateway.tlsSecretName | string | "ingress- | Name of the Kubernetes TLS secret referenced by the Gateway HTTPS listener. |  |
|  |  |  | certificate" |  |  |
|  | gateway.certManager.selfSigned | bool | false | When true, creates a self-signed Issuer and Certificate for dev/test use. |  |
|  | gateway.certManager.issuerRef | object | null | Reference to an existing cert-manager Issuer or ClusterIssuer. Must include name and kind; optionally group. |  |
|  | gateway.certManager.issuerRef.name | string | - | Name of the Issuer or ClusterIssuer. |  |
|  | gateway.certManager.issuerRef.kind | string | - | Issuer or ClusterIssuer. |  |
|  | gateway.certManager.issuerRef.group | string | cert-manager.io | API group of the issuer (usually left as default). |  |
|  | gateway.certManager.dnsNames | list | [] | DNS Subject Alternative Names for the certificate. Required when |  |
|  |  |  |  | issuerRef | is set. The |
|  |  |  |  | first entry is used as |  |
|  |  |  |  | commonName | . |
| Accessing the NAI Dashboard IP Address |  |  |  |  |  |
|  | Retrieve the IP address for the NAI dashboard. |  |  |  |  |

### Procedure

### 1.  Get the NAI dashboard IP address:

```bash
kubectl get svc -n envoy-gateway-system -l "gateway.envoyproxy.io/owning-gateway-
name=nai-ingress-gateway,gateway.envoyproxy.io/owning-gateway-namespace=nai-system" -
o jsonpath='{.items[0].status.loadBalancer.ingress[0].ip}'
```

The expected output is the IP address.

2. Copy the IP address and paste IP address in your browser.

The NAI login page is displayed.

3. Log in to NAI.

Log in to Nutanix Enterprise AI

For information on viewing the dashboard, see

on page 167.

#### Rotating Docker Registry Credentials

Rotate the Docker registry credentials for Nutanix Enterprise AI to maintain secure access to the registry. Rotate the credentials when the existing credentials expire or when you suspect they have been compromised. Regular credential rotation replaces old authentication secrets with new ones, reducing the risk of unauthorized access.

### About this task

When registry credentials expire or are compromised, you must update the secret in all namespaces where NAI components run.

To rotate Docker registry credentials, follow these steps:

### Procedure

### 1.  Rotate credentials:

```bash
export EXISTING_SECRET_NAME=<docker-registry-secret-name> # Use the same registry
secret name specified during the NAI installation or upgrade.
export REGISTRY=<container-registry> # For Docker: https://index.docker.io/v1/
kubectl -n nai-system create secret docker-registry ${EXISTING_SECRET_NAME} \
--docker-server=${REGISTRY} \
--docker-username=<username> \
--docker-password=<password> \
--docker-email=<email> \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n nai-admin create secret docker-registry ${EXISTING_SECRET_NAME} \
--docker-server=${REGISTRY} \
--docker-username=<username> \
--docker-password=<password> \
--docker-email=<email> \
--dry-run=client -o yaml | kubectl apply -f -
kubectl -n envoy-system-system create secret docker-registry ${EXISTING_SECRET_NAME}
\
--docker-server=${REGISTRY} \
--docker-username=<username> \
--docker-password=<password> \
--docker-email=<email> \
--dry-run=client -o yaml | kubectl apply -f -
```

#### 2.  Validate the rotation. To validate, follow these steps:

a. Verify that all pods are active in the following namespaces:
- 
```bash
nai-system
```

- 
```bash
nai-admin
```

- 
```bash
envoy-gateway-system
```

b. Validate the new models.
c. Validate the endpoints.

#### Endpoints

#### Status

#### Failed

On the

page, if the

column displays

for an endpoint, with the message Unable to

pull runtime image with provided credentials, follow these steps: