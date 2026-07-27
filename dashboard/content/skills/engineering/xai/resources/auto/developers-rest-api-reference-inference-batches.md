+++
title = "developers-rest-api-reference-inference-batches"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Inference API

# Batches

## POST /v1/batches

Create a new batch for processing requests.

### Request Body

* `name` (string, required) — The name of the batch created.

### Response Body

* `batch_id` (string, required) — The ID of the batch.

* `cancel_by_xai_message` (string | null) — If the batch was cancelled by xAI, an error message explaining why.

* `cancel_time` (string | null) — Time when the batch was cancelled.

* `create_api_key_id` (string, required) — ID of the API key that was used to create the batch.

* `create_time` (string, required) — The time when the batch was created.

* `expire_time` (string | null) — The time when the batch expires.

* `name` (string, required) — The human-readable name of the batch.

* `state` (object, required) — Holds aggregate information about the current state of a batch process.

  * `num_cancelled` (integer, required) — Total number of requests that have been cancelled.

  * `num_error` (integer, required) — Total number of requests that finished with an error.

  * `num_pending` (integer, required) — Total number of pending requests.

  * `num_requests` (integer, required) — Total number of requests in the batch.

  * `num_success` (integer, required) — Total number of requests that have finished successfully.

\*\*Request example:\*\*

```json
{
  "name": "My New Batch"
}
```

\*\*Response example:\*\*

```json
{
  "batch_id": "batch_1934e8b5-f3dc-45f1-8329-9841b0aee9d8",
  "name": "My New Batch",
  "create_time": "2025-11-11",
  "expire_time": "2025-11-12",
  "create_api_key_id": "********-****-****-****-************",
  "cancel_time": null,
  "cancel_by_xai_message": null,
  "state": {
    "num_requests": 0,
    "num_pending": 0,
    "num_success": 0,
    "num_error": 0,
    "num_cancelled": 0
  }
}
```

***

## GET /v1/batches

List all batches for the current team.

### Query Parameters

* `limit` (integer | null) — Number of elements to return.

