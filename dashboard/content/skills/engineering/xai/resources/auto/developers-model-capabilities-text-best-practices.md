+++
title = "developers-model-capabilities-text-best-practices"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Model Capabilities

# Best Practices Guide

This guide collects the practices that get the best results from Grok text and reasoning models on the xAI API. It covers which API to use, how to carry reasoning across turns, how to lower cost with prompt caching, how to configure requests, and how to run evaluations.

Start with these settings:

| Setting | Recommendation |
|---|---|
| [API](#use-the-responses-api) | Use the [Responses API](/developers/model-capabilities/text/generate-text). Chat Completions is deprecated. |
| [Multi-turn history](#send-encrypted-reasoning-back-with-every-turn) | Send the encrypted reasoning content back to the API with every turn. |
| [Prompt caching](#use-prompt-caching-to-lower-cost) | Give each conversation its own `prompt_cache_key`, and only append to the conversation. |
| [Reasoning effort](#reasoning-effort) | Set it explicitly on every request, and record it with every result. |
| [Streaming and timeouts](#streaming-and-timeouts) | Prefer streaming. Allow at least 900 seconds per request, and 1800 seconds or more for long-running evals. |
| [Output length](#output-length) | Set `max_output_tokens` above your longest expected answer, or leave it at its 128,000-token default. |
| [Sampling](#sampling-parameters) | Set `temperature` to 0.7 and `top_p` to 0.95, and don't stray far from them. |

## Use the Responses API

Use the Responses API (`POST /v1/responses`) with Grok reasoning models. The Chat Completions API is deprecated, and it has no field for the model's encrypted reasoning, so every multi-turn or tool-calling conversation sent through it discards the model's earlier thinking. New capabilities also arrive on the Responses API first.

If your code already uses Chat Completions, the [comparison guide](/developers/model-capabilities/text/comparison) maps each parameter and response field to its Responses API equivalent.

> [!WARNING]
>
> Chat Completions is offered only as a legacy endpoint. Results from it understate what Grok reasoning models can do, especially across multiple turns and tool calls.

## Send encrypted reasoning back with every turn

Grok reasoning models think before they answer. The Responses API returns that reasoning as an encrypted `reasoning` item in the response's `output` array, alongside the message and any function calls. When you send these items back unchanged in the next request's `input`, the model stays aware of its earlier reasoning traces. This drastically improves performance on multi-turn conversations and agent loops, where each step builds on decisions the model already reasoned through.

There are two ways to keep reasoning in the conversation:

* Chain requests with `previous_response_id` while `store` is on (the default). SpaceXAI stores the reasoning with the response and restores it for you.
* Manage the history yourself with `store: false`. Request the reasoning with `include: ["reasoning.encrypted_content"]` and append every output item to the next request's `input`. Each request is then self-contained and carries the full conversation, which you can log and replay.

`grok-4.7` returns `reasoning.encrypted_content` on every Responses API response, whether or not `include` lists it. Your code still has to send it back. See [Adding encrypted thinking content](/developers/model-capabilities/text/generate-text#adding-encrypted-thinking-content) for the full reference.

The following loop runs a scripted multi-turn conversation. It streams each turn, carries the encrypted reasoning forward, reuses one `prompt_cache_key` for the whole conversation, and prints how many input tokens were served from the cache:

```python customLanguage="pythonOpenAISDK"
import os
import uuid

import httpx
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
    timeout=httpx.Timeout(900.0),
)

conversation_id = str(uuid.uuid4())
history = [{"role": "system", "content": "You are a careful analyst. Answer concisely."}]

turns = [
    "A train leaves at 9:40 and arrives at 13:05. How long is the trip?",
    "The return trip is 20% faster. When does it arrive if it leaves at 15:00?",
]

for turn in turns:
    history.append({"role": "user", "content": turn})
    stream = client.responses.create(
        model="grok-4.7",
        input=history,
        reasoning={"effort": "high"},
        temperature=0.7,
        top_p=0.95,
        include=["reasoning.encrypted_content"],
        store=False,
        stream=True,
        extra_body={"prompt_cache_key": conversation_id},
    )
    response = None
    for event in stream:
        if event.type == "response.output_text.delta":
            print(event.delta, end="", flush=True)
        elif event.type in ("response.completed", "response.incomplete"):
            response = event.response
    print()
    # Append every output item, including reasoning items with encrypted_content.
    history.extend(response.output)
    print(f"Cached tokens: {response.usage.input_tokens_details.cached_tokens}")
```

```javascript customLanguage="javascriptOpenAISDK"
import { randomUUID } from "node:crypto";
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
    timeout: 900000,
});

const conversationId = randomUUID();
const history = [
    { role: "system", content: "You are a careful analyst. Answer concisely." },
];

const turns = [
    "A train leaves at 9:40 and arrives at 13:05. How long is the trip?",
    "The return trip is 20% faster. When does it arrive if it leaves at 15:00?",
];

for (const turn of turns) {
    history.push({ role: "user", content: turn });
    const stream = await client.responses.create({
        model: "grok-4.7",
        input: history,
        reasoning: { effort: "high" },
        temperature: 0.7,
        top_p: 0.95,
        include: ["reasoning.encrypted_content"],
        store: false,
        stream: true,
        prompt_cache_key: conversationId,
    });
    let response;
    for await (const event of stream) {
        if (event.type === "response.output_text.delta") {
            process.stdout.write(event.delta);
        } else if (event.type === "response.completed" || event.type === "response.incomplete") {
            response = event.response;
        }
    }
    console.log();
    // Append every output item, including reasoning items with encrypted_content.
    history.push(...response.output);
    console.log(`Cached tokens: ${response.usage.input_tokens_details.cached_tokens}`);
}
```

```python customLanguage="pythonXAI"
import os
import uuid

from xai_sdk import Client
from xai_sdk.chat import system, user

conversation_id = str(uuid.uuid4())

client = Client(
    api_key=os.getenv("XAI_API_KEY"),
    timeout=900,
    # Routes every turn of this conversation to the same server for cache reuse.
    metadata=(("x-grok-conv-id", conversation_id),),
)

chat = client.chat.create(
    model="grok-4.7",
    reasoning_effort="high",
    temperature=0.7,
    top_p=0.95,
    store_messages=False,
    use_encrypted_content=True,
)
chat.append(system("You are a careful analyst. Answer concisely."))

turns = [
    "A train leaves at 9:40 and arrives at 13:05. How long is the trip?",
    "The return trip is 20% faster. When does it arrive if it leaves at 15:00?",
]

for turn in turns:
    chat.append(user(turn))
    for response, chunk in chat.stream():
        print(chunk.content, end="", flush=True)
    print()
    # Appending the response also appends its encrypted reasoning.
    chat.append(response)
    print(f"Cached tokens: {response.usage.cached_prompt_text_tokens}")
```

The Vercel AI SDK carries encrypted reasoning automatically when you chain requests with `previousResponseId`; see [Reasoning](/developers/model-capabilities/text/reasoning#encrypted-reasoning-content).

### Keep reasoning in agent loops

Tool-calling workloads need the same treatment. When the model calls a function, its reasoning item arrives in the same `output` array as the `function_call`. Append the whole array, then your `function_call_output` items, so the model resumes with its plan intact when it reads the tool result:

```python customLanguage="pythonOpenAISDK"
import json

# client, tools, task_prompt, run_tool, and conversation_id come from your code.
def stream_turn(input_items):
    stream = client.responses.create(
        model="grok-4.7",
        input=input_items,
        tools=tools,
        reasoning={"effort": "high"},
        temperature=0.7,
        top_p=0.95,
        include=["reasoning.encrypted_content"],
        store=False,
        stream=True,
        extra_body={"prompt_cache_key": conversation_id},
    )
    for event in stream:
        if event.type in ("response.completed", "response.incomplete"):
            return event.response

input_items = [{"role": "user", "content": task_prompt}]

while True:
    response = stream_turn(input_items)
    # Reasoning, messages, and function calls all go back in order.
    input_items.extend(response.output)

    calls = [item for item in response.output if item.type == "function_call"]
    if not calls:
        break

    for call in calls:
        result = run_tool(call.name, json.loads(call.arguments))
        input_items.append({
            "type": "function_call_output",
            "call_id": call.call_id,
            "output": json.dumps(result),
        })

print(response.output_text)
```

Code drops reasoning most often in one of these ways:

* It extracts `output_text` and appends it as a plain assistant message, discarding the reasoning and function call items around it.
* It edits, reorders, or filters output items before sending them back. Pass them back exactly as received.
* It rebuilds the conversation from a transcript in a different format, such as a Chat Completions `messages` array, between turns.

For long agent loops, [WebSocket mode](/developers/advanced-api-usage/websocket-mode) reduces per-turn latency, and [context compaction](/developers/advanced-api-usage/context-compaction) keeps long conversations within the context window.

## Use prompt caching to lower cost

The xAI API caches prompts automatically, and cached input tokens are billed at a lower rate than regular input tokens; see [Pricing](/developers/pricing) for each model's cached token price. The cache matches your input from the start: when a request begins with exactly the same items as an earlier request, that matching prefix is served from the cache, and only the new items are processed. Multi-turn conversations and agent loops benefit the most, because each turn resends everything before it.

To get the most cache hits:

* Give each conversation its own [`prompt_cache_key`](/developers/advanced-api-usage/prompt-caching/maximizing-cache-hits) and reuse it on every turn. Cache entries live on individual servers, and the key routes every turn of the conversation to the same one. A UUID or your application's session ID works well. With the xAI SDK, send the same value as `x-grok-conv-id` gRPC metadata.
* Only append to the conversation. Editing, removing, or reordering earlier items breaks the prefix, and everything after the change is processed at the full price.
* Send the encrypted reasoning back unchanged. For reasoning models, leaving it out is the most common cause of cache misses. Chaining with `previous_response_id` keeps the prefix intact as well.
* Put static content first. System prompts, few-shot examples, and reference documents at the start of the input form a stable prefix that every turn can reuse.

Check `usage.input_tokens_details.cached_tokens` on each response. It should grow with each turn of a conversation; if it stays at 0, check that every turn sends the same `prompt_cache_key` and that earlier items are unchanged. Caching works with streaming and with tool calls, but cache hits are not guaranteed, because entries can be evicted under load. See [What Breaks Caching](/developers/advanced-api-usage/prompt-caching/multi-turn) for examples.

## Configure requests for reasoning models

Reasoning models spend tokens and time thinking before they respond, so reasoning effort, streaming and timeouts, output limits, and sampling parameters all affect your results.

### Reasoning effort

On the Responses API, `reasoning.effort` controls how much the model thinks. The supported values and the default depend on the model; `grok-4.7` defaults to `"high"`, while `grok-4.3` defaults to `"low"` and also accepts `"none"`. Set the effort explicitly on every request so a default never decides your result, and choose the effort you intend to run in production. If cost or latency matters, compare effort levels and weigh quality against tokens and time. See the [`reasoning_effort`](/developers/model-capabilities/text/reasoning#the-reasoning_effort-parameter) parameter for the supported values per model.

### Streaming and timeouts

Prefer [streaming](/developers/model-capabilities/text/streaming) (`stream: true`) to non-streaming requests. A streamed response sends events as the model works, which keeps the connection from dropping during long generations, and the final `response.completed` event carries the full response, including the encrypted reasoning items to send back.

Set the client timeout to at least 900 seconds, as the examples in this guide do. High reasoning effort on complex problems can run for several minutes, and long-running evals such as Humanity's Last Exam (HLE) can take upwards of 1800 seconds per request. Raise the timeout above your longest expected request so slow requests are not recorded as failures.

### Output length

`max_output_tokens` caps only the visible answer; reasoning and function call tokens do not count toward it. When unset, it defaults to 128,000 tokens. A tight cap cuts the answer off, so set the cap above the longest answer you expect, and treat responses with `status: "incomplete"` as truncated.

### Sampling parameters

> [!WARNING]
>
> Set `temperature` to 0.7 and `top_p` to 0.95, and don't deviate far from these values. Grok models don't respond well to other sampling settings, and quality usually drops when you change them.

On [`POST /v1/chat/completions`](/developers/rest-api-reference/inference/chat-completions) and [`POST /v1/responses`](/developers/rest-api-reference/inference/responses), sampling uses at most `temperature` 1 and at most the model's default `top_p`. Current Grok text models default `top_p` to 0.95. The API accepts `temperature` from 0 to 2 and `top_p` up to 1, and a value above a cap is sampled as that cap. The Responses API echoes the values you sent on the response, including values above the cap. Omitting a field uses the model's default for that field.

Set both values explicitly on every request so your results don't depend on server defaults, and use the same values for every model you compare.

### Errors, retries, and rate limits

Separate infrastructure failures from model failures. Retry `429` and `5xx` responses with exponential backoff, and rerun any request that still fails until it gets a response. Large workloads should respect your team's [rate limits](/developers/rate-limits).

## Evaluate Grok models

A good evaluation tells you how a model will behave on your workload, which is rarely the same as how it scores on a public leaderboard. Apply the settings above, then design and run the eval with these practices.

### Design the eval

Start by writing down what success means for your application in terms you can measure. "Answers support questions well" is too vague to test. A testable version is "resolves at least 85% of a 500-ticket sample without inventing a policy, with a median latency under 20 seconds." Most applications need several criteria at once, such as task accuracy, instruction following, format compliance, safety, latency, and cost.

Build the dataset from the work the model will actually do. Production logs, with personal data removed, are the most representative source; add hand-written edge cases, adversarial inputs, and long or multi-turn examples. Check whether public benchmark items might appear in training data, and keep in mind each model's knowledge cutoff (May 2026 for `grok-4.7`) when questions depend on recent events. If the task needs current information, enable [web search](/developers/tools/web-search) and evaluate that configuration directly.

Choose the cheapest grader that is reliable for the criterion:

| Grader | Best for | Watch out for |
|---|---|---|
| Code | Exact answers, unit tests, schema validation, tool-call arguments | Rejecting correct answers in an unexpected format |
| Model as judge | Open-ended quality, rubric scoring, pairwise comparison | Position bias, preference for longer answers, drift from human judgment |
| Human review | Calibrating other graders, high-stakes or subjective tasks | Cost, speed, and disagreement between reviewers |

When you use a model as a judge, give it a specific rubric with examples of each score, ask it to reason before it scores, and randomize the order of answers in pairwise comparisons. Measure its agreement with human labels on a sample before you trust it at scale.

Prompt each model fairly. Grok reasoning models reason before they answer, so state the task, constraints, and output format directly. If you tuned a prompt for another model, spend comparable effort tuning it for Grok before you compare scores.

### Run, measure, and compare

Reasoning models are not deterministic, so a single run can mislead you. Run each item several times, report the mean and spread, and use pass@k where any one correct attempt counts. Treat small differences between models with suspicion until they hold across runs.

If you must exclude an item that keeps failing, exclude it for every model you compare so all of them are scored on the same set, and report how many items you excluded; dropping failures for one model alone inflates its score.

Log the full response for every request, including its `usage` object. `usage.output_tokens_details.reasoning_tokens` reports how many tokens the model spent reasoning, `usage.input_tokens_details.cached_tokens` shows how much of the input came from the cache, and `usage.cost_in_usd_ticks` gives the exact billed cost of each request; together with wall-clock latency, they let you compare quality per dollar and per second. See [Cost Tracking](/developers/cost-tracking) for converting ticks to dollars. Record the model name, reasoning effort, sampling parameters, date, and harness version with every result, because models and harnesses change.

Before you report results, confirm that:

* Every request used the Responses API.
* Multi-turn and tool-calling items carried their encrypted reasoning forward unchanged.
* Reasoning effort, sampling parameters, model name, and harness version are recorded and match across the models you compare.
* Failed requests were retried, any exclusions apply to every model equally, and truncated responses were flagged.

## Related

* [Generate Text](/developers/model-capabilities/text/generate-text): the Responses API, stored conversations, and encrypted thinking content
* [Reasoning](/developers/model-capabilities/text/reasoning): reasoning effort, encrypted reasoning, and reasoning summaries
* [Streaming](/developers/model-capabilities/text/streaming): streaming responses with Server-Sent Events
* [Prompt Caching](/developers/advanced-api-usage/prompt-caching): how caching works and how to maximize cache hits
* [Function Calling](/developers/tools/function-calling): defining tools and handling tool calls
* [Comparison with Chat Completions](/developers/model-capabilities/text/comparison): migrating from Chat Completions


