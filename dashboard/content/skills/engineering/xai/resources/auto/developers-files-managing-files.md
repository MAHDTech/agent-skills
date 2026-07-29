+++
title = "developers-files-managing-files"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Files & Collections

# Managing Files

The Files API provides a complete set of operations for managing your files. If your files are publicly accessible, you can reference them directly by URL in chat conversations — see [Attaching Files](https://docs.x.ai/developers/model-capabilities/files/chat-with-files#attaching-files). For files that aren't publicly accessible, upload them using one of the methods described below.

You can also view and manage all of your uploaded files from the [Files page](https://console.x.ai/team/default/files) on the xAI Console.

## Uploading Files

You can upload files in several ways: from a file path, raw bytes, BytesIO object, or an open file handle.

### Upload from File Path

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Upload a file from disk
file = client.files.upload("/path/to/your/document.pdf")

print(f"File ID: {file.id}")
print(f"Filename: {file.filename}")
print(f"Size: {file.size} bytes")
print(f"Created at: {file.created_at}")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# Upload a file
with open("/path/to/your/document.pdf", "rb") as f:
    file = client.files.create(
        file=f,
        purpose="assistants"
    )

print(f"File ID: {file.id}")
print(f"Filename: {file.filename}")
```

```pythonRequests
import os
import requests

url = "https://api.x.ai/v1/files"
headers = {
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"
}

with open("/path/to/your/document.pdf", "rb") as f:
    files = {"file": f}
    data = {"purpose": "assistants"}
    response = requests.post(url, headers=headers, files=files, data=data)

file_data = response.json()
print(f"File ID: {file_data['id']}")
print(f"Filename: {file_data['filename']}")
```

```javascriptOpenAISDK
import OpenAI from "openai";
import fs from "fs";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Upload a file
const file = await client.files.create({
    file: fs.createReadStream("/path/to/your/document.pdf"),
    purpose: "assistants",
});

console.log("File ID: " + file.id);
console.log("Filename: " + file.filename);
```

```javascriptWithoutSDK
import fs from "fs";

const formData = new FormData();
formData.append("file", new Blob([fs.readFileSync("/path/to/your/document.pdf")]), "document.pdf");
formData.append("purpose", "assistants");

const response = await fetch("https://api.x.ai/v1/files", {
  method: "POST",
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
  body: formData,
});

const file = await response.json();
console.log("File ID: " + file.id);
console.log("Filename: " + file.filename);
```

```bash
curl https://api.x.ai/v1/files \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -F file=@/path/to/your/document.pdf \\
  -F purpose=assistants
```

### Upload from Bytes

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Upload file content directly from bytes
content = b"This is my document content.\\nIt can span multiple lines."
file = client.files.upload(content, filename="document.txt")

print(f"File ID: {file.id}")
print(f"Filename: {file.filename}")
```

### Upload from file object

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Upload a file directly from disk
file = client.files.upload(open("document.pdf", "rb"), filename="document.pdf")

print(f"File ID: {file.id}")
print(f"Filename: {file.filename}")
```

## Upload with Expiration (TTL)

Files default to permanent storage until you delete them. To have the platform automatically delete a file after a fixed window, set `expires_after` at upload time. This is useful for short-lived attachments, ephemeral session data, and compliance windows.

### How it works

`expires_after` is set in seconds, measured from upload time. It must be between `3600` (1 hour) and `2592000` (30 days), inclusive. Omit the field to keep the file permanently.

The response includes `expires_at`, the absolute UTC timestamp at which the file will be deleted. Once that time passes, the file is gone: it no longer appears in list responses, retrieving its metadata or content returns `not found`, and it can no longer be referenced by `id` in chat attachments.

You can also delete the file manually at any time before its TTL elapses.

> [!WARNING]
>
> **Multipart field ordering matters**: `expires_after` must appear **before** the `file` field in the multipart body. Requests that send `expires_after` after `file` are rejected with `400`.

```pythonXAI
import os
from datetime import timedelta
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Upload a file that will be auto-deleted in 24 hours.
# expires_after accepts an int (seconds) or a datetime.timedelta.
file = client.files.upload(
    "/path/to/document.pdf",
    expires_after=timedelta(hours=24),
)

print(f"File ID: {file.id}")
print(f"Expires at: {file.expires_at.ToDatetime()}")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# expires_after follows the OpenAI shape: an object anchored at "created_at".
with open("/path/to/document.pdf", "rb") as f:
    file = client.files.create(
        file=f,
        purpose="assistants",
        expires_after={"anchor": "created_at", "seconds": 86400},  # 24 hours
    )

print(f"File ID: {file.id}")
print(f"Expires at: {file.expires_at}")  # Unix seconds, 24h from now
```

```pythonRequests
import os
import requests

url = "https://api.x.ai/v1/files"
headers = {
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"
}

# expires_after MUST appear before file in the multipart body — see the note below.
with open("/path/to/document.pdf", "rb") as f:
    response = requests.post(
        url,
        headers=headers,
        data=[
            ("expires_after", "86400"),  # 24 hours in seconds
            ("purpose", "assistants"),
        ],
        files={"file": f},
    )

file_data = response.json()
print(f"File ID: {file_data['id']}")
print(f"Expires at: {file_data['expires_at']}")
```

```javascriptOpenAISDK
import OpenAI from "openai";
import fs from "fs";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

const file = await client.files.create({
    file: fs.createReadStream("/path/to/document.pdf"),
    purpose: "assistants",
    expires_after: { anchor: "created_at", seconds: 86400 },  // 24 hours
});

console.log("File ID: " + file.id);
console.log("Expires at: " + file.expires_at);
```

```javascriptWithoutSDK
import fs from "fs";

const formData = new FormData();
// expires_after MUST be appended before the file field.
formData.append("expires_after", "86400");  // 24 hours in seconds
formData.append("purpose", "assistants");
formData.append(
  "file",
  new Blob([fs.readFileSync("/path/to/document.pdf")]),
  "document.pdf",
);

const response = await fetch("https://api.x.ai/v1/files", {
  method: "POST",
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
  body: formData,
});

const file = await response.json();
console.log("File ID: " + file.id);
console.log("Expires at: " + file.expires_at);
```

```bash
# -F fields are sent in declaration order.
# expires_after must come before file in the form body.
curl https://api.x.ai/v1/files \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -F expires_after=86400 \\
  -F purpose=assistants \\
  -F file=@/path/to/document.pdf
```

## Upload with Progress Tracking

Track upload progress for large files using callbacks or progress bars.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Define a custom progress callback
def progress_callback(bytes_uploaded: int, total_bytes: int):
    percentage = (bytes_uploaded / total_bytes) * 100 if total_bytes else 0
    mb_uploaded = bytes_uploaded / (1024 * 1024)
    mb_total = total_bytes / (1024 * 1024)
    print(f"Progress: {mb_uploaded:.2f}/{mb_total:.2f} MB ({percentage:.1f}%)")

# Upload with progress tracking
file = client.files.upload(
    "/path/to/large-file.pdf",
    on_progress=progress_callback
)

print(f"Successfully uploaded: {file.filename}")
```

## Listing Files

Retrieve a list of your uploaded files with pagination and sorting options.

### Available Options

* **`limit`**: Maximum number of files to return. If not specified, uses the server default of 100. Maximum is 100.
* **`order`**: Sort order. Either `"asc"` (ascending) or `"desc"` (descending). Defaults to `"desc"`.
* **`sort_by`**: Field to sort by. Options: `"created_at"`, `"filename"`, or `"size"`. Defaults to `"created_at"`.
* **`pagination_token`**: Pass the `pagination_token` returned by the previous response to fetch the next page. Omit it for the first page.

The response always includes a `pagination_token`. When the returned page is shorter than `limit`, you've reached the end of the list.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# List files with pagination and sorting
response = client.files.list(
    limit=10,
    order="desc",
    sort_by="created_at"
)

for file in response.data:
    expires = file.expires_at.ToDatetime() if file.HasField("expires_at") else "never"
    print(f"File: {file.filename} (ID: {file.id}, Size: {file.size} bytes, Expires: {expires})")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# List files
files = client.files.list()

for file in files.data:
    print(f"File: {file.filename} (ID: {file.id})")
```

```pythonRequests
import os
import requests

url = "https://api.x.ai/v1/files"
headers = {
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"
}

response = requests.get(url, headers=headers)
files = response.json()

for file in files.get("data", []):
    print(f"File: {file['filename']} (ID: {file['id']})")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// List files
const files = await client.files.list();

for (const file of files.data) {
    console.log(\`File: \${file.filename} (ID: \${file.id})\`);
}
```

```javascriptWithoutSDK
const response = await fetch("https://api.x.ai/v1/files", {
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
});

const files = await response.json();

for (const file of files.data) {
  console.log(\`File: \${file.filename} (ID: \${file.id})\`);
}
```

```bash
curl https://api.x.ai/v1/files \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

### Paginating Through All Files

The List endpoint returns at most `limit` files per call (capped at 100). To enumerate every file, keep calling the endpoint with the `pagination_token` from the previous response until the response returns fewer than `limit` items.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Walk every page until the API returns a short page.
page_size = 100
token = None
all_files = []

while True:
    response = client.files.list(
        limit=page_size,
        order="desc",
        sort_by="created_at",
        pagination_token=token,
    )
    all_files.extend(response.data)
    if len(response.data) < page_size:
        break
    token = response.pagination_token

print(f"Total files: {len(all_files)}")
```

```pythonRequests
import os
import requests

url = "https://api.x.ai/v1/files"
headers = {"Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"}

page_size = 100
params = {"limit": page_size, "order": "desc", "sort_by": "created_at"}
all_files = []

while True:
    response = requests.get(url, headers=headers, params=params).json()
    all_files.extend(response.get("data", []))
    if len(response.get("data", [])) < page_size:
        break
    params["pagination_token"] = response["pagination_token"]

print(f"Total files: {len(all_files)}")
```

```javascriptWithoutSDK
const pageSize = 100;
const baseParams = { limit: String(pageSize), order: "desc", sort_by: "created_at" };
const allFiles = [];
let token;

while (true) {
  const params = new URLSearchParams({ ...baseParams, ...(token ? { pagination_token: token } : {}) });
  const response = await fetch(\`https://api.x.ai/v1/files?\${params}\`, {
    headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
  });
  const page = await response.json();
  allFiles.push(...page.data);
  if (page.data.length < pageSize) break;
  token = page.pagination_token;
}

console.log(\`Total files: \${allFiles.length}\`);
```

## Getting File Metadata

Retrieve detailed information about a specific file.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Get file metadata by ID
file = client.files.get("file-abc123")

print(f"Filename: {file.filename}")
print(f"Size: {file.size} bytes")
print(f"Created: {file.created_at}")
# expires_at is only set when the file was uploaded with expires_after
if file.HasField("expires_at"):
    print(f"Expires at: {file.expires_at.ToDatetime()}")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# Get file metadata
file = client.files.retrieve("file-abc123")

print(f"Filename: {file.filename}")
print(f"Size: {file.bytes} bytes")
# Unix seconds, or None if the file does not expire.
print(f"Expires at: {file.expires_at}")
```

```pythonRequests
import os
import requests

file_id = "file-abc123"
url = f"https://api.x.ai/v1/files/{file_id}"
headers = {
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"
}

response = requests.get(url, headers=headers)
file = response.json()

print(f"Filename: {file['filename']}")
print(f"Size: {file['bytes']} bytes")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Get file metadata
const file = await client.files.retrieve("file-abc123");

console.log("Filename: " + file.filename);
console.log("Size: " + file.bytes + " bytes");
```

```javascriptWithoutSDK
const response = await fetch("https://api.x.ai/v1/files/file-abc123", {
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
});

const file = await response.json();
console.log("Filename: " + file.filename);
console.log("Size: " + file.bytes + " bytes");
```

```bash
curl https://api.x.ai/v1/files/file-abc123 \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

## Getting File Content

Download the raw bytes of an uploaded file. The endpoint streams the response, so it works for files of any supported size without buffering the whole payload in memory at the API layer.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Returns the complete file content as bytes.
content = client.files.content("file-abc123")

# Save to disk
with open("downloaded.pdf", "wb") as f:
    f.write(content)

print(f"Saved {len(content)} bytes")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# Stream content straight to disk
client.files.content("file-abc123").write_to_file("downloaded.pdf")
```

```pythonRequests
import os
import requests

file_id = "file-abc123"
url = f"https://api.x.ai/v1/files/{file_id}/content"
headers = {"Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"}

# Stream to disk so large files don't sit in memory
with requests.get(url, headers=headers, stream=True) as response:
    response.raise_for_status()
    with open("downloaded.pdf", "wb") as f:
        for chunk in response.iter_content(chunk_size=1024 * 1024):
            f.write(chunk)
```

```javascriptOpenAISDK
import OpenAI from "openai";
import fs from "fs";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

const response = await client.files.content("file-abc123");
const buffer = Buffer.from(await response.arrayBuffer());
fs.writeFileSync("downloaded.pdf", buffer);
```

```javascriptWithoutSDK
import fs from "fs";

const response = await fetch("https://api.x.ai/v1/files/file-abc123/content", {
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
});

const buffer = Buffer.from(await response.arrayBuffer());
fs.writeFileSync("downloaded.pdf", buffer);
```

```bash
curl https://api.x.ai/v1/files/file-abc123/content \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  --output downloaded.pdf
```

## Deleting Files

Remove files when they're no longer needed.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Delete a file
delete_response = client.files.delete("file-abc123")

print(f"Deleted: {delete_response.deleted}")
print(f"File ID: {delete_response.id}")
```

```pythonOpenAISDK
import os
from openai import OpenAI

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

# Delete a file
delete_response = client.files.delete("file-abc123")

print(f"Deleted: {delete_response.deleted}")
print(f"File ID: {delete_response.id}")
```

```pythonRequests
import os
import requests

file_id = "file-abc123"
url = f"https://api.x.ai/v1/files/{file_id}"
headers = {
    "Authorization": f"Bearer {os.getenv('XAI_API_KEY')}"
}

response = requests.delete(url, headers=headers)
result = response.json()

print(f"Deleted: {result['deleted']}")
print(f"File ID: {result['id']}")
```

```javascriptOpenAISDK
import OpenAI from "openai";

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: "https://api.x.ai/v1",
});

// Delete a file
const deleteResponse = await client.files.delete("file-abc123");

console.log("Deleted: " + deleteResponse.deleted);
console.log("File ID: " + deleteResponse.id);
```

```javascriptWithoutSDK
const response = await fetch("https://api.x.ai/v1/files/file-abc123", {
  method: "DELETE",
  headers: { Authorization: \`Bearer \${process.env.XAI_API_KEY}\` },
});

const result = await response.json();
console.log("Deleted: " + result.deleted);
console.log("File ID: " + result.id);
```

```bash
curl -X DELETE https://api.x.ai/v1/files/file-abc123 \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

## The File Object

Every Files API endpoint that returns metadata (Upload, List, Get Metadata) returns the same `file` object shape:

| Field | Type | Description |
| --- | --- | --- |
| `id` | string | Unique file identifier (e.g. `file_a128090d-f0c9-4873-bd84-e499777e7417`). Use this anywhere a `file_id` is expected, including chat attachments. |
| `filename` | string | Original filename you supplied at upload time. |
| `bytes` | integer | File size in bytes. |
| `created_at` | integer | Upload time as a Unix timestamp (seconds). |
| `expires_at` | integer or null | Unix timestamp at which the file will be deleted. `null` for permanent files; set when the file was uploaded with `expires_after`. |
| `object` | string | Always `"file"`. Returned for OpenAI compatibility. |
| `purpose` | string | Echoes the `purpose` value sent at upload time. xAI does not enforce or interpret this field; it is stored for OpenAI SDK compatibility. Setting `"assistants"` is the conventional choice. |

## Limitations and Considerations

### File Size Limits

* **Maximum file size**: 48 MB per file
* **Processing time**: Larger files may take longer to process

### File Retention

* **Cleanup**: Delete files when no longer needed to manage storage
* **Access**: Files are scoped to your team/organization

### Supported Formats

While many text-based formats are supported, the system works best with:

* Structured documents (with clear sections, headings)
* Plain text and markdown
* Documents with clear information hierarchy

Supported file types include:

* Plain text files (.txt)
* Markdown files (.md)
* Code files (.py, .js, .java, etc.)
* CSV files (.csv)
* JSON files (.json)
* PDF documents (.pdf)
* And many other text-based formats

## Next Steps
{% endraw %}
