+++
title = "developers-files-public-urlslimitations"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Files & Collections

# Public URLs

Every file you upload through the [Files API](https://docs.x.ai/developers/files/managing-files) lives in private storage by default — fetching it requires your API key. **Public URLs** turn a stored file into a permanent, shareable link on the xAI CDN that anyone can open — no API key required.

You stay in control after creation:

* **Revoke at any time** with a single API call to immediately invalidate the URL.
* **Auto-expire** by setting `expires_after` (1 hour up to 30 days), or let the URL **inherit the file's own expiry** so both vanish together.

Creating a public URL doesn't modify the underlying private file, and revoking it doesn't delete the file — the two lifecycles are independent.

If you need to gate access (e.g. only logged-in users), keep the file private and serve it through your own backend using the authenticated [`GET /v1/files/{file_id}/content`](https://docs.x.ai/developers/files/managing-files#getting-file-content) endpoint instead.

> [!TIP]
>
> Generating images or videos with the [Imagine API](https://docs.x.ai/developers/model-capabilities/imagine)? You
> can create a public URL **in the same request that produces the asset** via
> `storage_options.public_url`. See [Imagine → Files API
> Integration](https://docs.x.ai/developers/model-capabilities/imagine/files).

## Quick Start

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# 1. Upload (or reference an existing) file
file = client.files.upload("/path/to/diagram.png")

# 2. Create the public URL
resp = client.files.create_public_url(file.id)

print(resp.public_url)
# https://files-cdn.x.ai/<token>/file_abc123.png

# 3. When you're done sharing, revoke it
client.files.revoke_public_url(file.id)
```

```bash
# 1. Upload (or reference an existing) file
FILE_ID=$(curl -s https://api.x.ai/v1/files \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -F purpose=assistants \\
  -F file=@/path/to/diagram.png | jq -r '.id')

# 2. Create the public URL (empty JSON body uses defaults)
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{}'
# {"public_url":"https://files-cdn.x.ai/<token>/file_abc123.png"}

# 3. When you're done sharing, revoke
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url/revoke" \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

> [!WARNING]
>
> Public URLs can only be created for files that **already exist** in your Files API storage. You
> cannot create a public URL during upload — upload the file first, then call `create_public_url`
> (or use `storage_options.public_url` on an Imagine request).

## Expiry Behaviour

You can optionally set an expiry on a public URL at creation time via `expires_after` (in seconds). Once the deadline passes, the URL is automatically revoked — subsequent requests return `404` and you don't need to make a follow-up API call to clean it up. The underlying file is unaffected and remains available through the authenticated Files API.

The URL's effective expiry comes from two inputs: whether you pass `expires_after` at creation, and whether the underlying file [has its own expiration](https://docs.x.ai/developers/files/managing-files#upload-with-expiration-ttl).

* **File has no expiration, `expires_after` omitted** — URL never expires. It lives until you explicitly call `revoke_public_url` or delete the underlying file.
* **File has no expiration, `expires_after` set to `N`** — URL auto-revokes `N` seconds from now. The file itself is untouched.
* **File has its own expiration at time `T`, `expires_after` omitted** — URL inherits the file's expiry. Both disappear at `T`.
* **File has its own expiration at time `T`, `expires_after` set to `N`** — URL auto-revokes `N` seconds from now. `N` must be ≤ the file's remaining lifetime, otherwise the request is rejected.

`expires_after` must be between **3600 seconds (1 hour)** and **2592000 seconds (30 days)**. A public URL can never outlive its file — requesting an `expires_after` greater than the file's remaining lifetime is rejected.

```pythonXAI
import os
from datetime import timedelta
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))
file = client.files.upload("/path/to/photo.png")

# 1. Indefinite: omit expires_after on a file with no expiry.
# Must call revoke_public_url to explicitly revoke the public URL.
resp = client.files.create_public_url(file.id)
assert not resp.HasField("expires_at")

# 2. URL-bound: pass expires_after as int seconds or a timedelta
resp = client.files.create_public_url(file.id, expires_after=timedelta(hours=24))
print(f"Expires at: {resp.expires_at.seconds}")

# 3. Inherited: file has its own expiration, omit expires_after on the URL
ttl_file = client.files.upload(
    b"\\x89PNG\\r\\n\\x1a\\n" + b"\\x00" * 32,
    filename="short-lived.png",
    expires_after=timedelta(hours=2),
)
resp = client.files.create_public_url(ttl_file.id)
# resp.expires_at matches the file's expires_at
```

```bash
# 1. Indefinite — file has no expiry.
# Must call POST /public-url/revoke to explicitly revoke.
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -H "Content-Type: application/json" -d '{}'
# {"public_url":"..."} <- no expires_at field

# 2. URL-bound (24h)
curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{"expires_after": 86400}'
# {"public_url":"...","expires_at":1755600000}

# 3. Inherited: upload with file expiration, then create with no expires_after
FILE_ID=$(curl -s https://api.x.ai/v1/files \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -F expires_after=7200 \\
  -F purpose=assistants \\
  -F file=@/path/to/photo.png | jq -r '.id')

curl -s -X POST "https://api.x.ai/v1/files/$FILE_ID/public-url" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -H "Content-Type: application/json" -d '{}'
# {"public_url":"...","expires_at":<matches file expiry>}
```

## Idempotency

A file can have **at most one active public URL at a time**. Calling `create_public_url` on a file that already has one returns the existing URL without producing a new one — it's safe to call repeatedly.

If you pass a different `expires_after` on a subsequent call, the existing URL's expiry is updated in place. The token in the URL stays the same.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))
file_id = "file_abc123"

# First call creates the URL
resp1 = client.files.create_public_url(file_id, expires_after=86400)  # 1 day

# Second call returns the same URL, no re-upload
resp2 = client.files.create_public_url(file_id, expires_after=86400)
assert resp1.public_url == resp2.public_url

# Calling again with a different expires_after extends/shortens the expiry
# while keeping the same URL
resp3 = client.files.create_public_url(file_id, expires_after=604800)  # 7 days
assert resp1.public_url == resp3.public_url
assert resp3.expires_at.seconds > resp1.expires_at.seconds
```

## Revoking a Public URL

Revoking invalidates the URL and clears it from the file's metadata. The original file is untouched and continues to be accessible through authenticated endpoints.

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# Revoke a public URL
resp = client.files.revoke_public_url("file_abc123")
print(f"Revoked: {resp.revoked}")    # True
print(f"Was URL: {resp.public_url}") # the URL that just stopped working

# The file itself is still available via authenticated endpoints
file = client.files.get("file_abc123")
print(file.filename)

# Revoke is idempotent and safe to call on:
# - files that never had a public URL (returns revoked=False)
# - files whose URL was already revoked (returns revoked=False)
# - files that have been deleted
client.files.revoke_public_url("file_abc123")  # no-op, no error
```

```bash
curl -s -X POST "https://api.x.ai/v1/files/file_abc123/public-url/revoke" \\
  -H "Authorization: Bearer $XAI_API_KEY"
# {"id":"file_abc123","revoked":true,"public_url":"https://files-cdn.x.ai/..."}

# Calling again is safe — returns revoked=false
curl -s -X POST "https://api.x.ai/v1/files/file_abc123/public-url/revoke" \\
  -H "Authorization: Bearer $XAI_API_KEY"
# {"id":"file_abc123","revoked":false}
```

**Revocation is all-or-nothing.** A file can only have one public URL at a time, so revoking breaks the link for everyone who has it. If a link leaks to the wrong party, the only remedy is to revoke and create a new URL — the new one will have a fresh token and the old URL stays permanently dead.

## Finding Files with a Public URL

`get_file` and `list_files` always return the current public URL state of a file. `public_url` and `public_url_expires_at` are populated on every file with an active public URL.

You can also use the [`filter`](https://docs.x.ai/developers/rest-api-reference/files/manage) parameter on `list_files` to find files with or without an active public URL:

```pythonXAI
import os
from xai_sdk import Client

client = Client(api_key=os.getenv("XAI_API_KEY"))

# All files that currently have a public URL
with_url = client.files.list(filter="public_url != null")
for f in with_url.data:
    print(f.id, f.filename)

# All files that do not currently have a public URL
without_url = client.files.list(filter="public_url = null")
```

```bash
# URL-encode the filter
curl -s "https://api.x.ai/v1/files?filter=public_url%20!%3D%20null" \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

## Limitations

* **Maximum file size: 50 MiB.** Larger files remain available through the authenticated Files API but cannot be made public.
* **Restricted content types.** Only the following are eligible:
  * `image/png` (`.png`)
  * `image/jpeg` (`.jpg`)
  * `video/mp4` (`.mp4`)
  * `application/pdf` (`.pdf`)
* **Expiry must be between 1 hour and 30 days**, and a public URL can never outlive its file.
* **Deleting the file revokes the public URL** automatically. You cannot keep a public URL alive after the file is deleted (manually or by expiration).
* **One public URL per file at a time.** `create_public_url` is idempotent and returns the same URL on repeat calls. After a revoke, the next `create_public_url` issues a new token — any previously shared URL becomes permanently invalid.
* **Up to 1,000 active public URLs per team.** Revoke URLs you no longer need before creating new ones.

## Related

* [Managing Files](https://docs.x.ai/developers/files/managing-files) — Upload, list, retrieve, and delete files.
* [Imagine → Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files) — Reference stored files as Imagine inputs, persist generated assets, and create public URLs in a single request.
* [Files API Reference](https://docs.x.ai/developers/rest-api-reference/files) — Full REST endpoint documentation.
{% endraw %}
