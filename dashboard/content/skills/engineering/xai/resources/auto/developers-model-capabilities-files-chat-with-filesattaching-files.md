+++
title = "developers-model-capabilities-files-chat-with-filesattaching-files"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Model Capabilities

# Chat with Files

You can attach files to chat conversations using a public URL or an uploaded file ID. When files are attached, the system automatically enables document search capabilities, transforming your request into an agentic workflow.

## Attaching Files

There are two ways to attach a file to a message:

**Public URL (`file_url`)** — reference any publicly accessible file directly, no upload step needed:

```json
{"type": "input_file", "file_url": "https://example.com/document.pdf"}
```

**Uploaded file (`file_id`)** — [upload](https://docs.x.ai/developers/files/managing-files) files first via the Files API and reference by ID. Useful for files that aren't publicly accessible, such as private or sensitive documents:

```json
{"type": "input_file", "file_id": "file-abc123"}
```

The examples below use `file_url` for simplicity. You can replace with `file_id` to use uploaded files instead.

## Basic Chat with a Single File

Attach a file to a conversation to let the model search through it for relevant information.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Attach a file by public URL (or use file(file_id) for uploaded files)
chat = client.chat.create(model="grok-4.5")
chat.append(user(
    "What was the total revenue in this report?",
    file(url="https://docs.x.ai/assets/api-examples/documents/sales-report.txt"),
))

# Get the response
response = chat.sample()

print(f"Answer: {response.content}")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# Attach a file by public URL (or use file_id for uploaded files)
response = client.responses.create(
    model="grok-4.5",
    input=[
        {
            "role": "user",
            "content": [
                {"type": "input_text", "text": "What was the total revenue in this report?"},
                {"type": "input_file", "file_url": "https://docs.x.ai/assets/api-examples/documents/sales-report.txt"}
            ]
        }
    ]
)

final_answer = response.output[-1].content[0].text
print(f"Answer: {final_answer}")
```

```pythonRequests
import os
import requests

api_key = os.getenv("XAI_API_KEY")
headers = {
    "Content-Type": "application/json",
    "Authorization": f"Bearer {api_key}"
}

# Attach a file by public URL (or use file_id for uploaded files)
chat_url = "https://api.x.ai/v1/responses"
payload = {
    "model": "grok-4.5",
    "input": [
        {
            "role": "user",
            "content": [
                {"type": "input_text", "text": "What was the total revenue in this report?"},
                {"type": "input_file", "file_url": "https://docs.x.ai/assets/api-examples/documents/sales-report.txt"}
            ]
        }
    ]
}
response = requests.post(chat_url, headers=headers, json=payload)
print(response.json())
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach a file by public URL (or use file_id for uploaded files)
const response = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                { type: "input_text", text: "What was the total revenue in this report?" },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/sales-report.txt" },
            ],
        },
    ],
});

const finalAnswer = response.output[response.output.length - 1].content[0].text;
console.log("Answer: " + finalAnswer);
```

```bash
# Attach a file by public URL (or use file_id for uploaded files)
curl -X POST "https://api.x.ai/v1/responses" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "model": "grok-4.5",
    "input": [
      {
        "role": "user",
        "content": [
          {"type": "input_text", "text": "What was the total revenue in this report?"},
          {"type": "input_file", "file_url": "https://docs.x.ai/assets/api-examples/documents/sales-report.txt"}
        ]
      }
    ]
  }'
```

## Streaming Chat with Files

Get real-time responses while the model searches through your documents.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Attach a file by public URL (or use file(file_id) for uploaded files)
chat = client.chat.create(model="grok-4.5")
chat.append(user(
    "What is the weight of the XR-2000?",
    file(url="https://docs.x.ai/assets/api-examples/documents/product-specs.txt"),
))

# Stream the response
is_thinking = True
for response, chunk in chat.stream():
    # Show tool calls as they happen
    for tool_call in chunk.tool_calls:
        print(f"\\nSearching: {tool_call.function.name}")
    
    if response.usage.reasoning_tokens and is_thinking:
        print(f"\\rThinking... ({response.usage.reasoning_tokens} tokens)", end="", flush=True)
    
    if chunk.content and is_thinking:
        print("\\n\\nAnswer:")
        is_thinking = False
    
    if chunk.content:
        print(chunk.content, end="", flush=True)

print(f"\\n\\nUsage: {response.usage}")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach a file by public URL (or use file_id for uploaded files)
const stream = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                { type: "input_text", text: "What is the weight of the XR-2000?" },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/product-specs.txt" },
            ],
        },
    ],
    stream: true,
});

for await (const event of stream) {
    if (event.type === "response.output_text.delta") {
        process.stdout.write(event.delta);
    }
}

console.log();
```

