#### Files API

# Upload

***

## POST /v1/files

Upload a file to xAI's storage. Returns the file's metadata. Files can
be referenced by ID anywhere a \`file\_id\` is accepted (e.g. chat
attachments). Maximum file size: 50 MB. Files are kept until you
delete them, or until \`expires\_after\` elapses if set at upload time.

### Request Body

* `expires_after` (integer | null) — Optional TTL in seconds (measured from upload time). Must be between
  3600 (1 hour) and 2592000 (30 days). If unset the file does not expire.

  Accepts either a plain integer or the OpenAI SDK deepObject form
  (\`expires\_after\[anchor]=created\_at\` + \`expires\_after\[seconds]=N\`)
  as separate multipart fields. The anchor+seconds form must arrive
  before the \`file\` part.

* `file` (string, required) — The file to upload. The filename from the multipart
  \`Content-Disposition: filename=\` header is recorded as the file's
  \`filename\`.

* `purpose` (string | null) — Optional purpose label, accepted for OpenAI SDK compatibility. xAI
  does not enforce or interpret this field. Setting \`"assistants"\`
  is the conventional choice.

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
{
  "id": "file_a128090d-f0c9-4873-bd84-e499777e7417",
  "object": "file",
  "bytes": 12345,
  "created_at": 1762345678,
  "expires_at": 1762432078,
  "filename": "document.pdf",
  "purpose": ""
}
```

***

## POST /v1/files:initialize

API endpoint for POST requests to /v1/files:initialize.

```
Method: POST
Path: /v1/files:initialize
```

***

## POST /v1/files:uploadChunks

API endpoint for POST requests to /v1/files:uploadChunks.

```
Method: POST
Path: /v1/files:uploadChunks
```