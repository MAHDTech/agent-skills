#### Collections API

# Collection Management

The base URL for `collection` management is shared with [Management API](https://docs.x.ai/developers/rest-api-reference/management) at `https://management-api.x.ai/`.
You have to authenticate using **xAI Management API Key** with the header `Authorization: Bearer <your xAI Management API key>`.

> [!NOTE]
>
> For more details on provisioning xAI Management API key and using Management API, you can visit
>
> [Using Management API](https://docs.x.ai/developers/management-api-guide)

***

## POST /v1/collections

Create a collection.

### Request Body

* `team_id` (string) — The ID of the team that will own this new collection.
  If not provided, the team ID will be derived from your request credentials.

* `collection_name` (string, required) — Name to use for the new collection.

* `index_configuration` (object)

  * `model_name` (string) — Embedding model that would make the conversion.

* `chunk_configuration` (object)

  * `chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `table_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `code_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `code_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `bytes_configuration` (object)

    * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

    * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

  * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

  * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

* `metric_space` ("HNSW\_METRIC\_UNKNOWN" | "HNSW\_METRIC\_COSINE" | "HNSW\_METRIC\_EUCLIDEAN" | "HNSW\_METRIC\_INNER\_PRODUCT") — Distance space for the HNSW index.

* `version` (integer) — Internal only.
  Version number of the Collection API used under the hood.
  This is an internal only setting so it is okay to be left ambiguous (no enum).

* `field_definitions` (array\<object>)

  * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

  * `required` (boolean) — If true, this field must be provided for every document added to the collection.
    Documents missing required fields will be rejected at upload time.

  * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
    generated from documents (for contextual retrieval).
    Improves retrieval accuracy by providing context about the document.

  * `unique` (boolean) — If true, this field's value must be unique across all documents
    within this collection. Duplicate values will be rejected.

  * `description` (string) — Optional description of what this field represents.

* `collection_description` (string) — Human-friendly description displayed to users and agents.

### Response Body

* `collection_id` (string) — UUIDv4 that represents an ID of the collection.

* `collection_name` (string) — Name of the collection.

* `created_at` (string) — The Unix timestamp for when the document was created.

* `index_configuration` (object)

  * `model_name` (string) — Embedding model that would make the conversion.

* `chunk_configuration` (object)

  * `chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `table_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `code_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `code_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `bytes_configuration` (object)

    * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

    * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

  * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

  * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

* `documents_count` (integer) — How many files the collection contains.

* `field_definitions` (array\<object>) — Field definitions for documents in this collection.
  Defines what fields documents can have and their constraints.

  * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

  * `required` (boolean) — If true, this field must be provided for every document added to the collection.
    Documents missing required fields will be rejected at upload time.

  * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
    generated from documents (for contextual retrieval).
    Improves retrieval accuracy by providing context about the document.

  * `unique` (boolean) — If true, this field's value must be unique across all documents
    within this collection. Duplicate values will be rejected.

  * `description` (string) — Optional description of what this field represents.

* `collection_description` (string) — Optional description of the collection.

\*\*Request example:\*\*

```json
{
  "collection_name": "SEC Filings",
  "index_configuration": {
    "model_name": "grok-embedding-small"
  },
  "chunk_configuration": {
    "tokens_configuration": {
      "max_chunk_size_tokens": 1024,
      "chunk_overlap_tokens": 200,
      "encoding_name": "o200k_base"
    },
    "strip_whitespace": true
  },
  "collection_description": "Filings from the SEC for financial analysis"
}
```

\*\*Response example:\*\*

```json
{
  "collection_id": "collection_80100614-300c-4609-959b-a138fa90f542",
  "collection_name": "SEC Filings",
  "created_at": "2025-09-16T18:36:09.790629Z",
  "index_configuration": {
    "model_name": "grok-embedding-small"
  },
  "chunk_configuration": {
    "tokens_configuration": {
      "max_chunk_size_tokens": 1024,
      "chunk_overlap_tokens": 200,
      "encoding_name": "o200k_base"
    },
    "strip_whitespace": true,
    "inject_name_into_chunks": false
  },
  "documents_count": 0,
  "collection_description": "Filings from the SEC for financial analysis"
}
```

***

## GET /v1/collections

List all the collections a team has.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the collections being listed.
  If not provided, the team ID will be derived from your request credentials.

* `limit` (integer) — A limit on the number of objects to be returned. Max 100 items per request.
  If not provided, set the default to 100 items.

* `order` ("ORDERING\_UNKNOWN" | "ORDERING\_ASCENDING" | "ORDERING\_DESCENDING") — The ordering to sort the returned collections.
  If not provided, the default order is Descending.

* `sort_by` ("COLLECTIONS\_SORT\_BY\_NAME" | "COLLECTIONS\_SORT\_BY\_AGE") — The parameter that the collections will be sorted by.
  If not provided, the default is to sort by \`collection\_name\`.

* `pagination_token` (string) — Optional token to retrieve the next page. Provided by \`pagination\_token\` in a previous \`ListCollectionsResponse\`.

* `filter` (string) — Filter expression to narrow down results.
  Supports filtering on: collection\_id, collection\_name (partial string matching), created\_at, documents\_count
  Examples:
  &#x20; \- 'collection\_id = "collection\_123"'
  &#x20; \- 'collection\_name:"SEC" AND documents\_count:>10'
  &#x20; \- 'collection\_name = "report"' (partial match)
  &#x20; \- 'created\_at:>2025-01-01T00:00:00Z'

### Response Body

* `collections` (array\<object>) — List of collections.

  * `collection_id` (string) — UUIDv4 that represents an ID of the collection.

  * `collection_name` (string) — Name of the collection.

  * `created_at` (string) — The Unix timestamp for when the document was created.

  * `index_configuration` (object)

    * `model_name` (string) — Embedding model that would make the conversion.

  * `chunk_configuration` (object)

    * `chars_configuration` (object)

      * `max_chunk_size_chars` (integer) — Max length per chunk.

      * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

    * `tokens_configuration` (object)

      * `max_chunk_size_tokens` (integer) — Max length per chunk.

      * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

      * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

    * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

      * `max_chunk_size_tokens` (integer) — Max length per chunk.

      * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

    * `table_configuration` (object)

      * `max_chunk_size_tokens` (integer) — Max length per chunk.

      * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

    * `markdown_tokens_configuration` (object)

      * `max_chunk_size_tokens` (integer) — Max length per chunk.

      * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

      * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

    * `markdown_chars_configuration` (object)

      * `max_chunk_size_chars` (integer) — Max length per chunk.

      * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

    * `code_tokens_configuration` (object)

      * `max_chunk_size_tokens` (integer) — Max length per chunk.

      * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

      * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

    * `code_chars_configuration` (object)

      * `max_chunk_size_chars` (integer) — Max length per chunk.

      * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

    * `bytes_configuration` (object)

      * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

      * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

    * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

    * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

  * `documents_count` (integer) — How many files the collection contains.

  * `field_definitions` (array\<object>) — Field definitions for documents in this collection.
    Defines what fields documents can have and their constraints.

    * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

    * `required` (boolean) — If true, this field must be provided for every document added to the collection.
      Documents missing required fields will be rejected at upload time.

    * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
      generated from documents (for contextual retrieval).
      Improves retrieval accuracy by providing context about the document.

    * `unique` (boolean) — If true, this field's value must be unique across all documents
      within this collection. Duplicate values will be rejected.

    * `description` (string) — Optional description of what this field represents.

  * `collection_description` (string) — Optional description of the collection.

* `pagination_token` (string) — Token to be sent in the next \`ListCollectionsRequest\`'s \`pagination\_token\` for retrieving the next page.

\*\*Response example:\*\*

```json
{
  "collections": [
    {
      "collection_id": "collection_80100614-300c-4609-959b-a138fa90f542",
      "collection_name": "SEC Filings",
      "created_at": "2025-09-16T18:36:09.790629Z",
      "index_configuration": {
        "model_name": "grok-embedding-small"
      },
      "chunk_configuration": {
        "tokens_configuration": {
          "max_chunk_size_tokens": 1024,
          "chunk_overlap_tokens": 200,
          "encoding_name": "o200k_base"
        },
        "strip_whitespace": true,
        "inject_name_into_chunks": false
      },
      "documents_count": 0,
      "collection_type": "text",
      "collection_description": "Filings from the SEC for financial analysis"
    }
  ]
}
```

***

## GET /v1/collections/\{collection\_id}

Get a collection's metadata.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection to request.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the collection.
  If not provided, the team ID will be derived from your request credentials.

### Response Body

* `collection_id` (string) — UUIDv4 that represents an ID of the collection.

* `collection_name` (string) — Name of the collection.

* `created_at` (string) — The Unix timestamp for when the document was created.

* `index_configuration` (object)

  * `model_name` (string) — Embedding model that would make the conversion.

* `chunk_configuration` (object)

  * `chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `table_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `code_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `code_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `bytes_configuration` (object)

    * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

    * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

  * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

  * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

* `documents_count` (integer) — How many files the collection contains.

* `field_definitions` (array\<object>) — Field definitions for documents in this collection.
  Defines what fields documents can have and their constraints.

  * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

  * `required` (boolean) — If true, this field must be provided for every document added to the collection.
    Documents missing required fields will be rejected at upload time.

  * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
    generated from documents (for contextual retrieval).
    Improves retrieval accuracy by providing context about the document.

  * `unique` (boolean) — If true, this field's value must be unique across all documents
    within this collection. Duplicate values will be rejected.

  * `description` (string) — Optional description of what this field represents.

* `collection_description` (string) — Optional description of the collection.

\*\*Response example:\*\*

```json
{
  "collection_id": "collection_80100614-300c-4609-959b-a138fa90f542",
  "collection_name": "SEC Filings",
  "created_at": "2025-09-16T18:36:09.790629Z",
  "index_configuration": {
    "model_name": "grok-embedding-small"
  },
  "chunk_configuration": {
    "tokens_configuration": {
      "max_chunk_size_tokens": 1024,
      "chunk_overlap_tokens": 200,
      "encoding_name": "o200k_base"
    },
    "strip_whitespace": true,
    "inject_name_into_chunks": false
  },
  "documents_count": 0,
  "collection_description": "Filings from the SEC for financial analysis"
}
```

***

## DELETE /v1/collections/\{collection\_id}

Delete a specific collection.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection to delete.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the collection.
  If not provided, the team ID will be derived from your request credentials.

\*\*Response example:\*\*

```json
{}
```

***

## PUT /v1/collections/\{collection\_id}

Update collection's config.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection to update.

### Request Body

* `team_id` (string) — The ID of the team that owns the document.
  If not provided, the team ID will be derived from your request credentials.

* `collection_name` (string) — Name of the collection.

* `chunk_configuration` (object)

  * `chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `table_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `code_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `code_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `bytes_configuration` (object)

    * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

    * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

  * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

  * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

* `field_definition_updates` (array\<object>) — Field definition updates to apply to this collection (ADD or DELETE).

  * `field_definition` (object, required) — Definition of a field that can be attached to documents in a collection.
    Field definitions specify constraints and behaviors for document metadata within a collection.

    * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

    * `required` (boolean) — If true, this field must be provided for every document added to the collection.
      Documents missing required fields will be rejected at upload time.

    * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
      generated from documents (for contextual retrieval).
      Improves retrieval accuracy by providing context about the document.

    * `unique` (boolean) — If true, this field's value must be unique across all documents
      within this collection. Duplicate values will be rejected.

    * `description` (string) — Optional description of what this field represents.

  * `operation` ("FIELD\_DEFINITION\_ADD" | "FIELD\_DEFINITION\_DELETE") — Operation to perform on a collection's field definition.

    &#x20;\- FIELD\_DEFINITION\_ADD: Add a new field definition or update an existing one.
    If the field key already exists, the definition will be updated.
    Note: New fields with \`required=true\` are not allowed (existing documents would fail validation).
    &#x20;\- FIELD\_DEFINITION\_DELETE: Delete an existing field definition.
    CASCADE behavior: Also removes the field value from all documents in the collection.

* `collection_description` (string) — Optional description of the collection.

### Response Body

* `collection_id` (string) — UUIDv4 that represents an ID of the collection.

* `collection_name` (string) — Name of the collection.

* `created_at` (string) — The Unix timestamp for when the document was created.

* `index_configuration` (object)

  * `model_name` (string) — Embedding model that would make the conversion.

* `chunk_configuration` (object)

  * `chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `ast_configuration` (object) — Deprecated: Use CodeTokensConfiguration or CodeCharsConfiguration instead.

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `table_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `markdown_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `code_tokens_configuration` (object)

    * `max_chunk_size_tokens` (integer) — Max length per chunk.

    * `chunk_overlap_tokens` (integer) — Overlap between chunks, both sides.

    * `encoding_name` (string) — Name of the encoding to use for the tokenizer.

  * `code_chars_configuration` (object)

    * `max_chunk_size_chars` (integer) — Max length per chunk.

    * `chunk_overlap_chars` (integer) — Overlap between chunks, both sides.

  * `bytes_configuration` (object)

    * `max_chunk_size_bytes` (integer) — Max length per chunk in bytes.

    * `chunk_overlap_bytes` (integer) — Overlap between chunks in bytes.

  * `strip_whitespace` (boolean) — Remove leading/trailing whitespce.

  * `inject_name_into_chunks` (boolean) — Inject name into produced chunks.

* `documents_count` (integer) — How many files the collection contains.

* `field_definitions` (array\<object>) — Field definitions for documents in this collection.
  Defines what fields documents can have and their constraints.

  * `key` (string, required) — The key/name of the field (e.g., "title", "author", "isbn").

  * `required` (boolean) — If true, this field must be provided for every document added to the collection.
    Documents missing required fields will be rejected at upload time.

  * `inject_into_chunk` (boolean) — If true, this field's value will be injected at the start of each chunk
    generated from documents (for contextual retrieval).
    Improves retrieval accuracy by providing context about the document.

  * `unique` (boolean) — If true, this field's value must be unique across all documents
    within this collection. Duplicate values will be rejected.

  * `description` (string) — Optional description of what this field represents.

* `collection_description` (string) — Optional description of the collection.

\*\*Request example:\*\*

```json
{
  "collectionName": "SEC Filings (New)",
  "chunkConfiguration": {
    "tokensConfiguration": {
      "maxChunkSizeTokens": 1024,
      "chunkOverlapTokens": 200,
      "encodingName": "o200k_base"
    },
    "stripWhitespace": true,
    "injectNameIntoChunks": false
  },
  "collectionDescription": "Updated description of the collection"
}
```

\*\*Response example:\*\*

```json
{
  "collection_id": "collection_80100614-300c-4609-959b-a138fa90f542",
  "collection_name": "SEC Filings",
  "created_at": "2025-09-16T18:36:09.790629Z",
  "index_configuration": {
    "model_name": "grok-embedding-small"
  },
  "chunk_configuration": {
    "tokens_configuration": {
      "max_chunk_size_tokens": 1024,
      "chunk_overlap_tokens": 200,
      "encoding_name": "o200k_base"
    },
    "strip_whitespace": true,
    "inject_name_into_chunks": false
  },
  "documents_count": 0,
  "collection_description": "Filings from the SEC for financial analysis"
}
```

***

## POST /v1/collections/\{collection\_id}/documents/\{file\_id}

Add a document to collection.

### Path Parameters

* `collection_id` (string, required) — The id of the collection this document will be added to.

* `file_id` (string, required) — The ID of the document to use for this request.

### Request Body

* `team_id` (string) — The ID of the team the document belongs to.
  If not provided, the team ID will be derived from your request credentials.

* `fields` (object) — User-defined fields to add to this document in this new collection.

\*\*Request example:\*\*

```json
{
  "fields": {
    "type": "10-Q"
  }
}
```

\*\*Response example:\*\*

```json
{}
```

***

## GET /v1/collections/\{collection\_id}/documents

List documents in a collection.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection to list documents from.

### Query Parameters

* `team_id` (string) — The ID of the team owning the documents.
  If not provided, the team ID will be derived from your request credentials.

* `limit` (integer) — A limit on the number of objects to be returned. Max 100 items per request.
  If not provided, set the default to 100 items.

* `order` ("ORDERING\_UNKNOWN" | "ORDERING\_ASCENDING" | "ORDERING\_DESCENDING") — The ordering to sort the returned documents.
  If not provided, the default order is Descending.

* `sort_by` ("DOCUMENTS\_SORT\_BY\_NAME" | "DOCUMENTS\_SORT\_BY\_SIZE" | "DOCUMENTS\_SORT\_BY\_AGE") — The parameter that the documents will be sorted by.
  If not provided, the default is to sort by \`name\`.

* `pagination_token` (string) — Optional token to retrieve the next page. Provided by \`pagination\_token\` in a previous \`ListDocumentsResponse\`.

* `name` (string) — The name of the documents to get.
  DEPRECATED: Use filter field instead with "name:value"

* `filter` (string) — Filter expression to narrow down results.
  Supports filtering on file metadata (name, content\_type, size\_bytes, created\_at)
  and document fields (status, fields.\{key})
  Examples:
  &#x20; \- 'status:DOCUMENT\_STATUS\_PROCESSED'
  &#x20; \- 'name:"quarterly" AND status:!DOCUMENT\_STATUS\_FAILED'
  &#x20; \- 'fields.isbn:"978-1-234567-89-0"'
  &#x20; \- 'size\_bytes:>5000000 AND content\_type:application/pdf'

### Response Body

* `documents` (array\<object>) — List of documents.

  * `file_metadata` (object) — Metadata of an uploaded file.

    * `file_id` (string) — The document ID.

    * `name` (string) — The name of the document.

    * `size_bytes` (string) — The size of the document, in bytes.

    * `content_type` (string) — MIME type.

    * `created_at` (string) — The Unix timestamp for when the document was created.

    * `expires_at` (string) — The Unix timestamp for when the document will expire.

    * `hash` (string)

    * `upload_status` (string)

    * `upload_error_message` (string) — Error message if upload failed.

    * `processing_status` (string) — Processing status of the file (pending, processing, complete, failed, skipped).

    * `file_path` (string) — Optional: hierarchical path for the file (e.g., "folder1/subfolder").
      This is relative to the team root and does not include the filename.

  * `fields` (object)

  * `status` ("DOCUMENT\_STATUS\_UNKNOWN" | "DOCUMENT\_STATUS\_PROCESSING" | "DOCUMENT\_STATUS\_PROCESSED" | "DOCUMENT\_STATUS\_FAILED")

  * `error_message` (string) — Any error that occurred while processing.

  * `last_indexed_at` (string) — Timestamp of when this document was last indexed. Empty if it hasn't been.

* `pagination_token` (string) — Token to be sent in the next \`ListDocumentsRequest\`'s \`pagination\_token\` for retrieving the next page.

\*\*Response example:\*\*

```json
{
  "documents": [
    {
      "file_metadata": {
        "file_id": "file_94847856-a56f-4b1e-82dd-7fe0b3af43d9",
        "name": "tsla-20250630.txt",
        "size_bytes": "119237",
        "content_type": "text/plain",
        "created_at": "2025-09-16T19:06:53.472088Z",
        "expires_at": null,
        "hash": "a15b2225695f242af60e5d99a7455b0a2e371dac88283401ebc013dba1dfbc84"
      },
      "fields": {
        "type": "10-Q"
      },
      "status": "DOCUMENT_STATUS_PROCESSED",
      "error_message": ""
    }
  ]
}
```

***

## GET /v1/collections/\{collection\_id}/documents/\{file\_id}

Retrieve document metadata in a collection.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection this document belongs to.

* `file_id` (string, required) — The ID of the document to use for this request.

### Query Parameters

* `team_id` (string) — The ID of the team the document belongs to.
  If not provided, the team ID will be derived from your request credentials.

### Response Body

* `file_metadata` (object) — Metadata of an uploaded file.

  * `file_id` (string) — The document ID.

  * `name` (string) — The name of the document.

  * `size_bytes` (string) — The size of the document, in bytes.

  * `content_type` (string) — MIME type.

  * `created_at` (string) — The Unix timestamp for when the document was created.

  * `expires_at` (string) — The Unix timestamp for when the document will expire.

  * `hash` (string)

  * `upload_status` (string)

  * `upload_error_message` (string) — Error message if upload failed.

  * `processing_status` (string) — Processing status of the file (pending, processing, complete, failed, skipped).

  * `file_path` (string) — Optional: hierarchical path for the file (e.g., "folder1/subfolder").
    This is relative to the team root and does not include the filename.

* `fields` (object)

* `status` ("DOCUMENT\_STATUS\_UNKNOWN" | "DOCUMENT\_STATUS\_PROCESSING" | "DOCUMENT\_STATUS\_PROCESSED" | "DOCUMENT\_STATUS\_FAILED")

* `error_message` (string) — Any error that occurred while processing.

* `last_indexed_at` (string) — Timestamp of when this document was last indexed. Empty if it hasn't been.

\*\*Response example:\*\*

```json
{
  "file_metadata": {
    "file_id": "file_94847856-a56f-4b1e-82dd-7fe0b3af43d9",
    "name": "tsla-20250630.txt",
    "size_bytes": "119237",
    "content_type": "text/plain",
    "created_at": "2025-09-16T19:06:53.472088Z",
    "expires_at": null,
    "hash": "a15b2225695f242af60e5d99a7455b0a2e371dac88283401ebc013dba1dfbc84"
  },
  "fields": {
    "type": "10-Q"
  },
  "status": "DOCUMENT_STATUS_PROCESSED",
  "error_message": ""
}
```

***

## PATCH /v1/collections/\{collection\_id}/documents/\{file\_id}

Regenerate indices for the given document.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection that includes the document.

* `file_id` (string, required) — The ID of the file to update.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the document.
  If not provided, the team ID will be derived from your request credentials.

\*\*Response example:\*\*

```json
{}
```

***

## DELETE /v1/collections/\{collection\_id}/documents/\{file\_id}

Remove document from collection.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection the document will be remove from.

* `file_id` (string, required) — The file ID of the document to use for this request.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the collection.
  If not provided, the team ID will be derived from your request credentials.

\*\*Response example:\*\*

```json
{}
```

***

## GET /v1/collections/\{collection\_id}/documents:batchGet

Get documents metadata in a batch request.

### Path Parameters

* `collection_id` (string, required) — The ID of the collection that includes the documents.

### Query Parameters

* `team_id` (string) — The ID of the team that owns the document.
  If \`None\`, the team ID will be derived from your request credentials.

* `file_ids` (array\<string>, required) — The IDs of the files to retrieve the document metadata from.

### Response Body

* `documents` (array\<object>) — Documents' metadata requested.

  * `file_metadata` (object) — Metadata of an uploaded file.

    * `file_id` (string) — The document ID.

    * `name` (string) — The name of the document.

    * `size_bytes` (string) — The size of the document, in bytes.

    * `content_type` (string) — MIME type.

    * `created_at` (string) — The Unix timestamp for when the document was created.

    * `expires_at` (string) — The Unix timestamp for when the document will expire.

    * `hash` (string)

    * `upload_status` (string)

    * `upload_error_message` (string) — Error message if upload failed.

    * `processing_status` (string) — Processing status of the file (pending, processing, complete, failed, skipped).

    * `file_path` (string) — Optional: hierarchical path for the file (e.g., "folder1/subfolder").
      This is relative to the team root and does not include the filename.

  * `fields` (object)

  * `status` ("DOCUMENT\_STATUS\_UNKNOWN" | "DOCUMENT\_STATUS\_PROCESSING" | "DOCUMENT\_STATUS\_PROCESSED" | "DOCUMENT\_STATUS\_FAILED")

  * `error_message` (string) — Any error that occurred while processing.

  * `last_indexed_at` (string) — Timestamp of when this document was last indexed. Empty if it hasn't been.

\*\*Response example:\*\*

```json
{
  "documents": [
    {
      "file_metadata": {
        "file_id": "file_94847856-a56f-4b1e-82dd-7fe0b3af43d9",
        "name": "tsla-20250630.txt",
        "size_bytes": "119237",
        "content_type": "text/plain",
        "created_at": "2025-09-16T19:06:53.472088Z",
        "expires_at": null,
        "hash": "a15b2225695f242af60e5d99a7455b0a2e371dac88283401ebc013dba1dfbc84"
      },
      "fields": {},
      "status": "DOCUMENT_STATUS_PROCESSED",
      "error_message": ""
    }
  ]
}
```