## Multiple File Attachments

Query across multiple documents simultaneously.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Attach files by public URL (or use file(file_id) for uploaded files)
chat = client.chat.create(model="grok-4.5")
chat.append(
    user(
        "Based on these documents, when did the project start, what is the budget, and how many people are on the team?",
        file(url="https://docs.x.ai/assets/api-examples/documents/project-timeline.txt"),
        file(url="https://docs.x.ai/assets/api-examples/documents/project-budget.txt"),
        file(url="https://docs.x.ai/assets/api-examples/documents/project-team.txt"),
    )
)

response = chat.sample()

print(f"Answer: {response.content}")
print("\\nDocuments searched: 3")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach files by public URL (or use file_id for uploaded files)
const response = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                {
                    type: "input_text",
                    text: "Based on these documents, when did the project start, what is the budget, and how many people are on the team?",
                },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/project-timeline.txt" },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/project-budget.txt" },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/project-team.txt" },
            ],
        },
    ],
});

const finalAnswer = response.output[response.output.length - 1].content[0].text;
console.log("Answer: " + finalAnswer);
console.log("Documents searched: 3");
```

## Multi-Turn Conversations with Files

Maintain context across multiple questions about the same documents. Use encrypted content to preserve file context efficiently across multiple turns.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Create a multi-turn conversation with encrypted content
chat = client.chat.create(
    model="grok-4.5",
    use_encrypted_content=True,  # Enable encrypted content for efficient multi-turn
)

# First turn: Attach a file by public URL (or use file(file_id) for uploaded files)
chat.append(user(
    "What is the employee's name?",
    file(url="https://docs.x.ai/assets/api-examples/documents/employee-info.txt"),
))
response1 = chat.sample()
print("Q1: What is the employee's name?")
print(f"A1: {response1.content}\\n")

# Add the response to conversation history
chat.append(response1)

# Second turn: Ask about department (agentic context is retained via encrypted content)
chat.append(user("What department does this employee work in?"))
response2 = chat.sample()
print("Q2: What department does this employee work in?")
print(f"A2: {response2.content}\\n")

# Add the response to conversation history
chat.append(response2)

# Third turn: Ask about skills
chat.append(user("What skills does this employee have?"))
response3 = chat.sample()
print("Q3: What skills does this employee have?")
print(f"A3: {response3.content}\\n")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach a file by public URL (or use file_id for uploaded files)

// First turn: Ask about the document
const response1 = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                { type: "input_text", text: "What is the employee's name?" },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/employee-info.txt" },
            ],
        },
    ],
});

console.log("Q1: What is the employee's name?");
console.log("A1: " + response1.output[response1.output.length - 1].content[0].text + "\\n");

// Second turn: Ask about department (uses previous_response_id for context)
const response2 = await client.responses.create({
    model: "grok-4.5",
    previous_response_id: response1.id,
    input: [
        { role: "user", content: "What department does this employee work in?" },
    ],
});

console.log("Q2: What department does this employee work in?");
console.log("A2: " + response2.output[response2.output.length - 1].content[0].text + "\\n");

// Third turn: Ask about skills
const response3 = await client.responses.create({
    model: "grok-4.5",
    previous_response_id: response2.id,
    input: [
        { role: "user", content: "What skills does this employee have?" },
    ],
});

console.log("Q3: What skills does this employee have?");
console.log("A3: " + response3.output[response3.output.length - 1].content[0].text + "\\n");
```

