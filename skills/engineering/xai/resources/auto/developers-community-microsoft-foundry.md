#### Community Integrations

# Microsoft Foundry

Access SpaceXSI’s frontier reasoning and agentic models through Azure AI Foundry with enterprise-grade security, governance, and unified billing.

This guide walks through setting up and using Grok models on Microsoft Foundry. Grok models on Foundry give you strong reasoning, native tool use, enterprise authentication through Microsoft Entra ID, Azure-native monitoring, and an OpenAI-compatible API.

Usage is billed through the Azure Marketplace / your Azure subscription. Grok models are delivered through the SpaceXSI–Microsoft partnership with Azure-managed endpoints and optional Azure AI Content Safety layers. Review the specific model card in the Foundry catalog for the latest details on data processing, retention, and terms.

Grok on Foundry works with the official OpenAI Python/TypeScript SDKs, `azure-ai-projects`, LangChain, Semantic Kernel, LlamaIndex, and most OpenAI-compatible frameworks. Streaming, tool calling, and structured outputs are supported.

## Prerequisites

Before you begin, ensure you have:

* An active Azure subscription.
* Access to Azure AI Foundry.
* Sufficient permissions to create or manage Foundry resources/projects and deploy models, typically Contributor or custom roles with model deployment rights.
* Optional but recommended: the Azure CLI installed for resource management and authentication testing.
* Python 3.10+ for the examples in this guide.

## Install required packages

```bash customLanguage="bash"
pip install -U openai azure-identity
```

Optional, for higher-level project client patterns:

```bash customLanguage="bash"
pip install azure-ai-projects
```

## Provisioning

Foundry organizes work into resources for security, billing, and networking, and projects for deployments and collaboration. Create a resource/project first, then deploy one or more Grok model instances inside it.

The deployment name you choose becomes the value passed in the `model` parameter of your API requests.

### Create or select a Foundry resource and project

1. Navigate to the Foundry portal.
2. Create a new Foundry resource, or select an existing one.
3. Within the resource, create a new project if your workflow uses projects.
4. Configure access management:
   * Use Microsoft Entra ID with role-based access control (RBAC).
   * Assign the Cognitive Services OpenAI User role, or equivalent, to identities that will call models.
   * Optionally configure private networking through Azure Virtual Network.
5. Note your resource name and project name for later.

The resulting endpoint base will be:

```text customLanguage="text"
https://{resource-name}.services.ai.azure.com/api/projects/{project-name}/openai/v1
```

### Deploy a Grok model

1. In the Foundry portal, go to your resource or project, then **Models + endpoints**.
2. Click **+ Deploy model** → **Deploy base model**, or browse the Model catalog directly and search for “Grok”.
3. Browse or search the catalog for the desired Grok model, for example, `grok-4.3`.
4. Review the model card for capabilities, context window, tool calling support, safety evaluations, pricing, and deployment options.
5. Click **Deploy**.
6. Configure deployment settings:
   * **Deployment name:** Choose a clear, stable name, such as `grok-4.3`. This name cannot be changed after creation and is the value you use in the `model` parameter.
   * **Deployment type / SKU:** Select Serverless for pay-as-you-go workloads, or Provisioned Throughput Units (PTU) for predictable high-volume performance.
7. Review and select **Deploy**. Wait for the deployment to reach **Ready / Running** status.

Once deployed, you can test in the built-in Playground, view generated code snippets, manage keys/endpoints if API key auth is enabled, and monitor usage and metrics.

## Authentication

Grok on Foundry uses Azure-native authentication. The recommended approach is Microsoft Entra ID (keyless) with `DefaultAzureCredential`. API keys from the portal may also be supported depending on your resource configuration.

All requests go to your Foundry project’s OpenAI-compatible endpoint:

```text customLanguage="text"
https://{resource-name}.services.ai.azure.com/api/projects/{project-name}/openai/v1
```

### Recommended: Entra ID authentication

Use `azure.identity` and `get_bearer_token_provider`. This enables seamless RBAC, managed identities, and avoids secret management.

```python customLanguage="pythonOpenAISDK"
from azure.identity import DefaultAzureCredential, get_bearer_token_provider
from openai import OpenAI

project_endpoint = "https://YOUR-RESOURCE-NAME.services.ai.azure.com/api/projects/YOUR_PROJECT_NAME"
base_url = project_endpoint.rstrip("/") + "/openai/v1"

credential = DefaultAzureCredential()
token_provider = get_bearer_token_provider(
    credential,
    "https://ai.azure.com/.default",
)

client = OpenAI(
    base_url=base_url,
    api_key=token_provider,
)

response = client.responses.create(
    model="grok-4.3",  # your deployment name
    input="Explain the significance of Grok's tool-calling capabilities for building reliable agents. Be concise but insightful.",
    max_output_tokens=800,
)

print(response.output_text)
```

Important:

* Assign the Cognitive Services OpenAI User, or appropriate role, to the identity running this code.
* `DefaultAzureCredential` handles local development through Azure CLI / VS Code, managed identities, service principals, and other supported flows.
* No `api-version` query parameter is needed; the `/openai/v1` path handles compatibility.

### Alternative: API key authentication

If your Foundry resource exposes keys under Keys and Endpoint, copy a primary or secondary key and use it directly as the `api_key`:

```python customLanguage="pythonOpenAISDK"
client = OpenAI(
    base_url=base_url,
    api_key="your-foundry-api-key-here",
)
```

Prefer Entra ID + RBAC in production. Never commit keys to source control, and rotate keys regularly.

