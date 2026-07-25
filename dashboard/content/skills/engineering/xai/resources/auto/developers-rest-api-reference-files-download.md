+++
title = "developers-rest-api-reference-files-download"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Files API

# Download

***

## GET /v1/files/\{file\_id}/content

Download the contents of a file as a stream of raw bytes. The response
\`Content-Type\` is \`application/octet-stream\`. Use this for the binary
payload; use \`GET /v1/files/\{file\_id}\` for metadata only.

### Path Parameters

* `file_id` (string, required) — The file's \`id\` to download.

### Query Parameters

* `format` ("original" | "text") — Format of the downloaded content.