* `pagination_token` (string | null) — Optional page token to retrieve a specific page. Provided by \`pagination\_token\` in \`ListBatchesResponse\`.

### Response Body

* `batches` (array\<object>, required) — The information about the batches.

  * `batch_id` (string, required) — The ID of the batch.

  * `cancel_by_xai_message` (string | null) — If the batch was cancelled by xAI, an error message explaining why.

  * `cancel_time` (string | null) — Time when the batch was cancelled.

  * `create_api_key_id` (string, required) — ID of the API key that was used to create the batch.

  * `create_time` (string, required) — The time when the batch was created.

  * `expire_time` (string | null) — The time when the batch expires.

  * `name` (string, required) — The human-readable name of the batch.

  * `state` (object, required) — Holds aggregate information about the current state of a batch process.

    * `num_cancelled` (integer, required) — Total number of requests that have been cancelled.

    * `num_error` (integer, required) — Total number of requests that finished with an error.

    * `num_pending` (integer, required) — Total number of pending requests.

    * `num_requests` (integer, required) — Total number of requests in the batch.

    * `num_success` (integer, required) — Total number of requests that have finished successfully.

* `pagination_token` (string | null) — The page token to retrieve batches from the next page. Will be empty if this is the last page.

\*\*Response example:\*\*

```json
{
  "batches": [
    {
      "batch_id": "batch_1934e8b5-f3dc-45f1-8329-9841b0aee9d8",
      "name": "My New Batch",
      "create_time": "2025-11-11",
      "expire_time": "2025-11-12",
      "create_api_key_id": "********-****-****-****-************",
      "cancel_time": null,
      "cancel_by_xai_message": null,
      "state": {
        "num_requests": 0,
        "num_pending": 0,
        "num_success": 0,
        "num_error": 0,
        "num_cancelled": 0
      }
    },
    {
      "batch_id": "batch_bac0e657-6bbf-46ba-a671-1d73a67c132a",
      "name": "MyNewBatch",
      "create_time": "2025-11-09",
      "expire_time": "2025-11-10",
      "create_api_key_id": "********-****-****-****-************",
      "cancel_time": "2025-11-09",
      "cancel_by_xai_message": null,
      "state": {
        "num_requests": 1,
        "num_pending": 0,
        "num_success": 1,
        "num_error": 0,
        "num_cancelled": 0
      }
    }
  ]
}
```

***

## GET /v1/batches/\{batch\_id}

Get information about a specific batch.

### Path Parameters

* `batch_id` (string, required) — The unique identifier of the batch

### Response Body

* `batch_id` (string, required) — The ID of the batch.

* `cancel_by_xai_message` (string | null) — If the batch was cancelled by xAI, an error message explaining why.

* `cancel_time` (string | null) — Time when the batch was cancelled.

* `create_api_key_id` (string, required) — ID of the API key that was used to create the batch.

* `create_time` (string, required) — The time when the batch was created.

* `expire_time` (string | null) — The time when the batch expires.

* `name` (string, required) — The human-readable name of the batch.

* `state` (object, required) — Holds aggregate information about the current state of a batch process.

  * `num_cancelled` (integer, required) — Total number of requests that have been cancelled.

  * `num_error` (integer, required) — Total number of requests that finished with an error.

  * `num_pending` (integer, required) — Total number of pending requests.

  * `num_requests` (integer, required) — Total number of requests in the batch.

  * `num_success` (integer, required) — Total number of requests that have finished successfully.

\*\*Response example:\*\*

```json
{
  "batch_id": "batch_1934e8b5-f3dc-45f1-8329-9841b0aee9d8",
  "name": "My New Batch",
  "create_time": "2025-11-11",
  "expire_time": "2025-11-12",
  "create_api_key_id": "********-****-****-****-************",
  "cancel_time": null,
  "cancel_by_xai_message": null,
  "state": {
    "num_requests": 0,
    "num_pending": 0,
    "num_success": 0,
    "num_error": 0,
    "num_cancelled": 0
  }
}
```

***

## GET /v1/batches/\{batch\_id}/requests

List metadata for all requests in a batch.

### Path Parameters

* `batch_id` (string, required) — The unique identifier of the batch

### Query Parameters

* `limit` (integer | null) — Maximum number of items to return in a single page (max 1000)

* `pagination_token` (string | null) — Token for retrieving the next page of results

### Response Body

* `batch_request_metadata` (array\<object>, required) — The batch request metadata for the given batch.

  * `batch_request_id` (string, required) — ID of the request. Unique within this batch.

  * `create_time` (string, required) — Time when the request was recorded.

  * `endpoint` (string, required) — API endpoint to query.

  * `finish_time` (string | null) — Time when the response was recorded.

  * `model` (string, required) — Model name to query.

  * `state` ("unknown" | "pending" | "succeeded" | "cancelled" | "failed", required)

* `pagination_token` (string | null) — The page token to retrieve results from the next page. Will be empty if this is the last page.

\*\*Response example:\*\*

```json
{
  "batch_request_metadata": [
    {
      "batch_request_id": "test_request_0",
      "endpoint": "xai_api.Chat/GetCompletion",
      "model": "grok-4",
      "state": "succeeded",
      "create_time": "2025-11-11",
      "finish_time": "2025-11-12"
    }
  ],
  "pagination_token": null
}
```

***

## POST /v1/batches/\{batch\_id}/requests

Add multiple requests to an existing batch.

### Path Parameters

* `batch_id` (string, required) — The unique identifier of the batch

### Request Body

* `batch_requests` (array\<object>, required) — List of batch requests to add to the batch

  * `batch_request` (object, required)

    * `chat_get_completion` (object, required) — The chat request body for \`/v1/chat/completions\` endpoint.

      * `deferred` (boolean | null) — If set to \`true\`, the request returns a \`request\_id\`. You can then get the deferred response by GET \`/v1/chat/deferred-completion/\{request\_id}\`.

      * `frequency_penalty` (number | null) — (Not supported by reasoning models) Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

      * `logit_bias` (object | null) — (Unsupported) A JSON object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

      * `logprobs` (boolean | null) — Whether to return log probabilities of the output tokens or not. If true, returns the log probabilities of each output token returned in the content of message.

      * `max_completion_tokens` (integer | null) — An upper bound for the number of tokens that can be generated for a completion, only applies to visible output tokens (i.e. does not apply to tokens used for reasoning or function calls). Defaults to None, meaning the model will generate as many tokens as needed up until the model's maximum context length.

      * `max_tokens` (integer | null) — \\\[DEPRECATED\\] The maximum number of tokens that can be generated in the chat completion. Deprecated in favor of \`max\_completion\_tokens\`.

      * `messages` (array\<object | object | object | object | object>) — A list of messages that make up the the chat conversation. Different models support different message types, such as image and text.

      * `model` (string) — Model name for the model to use. Obtainable from \<https://console.x.ai/team/default/models> or \<https://docs.x.ai/docs/models>.

      * `n` (integer | null) — How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep n as 1 to minimize costs.

      * `parallel_tool_calls` (boolean | null) — If set to false, the model can perform maximum one tool call.

      * `presence_penalty` (number | null) — (Not supported by \`grok-3\` and reasoning models) Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

      * `reasoning_effort` (string | null) — Constrains how hard a reasoning model thinks before responding. Not supported by \`grok-4\` and will result in error if used with \`grok-4\`. Possible values are \`low\` (uses fewer reasoning tokens) and \`high\` (uses more reasoning tokens).

      * `response_format` (object | object | object)

      * `search_parameters` (object)

        * `from_date` (string | null) — Date from which to consider the results in ISO-8601 YYYY-MM-DD. See
          \<https://en.wikipedia.org/wiki/ISO\_8601>.

        * `max_search_results` (integer | null) — Maximum number of search results to use.

        * `mode` (string | null) — Choose the mode to query realtime data:
          \* \`off\`: no search performed and no external will be considered.
          \* \`on\` (default): the model will search in every sources for relevant data.
          \* \`auto\`: the model choose whether to search data or not and where to search the data.

        * `return_citations` (boolean | null) — Whether to return citations in the response or not.

        * `sources` (array | null) — List of sources to search in. If no sources specified, the model will look over the web and X by default.

        * `to_date` (string | null) — Date up to which to consider the results in ISO-8601 YYYY-MM-DD. See
          \<https://en.wikipedia.org/wiki/ISO\_8601>.

      * `seed` (integer | null) — If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same \`seed\` and parameters should return the same result. Determinism is not guaranteed, and you should refer to the \`system\_fingerprint\` response parameter to monitor changes in the backend.

      * `stop` (array | null) — (Not supported by reasoning models) Up to 4 sequences where the API will stop generating further tokens.

      * `stream` (boolean | null) — If set, partial message deltas will be sent. Tokens will be sent as data-only server-sent events as they become available, with the stream terminated by a \`data: \[DONE]\` message.

      * `stream_options` (object)

        * `include_usage` (boolean, required) — Set an additional chunk to be streamed before the \`data: \[DONE]\` message. The other chunks will return \`null\` in \`usage\` field.

      * `temperature` (number | null) — What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.

      * `tool_choice` (string | object)

      * `tools` (array | null) — A list of tools the model may call in JSON-schema. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.

      * `top_logprobs` (integer | null) — An integer between 0 and 8 specifying the number of most likely tokens to return at each token position, each with an associated log probability. logprobs must be set to true if this parameter is used.

      * `top_p` (number | null) — An alternative to sampling with \`temperature\`, called nucleus sampling, where the model considers the results of the tokens with \`top\_p\` probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered. It is generally recommended to alter this or \`temperature\` but not both.

      * `user` (string | null) — A unique identifier representing your end-user, which can help xAI to monitor and detect abuse.

      * `web_search_options` (object)

        * `filters` (object) — Only included for compatibility.

        * `search_context_size` (string | null) — This field included for compatibility reason with OpenAI's API. It is mapped to \`max\_search\`.

        * `user_location` (object) — Only included for compatibility.

  * `batch_request_id` (string | null) — User-provided identifier for the input request. If provided, it must be unique within the batch.
    Used to identify the corresponding result when the response is returned to the user.
    This is because the order of the returned results is not guaranteed to be the same as the order of the requests.

\*\*Request example:\*\*

```json
{
  "batch_requests": [
    {
      "batch_request_id": "test_request_0",
      "batch_request": {
        "chat_get_completion": {
          "messages": [
            {
              "role": "system",
              "content": "You are a helpful assistant that can answer questions and help with tasks."
            },
            {
              "role": "user",
              "content": "What is 101*3?"
            }
          ],
          "model": "grok-4"
        }
      }
    }
  ]
}
```

***

## GET /v1/batches/\{batch\_id}/results

List the processing results for a batch.

### Path Parameters

* `batch_id` (string, required) — The unique identifier of the batch

### Query Parameters

* `limit` (integer | null) — Maximum number of items to return in a single page (max 1000)

* `pagination_token` (string | null) — Token for retrieving the next page of results

### Response Body

* `pagination_token` (string | null) — The page token to retrieve results from the next page. Will be empty if this is the last page.

* `results` (array\<object>, required) — The results that has been processed.

  * `batch_request_id` (string, required) — User-provided or generated identifier for the input request. If a user has provided \`batch\_request\_id\` in the
    \`BatchRequest\`, the value will match the user-provided value.
    The value is unique within the batch.

  * `batch_result` (object | object, required)

    * `error` (string, required)

    * `response` ("unknown" | object, required)

      * `chat_get_completion` (object, required) — The chat response body for \`/v1/chat/completions\` endpoint.

        * `choices` (array\<object>, required) — A list of response choices from the model. The length corresponds to the \`n\` in request body (default to 1).

          * `finish_reason` (string | null) — Finish reason. \`"stop"\` means the inference has reached a model-defined or user-supplied stop sequence in \`stop\`. \`"length"\` means the inference result has reached models' maximum allowed token length or user defined value in \`max\_tokens\`. \`"end\_turn"\` or \`null\` in streaming mode when the chunk is not the last.

          * `index` (integer, required) — Index of the choice within the response choices, starting from 0.

          * `logprobs` (object)

            * `content` (array | null) — An array the log probabilities of each output token returned.

          * `message` (object, required)

            * `content` (string | null) — The content of the message.

            * `reasoning_content` (string | null) — The reasoning trace generated by the model.

            * `refusal` (string | null) — The reason given by model if the model is unable to generate a response. null if model is able to generate.

            * `role` (string, required) — The role that the message belongs to, the response from model is always \`"assistant"\`.

            * `tool_calls` (array | null) — A list of tool calls asked by model for user to perform.

        * `citations` (array | null) — List of all the external pages used by the model to answer.

        * `created` (integer, required) — The chat completion creation time in Unix timestamp.

        * `debug_output` (object)

          * `attempts` (integer, required) — Number of attempts made to the model.

          * `cache_read_count` (integer, required) — Number of cache reads

          * `cache_read_input_bytes` (integer, required) — Size of cache read

          * `cache_write_count` (integer, required) — Number of cache writes

          * `cache_write_input_bytes` (integer, required) — Size of cache write

          * `chunks` (array\<string>, required) — The individual chunks returned from the pipeline of samplers.

          * `engine_request` (string, required) — JSON-serialized request sent to the inference engine.

          * `lb_address` (string, required) — The load balancer address

          * `prompt` (string, required) — The prompt sent to the model in text form.

          * `request` (string, required) — The request received from the user.

          * `responses` (array\<string>, required) — The response(s) received from the model.

          * `sampler_checkpoint_mount` (string, required) — The underlying checkpoint mount path for the sampler that served this request.

          * `sampler_tag` (string, required) — The tag of the actual engines sitting behind the GTP address. Eg "grok-4-code-eapi-lap4-unified-sblbm-0"

        * `id` (string, required) — A unique ID for the chat response.

        * `model` (string, required) — Model ID used to create chat completion.

        * `object` (string, required) — The object type, which is always \`"chat.completion"\`.

        * `system_fingerprint` (string | null) — System fingerprint, used to indicate xAI system configuration changes.

        * `usage` (object)

          * `completion_tokens` (integer, required) — Total completion token used.

          * `completion_tokens_details` (object, required) — Details of completion usage.

            * `accepted_prediction_tokens` (integer, required) — The number of tokens in the prediction that appeared in the completion.

            * `audio_tokens` (integer, required) — Audio input tokens generated by the model.

            * `reasoning_tokens` (integer, required) — Tokens generated by the model for reasoning.

            * `rejected_prediction_tokens` (integer, required) — The number of tokens in the prediction that did not appear in the completion.

          * `num_sources_used` (integer, required) — Number of individual live search source used.

          * `prompt_tokens` (integer, required) — Total prompt token used.

          * `prompt_tokens_details` (object, required) — Details of prompt usage.

            * `audio_tokens` (integer, required) — Audio prompt token used.

            * `cached_tokens` (integer, required) — Token cached by xAI from previous requests and reused for this request.

            * `image_tokens` (integer, required) — Image prompt token used.

            * `text_tokens` (integer, required) — Text prompt token used.

          * `total_tokens` (integer, required) — Total token used, the sum of prompt token and completion token amount.

\*\*Response example:\*\*

```json
{
  "results": [
    {
      "batch_request_id": "test_request_0",
      "batch_result": {
        "response": {
          "chat_get_completion": {
            "id": "e7c2162b-ca73-c181-2364-1feabef778fe_us-east-1",
            "object": "chat.completion",
            "created": 1762801725,
            "model": "grok-4",
            "choices": [
              {
                "index": 0,
                "message": {
                  "role": "assistant",
                  "content": "101 multiplied by 3 is 303. If you have more calculations or questions, feel free to ask!",
                  "refusal": null
                },
                "finish_reason": "stop"
              }
            ],
            "usage": {
              "prompt_tokens": 706,
              "completion_tokens": 22,
              "total_tokens": 827,
              "prompt_tokens_details": {
                "text_tokens": 706,
                "audio_tokens": 0,
                "image_tokens": 0,
                "cached_tokens": 679
              },
              "completion_tokens_details": {
                "reasoning_tokens": 99,
                "audio_tokens": 0,
                "accepted_prediction_tokens": 0,
                "rejected_prediction_tokens": 0
              },
              "num_sources_used": 0
            },
            "system_fingerprint": "fp_1944a19e1f"
          }
        }
      }
    }
  ],
  "pagination_token": null
}
```

***

## POST /v1/batches/\{batch\_id}:cancel

Cancel processing of all requests in a batch.

### Path Parameters

* `batch_id` (string, required) — The unique identifier of the batch to cancel

### Response Body

* `batch_id` (string, required) — The ID of the batch.

* `cancel_by_xai_message` (string | null) — If the batch was cancelled by xAI, an error message explaining why.

* `cancel_time` (string | null) — Time when the batch was cancelled.

* `create_api_key_id` (string, required) — ID of the API key that was used to create the batch.

* `create_time` (string, required) — The time when the batch was created.

* `expire_time` (string | null) — The time when the batch expires.

* `name` (string, required) — The human-readable name of the batch.

* `state` (object, required) — Holds aggregate information about the current state of a batch process.

  * `num_cancelled` (integer, required) — Total number of requests that have been cancelled.

  * `num_error` (integer, required) — Total number of requests that finished with an error.

  * `num_pending` (integer, required) — Total number of pending requests.

  * `num_requests` (integer, required) — Total number of requests in the batch.

  * `num_success` (integer, required) — Total number of requests that have finished successfully.

\*\*Response example:\*\*

```json
{
  "batch_id": "batch_1934e8b5-f3dc-45f1-8329-9841b0aee9d8",
  "name": "My New Batch",
  "create_time": "2025-11-11",
  "expire_time": "2025-11-12",
  "create_api_key_id": "********-****-****-****-************",
  "cancel_time": "2025-11-11",
  "cancel_by_xai_message": null,
  "state": {
    "num_requests": 1,
    "num_pending": 0,
    "num_success": 1,
    "num_error": 0,
    "num_cancelled": 0
  }
}
```
{% endraw %}