## Make your first API call

### Simple reasoning call

```python customLanguage="pythonOpenAISDK"
response = client.responses.create(
    model="grok-4.3",
    input="Walk through the first-principles reasoning to determine why reusable rockets dramatically reduce the cost of space access.",
    max_output_tokens=1500,
)
print(response.output_text)
```

### Tool calling example

Grok excels at tool use. Here is a pattern for parallel tool calling:

```python customLanguage="pythonOpenAISDK"
tools = [
    {
        "type": "function",
        "function": {
            "name": "get_current_weather",
            "description": "Get the current weather in a given location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "City and country, e.g., San Francisco, CA",
                    },
                    "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
                },
                "required": ["location"],
            },
        },
    },
    {
        "type": "function",
        "function": {
            "name": "search_web",
            "description": "Search the web for recent information",
            "parameters": {
                "type": "object",
                "properties": {
                    "query": {"type": "string"},
                },
                "required": ["query"],
            },
        },
    },
]

response = client.responses.create(
    model="grok-4.3",
    input="What's the weather like in Palo Alto right now and any major tech news from today?",
    tools=tools,
    # parallel_tool_calls=True,  # enable if supported in your deployment
    max_output_tokens=1000,
)

print(response)
```

In a real agent loop, execute the tool calls and continue the conversation with the tool results.

### Streaming response

```python customLanguage="pythonOpenAISDK"
stream = client.responses.create(
    model="grok-4.3",
    input="Write a short, helpful onboarding guide for a new engineer joining SpaceXSI.",
    max_output_tokens=600,
    stream=True,
)

for chunk in stream:
    if hasattr(chunk, "choices") and chunk.choices:
        delta = chunk.choices[0].delta
        if hasattr(delta, "content") and delta.content:
            print(delta.content, end="", flush=True)
```

Start with the Playground in the Foundry portal for rapid prompt iteration, then move to code.

## Correlation IDs and debugging

Foundry includes standard Azure request identifiers in response headers, such as `request-id`, `apim-request-id`, and `x-ms-request-id`. When contacting Microsoft or SpaceXSI support, include these IDs with your deployment name and approximate timestamp.

## Feature support and capabilities

| Capability | Notes |
|---|---|
| Reasoning | Strong first-principles reasoning. “Think mode” style prompting works well. |
| Tool / function calling | Native support for reliable agentic workflows. |
| Structured outputs / JSON mode | Supported. Ask for `response_format`, or instruct JSON explicitly. |
| Streaming | Supported for low-latency user experiences. |
| Long context | Check the specific model card for current context windows. |
| Code generation | Strong performance for code generation and editing tasks. |

## Safety and responsible AI

Grok models include SpaceXSI’s safety training and alignment. On Foundry, Azure AI Content Safety is available and often enabled by default or easily integrated.

Before production deployment:

* Review the full model card and safety benchmark tab in the Foundry catalog.
* Use clear system prompts that define safety boundaries and desired behavior.
* Implement Azure Content Safety filters for input/output where appropriate.
* Conduct your own red-teaming and evaluations.
* Monitor usage and any required mitigations.

## Limitations

* Feature parity with the direct xAI API at `api.x.ai` may differ slightly, especially for the latest experimental features.
* Validate vision/multimodal support and exact parameter availability for your chosen model/deployment.
* Rate limits and quotas are managed at the Azure resource level.

For the authoritative list of supported parameters and behaviors, consult the model card inside Azure AI Foundry and SpaceXSI Grok documentation linked from the catalog.

## Best practices for production

### Model selection

* Use full Grok reasoning models for maximum reasoning depth and capability.
* Use balanced reasoning settings for simpler tasks.
* Choose the deployment settings that match your latency, throughput, and cost requirements.

### Prompting Grok effectively

* Encourage step-by-step reasoning when needed.
* Specify the desired output format.
* Use clear tool schemas.

### Cost management

* Monitor spend in Azure Cost Management + Billing.
* Use serverless for spiky or experimental workloads; use PTU for steady high throughput.
* Right-size the model and deployment type for your expected traffic pattern.

### Security and compliance

* Prefer Entra ID + RBAC over long-lived keys.
* Use private endpoints / VNet injection where required.
* Log requests with correlation IDs for auditability.

### Observability

* Integrate Azure Monitor, Application Insights, or Log Analytics.
* Track token usage, latency, and error rates per deployment.

## Troubleshooting

| Issue | What to check |
|---|---|
| 401 Unauthorized | Missing or incorrect Entra role; wrong token scope; check the `DefaultAzureCredential` chain. |
| 404 Not Found / model not found | Wrong deployment name; it must match exactly what you created in the portal. |
| Deployment stuck in “Running” | Check region quotas, resource health, portal notifications, or try redeploying. |
| Slow responses or high latency | Consider Provisioned Throughput. Check the network path to the Azure region. |
| Tool calls not executing as expected | Verify the tool schema and whether parallel tool calling is enabled/supported for the deployment. |
| Content filtered / blocked | Review Azure Content Safety configuration and your system prompt. Adjust safety thresholds if needed. |

## Next steps

* Use the Playground inside your Foundry project.
* Combine Grok with Azure AI Agent Service or popular frameworks such as LangChain, Semantic Kernel, and CrewAI.
* Add retrieval, memory, and orchestration layers for production systems.
* Use Foundry tracing and your internal eval harness to evaluate and improve behavior.
* When migrating from the direct xAI API, update authentication and endpoint configuration. Most prompts and tool schemas transfer with minimal changes.

