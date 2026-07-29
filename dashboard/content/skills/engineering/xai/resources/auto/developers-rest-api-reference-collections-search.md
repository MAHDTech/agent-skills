+++
title = "developers-rest-api-reference-collections-search"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Collections API

# Search in Collections

The base url for searching `collections` is shared with [REST API](https://docs.x.ai/developers/rest-api-reference) at `https://api.x.ai`. You have to authenticate using **xAI API Key** with the header `Authorization: Bearer <your xAI API key>`.

***

## POST /v1/documents/search

Search for content related to the query within the given collections.

### Request Body

* `filter` (string | null) — Optional metadata filter string to apply to search results.
  Uses AIP-160 filter syntax for querying document metadata.
  Supports comparison operators: \`=\`, \`!=\`, \`>\`, \`>=\`, \`\<\`, \`\<=\`
  Supports logical operators: \`AND\`, \`OR\`
  Supports range syntax: \`field:10..20\` (inclusive)
  Examples: \`author = "John"\` or \`year > 2020 AND category = "finance"\`

* `group_by` (object)

  * `aggregate` (object | object)

  * `keys` (array\\<string\>, required) — Metadata key(s) to group results by (e.g. "category", "department").
    At least one key is required.

* `instructions` (string | null) — User-defined instructions to be included in the search query. Defaults to generic search instructions.

* `limit` (integer | null) — The number of chunks to return.
  Will always return the top matching chunks.
  Optional, defaults to 10.

* `query` (string, required) — The query to search for which will be embedded using the
  same embedding model as the one used for the source to query.

* `ranking_metric` ("RANKING\_METRIC\_UNKNOWN" | "RANKING\_METRIC\_L2\_DISTANCE" | "RANKING\_METRIC\_COSINE\_SIMILARITY")

* `retrieval_mode` (object | object | object)

* `source` (object, required) — DocumentsSource defines the source of documents to search over.

  * `collection_ids` (array\\<string\>, required) — The collection IDs to search in.

  * `rag_pipeline` ("chroma\_db" | "es")

### Response Body

* `matches` (array\\<object\>, required) — The search matches.

  * `chunk_content` (string, required) — The chunk content.

  * `chunk_id` (string, required) — The chunk ID.

  * `collection_ids` (array\\<string\>, required) — The collection ID(s).

  * `fields` (object, required) — Metadata fields belonging to the document of this chunk.

  * `file_id` (string, required) — The document ID.

  * `page_number` (integer) — The dominant page number this chunk belongs to (0 for single-page docs).

  * `score` (number, required) — The relevance score.

\*\*Request example:\*\*

```json
{
  "query": "What is the revenue in the last quarter?",
  "source": {
    "collection_ids": [
      "collection_80100614-300c-4609-959b-a138fa90f542"
    ]
  },
  "filter": "document_type = \"financial_report\" AND year > 2020"
}
```

\*\*Response example:\*\*

```json
{
  "matches": [
    {
      "file_id": "file_ac3c5728-7399-41fc-bd62-0fef0042de9c",
      "chunk_id": "0199717c-511b-7a80-bab3-dfe9a27f82ab",
      "chunk_content": ", deferred revenue related to such customer payments amounted to $2.10 billion and $1.77 billion, respectively, mainly due to contractual payment terms. Revenue recognized from the deferred revenue balances as of December 31, 2024 and 2023 was $944 million and $873 million for the six months ended June 30, 2025 and 2024, respectively. We have elected the practical expedient to omit disclosure of the amount of the transaction price allocated to remaining performance obligations for contracts with an original expected contract length of one year or less. As of June 30, 2025, total transaction price allocated to performance obligations that were unsatisfied or partially unsatisfied for contracts with an original expected length of more than one year was $10.38 billion. Of this amount, we expect to recognize $5.47 billion in the next 12 months and the rest over the remaining performance obligation period. Changes in government and economic incentives or tariffs may impact the transaction price or our ability to e",
      "score": 1.1447691,
      "collection_ids": [
        "collection_80100614-300c-4609-959b-a138fa90f542"
      ]
    }
  ]
}
```
{% endraw %}
