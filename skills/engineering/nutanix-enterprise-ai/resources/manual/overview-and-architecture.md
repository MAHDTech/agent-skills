# Nutanix Enterprise AI Manual: Overview and Architecture

Nutanix Enterprise AI (NAI) architecture overview, core components, microservices, end-to-end workflow, deployment models (GPT-in-a-Box 2.0, Bare Metal, Standalone), and NAI dashboard overview.

---

## ABOUT THIS PUBLICATION

This document provides information about Nutanix Enterprise AI.

### Intended Audience

This document is intended for site personnel, the operation teams, and any tier or individual entity defined by you as equipped to perform site configurations and modifications, or anyone who intends to learn and develop an understanding of Nutanix Enterprise AI.

### Related Documentation

Nutanix Support Portal

The

provides software download pages, documentation, compatibility, and other

information.

**Table 1: Related Documentation and Resources**

| Property 1 | Documentation | Description |
| --- | --- | --- |
|  | Release Notes / Nutanix Enterprise AI | Release Notes for Nutanix Enterprise AI. To view the release notes, you must log in to the Nutanix support portal using your email and password. |
|  | Nutanix Kubernetes Platform | This document provides information and code to enable the best-in-class hybrid cloud experience using the Nutanix Cloud Platform and Nutanix Kubernetes Platform. |
|  | Ports and Protocols | This page provides information about the ports that must be opened in the firewalls to enable Nutanix Enterprise AI to function. |
|  | REST API | REST API reference documentation for Nutanix Enterprise AI. |
| GETTING STARTED WITH NUTANIX ENTERPRISE AI |  |  |

Overview of Nutanix Enterprise AI including architecture, requirements, deployment and license types, and installation and login procedures.

This section provides an overview of Nutanix Enterprise AI that includes the following information:

- 

Architecture and a high-level end-to-end workflow

- 

Requirements and limitations

- 

Deployment and license types

- 

Installation and login procedure

#### Nutanix Enterprise AI Overview

Nutanix Enterprise AI (NAI) is a comprehensive inference endpoint management product designed to streamline and optimize your AI model orchestration experience. Nutanix Enterprise AI allows you to select, deploy, and manage large language models (LLMs) on a Kubernetes® cluster.

The following diagram illustrates the high-level architecture of Nutanix Enterprise AI.

#### Figure 1: Nutanix Enterprise AI Architecture

Nutanix Enterprise AI comprises the following features:

Model access integration

Nutanix Enterprise AI supports selecting and deploying text-based generative AI LLMs from Hugging Face and NVIDIA.

Model access control

Nutanix Enterprise AI enables you to control the models that can be deployed by the user.

Inference endpoint integration

Nutanix Enterprise AI supports creating an inference endpoint and sharing it with developers to incorporate into their AI applications. You can also manage and validate these endpoints to ensure that they are configured correctly.

API-based access control

Nutanix Enterprise AI supports accessing an inference endpoint by an application using API keys. API key management allows you to provide and revoke API access to ensure proper access security controls.

User access and role-based access control

Nutanix Enterprise AI supports role-based access control (RBAC), which you can configure to provide customized access permissions to users based on their assigned roles. The Users dashboard displays information about all the defined roles.

Enterprise user interface

Nutanix Enterprise AI has a simple and dynamic user interface that streamlines your deployment processes using one-click deployment.

Metrics dashboard

Nutanix Enterprise AI has a dynamic dashboard where you can monitor the health of your deployment with real-time monitoring tools that identify bottlenecks, track performance, and troubleshoot issues.

HTTP Proxy

Nutanix Enterprise AI provides support to configure forward proxy to download models from Hugging Face Model Hub.

Dark site deployment

Nutanix Enterprise AI provides support for deploying secure models in a dark site.

Built-in support for Nutanix products

Nutanix Enterprise AI supports seamless integration with existing Nutanix products, thus providing a robust and compliant solution.

Quick Validation of Model Endpoints Using Sample Applications

#### NAI Labs

provides the following applications:

#### Chat

,

#### Talk to my data

,

#### Agent

You can use these applications to quickly verify inference endpoints, observe end-to-end functionality, and visualize operational workflows without custom app development. NAI Labs are for testing and validation purposes only.

The data you upload to the applications is stored only in your environment.

NAI Labs

For more information, see

on page 313.

Remote Syslog server integration

View endpoint logs to troubleshoot inference issues

View model import logs to track model downloads.

User access and role-based access control

Nutanix Enterprise AI supports Active Directory, OpenLDAP, and SAML SSO based authentication and role-based access control (RBAC), which you can configure to provide customized access permissions to users based on their authorization policies.

Speculative Decoding

Experimental Endpoints

Batch inference

Inline model scanning and endpoint security

Create unified endpoints using providers or local endpoints

Tech Preview features

The following are the tech preview features:

- 

KV cache aware routing.

- 

