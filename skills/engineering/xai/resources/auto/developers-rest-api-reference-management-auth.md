#### Management API

# Accounts and Authorization

***

## POST /auth/teams/\{teamId}/api-keys

API keys are used to access the xAI API (https://api.x.ai). They are included on requests as a
HTTP Bearer token. API keys are bound to teams and are associated with the user who created
them.

### Path Parameters

* `teamId` (string, required) — ID of the team this API key belongs to. The team ID can be copied here:
  https://console.x.ai/team/default/settings/team.

### Request Body

* `name` (string, required) — Human-readable name for the API key. Should not be empty.

* `acls` (array\<string>) — By default API keys don't have access to anything. In order to actually use an API key, you
  must grant it access to (1) endpoints and (2) models.

  Access is granted via strings of the form \`api-key:endpoint:\[endpoint name]\` and
  \`api-key:model:\[model name]\` where \`endpoint name\` and \`model name\` can be retrieved via the
  \`/auth/teams/\{team\_id}/endpoints\` and \`/auth/teams/\{team\_id}/models\` endpoints respectively.

  ACLs also support wildcards. If you want to create an API key that has access to all models and
  endpoints, then add the two ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

* `qps` (integer) — If set, the API key is limited to performing this number of queries per second.

* `qpm` (integer) — If set, the API key is limited to performing this number of queries per minute.

* `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
  The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
  limit to get exceeded will not be aborted.

* `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

### Response Body

* `redactedApiKey` (string) — A redacted API key. We don't expose the full key after it has been created.

* `apiKey` (string) — Only set when the API key is created.

* `userId` (string) — ID of the User who created this API key.

* `name` (string) — Human-readable name for the API key.

* `createTime` (string) — Timestamp when the API key was created.

* `modifyTime` (string) — Timestamp when the API key was modified.

* `teamId` (string) — ID of the team this API key belongs to.

* `apiKeyId` (string) — The ID of the API key.

* `disabled` (boolean) — If API is disabled (by the user) or not. Users can disable API keys to prevent them from making
  API calls.

* `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

* `qps` (integer) — If set, this API key can only perform the stated number of requests per second.

* `qpm` (integer) — If set, this API key can only perform the stated number of requests per minute.

* `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
  The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
  limit to get exceeded will not be aborted.

* `aclStrings` (array\<string>) — The permissions the API key has. By default, API keys don't have any permissions, which means
  all requests fail if this field is empty. There are two kind of permissions users can grant:
  (1) endpoints and (2) models via the \`api-key:endpoint:\[endpoint name]\` and
  \`api-key:model:\[model name]\` ACLs.

  If you want to grant access to specific endpoints and ACLs, you can use the corresponding
  \`/endpoints\` and \`/models\` endpoints to retrieve possible values for \`endpoint name\` and
  \`model name\`.

  If you want to create an API key that has access to all endpoints and models, you can use the
  wildcard ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

\*\*Request example:\*\*

```json
{
  "name": "My API key",
  "acls": [
    "api-key:endpoint:*",
    "api-key:model:*"
  ]
}
```

\*\*Response example:\*\*

```json
{
  "redactedApiKey": "xai-...oYZ4",
  "apiKey": "xai-2xr2bnFV7lAbcdelZbqSxLd77pjlU6gcFmt3tPLSK3d7z7302htoH6mc4LlKlTY7nJtlU0lRXXQjoYZ4",
  "userId": "6ad994df-7963-45e0-96c8-96b1ba849d9b",
  "name": "My API key",
  "createTime": "2025-11-13T16:14:38.164829Z",
  "modifyTime": "2025-11-13T16:38:07.059039Z",
  "teamId": "c9a0c990-53e6-491e-8df7-b9f18e6983ac",
  "apiKeyId": "fe15b799-32e1-4d02-9e65-80236f9995f7",
  "disabled": false,
  "aclStrings": [
    "api-key:endpoint:*",
    "api-key:model:*"
  ]
}
```

***

## GET /auth/teams/\{teamId}/api-keys

Lists API keys belonging to a user within a team. If the caller user is an admin, returns all
team API keys. If the caller user is a member, returns that user's API keys.

### Path Parameters

* `teamId` (string, required) — ID of the team whose API keys shall be listed.

### Query Parameters

* `pageSize` (integer) — Control page size for result. It None, result is returned as one page.

* `paginationToken` (string) — Pagination token received from a previous call when using pagination. Set to \`undefined\` to
  retrieve the first page.

* `aclFilters` (array\<string>) — Optional filter to only return API keys that match the given ACLs.

* `activeOnly` (boolean) — If true, only return non-expired API keys. Defaults to false (return all keys).

### Response Body

* `apiKeys` (array\<object>) — API keys belonging to the team.

  * `redactedApiKey` (string) — A redacted API key. We don't expose the full key after it has been created.

  * `apiKey` (string) — Only set when the API key is created.

  * `userId` (string) — ID of the User who created this API key.

  * `name` (string) — Human-readable name for the API key.

  * `createTime` (string) — Timestamp when the API key was created.

  * `modifyTime` (string) — Timestamp when the API key was modified.

  * `teamId` (string) — ID of the team this API key belongs to.

  * `apiKeyId` (string) — The ID of the API key.

  * `disabled` (boolean) — If API is disabled (by the user) or not. Users can disable API keys to prevent them from making
    API calls.

  * `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

  * `qps` (integer) — If set, this API key can only perform the stated number of requests per second.

  * `qpm` (integer) — If set, this API key can only perform the stated number of requests per minute.

  * `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
    The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
    limit to get exceeded will not be aborted.

  * `aclStrings` (array\<string>) — The permissions the API key has. By default, API keys don't have any permissions, which means
    all requests fail if this field is empty. There are two kind of permissions users can grant:
    (1) endpoints and (2) models via the \`api-key:endpoint:\[endpoint name]\` and
    \`api-key:model:\[model name]\` ACLs.

    If you want to grant access to specific endpoints and ACLs, you can use the corresponding
    \`/endpoints\` and \`/models\` endpoints to retrieve possible values for \`endpoint name\` and
    \`model name\`.

    If you want to create an API key that has access to all endpoints and models, you can use the
    wildcard ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

* `paginationToken` (string) — Include this token on a follow-up request to retrieve the next page. If \`undefined\`, this is
  the last page.

\*\*Response example:\*\*

```json
{
  "apiKeys": [
    {
      "redactedApiKey": "xai-a**b",
      "userId": "12106c22-fa54-4255-9887-6eb73c55787f",
      "apiKeyId": "faf5f6c2-5322-4793-9b74-23129addca0c",
      "teamId": "fef05426-f6a4-4242-b9a6-b1a0ea5dc5fd",
      "disabled": "false",
      "tpm": "100000",
      "acl_strings": [
        "api-key:endpoint:*",
        "api-key:model:*"
      ]
    }
  ]
}
```

***

## PUT /auth/api-keys/\{api\_key\_id}

Selectively updates a subset of fields on an API key.

### Path Parameters

* `api_key_id` (string, required) — The ID of the API key.

### Request Body

* `apiKey` (object, required) — API key.

  * `redactedApiKey` (string) — A redacted API key. We don't expose the full key after it has been created.

  * `apiKey` (string) — Only set when the API key is created.

  * `userId` (string) — ID of the User who created this API key.

  * `name` (string) — Human-readable name for the API key.

  * `createTime` (string) — Timestamp when the API key was created.

  * `modifyTime` (string) — Timestamp when the API key was modified.

  * `teamId` (string) — ID of the team this API key belongs to.

  * `disabled` (boolean) — If API is disabled (by the user) or not. Users can disable API keys to prevent them from making
    API calls.

  * `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

  * `qps` (integer) — If set, this API key can only perform the stated number of requests per second.

  * `qpm` (integer) — If set, this API key can only perform the stated number of requests per minute.

  * `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
    The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
    limit to get exceeded will not be aborted.

  * `aclStrings` (array\<string>) — The permissions the API key has. By default, API keys don't have any permissions, which means
    all requests fail if this field is empty. There are two kind of permissions users can grant:
    (1) endpoints and (2) models via the \`api-key:endpoint:\[endpoint name]\` and
    \`api-key:model:\[model name]\` ACLs.

    If you want to grant access to specific endpoints and ACLs, you can use the corresponding
    \`/endpoints\` and \`/models\` endpoints to retrieve possible values for \`endpoint name\` and
    \`model name\`.

    If you want to create an API key that has access to all endpoints and models, you can use the
    wildcard ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

* `fieldMask` (string) — Field mask indicating which fields will be modified.

### Response Body

* `redactedApiKey` (string) — A redacted API key. We don't expose the full key after it has been created.

* `apiKey` (string) — Only set when the API key is created.

* `userId` (string) — ID of the User who created this API key.

* `name` (string) — Human-readable name for the API key.

* `createTime` (string) — Timestamp when the API key was created.

* `modifyTime` (string) — Timestamp when the API key was modified.

* `teamId` (string) — ID of the team this API key belongs to.

* `apiKeyId` (string) — The ID of the API key.

* `disabled` (boolean) — If API is disabled (by the user) or not. Users can disable API keys to prevent them from making
  API calls.

* `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

* `qps` (integer) — If set, this API key can only perform the stated number of requests per second.

* `qpm` (integer) — If set, this API key can only perform the stated number of requests per minute.

* `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
  The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
  limit to get exceeded will not be aborted.

* `aclStrings` (array\<string>) — The permissions the API key has. By default, API keys don't have any permissions, which means
  all requests fail if this field is empty. There are two kind of permissions users can grant:
  (1) endpoints and (2) models via the \`api-key:endpoint:\[endpoint name]\` and
  \`api-key:model:\[model name]\` ACLs.

  If you want to grant access to specific endpoints and ACLs, you can use the corresponding
  \`/endpoints\` and \`/models\` endpoints to retrieve possible values for \`endpoint name\` and
  \`model name\`.

  If you want to create an API key that has access to all endpoints and models, you can use the
  wildcard ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

\*\*Request example:\*\*

```json
{
  "apiKey": {
    "tpm": "42"
  },
  "fieldMask": "tpm"
}
```

\*\*Response example:\*\*

```json
{
  "redactedApiKey": "xai-...oYZ4",
  "apiKey": "xai-2xr2bnFV7lAbcdelZbqSxLd77pjlU6gcFmt3tPLSK3d7z7302htoH6mc4LlKlTY7nJtlU0lRXXQjoYZ4",
  "userId": "6ad994df-7963-45e0-96c8-96b1ba849d9b",
  "name": "My API key",
  "createTime": "2025-11-13T16:14:38.164829Z",
  "modifyTime": "2025-11-13T16:38:07.059039Z",
  "teamId": "c9a0c990-53e6-491e-8df7-b9f18e6983ac",
  "apiKeyId": "fe15b799-32e1-4d02-9e65-80236f9995f7",
  "disabled": false,
  "aclStrings": [
    "api-key:endpoint:*",
    "api-key:model:*"
  ]
}
```

***

## POST /auth/api-keys/\{apiKeyId}/rotate

!!CAUTION!! Rotates the secret of an existing API key, permanently invalidating the old one.

### Path Parameters

* `apiKeyId` (string, required) — ID of the API key whose secret should be rotated.

### Request Body

* `expireTime` (string) — If set, updates the expiration time of the new API key secret.
  If not set, the new secret inherits the old secret's expiration time.

* `oldSecretExpireTime` (string) — The time at which the old secret stops being accepted.
  Defaults to 24 hours from now if not set for non-expired keys. Must not be more than 7 days from now.

### Response Body

* `redactedApiKey` (string) — A redacted API key. We don't expose the full key after it has been created.

* `apiKey` (string) — Only set when the API key is created.

* `userId` (string) — ID of the User who created this API key.

* `name` (string) — Human-readable name for the API key.

* `createTime` (string) — Timestamp when the API key was created.

* `modifyTime` (string) — Timestamp when the API key was modified.

* `teamId` (string) — ID of the team this API key belongs to.

* `apiKeyId` (string) — The ID of the API key.

* `disabled` (boolean) — If API is disabled (by the user) or not. Users can disable API keys to prevent them from making
  API calls.

* `expireTime` (string) — Expiration time for the API key. If set and in the past, the key is rejected.

* `qps` (integer) — If set, this API key can only perform the stated number of requests per second.

* `qpm` (integer) — If set, this API key can only perform the stated number of requests per minute.

* `tpm` (string) — If set, the API key is limited to producing/consuming the set number of tokens per minute.
  The limiter engages when the limit is strictly exceeded. In-flight requests that cause the
  limit to get exceeded will not be aborted.

* `aclStrings` (array\<string>) — The permissions the API key has. By default, API keys don't have any permissions, which means
  all requests fail if this field is empty. There are two kind of permissions users can grant:
  (1) endpoints and (2) models via the \`api-key:endpoint:\[endpoint name]\` and
  \`api-key:model:\[model name]\` ACLs.

  If you want to grant access to specific endpoints and ACLs, you can use the corresponding
  \`/endpoints\` and \`/models\` endpoints to retrieve possible values for \`endpoint name\` and
  \`model name\`.

  If you want to create an API key that has access to all endpoints and models, you can use the
  wildcard ACLs \`api-key:endpoint:\*\` and \`api-key:model:\*\`.

\*\*Response example:\*\*

```json
{
  "redactedApiKey": "xai-...oYZ4",
  "apiKey": "xai-2xr2bnFV7lAbcdelZbqSxLd77pjlU6gcFmt3tPLSK3d7z7302htoH6mc4LlKlTY7nJtlU0lRXXQjoYZ4",
  "userId": "6ad994df-7963-45e0-96c8-96b1ba849d9b",
  "name": "My API key",
  "createTime": "2025-11-13T16:14:38.164829Z",
  "modifyTime": "2025-11-13T16:38:07.059039Z",
  "teamId": "c9a0c990-53e6-491e-8df7-b9f18e6983ac",
  "apiKeyId": "fe15b799-32e1-4d02-9e65-80236f9995f7",
  "disabled": false,
  "aclStrings": [
    "api-key:endpoint:*",
    "api-key:model:*"
  ]
}
```

***

## DELETE /auth/api-keys/\{apiKeyId}

!!CAUTION!! Permanently and irrevocably deletes an API key.

### Path Parameters

* `apiKeyId` (string, required) — ID of the API key to delete.

\*\*Response example:\*\*

```json
{}
```

***

## GET /auth/api-keys/\{apiKeyId}/propagation

Checks if an API key has successfully been propagated.

### Path Parameters

* `apiKeyId` (string, required) — ID of the API whose propagation status shall be checked.

### Response Body

* `icPropagation` (object) — Map from the Inference Cluster address to a flag indicating if the API key has propagated.

\*\*Response example:\*\*

```json
{
  "icPropagation": {
    "cloud9.api.x.ai": true,
    "us-east-1.api.x.ai": true
  }
}
```

***

## GET /auth/teams/\{teamId}/models

Lists all models that are accessible by a team.

### Path Parameters

* `teamId` (string, required) — ID of the team whose models shall be retrieved.

### Response Body

* `clusterConfigs` (array\<object>) — Contains the models accessible via the individual inference clusters.

  * `languageModels` (array\<object>) — Available language models.

    * `name` (string) — The name under which the model is available in the API.

    * `version` (string) — The version number of this model. Sometimes models get updated in-place (i.e. they keep their
      name but an updated version of the same model is deployed).

    * `inputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of input modalities the model supports.

    * `outputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of output modalities the model supports.

    * `promptTextTokenPrice` (string) — The price (in USD cents) per 100 million text prompt tokens.

    * `promptImageTokenPrice` (string) — The price (in USD cents) per 100 million image prompt tokens.

    * `promptTextTokenPriceLongContext` (string) — The price to charge per prompt-token if the request is a long context request. This apply to ALL the tokens of the
      request. The price is stored as 1/100,000,000th of a USD cent.

    * `cachedPromptTokenPrice` (string) — The price (in USD cents) per 100 million cached prompt tokens.

    * `cachedPromptTokenPriceLongContext` (string) — The price (in USD cents) per 100 million cached prompt tokens for long-context requests.

    * `completionTextTokenPrice` (string) — The price (in USD cents) per 100 million text completion token.

    * `completionTokenPriceLongContext` (string) — The price to charge per completions tokens if the request is a long context request. This apply to ALL the tokens
      of the request. The price is stored as 1/100,000,000th of a USD cent.

    * `searchPrice` (string) — The price (in USD cents) per 1000 search.

    * `rps` (string) — Max requests-per-second allowed for this team.

    * `rpm` (string) — Max requests-per-minute if configured.

    * `tpm` (string) — How many tokens-per-minute are allowed. If 0, limits are NOT applied.

    * `rph` (string) — Max requests-per-hour if configured.

    * `rpd` (string) — Max requests-per-day if configured.

    * `cluster` (string) — Name of the cluster on which the model is available.

    * `maxPromptLength` (integer) — Maximum length of the prompt/input (this includes tokens of all kinds).

    * `aliases` (array\<string>) — Other names under which the model is available.

    * `features` (object)

      * `functionCalling` (boolean)

      * `structuredOutputs` (boolean)

      * `reasoning` (boolean)

    * `algorithm` (string) — The name of the prompting algorithm to use.

    * `longContextThreshold` (string) — Threshold after which a request is considered as long context. If not set, never apply the long context prices.

    * `rateLimits` (object)

      * `queryOffset` (string)

      * `queryBaseRate` (string)

      * `queryMultiplier` (string)

      * `tokenOffset` (string)

      * `tokenBaseRate` (string)

      * `tokenMultiplier` (string)

    * `batchDiscountPercent` (integer)

    * `provisionedThroughput` (object) — Per-model Provisioned Throughput configuration.

      * `tokensPerInputUnit` (string) — Conversion rate: tokens per input unit per minute.

      * `tokensPerOutputUnit` (string) — Conversion rate: tokens per output unit per minute.

    * `tier` (integer) — Effective integer rate-limiting tier for this team+model.

  * `embeddingModels` (array\<object>) — Available embedding models.

    * `name` (string) — The name under which the model is available in the API.

    * `version` (string) — The version number of this model. Sometimes models get updated in-place (i.e. they keep their
      name but an updated version of the same model is deployed).

    * `inputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of input modalities the model supports.

    * `promptTextTokenPrice` (string) — The price (in USD cents) per 100 million text prompt tokens.

    * `promptImageTokenPrice` (string) — The price (in USD cents) per 100 million image prompt tokens.

    * `rps` (string) — Max requests-per-second allowed for this team.

    * `rpm` (string) — Max requests-per-minute if configured.

    * `tpm` (string) — How many tokens-per-minute are allowed. If 0, limits are NOT applied.

    * `rph` (string) — Max requests-per-hour if configured.

    * `rpd` (string) — Max requests-per-day if configured.

    * `cluster` (string) — Name of the cluster on which the model is available.

    * `aliases` (array\<string>) — Other names under which the model is available.

    * `rateLimits` (object)

      * `queryOffset` (string)

      * `queryBaseRate` (string)

      * `queryMultiplier` (string)

      * `tokenOffset` (string)

      * `tokenBaseRate` (string)

      * `tokenMultiplier` (string)

    * `tier` (integer) — Effective integer rate-limiting tier for this team+model.

  * `imageGenerationModels` (array\<object>) — Available image generation models.

    * `name` (string) — The name under which the model is available in the API.

    * `version` (string) — The version number of this model. Sometimes models get updated in-place (i.e. they keep their
      name but an updated version of the same model is deployed).

    * `inputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of input modalities the model supports.

    * `outputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of output modalities the model supports.

    * `imagePrice` (string) — \[DEPRECATED] The flat price per image. Use resolution\_pricing instead.

    * `rps` (string) — Max requests-per-second allowed for this team.

    * `rpm` (string) — Max requests-per-minute if configured.

    * `tpm` (string) — How many tokens-per-minute are allowed. If 0, limits are NOT applied.

    * `rph` (string) — Max requests-per-hour if configured.

    * `rpd` (string) — Max requests-per-day if configured.

    * `cluster` (string) — Name of the cluster on which the model is available.

    * `aliases` (array\<string>) — Other names under which the model is available.

    * `resolutionPricing` (array\<object>) — Resolution-based pricing tiers.

      * `resolution` ("IMAGE\_RESOLUTION\_UNSPECIFIED" | "IMAGE\_RESOLUTION\_1K" | "IMAGE\_RESOLUTION\_2K" | "IMAGE\_RESOLUTION\_4K") — Image resolution options for pricing.

        &#x20;\- IMAGE\_RESOLUTION\_1K: ~1024x1024
        &#x20;\- IMAGE\_RESOLUTION\_2K: ~2048x2048
        &#x20;\- IMAGE\_RESOLUTION\_4K: ~4096x4096

      * `pricePerImage` (string) — Price per image at this resolution (in 1/100,000,000th of a USD cent).

    * `pricePerInputImage` (string) — Price per input image (in 1/100,000,000th of a USD cent).

    * `rateLimits` (object)

      * `queryOffset` (string)

      * `queryBaseRate` (string)

      * `queryMultiplier` (string)

      * `tokenOffset` (string)

      * `tokenBaseRate` (string)

      * `tokenMultiplier` (string)

  * `audioModels` (array\<object>) — Available audio models.

    * `name` (string) — The name under which the model is available in the API.

    * `version` (string) — The version number of this model. Sometimes models get updated in-place (i.e. they keep their
      name but an updated version of the same model is deployed).

    * `inputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of input modalities the model supports.

    * `outputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of output modalities the model supports.

    * `promptTokenPrice` (string) — The price (in USD cents) per 100 million prompt tokens.

    * `completionTokenPrice` (string) — The price (in USD cents) per 100 million completion tokens.

    * `supportedClients` (array\<object>) — Supported clients. For example, this is something you can pass as the
      query parameter to the model websocket endpoint as "?client=...".

      * `clientName` (string) — Short ID-like string describing this client in the API.

      * `description` (string) — Human readable description for this type of client.

    * `rps` (string) — Max requests-per-second allowed for this team.

    * `rpm` (string) — Max requests-per-minute if configured.

    * `tpm` (string) — How many tokens-per-minute are allowed. If 0, limits are NOT applied.

    * `rph` (string) — Max requests-per-hour if configured.

    * `rpd` (string) — Max requests-per-day if configured.

    * `cluster` (string) — Name of the cluster on which the model is available.

    * `aliases` (array\<string>) — Other names under which the model is available.

    * `rateLimits` (object)

      * `queryOffset` (string)

      * `queryBaseRate` (string)

      * `queryMultiplier` (string)

      * `tokenOffset` (string)

      * `tokenBaseRate` (string)

      * `tokenMultiplier` (string)

  * `videoGenerationModels` (array\<object>) — Available video generation models.

    * `name` (string) — The name under which the model is available in the API.

    * `version` (string) — The version number of this model. Sometimes models get updated in-place (i.e. they keep their
      name but an updated version of the same model is deployed).

    * `inputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of input modalities the model supports (e.g. TEXT, IMAGE for image-to-video).

    * `outputModalities` (array\<"INVALID" | "TEXT" | "IMAGE" | "AUDIO" | "VIDEO">) — The kind of output modalities the model supports.

    * `rps` (string) — Max requests-per-second allowed for this team.

    * `rpm` (string) — Max requests-per-minute if configured.

    * `tpm` (string) — How many tokens-per-minute are allowed. If 0, limits are NOT applied.

    * `rph` (string) — Max requests-per-hour if configured.

    * `rpd` (string) — Max requests-per-day if configured.

    * `cluster` (string) — Name of the cluster on which the model is available.

    * `aliases` (array\<string>) — Other names under which the model is available.

    * `resolutionPricing` (array\<object>) — Resolution-based pricing tiers.

      * `resolution` ("VIDEO\_RESOLUTION\_UNSPECIFIED" | "VIDEO\_RESOLUTION\_480P" | "VIDEO\_RESOLUTION\_720P" | "VIDEO\_RESOLUTION\_1080P") — Video resolution options for pricing.

      * `pricePerSecond` (string) — Price per second of video at this resolution (in 1/100,000,000th of a USD cent).

    * `pricePerInputImage` (string) — Price per input image (in 1/100,000,000th of a USD cent).

    * `pricePerInputVideoSecond` (string) — Price per input video second (in 1/100,000,000th of a USD cent).

    * `rateLimits` (object)

      * `queryOffset` (string)

      * `queryBaseRate` (string)

      * `queryMultiplier` (string)

      * `tokenOffset` (string)

      * `tokenBaseRate` (string)

      * `tokenMultiplier` (string)

  * `clusterName` (string) — Name of the inference cluster, e.g. 'us-east-1'.

\*\*Response example:\*\*

```json
{
  "clusterConfigs": [
    {
      "languageModels": [
        {
          "name": "grok-2-vision-1212",
          "version": "1.0",
          "inputModalities": [
            "TEXT",
            "IMAGE"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "20000",
          "promptImageTokenPrice": "20000",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "0",
          "completionTextTokenPrice": "100000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "0",
          "rps": "10",
          "rpm": "600",
          "tpm": "0",
          "cluster": "us-east-1",
          "maxPromptLength": 32768,
          "aliases": [
            "grok-2-vision",
            "grok-2-vision-latest"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": false
          },
          "algorithm": "MultimodalV1"
        },
        {
          "name": "grok-code-fast-1",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "2000",
          "promptImageTokenPrice": "0",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "200",
          "completionTextTokenPrice": "15000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "0",
          "rps": "8",
          "rpm": "480",
          "tpm": "2000000",
          "cluster": "us-east-1",
          "maxPromptLength": 256000,
          "aliases": [
            "grok-code-fast",
            "grok-code-fast-1-0825"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": true
          },
          "algorithm": "grok4Code"
        },
        {
          "name": "grok-2-1212",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "20000",
          "promptImageTokenPrice": "0",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "0",
          "completionTextTokenPrice": "100000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "0",
          "rps": "15",
          "rpm": "900",
          "tpm": "0",
          "cluster": "us-east-1",
          "maxPromptLength": 131072,
          "aliases": [
            "grok-2",
            "grok-2-latest"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": false
          },
          "algorithm": "MultimodalV1"
        },
        {
          "name": "grok-3-mini",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "3000",
          "promptImageTokenPrice": "0",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "750",
          "completionTextTokenPrice": "5000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "2500",
          "rps": "8",
          "rpm": "480",
          "tpm": "0",
          "cluster": "us-east-1",
          "maxPromptLength": 131072,
          "aliases": [
            "grok-3-mini-latest",
            "grok-3-mini-beta",
            "grok-3-mini-fast",
            "grok-3-mini-fast-latest",
            "grok-3-mini-fast-beta"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": true
          },
          "algorithm": "grok3Reasoning"
        },
        {
          "name": "grok-4-0709",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "30000",
          "promptImageTokenPrice": "30000",
          "promptTextTokenPriceLongContext": "60000",
          "cachedPromptTokenPrice": "7500",
          "completionTextTokenPrice": "150000",
          "completionTokenPriceLongContext": "300000",
          "searchPrice": "2500",
          "rps": "8",
          "rpm": "480",
          "tpm": "2000000",
          "cluster": "us-east-1",
          "maxPromptLength": 256000,
          "aliases": [
            "grok-4",
            "grok-4-latest"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": true
          },
          "algorithm": "grok4",
          "longContextThreshold": "128000"
        },
        {
          "name": "grok-3",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "30000",
          "promptImageTokenPrice": "0",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "7500",
          "completionTextTokenPrice": "150000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "2500",
          "rps": "10",
          "rpm": "600",
          "tpm": "0",
          "cluster": "us-east-1",
          "maxPromptLength": 131072,
          "aliases": [
            "grok-3-latest",
            "grok-3-beta",
            "grok-3-fast",
            "grok-3-fast-latest",
            "grok-3-fast-beta"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": false
          },
          "algorithm": "grok3"
        }
      ],
      "embeddingModels": [],
      "imageGenerationModels": [
        {
          "name": "grok-2-image-1212",
          "version": "1.0",
          "inputModalities": [
            "TEXT",
            "IMAGE"
          ],
          "outputModalities": [
            "IMAGE"
          ],
          "imagePrice": "700000000",
          "rps": "5",
          "rpm": "300",
          "tpm": "0",
          "cluster": "us-east-1",
          "aliases": [
            "grok-2-image",
            "grok-2-image-latest"
          ]
        }
      ],
      "audioModels": [],
      "clusterName": "us-east-1"
    },
    {
      "languageModels": [
        {
          "name": "grok-2-1212",
          "version": "1.0",
          "inputModalities": [
            "TEXT"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "20000",
          "promptImageTokenPrice": "0",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "0",
          "completionTextTokenPrice": "100000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "0",
          "rps": "50",
          "tpm": "0",
          "cluster": "eu-west-1",
          "maxPromptLength": 131072,
          "aliases": [
            "grok-2",
            "grok-2-latest"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": false
          },
          "algorithm": "MultimodalV1"
        },
        {
          "name": "grok-2-vision-1212",
          "version": "1.0",
          "inputModalities": [
            "TEXT",
            "IMAGE"
          ],
          "outputModalities": [
            "TEXT"
          ],
          "promptTextTokenPrice": "20000",
          "promptImageTokenPrice": "20000",
          "promptTextTokenPriceLongContext": "0",
          "cachedPromptTokenPrice": "0",
          "completionTextTokenPrice": "100000",
          "completionTokenPriceLongContext": "0",
          "searchPrice": "0",
          "rps": "50",
          "tpm": "0",
          "cluster": "eu-west-1",
          "maxPromptLength": 32768,
          "aliases": [
            "grok-2-vision",
            "grok-2-vision-latest"
          ],
          "features": {
            "functionCalling": true,
            "structuredOutputs": true,
            "reasoning": false
          },
          "algorithm": "MultimodalV1"
        }
      ],
      "embeddingModels": [],
      "imageGenerationModels": [],
      "audioModels": [],
      "clusterName": "eu-west-1"
    }
  ]
}
```

***

## GET /auth/teams/\{teamId}/endpoints

Lists all the endpoint ACLs that can be used on API keys.

### Path Parameters

* `teamId` (string, required) — ID of the team whose accessible endpoints shall be received.

### Response Body

* `acls` (array\<object>) — List of ACLs that can be assigned to an API key.

  * `acl` (string) — The acl key to be used on an Api key.

  * `description` (string) — A description indicating what this ACL does.

  * `namespace` (string) — The ACL namespace.

  * `key` (string) — The ACL key.

  * `value` (string) — The ACL value (optional).

\*\*Response example:\*\*

```json
{
  "acls": [
    {
      "acl": "api-key:endpoint:chat",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "chat"
    },
    {
      "acl": "api-key:endpoint:embed",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "embed"
    },
    {
      "acl": "api-key:endpoint:image",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "image"
    },
    {
      "acl": "api-key:endpoint:models",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "models"
    },
    {
      "acl": "api-key:endpoint:sample",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "sample"
    },
    {
      "acl": "api-key:endpoint:tokenize",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "tokenize"
    },
    {
      "acl": "api-key:endpoint:documents",
      "description": "Grants access to use a specific endpoint with an API Key.",
      "namespace": "api-key",
      "key": "endpoint",
      "value": "documents"
    }
  ]
}
```

***

## GET /auth/management-keys/validation

API endpoint for GET requests to /auth/management-keys/validation.

### Response Body

* `apiKeyId` (string) — ID of this key.

* `teamId` (string) — \[DEPRECATED] Use scope and scope\_id instead.
  ID of the team on whose behalf the key can act.

* `scope` ("SCOPE\_UNSPECIFIED" | "SCOPE\_TEAM" | "SCOPE\_ORGANIZATION") — Scope the key is associated with.

* `scopeId` (string) — ID of the scope the key is associated with.

* `ownerUserId` (string) — ID of the user who owns this key.

* `createTime` (string) — Time when the key was created.

* `modifyTime` (string) — Time when the key was last modified.

* `name` (string) — Name of this key (Just for humans).

* `acls` (array\<string>) — ACLs of this key (controls what APIs can be accessed using the key).

* `apiKey` (string) — Only set when the key is created.

* `redactedApiKey` (string) — A shorted version of the actual key.

* `ipRanges` (object)

  * `ipRanges` (array\<object>) — A list of IP ranges.

    * `address` (object)

      * `ipv4` (string)

      * `ipv6` (string)

    * `prefixLength` (integer)

\*\*Response example:\*\*

```json
{
  "apiKeyId": "b86ba29d-9f47-4b3a-a6ae-e69432d5f0dc",
  "teamId": "65c1e471-205f-4566-9c5a-07198badf4ce",
  "scope": "SCOPE_TEAM",
  "scopeId": "65c1e471-205f-4566-9c5a-07198badf4ce",
  "ownerUserId": "4d52c406-6ec6-4361-9b7c-40dc7e8ff284",
  "createTime": "2025-12-10T23:17:49.460374Z",
  "modifyTime": "2025-12-10T23:17:49.460374Z",
  "name": "test key",
  "acls": [
    "team-token:endpoint:ListApiKeys",
    "team-token:endpoint:CheckApiKeyPropagation",
    "team-token:endpoint:ListPossibleEndpointAcls"
  ],
  "reactedApiKey": "xai-...r42q",
  "ipRanges": null
}
```