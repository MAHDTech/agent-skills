+++
title = "observability-and-opentelemetry"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "nutanix-enterprise-ai"
+++

# Nutanix Enterprise AI Manual: Observability, Metrics, and OpenTelemetry Collector

OpenTelemetry collector configuration, ClickHouse server and keeper architecture, exporting metrics and logs to external monitoring systems (Prometheus, Grafana, SIEM), observability daemonsets and collectors.

---

CONFIGURING OPENTELEMETRY COLLECTOR TO VIEW NUTANIX ENTERPRISE AI METRICS

Configure OpenTelemetry Collector to export the metrics available in Nutanix Enterprise AI and view the metrics in any OpenTelemetry compliant observability tool.

### Before you begin

- 

OpenTelemetry

Ensure that you install an active version of OpenTelemetry Collector. For more information, see

documentation

.

- 

Ensure that you enable the appropriate access and permissions in OpenTelemetry Collector. For more information, see

OpenTelemetry documentation

.

- 

Ensure that each receiver has the correct namespace and selector labels configured in  kubernetes_sd_configs  for your cluster.

### About this task

The Nutanix Enterprise AI dashboard displays inference request metrics, such as Request Count By Status, Request Latency, and infrastructure metrics such as CPU, Memory, and GPU utilization.

In addition, Nutanix Enterprise AI provides other metrics, categorised by the following metric sources:

- 

API server metrics such as Inference Request Count, Latency, and Throughput.

- 

GPU metrics exposed by

NVIDIA DCGM Exporter

- 

Node metrics exposed by

Node Exporter

- 

vLLM Runtime

vLLM endpoint metrics exposed by

- 

TGI Engine

TGI endpoint metrics exposed by

- 

NIM endpoint metrics exposed by

NIM Engine

To collect these metrics, add Prometheus receivers to the OpenTelemetry Collector. You can then export these metrics to any OpenTelemetry compatible observability tool to view these metrics.

The configuration in this procedure creates Prometheus receivers for each metric source in Nutanix Enterprise AI. If you have an OpenTelemetry Collector installed, add these Prometheus receivers to your configuration to start collecting metrics from all Nutanix Enterprise AI sources.

To configure OpenTelemetry Collector to export Nutanix Enterprise AI metrics, follow these steps:

### Procedure

1. Access the configuration file of the OpenTelemetry Collector installed on your cluster.
2. Enter the following Prometheus receiver configuration in the configuration file.

> [!NOTE]
> Note:   Ensure that the  kubernetes_sd_configs  of the scrape jobs in the configuration file have the correct namespace and selector labels configured according to your cluster.

```bash
receivers:
prometheus:
config:
scrape_configs:
- job_name: nai
scrape_interval: 10s
metrics_path: /metrics/all
static_configs:
- targets:
- nai-api.nai-system.svc.cluster.local:8080
- job_name: dcgm-exporter
scrape_interval: 10s
metrics_path: /metrics
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 9400
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- <GPU_OPERATOR_NAMESPACE>
selectors:
- role: pod
label: "app=nvidia-dcgm-exporter"
- job_name: node-exporter
scrape_interval: 10s
metrics_path: /metrics
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 9100
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- <NODE_EXPORTER_NAMESPACE>
selectors:
- role: pod
label: "app.kubernetes.io/name=prometheus-node-exporter"
- job_name: vllm-cpu-endpoints
scrape_interval: 10s
metrics_path: /metrics
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- nai-admin
selectors:
- role: pod
label: "endpoint.iep.nai.nutanix.com/engine=vllm-cpu"
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 8080
- job_name: vllm-endpoints
scrape_interval: 10s
metrics_path: /metrics
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- nai-admin
selectors:
- role: pod
label: "endpoint.iep.nai.nutanix.com/engine=vllm"
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 8080
- job_name: tgi-endpoints
scrape_interval: 10s
metrics_path: /metrics
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- nai-admin
selectors:
- role: pod
label: "endpoint.iep.nai.nutanix.com/engine=tgi"
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 8080
- source_labels: [__meta_kubernetes_namespace]
target_label: namespace
- source_labels: [__meta_kubernetes_pod_name]
target_label: pod
- job_name: nim-endpoints
scrape_interval: 10s
metrics_path: /metrics
kubernetes_sd_configs:
- role: pod
namespaces:
names:
- nai-admin
selectors:
- role: pod
label: "endpoint.iep.nai.nutanix.com/engine=nim"
relabel_configs:
- source_labels:
[
__meta_kubernetes_pod_container_port_number,
]
action: keep
regex: 8000
```

3. Add the Prometheus receiver in the metrics pipeline definition.
```bash
service:
pipelines:
metrics:
receivers: [prometheus]
```

4. Save the file.
