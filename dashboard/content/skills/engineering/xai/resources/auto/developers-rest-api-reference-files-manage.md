+++
title = "developers-rest-api-reference-files-manage"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Files API

# Manage

***

## GET /v1/files

List files owned by the authenticated team, paginated. The response
always returns a \`pagination\_token\`; pass it back as a query parameter
to fetch the next page. The end of the list is reached when the
returned \`data\` array is shorter than \`limit\`.

### Query Parameters

* `limit` (integer) — The maximum number of objects to be returned in a single response.

* `order` (string) — The ordering to sort the returned files. Use \`asc\` for ascending and \`desc\` for descending order.

* `sort_by` (string) — The field to sort by. Valid options: \`created\_at\`, \`filename\`, \`size\`. Defaults to \`created\_at\`.

* `pagination_token` (string) — The pagination token returned by the previous list files request.

* `after` (string) — Only included for compatibility. Use \`pagination\_token\` instead.

* `filter` (string) — AIP-160 filter expression to narrow down results.

  \*\*Filterable fields:\*\*

  | Field | Type | Description |
  |-------|------|-------------|
  | \`name\` (or \`file\_name\`) | string | Fuzzy match on filename |
  | \`file\_id\` | string | Exact match on file ID |
  | \`size\_bytes\` | integer | File size in bytes |
  | \`content\_type\` | string | Partial match on MIME type (e.g. \`"pdf"\` matches \`"application/pdf"\`) |
  | \`created\_at\` | timestamp | RFC 3339 timestamp (e.g. \`"2024-01-01T00:00:00Z"\`) |
  | \`expires\_at\` | timestamp | RFC 3339 timestamp |
  | \`upload\_status\` | string | Upload status (\`"Complete"\`) |
  | \`user\_defined\_id\` | string | Exact match on user-defined ID |

  \*\*Operators:\*\* \`=\`, \`!=\`, \`>\`, \`>=\`, \`\<\`, \`\<=\`

  \*\*Logical:\*\* \`AND\`, \`OR\`, \`NOT\`

  \*\*Examples:\*\*
  \- \`name:"quarterly report"\` — fuzzy match on filename
  \- \`content\_type = "pdf"\` — files with PDF content type
  \- \`size\_bytes > 1000000 AND created\_at > "2024-01-01T00:00:00Z"\` — files larger than 1 MB created after Jan 1, 2024
  \- \`file\_id = "file\_abc123"\` — exact file ID match

### Response Body

* `data` (array\<object>, required) — List of files.

  * `bytes` (integer, required) — The size of the file, in bytes.

  * `created_at` (integer, required) — The Unix timestamp (in seconds) for file creation time.

  * `expires_at` (integer | null) — The Unix timestamp (in seconds) for file expiry time. null if file does not expire.

  * `filename` (string, required) — The name of the file.

  * `id` (string, required) — The file identifier, which can be used in other API requests.

  * `object` (string, required) — The object type, which is always \`file\`. Only included for compatability.

  * `public_url` (string | null) — Public URL for the file. Only present when the file has an active public URL.

  * `public_url_expires_at` (integer | null) — Unix timestamp (seconds) when the public URL expires. Only present when
    the public URL has an independent expiry.

  * `purpose` (string) — The intended purpose of the uploaded file. Only included for OAI compatability.

* `pagination_token` (string | null) — Pagination token to use with next request.

\*\*Response example:\*\*

```json
{
  "data": [
    {
      "id": "file_a128090d-f0c9-4873-bd84-e499777e7417",
      "object": "file",
      "bytes": 12345,
      "created_at": 1762345678,
      "expires_at": null,
      "filename": "document.pdf",
      "purpose": ""
    }
  ],
  "pagination_token": "file_a128090d-f0c9-4873-bd84-e499777e7417"
}
```

***

## GET /v1/files/\{file\_id}

Retrieve metadata for a single file by ID. Errors with 404 if the file
doesn't exist, has been deleted, or has passed its \`expires\_at\`.

### Path Parameters

* `file_id` (string, required) — The file's \`id\` returned by upload or list.

### Response Body

* `bytes` (integer, required) — The size of the file, in bytes.

* `created_at` (integer, required) — The Unix timestamp (in seconds) for file creation time.

* `expires_at` (integer | null) — The Unix timestamp (in seconds) for file expiry time. null if file does not expire.

* `filename` (string, required) — The name of the file.

* `id` (string, required) — The file identifier, which can be used in other API requests.

* `object` (string, required) — The object type, which is always \`file\`. Only included for compatability.

* `public_url` (string | null) — Public URL for the file. Only present when the file has an active public URL.

* `public_url_expires_at` (integer | null) — Unix timestamp (seconds) when the public URL expires. Only present when
  the public URL has an independent expiry.

* `purpose` (string) — The intended purpose of the uploaded file. Only included for OAI compatability.

\*\*Response example:\*\*

```json
{}
```

***

## PUT /v1/files/\{file\_id}

API endpoint for PUT requests to /v1/files/\{file\_id}.

```
Method: PUT
Path: /v1/files/{file_id}
```

***

## DELETE /v1/files/\{file\_id}

Delete a file by ID. After this returns, the file no longer appears in
\`GET /v1/files\`, content download returns 404, and the ID can no longer
be referenced in chat attachments.

### Path Parameters

* `file_id` (string, required) — The file's \`id\` to delete.

### Response Body

* `deleted` (boolean, required) — Whether the file was deleted.

* `id` (string, required) — The ID of the file.

* `object` (string, required) — The object type, which is always "file". Only included for compatibility.

\*\*Response example:\*\*

```json
{
  "id": "file_a128090d-f0c9-4873-bd84-e499777e7417",
  "deleted": true
}
```
