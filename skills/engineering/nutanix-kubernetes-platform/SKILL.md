---
name: nutanix-kubernetes-platform
description: Expert guidance, CLI commands, architecture reference, cluster lifecycle operations, air-gapped installation, image building, upgrades, and troubleshooting for Nutanix Kubernetes Platform (NKP v2.18), including Konvoy, Kommander, and NKP Insights. Use when working with NKP, Nutanix Kubernetes Platform, Konvoy, Kommander, nkp CLI commands, or Nutanix Kubernetes cluster management.
---

# Nutanix Kubernetes Platform (NKP)

Nutanix Kubernetes Platform (NKP) is an enterprise Kubernetes distribution providing automated cluster lifecycle management, multi-cluster governance, observability, and workload operations across Nutanix Cloud Infrastructure (NCI), public clouds (AWS, Azure, GCP), and managed Kubernetes services (EKS, AKS).

NKP combines:

- **Konvoy**: Production-ready Kubernetes engine for provisioning and cluster lifecycle management.
- **Kommander**: Multi-cluster management, workspace management, governance, security, and catalog applications.
- **NKP Insights**: Real-time telemetry, anomaly detection, and proactive cluster health analytics.
- **AI Navigator**: Built-in AI assistant and MCP server integration for operating NKP clusters.

---

## Core NKP CLI Workflows

### 1. Diagnostic Data & Troubleshooting

When diagnosing cluster issues, collecting diagnostics is the first step:

```bash
# Generate complete diagnostic bundle for support/troubleshooting
nkp get diagnostics --output-directory ./nkp-diagnostics

# Inspect cluster objects and infrastructure status
nkp get clusters
nkp get nodes
```

### 2. Cluster Creation Quickstarts

- **Nutanix NCI**:

  ```bash
  nkp create cluster nutanix \
    --cluster-name my-nkp-cluster \
    --endpoint https://prism-central.domain.com:9440 \
    --subnet-name k8s-subnet \
    --vm-image CentOS-7-x86_64
  ```

- **AWS / EKS**:

  ```bash
  nkp create cluster aws --cluster-name my-aws-cluster --region us-west-2
  nkp create cluster eks --cluster-name my-eks-cluster --region us-west-2
  ```

### 3. Upgrades & Lifecycle

```bash
# Prepare and upgrade NKP management cluster
nkp upgrade management nutanix --cluster-name my-mgmt-cluster

# Upgrade workload cluster
nkp upgrade workload nutanix --cluster-name my-workload-cluster
```

---

## Progressive Disclosure Reference Index

When executing specific NKP lifecycle tasks, refer to the dedicated manual reference guides in `resources/manual/`:

| Topic / Requirement                  | Reference File Pointer                                                                                               | Key Content Covered                                                                                |
| :----------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| **Release Notes & Versions**         | [`resources/manual/release-notes.md`](resources/manual/release-notes.md)                                             | NKP v2.18 feature matrix, component versions, Prism Central / AOS compatibility, network ports.    |
| **Architecture & Platform Overview** | [`resources/manual/overview.md`](resources/manual/overview.md)                                                       | Node roles, control plane components, supported Linux OS distros, storage integrations.            |
| **Getting Started & Binaries**       | [`resources/manual/downloading-and-getting-started.md`](resources/manual/downloading-and-getting-started.md)         | Binary downloads, license tiers (Starter, Pro, Enterprise), kubeconfig setup, storage providers.   |
| **Resource & Network Requirements**  | [`resources/manual/requirements.md`](resources/manual/requirements.md)                                               | Prerequisite sizing for management and workload clusters across Konvoy and Kommander.              |
| **Nutanix Image Builder (NIB)**      | [`resources/manual/image-builder.md`](resources/manual/image-builder.md)                                             | Custom OS image creation, Packer builds for Nutanix NCI, AWS, vSphere, Azure, GCP, RHEL subs.      |
| **Basic Cluster Installation**       | [`resources/manual/basic-installations.md`](resources/manual/basic-installations.md)                                 | Step-by-step installation quickstarts for Nutanix NCI, AWS, EKS, vSphere, Azure, AKS, GCP.         |
| **Cluster Operations & Add-ons**     | [`resources/manual/cluster-operations.md`](resources/manual/cluster-operations.md)                                   | Workspaces, projects, Velero backup/restore, Loki logging, Prometheus/Grafana, Cilium/Calico, GPU. |
| **Air-gapped & Custom Installs**     | [`resources/manual/custom-installation-and-tools.md`](resources/manual/custom-installation-and-tools.md)             | Offline/air-gapped bundle seeding, custom registries, custom network topologies.                   |
| **Advanced Component Config**        | [`resources/manual/kommander-and-konvoy-configurations.md`](resources/manual/kommander-and-konvoy-configurations.md) | FIPS 140-3 compliance, image mirrors, control plane PDBs, custom registry overrides.               |
| **Upgrades & Migrations**            | [`resources/manual/upgrade-guide.md`](resources/manual/upgrade-guide.md)                                             | Step-by-step upgrade paths, GitOps-managed cluster upgrades, rollback & verification.              |
| **Troubleshooting & Diagnostics**    | [`resources/manual/troubleshooting.md`](resources/manual/troubleshooting.md)                                         | `nkp get diagnostics`, discovery commands, application debugging, Rook Ceph storage fixes.         |
| **AI Navigator & MCP Integration**   | [`resources/manual/ai-navigator.md`](resources/manual/ai-navigator.md)                                               | AI Navigator setup, Azure OpenAI integration, NKP MCP server capabilities.                         |
| **NKP Insights & Telemetry**         | [`resources/manual/nkp-insights-guide.md`](resources/manual/nkp-insights-guide.md)                                   | NKP Insights configuration, alert rules, Bring Your Own Storage (BYOS) metrics.                    |
| **Master Documentation Index**       | [`resources/manual/master-index.md`](resources/manual/master-index.md)                                               | Master overview of all 13 documentation modules.                                                   |
