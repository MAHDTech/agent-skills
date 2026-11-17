+++
title = "developers-community-google-cloud-vertex-ai"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Community Integrations

# Google Cloud Vertex AI

Access SpaceXSI’s Grok models through Google Cloud’s managed platform with enterprise security, governance, and unified billing.

This guide walks through setting up and using Grok models on Google Cloud Vertex AI / Gemini Enterprise Agent Platform. Grok on Vertex AI is accessed as a partner model through the OpenAI-compatible API, including the Responses API and Chat Completions. Models are enabled through Model Garden.

## Prerequisites

Before you begin, ensure you have:

* An active Google Cloud Platform (GCP) project with billing enabled.
* Permissions to enable APIs and access Model Garden, such as the Vertex AI User or Project Editor role.
* The `aiplatform.googleapis.com` API, or equivalent Agent Platform API, enabled in your project.
* Google Cloud CLI (`gcloud`) installed and authenticated for Application Default Credentials (ADC).

Set up ADC and your project:

```bash customLanguage="bash"
gcloud auth application-default login
gcloud config set project YOUR_PROJECT_ID
```

Enable the required API if it is not already enabled:

```bash customLanguage="bash"
gcloud services enable aiplatform.googleapis.com
```

## Install required packages

```bash customLanguage="bash"
pip install -U openai google-cloud-aiplatform
```

## Enable Grok models in Model Garden

1. Go to the Google Cloud Console Model Garden, or search for “Model Garden” in the console.
2. Search for “Grok”, or browse by publisher xAI.
3. Select the desired Grok model, such as Grok 4.2 or Grok 4.3.
4. Review the model card for capabilities, quotas, pricing, and regions.
5. Click **Enable** or **Deploy / request access** if prompted.
6. Once enabled, the model becomes available for API calls.

Use the model ID shown in Model Garden. Vertex model names may use a publisher prefix, for example:

* `xai/grok-4.7`

Model availability generally matches the xAI API, subject to Google Cloud regional availability and quotas.

## Make your first API call

Grok on Vertex uses the OpenAI-compatible interface. You can use the standard `openai` Python library.

### Authentication

Use Application Default Credentials. The client can pick up your `gcloud` auth or service account credentials.

You may need to set the Vertex/OpenAI-compatible base URL or endpoint with an environment variable or directly in the client. Use the exact endpoint from the model card or Google documentation for the Agent Platform.

```bash customLanguage="bash"
export OPENAI_BASE_URL="https://YOUR_VERTEX_ENDPOINT"
```

### Responses API example

```python customLanguage="pythonOpenAISDK"
from openai import OpenAI

client = OpenAI()  # Uses ADC / env vars automatically

response = client.responses.create(
    model="xai/grok-4.7",
    input="Explain the advantages of using Grok for agentic workflows with parallel tool calling.",
    max_output_tokens=800,
)

print(response.output_text)
```

### Chat Completions example

```python customLanguage="pythonOpenAISDK"
from openai import OpenAI

client = OpenAI()

response = client.chat.completions.create(
    model="xai/grok-4.7",
    messages=[
        {
            "role": "user",
            "content": "Which city has a higher temperature right now, Boston or New Delhi, and by how much in Fahrenheit?",
        }
    ],
    tools=[
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
                            "description": "The city and state, e.g., San Francisco, CA",
                        },
                        "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
                    },
                    "required": ["location"],
                },
            },
        }
    ],
    tool_choice="auto",
)

print(response.choices[0].message.content)
```

Streaming is supported on both interfaces for lower-latency experiences.

## Function calling and tool use

Grok excels at tool use and parallel function calling across the Responses API and Chat Completions interfaces. Define clear, strict schemas for tools so the model can select and call them reliably.

## Data retention and compliance

Data retention and processing for Grok models on Google Cloud are governed by Google Cloud Vertex AI policies.

* Many deployments support Zero Data Retention (ZDR) options.
* Review the specific model card and your organization’s Google Cloud data governance settings.
* Activity logging can be enabled with Vertex AI request-response logging for audit and debugging purposes.

See Google Cloud documentation on Vertex AI data governance and logging for details.

## Feature support

Supported capabilities include:

* Responses API and Chat Completions.
* Function calling and tool use, including parallel function calling.
* Reasoning modes / extended thinking.
* Structured outputs / JSON mode.
* Streaming.
* Fixed quotas and committed use discounts through Google Cloud.

Context windows vary by model. Check the specific Grok model card in Model Garden for the current limit.

## Global, multi-region, and regional endpoints

Vertex AI / Gemini Enterprise Agent Platform offers flexible endpoint routing:

* **Global endpoints:** maximum availability with dynamic routing; recommended for most use cases.
* **Regional endpoints:** routing through specific regions for strict compliance requirements.

## Best practices

* Choose the Grok model and endpoint configuration that match your latency, throughput, and reasoning requirements.
* Prefer Application Default Credentials and IAM roles over long-lived keys. Use service accounts for production workloads.
* Monitor usage in Google Cloud Billing and Quotas pages. Request quota increases as needed.
* Use clear tool schemas and explicit output formats.
* Enable request logging and integrate with Google Cloud Monitoring / Logging.
* When migrating from the direct xAI API, update the base URL, client configuration, and model prefix. Most prompts and tool definitions transfer with minimal changes.

## Troubleshooting

| Issue | What to check |
|---|---|
| Authentication errors | Run `gcloud auth application-default login` and verify project permissions. |
| Model not found | Confirm the model is enabled in Model Garden and use the exact `xai/...` ID. |
| Quota exceeded | Check quotas in the Google Cloud console and request increases as needed. |
| Endpoint / base URL issues | Use the exact endpoint or environment variable from the model card or Google documentation. |

Start in the Google Cloud console playground / Model Garden interface when available, then move to code.

## Next steps

* Explore enabled models in Model Garden.
* Build agentic applications that use Grok’s tool-calling strengths.
* Integrate with Google Cloud services such as Cloud Functions and Vertex AI Pipelines.
* Review the full SpaceXSI Grok documentation and model cards for prompting tips and capabilities.