KV Cache Offloading

- 

Multi-Node inferencing

You can deploy Nutanix Enterprise AI on Nutanix Kubernetes Platform (NKP), and public cloud Kubernetes platforms such as Amazon Elastic Kubernetes Service (EKS), Azure Kubernetes Service (AKS), and Google Kubernetes Engine (GKE).

#### Workflow for Nutanix Enterprise AI

This section describes the high-level end-to-end workflow for Nutanix Enterprise AI.

The following procedure summarizes the workflow.

Review deployment requirements and limitations.

1. 

Nutanix Enterprise AI - Private Inference and Agent Gateway Requirements

For more information, see

Nutanix Enterprise AI Limitations

on page 10  and

on page  27.

2. 

Deploy Nutanix Enterprise AI on a supported Kubernetes platform.

For more information, see

Deploy Nutanix Enterprise AI

on page 27.

3. 

Configure TLS and verify secure access.

For more information, see

TLS Encryption on Nutanix Enterprise AI

on page  119.

4. 

Generate the IP address for NAI

5. 

Log in to Nutanix Enterprise AI.

For more information, see

Logging in After Installing Nutanix Enterprise AI

on page  131.

6. 

Configure identity and access management.

Fine-Grained Authorization

For more information, see

on page  132.

7. 

Import a model.

Generative AI Models in Nutanix Enterprise AI

For more information, see

on page 183.

8. 

Create an endpoint.

For more information, see

Local Endpoints in Nutanix Enterprise AI

on page  214.

9. 

Test an endpoint.

For more information, see

Test the endpoint

.

10. Share the endpoint with your developer or data scientist user.

The following diagram illustrates the end-to-end workflow in Nutanix Enterprise AI.

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

#### VIEWING THE DASHBOARD IN NUTANIX ENTERPRISE AI

The dashboard provides a dynamic summary view of all the endpoints, the API requests, and the API keys.

### About this task

To access the  Dashboard  page, follow these steps.

### Procedure

1. Log in to Nutanix Enterprise AI.

### 2.  From the left navigation pane, select

#### Dashboard

.

The system displays the dashboard with the widgets. For information on the widgets that appear on the dashboard, see

Dashboard Widgets

on page 177.

#### Dashboard Widgets

The following tables describe the widgets that appear on the Nutanix Enterprise AI dashboard.

**Table 48: Dashboard Widgets**

| Widget Name Summary | Description Displays the total number of unified endpoints, local endpoints based on their status, such as  Active , Hibernated ,  Failed , and  Pending , providers, MCP connectors, and MCP servers. | Property 3 | Property 4 | Property 5 | Property 6 | Property 7 |
| --- | --- | --- | --- | --- | --- | --- |
| Infrastructure Summary | Displays the summary of the health and infrastructure usage of the Kubernetes cluster that hosts Nutanix Enterprise AI. |  |  |  |  |  |
|  | This widget displays the health status of all the active inference endpoints, the memory usage, CPU usage, disk usage, and the number of accelerators in the cluster. |  |  |  |  |  |
|  |  | Important:   The number of GPUs displayed might be incorrect if the GPU operator pods in your Kubernetes cluster are unhealthy. To fix these unhealthy pods, contact NVIDIA support. |  |  |  |  |
|  | Click | View Details |  | to view the usage statistics of the |  |  |
|  | cluster and the individual nodes in detail. For more information, see |  |  |  |  |  |
|  |  |  | Cluster Usage Metrics |  |  | on page 288 |
|  | and | Node Usage Metrics |  |  | on page  289. |  |
| Endpoint Requests | Displays the total number of API requests and API requests based on status, such as  Successful Requests , Failed Requests , and  Invalid Requests , You can also filter by  Local Endpoints , Unified Endpoints  and  All Endpoints . |  |  |  |  |  |
| Widget Name | Description |  |  |  |  |  |
| Top 5 Endpoints | Displays the five most used inference endpoints.You can also filter by  All Requests , Successful Requests ,  Failed Requests ,  Input Token Usage , and  Output Token Usage . |  |  |  |  |  |
| Requests Trend | Displays the trend in API requests over a period of time, based on status, such as  Successful Requests ,  Failed Requests , and  Invalid Requests . You can also filter by  Local Endpoints , Unified Endpoints  and  All Endpoints . |  |  |  |  |  |

> [!NOTE]
> Note:   The trends chart shows the end time of the last closing window instead of the current time.

API Client Keys

Displays the number of endpoint keys and MCP client keys.

MCP Connector Requests

Displays the trend in API requests over a period of time, based on status, such as  Successful Requests ,  Failed Requests , and  Invalid Requests .

> [!NOTE]
> Note:   The trends chart shows the end time of the last closing window instead of the current time.

Top 5 MCP Connectors

Displays the top five MCP connectors by count of requests. You can filter by  All Requests , Successful Requests , and  Failed Requests .