+++
title = "nutanix-kubernetes-platform"
description = "Configure, operate, or troubleshoot Nutanix Kubernetes Platform (NKP), Konvoy, or Kommander. Use for NKP cluster lifecycle, image builds, air-gapped installs, or Insights."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# Nutanix Kubernetes Platform (NKP)

Nutanix Kubernetes Platform (NKP) is an enterprise Kubernetes distribution providing automated cluster lifecycle management, multi-cluster governance, observability, and workload operations across Nutanix Cloud Infrastructure (NCI), public clouds (AWS, Azure, GCP), and managed Kubernetes services (EKS, AKS).

NKP combines:

- **Konvoy**: Production-ready Kubernetes engine for provisioning and cluster lifecycle management.
- **Kommander**: Multi-cluster management, workspace management, governance, security, and catalog applications.
- **NKP Insights**: Real-time telemetry, anomaly detection, and proactive cluster health analytics.
- **AI Navigator**: Built-in AI assistant and MCP server integration for operating NKP clusters.

---

## Core NKP CLI Workflows

### 1. Diagnostic data and troubleshooting

When diagnosing cluster issues, collect a support bundle first. `nkp diagnose` uses the same kubeconfig as `kubectl` and writes a `support-bundle-<date>.tar.gz` archive:

```bash
nkp diagnose

# Inspect Kubernetes objects with kubectl, not nkp
kubectl get nodes
kubectl get clusters -A
```

### 2. Cluster creation quickstarts

- **Nutanix NCI**: build or import a node image with Nutanix Image Builder first (see `resources/manual/image-builder.md`), then reference it by name:

  ```bash
  nkp create cluster nutanix \
    --cluster-name=<cluster-name> \
    --endpoint="https://<prism-central>:9440" \
    --control-plane-prism-element-cluster=<pe-cluster> \
    --control-plane-subnets=<subnet> \
    --control-plane-endpoint-ip=<vip> \
    --control-plane-vm-image=<nib-image> \
    --worker-prism-element-cluster=<pe-cluster> \
    --worker-subnets=<subnet> \
    --worker-vm-image=<nib-image> \
    --csi-storage-container=<storage-container>
  ```

- **AWS / EKS**: see `resources/manual/basic-installations.md` for the required flags per provider (`nkp create cluster aws`, `nkp create cluster eks`).

### 3. Upgrades and lifecycle

Follow the order in `resources/manual/upgrade-guide.md`: Kommander, then each workspace's platform applications, then the management cluster, then managed clusters. Upgrade one minor version at a time. The VM image must already exist in Prism Central:

```bash
nkp upgrade kommander
nkp upgrade workspace <workspace-name>   # names from: nkp get workspaces

nkp upgrade cluster nutanix \
  --cluster-name=<management-cluster> \
  --vm-image=<nib-image>

nkp upgrade cluster nutanix \
  --cluster-name=<workload-cluster> \
  --vm-image=<nib-image> \
  -n <workload-namespace>
```

---

## Progressive Disclosure Reference Index

When executing specific NKP lifecycle tasks, refer to the dedicated manual reference guides in `resources/manual/`:

| Topic / Requirement                  | Reference File Pointer                                                                                               | Key Content Covered                                                                                          |
| :----------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------- |
| **Release Notes & Versions**         | [`resources/manual/release-notes.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/release-notes.md)                                             | NKP v2.18 feature matrix, component versions, Prism Central / AOS compatibility, network ports.              |
| **Architecture & Platform Overview** | [`resources/manual/overview.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/overview.md)                                                       | Node roles, control plane components, supported Linux OS distros, storage integrations.                      |
| **Getting Started & Binaries**       | [`resources/manual/downloading-and-getting-started.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/downloading-and-getting-started.md)         | Binary downloads, license tiers (Starter, Pro, Enterprise), kubeconfig setup, storage providers.             |
| **Resource & Network Requirements**  | [`resources/manual/requirements.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/requirements.md)                                               | Prerequisite sizing for management and workload clusters across Konvoy and Kommander.                        |
| **Nutanix Image Builder (NIB)**      | [`resources/manual/image-builder.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/image-builder.md)                                             | Custom OS image creation, Packer builds for Nutanix NCI, AWS, vSphere, Azure, GCP, RHEL subs.                |
| **Basic Cluster Installation**       | [`resources/manual/basic-installations.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/basic-installations.md)                                 | Step-by-step installation quickstarts for Nutanix NCI, AWS, EKS, vSphere, Azure, AKS, GCP.                   |
| **Cluster Operations & Add-ons**     | [`resources/manual/cluster-operations.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/cluster-operations.md)                                   | Workspaces, projects, Velero backup/restore, Loki logging, Prometheus/Grafana, Cilium/Calico, GPU.           |
| **Air-gapped & Custom Installs**     | [`resources/manual/custom-installation-and-tools.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/custom-installation-and-tools.md)             | Offline/air-gapped bundle seeding, custom registries, custom network topologies.                             |
| **Advanced Component Config**        | [`resources/manual/kommander-and-konvoy-configurations.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/kommander-and-konvoy-configurations.md) | FIPS 140-3 compliance, image mirrors, control plane PDBs, custom registry overrides.                         |
| **Upgrades & Migrations**            | [`resources/manual/upgrade-guide.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/upgrade-guide.md)                                             | `nkp upgrade kommander` and `nkp upgrade cluster`, GitOps-managed cluster upgrades, rollback & verification. |
| **Troubleshooting & Diagnostics**    | [`resources/manual/troubleshooting.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/troubleshooting.md)                                         | `nkp diagnose` support bundles, discovery commands, application debugging, Rook Ceph storage fixes.          |
| **AI Navigator & MCP Integration**   | [`resources/manual/ai-navigator.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/ai-navigator.md)                                               | AI Navigator setup, Azure OpenAI integration, NKP MCP server capabilities.                                   |
| **NKP Insights & Telemetry**         | [`resources/manual/nkp-insights-guide.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/nkp-insights-guide.md)                                   | NKP Insights configuration, alert rules, Bring Your Own Storage (BYOS) metrics.                              |
| **Master Documentation Index**       | [`resources/manual/master-index.md`](@/skills/engineering/nutanix-kubernetes-platform/resources/manual/master-index.md)                                               | Master overview of all 13 documentation modules.                                                             |

---

## Completion criteria

- Verify cluster, node, and workspace health with `kubectl get nodes`, `kubectl get clusters -A`, and `nkp get workspaces`, and capture `nkp diagnose` when troubleshooting failures.
- Confirm upgrades follow the documented sequence (Kommander, workspace apps, management cluster, then workload clusters, one minor version at a time) with target VM images already present in Prism Central.
- When advising without live cluster access, cite the matching `resources/manual/` module and the documented NKP version snapshot (v2.18).