## Combining Files with Other Modalities

You can combine file attachments with images and other content types in a single message.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file, image

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Attach files by public URL (or use file(file_id) for uploaded files)
chat = client.chat.create(model="grok-4.5")
chat.append(
    user(
        "Based on the attached care guide, do you have any advice about the pictured cat?",
        file(url="https://docs.x.ai/assets/api-examples/documents/cat-care.txt"),
        image("https://media.x.ai/v1/docs/example-cat-in-tree-8e9ac3e0.png"),
    )
)

response = chat.sample()

print(f"Analysis: {response.content}")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach files by public URL (or use file_id for uploaded files)
const response = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                {
                    type: "input_text",
                    text: "Based on the attached care guide, do you have any advice about the pictured cat?",
                },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/cat-care.txt" },
                {
                    type: "input_image",
                    image_url: "https://media.x.ai/v1/docs/example-cat-in-tree-8e9ac3e0.png",
                },
            ],
        },
    ],
});

const analysis = response.output[response.output.length - 1].content[0].text;
console.log("Analysis: " + analysis);
```

## Combining Files with Code Execution

For data analysis tasks, you can attach data files and enable the code execution tool. This allows Grok to write and run Python code to analyze and process your data.

```pythonXAI
import os
from xai_sdk import Client
from xai_sdk.chat import user, file
from xai_sdk.tools import code_execution

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Attach a file by public URL (or use file(file_id) for uploaded files)
chat = client.chat.create(
    model="grok-4.5",
    tools=[code_execution()],  # Enable code execution
)

chat.append(
    user(
        "Analyze this sales data and calculate: 1) Total revenue by product, 2) Average units sold by region, 3) Which product-region combination has the highest revenue",
        file(url="https://docs.x.ai/assets/api-examples/documents/sales-data.csv"),
    )
)

# Stream the response to see code execution in real-time
is_thinking = True
for response, chunk in chat.stream():
    for tool_call in chunk.tool_calls:
        if tool_call.function.name == "code_execution":
            print("\\n[Executing Code]")
    
    if response.usage.reasoning_tokens and is_thinking:
        print(f"\\rThinking... ({response.usage.reasoning_tokens} tokens)", end="", flush=True)
    
    if chunk.content and is_thinking:
        print("\\n\\nAnalysis Results:")
        is_thinking = False
    
    if chunk.content:
        print(chunk.content, end="", flush=True)

print(f"\\n\\nUsage: {response.usage}")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Attach a file by public URL (or use file_id for uploaded files)
const stream = await client.responses.create({
    model: "grok-4.5",
    input: [
        {
            role: "user",
            content: [
                {
                    type: "input_text",
                    text: "Analyze this sales data and calculate: 1) Total revenue by product, " +
                        "2) Average units sold by region, " +
                        "3) Which product-region combination has the highest revenue",
                },
                { type: "input_file", file_url: "https://docs.x.ai/assets/api-examples/documents/sales-data.csv" },
            ],
        },
    ],
    tools: [{ type: "code_interpreter" }],
    stream: true,
});

for await (const event of stream) {
    if (event.type === "response.output_text.delta") {
        process.stdout.write(event.delta);
    }
}

console.log();
```

The model will:

1. Access the attached data file
2. Write Python code to load and analyze the data
3. Execute the code in a sandboxed environment
4. Perform calculations and statistical analysis
5. Return the results and insights in the response

## Limitations and Considerations

### Request Constraints

* **No batch requests**: File attachments with document search are agentic requests and do not support batch mode (`n > 1`)
* **Streaming recommended**: Use streaming mode for better observability of document search process

### Document Complexity

* Highly unstructured or very long documents may require more processing
* Well-organized documents with clear structure are easier to search
* Large documents with many searches can result in higher token usage

### Model Compatibility

* **Recommended model**: `grok-4.5` for best document understanding
* **Agentic requirement**: File attachments require [agentic-capable](https://docs.x.ai/developers/tools/overview) models that support server-side tools.

## Next Steps

Learn more about managing your files:
{% endraw %}
