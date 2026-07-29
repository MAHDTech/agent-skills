#### Inference API

# Videos

## POST /v1/videos/generations

Generate a video from a text prompt and optionally an image.
This is an asynchronous operation that returns a request\_id for polling.

### Request Body

* `aspect_ratio` ("1:1" | "16:9" | "9:16" | "4:3" | "3:4" | "3:2" | "2:3")

* `duration` (integer | null) — Video duration in seconds. Range: \[1, 15]. Default: 8.
  Also accepts \`seconds\` for OpenAI API compatibility.
  Accepts both number (8) and string ("8") values.

* `image` (object)

  * `file_id` (string | null) — File ID from the xAI Files API. Mutually exclusive with \`url\`.
    The file must be an image (JPEG, PNG, or WebP) and fully uploaded.

  * `url` (string) — Public URL or base64-encoded data URL of the image (JPEG, PNG, or WebP).
    Also accepts \`image\_url\` for compatibility.
    Required when \`file\_id\` is not set.

* `model` (string | null) — Model to be used.

* `output` (object)

  * `upload_url` (string, required) — Signed URL to upload the generated video via HTTP PUT.

* `prompt` (string) — Prompt for video generation. Required for text-to-video (T2V) and
  reference-to-video (R2V). Optional for image-to-video (I2V) — when
  omitted, the model generates a video from the image alone.

* `reference_images` (array\\<object\>) — Optional reference images for reference-to-video (R2V) generation.
  When provided generates video using these images
  as style/content references.

  * `file_id` (string | null) — File ID from the xAI Files API. Mutually exclusive with \`url\`.
    The file must be an image (JPEG, PNG, or WebP) and fully uploaded.

  * `url` (string) — Public URL or base64-encoded data URL of the image (JPEG, PNG, or WebP).
    Also accepts \`image\_url\` for compatibility.
    Required when \`file\_id\` is not set.

* `resolution` ("480p" | "720p" | "1080p")

* `storage_options` (object)

  * `expires_after` (integer | null) — Seconds from now until the file auto-expires. Maximum 2592000 (30 days).
    If omitted, the file does not expire.

  * `filename` (string, required) — Filename for the stored file.

  * `public_url` (boolean | object)

* `user` (string | null) — A unique identifier representing your end-user.

### Response Body

* `request_id` (string, required) — A unique request ID to poll for the result.

### Code Examples

```bash
curl -s https://api.x.ai/v1/videos/generations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-imagine-video",
    "prompt": "A serene lake at sunrise with mist rolling over the water"
  }'
```

```javascriptWithoutSDK
const response = await fetch("https://api.x.ai/v1/videos/generations", {
  method: "POST",
  headers: {
    Authorization: `Bearer ${process.env.XAI_API_KEY}`,
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    model: "grok-imagine-video",
    prompt: "A serene lake at sunrise with mist rolling over the water",
  }),
});

console.log(JSON.stringify(await response.json(), null, 2));
```

```pythonWithoutSDK
import json
import os

import requests

response = requests.post(
    "https://api.x.ai/v1/videos/generations",
    headers={
        "Authorization": f"Bearer {os.environ['XAI_API_KEY']}",
        "Content-Type": "application/json",
    },
    json={
        "model": "grok-imagine-video",
        "prompt": "A serene lake at sunrise with mist rolling over the water",
    },
)

print(json.dumps(response.json(), indent=2))
```

\*\*Response example:\*\*

```json
{
  "request_id": "a3d1008e-4544-40d4-d075-11527e794e4a"
}
```

***

## POST /v1/videos/edits

Edit a video based on a prompt.
This is an asynchronous operation that returns a request\_id for polling.

### Request Body

* `model` (string | null) — Model to be used.

* `output` (object)

  * `upload_url` (string, required) — Signed URL to upload the generated video via HTTP PUT.

* `prompt` (string, required) — Prompt for video editing.

* `storage_options` (object)

  * `expires_after` (integer | null) — Seconds from now until the file auto-expires. Maximum 2592000 (30 days).
    If omitted, the file does not expire.

  * `filename` (string, required) — Filename for the stored file.

  * `public_url` (boolean | object)

* `user` (string | null) — A unique identifier representing your end-user.

* `video` (object, required) — Video input for editing and extension requests.
  Accepts a public URL, a base64-encoded data URL, or a file\_id from the xAI Files API.

  * `file_id` (string | null) — File ID from the xAI Files API. Mutually exclusive with \`url\`.
    The file must be a video (e.g., MP4) and fully uploaded.

  * `url` (string) — URL of the video (public URL or base64-encoded data URL).
    The video must have the \`.mp4\` file extension and be encoded with \`.mp4\` supported codecs such as H.265, H.264, AV1, etc.
    Required when \`file\_id\` is not set.

### Response Body

* `request_id` (string, required) — A unique request ID to poll for the result.

### Code Examples

```bash
curl -s https://api.x.ai/v1/videos/edits \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d "{
    \"model\": \"grok-imagine-video\",
    \"prompt\": \"Give the woman a silver necklace\",
    \"video\": {
      \"url\": \"$VIDEO_URL\"
    }
  }"
```

```javascriptWithoutSDK
const response = await fetch("https://api.x.ai/v1/videos/edits", {
  method: "POST",
  headers: {
    Authorization: `Bearer ${process.env.XAI_API_KEY}`,
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    model: "grok-imagine-video",
    prompt: "Give the woman a silver necklace",
    video: {
      url: process.env.VIDEO_URL,
    },
  }),
});

console.log(JSON.stringify(await response.json(), null, 2));
```

```pythonWithoutSDK
import json
import os

import requests

response = requests.post(
    "https://api.x.ai/v1/videos/edits",
    headers={
        "Authorization": f"Bearer {os.environ['XAI_API_KEY']}",
        "Content-Type": "application/json",
    },
    json={
        "model": "grok-imagine-video",
        "prompt": "Give the woman a silver necklace",
        "video": {
            "url": os.environ["VIDEO_URL"],
        },
    },
)

print(json.dumps(response.json(), indent=2))
```

\*\*Response example:\*\*

```json
{
  "request_id": "a3d1008e-4544-40d4-d075-11527e794e4a"
}
```

***

## POST /v1/videos/extensions

Extend a video by generating continuation content.
This is an asynchronous operation that returns a request\_id for polling.

### Request Body

* `duration` (integer | null) — Duration of the extension segment to generate in seconds (2-10).
  Defaults to 6 seconds if not specified.

* `model` (string | null) — Model to be used.

* `output` (object)

  * `upload_url` (string, required) — Signed URL to upload the generated video via HTTP PUT.

* `prompt` (string, required) — Prompt describing what should happen next in the video.

* `storage_options` (object)

  * `expires_after` (integer | null) — Seconds from now until the file auto-expires. Maximum 2592000 (30 days).
    If omitted, the file does not expire.

  * `filename` (string, required) — Filename for the stored file.

  * `public_url` (boolean | object)

* `video` (object, required) — Video input for editing and extension requests.
  Accepts a public URL, a base64-encoded data URL, or a file\_id from the xAI Files API.

  * `file_id` (string | null) — File ID from the xAI Files API. Mutually exclusive with \`url\`.
    The file must be a video (e.g., MP4) and fully uploaded.

  * `url` (string) — URL of the video (public URL or base64-encoded data URL).
    The video must have the \`.mp4\` file extension and be encoded with \`.mp4\` supported codecs such as H.265, H.264, AV1, etc.
    Required when \`file\_id\` is not set.

### Response Body

* `request_id` (string, required) — A unique request ID to poll for the result.

\*\*Request example:\*\*

```json
{
  "prompt": "The camera slowly zooms out to reveal the city skyline",
  "video": {
    "url": "https://example.com/video.mp4"
  },
  "model": "grok-imagine-video",
  "duration": 6
}
```

\*\*Response example:\*\*

```json
{
  "request_id": "a3d1008e-4544-40d4-d075-11527e794e4a"
}
```

***

## GET /v1/videos/\{request\_id}

Get the result of a deferred video generation request.

### Path Parameters

* `request_id` (string, required) — The deferred request id returned by a previous video generation request.

### Response Body

* `error` (object)

  * `code` ("invalid\_argument" | "permission\_denied" | "failed\_precondition" | "service\_unavailable" | "internal\_error", required) — Machine-readable error codes for video generation failures.

    These are the codes that can appear in \`VideoError.code\` when polling a
    deferred video generation result. Authentication, model-not-found, and
    synchronous rate-limit errors are returned as HTTP errors and never
    appear in \`VideoError\`. Engine overload encountered mid-generation
    surfaces here as \`service\_unavailable\` (HTTP 503).

    Serializes to/from snake\_case strings (e.g. \`"invalid\_argument"\`,
    \`"internal\_error"\`) for JSON compatibility.

  * `message` (string, required) — Human-readable error message describing the failure.

* `model` (string | null) — The model used to generate the video. Omitted when status is "failed".

* `progress` (integer | null) — Approximate completion percentage for the video generation task (0-100).
  \- When status is "pending": progress is between 0-99, indicating current completion.
  \- When status is "done": progress is 100.
  \- When status is "failed": progress is omitted.

* `status` (string, required) — Status of the video generation: "done" when the video is ready.

* `usage` (object)

  * `cost_in_usd_ticks` (integer, required) — The cost of this request expressed in USD ticks.
    One USD cent equals 100,000,000 ticks, so one US dollar equals
    10,000,000,000 ticks.

* `video` (object)

  * `duration` (integer, required) — Duration of the generated video in seconds.

  * `file_output` (object)

    * `expires_at` (integer | null) — Unix timestamp (seconds) when the stored file expires and will be
      automatically deleted. Only present when the file has an expiration.

    * `file_id` (string, required) — Files API file\_id of the stored file.

    * `filename` (string, required) — Filename of the stored file.

    * `public_url` (string | null) — Public URL for the stored file. Only present when the request included
      \`storage\_options.public\_url\` and creation succeeded.

    * `public_url_error` (string | null) — Human-readable error when \`storage\_options.public\_url\` was set but
      public URL creation failed. The file was stored successfully.

    * `public_url_expires_at` (integer | null) — Unix timestamp (seconds) when the public URL expires. Present when
      the public URL has an expiry, either from an explicit \`expires\_after\`
      in the request or inherited from the file's TTL.

  * `respect_moderation` (boolean, required) — Whether the video generated by the model respects moderation rules.
    The field will be true if the video respects moderation rules. Otherwise
    the field will be false and the video url field will be empty.

  * `storage_error` (string | null) — Human-readable error when \`storage\_options\` was set but the upload
    failed. Absent on success or when storage was not requested.

  * `url` (string | null) — A url to the generated video.

### Code Examples

```bash
curl -s "https://api.x.ai/v1/videos/$VIDEO_REQUEST_ID" \
  -H "Authorization: Bearer $XAI_API_KEY"
```

```javascriptWithoutSDK
const response = await fetch(
  `https://api.x.ai/v1/videos/${process.env.VIDEO_REQUEST_ID}`,
  {
    headers: {
      Authorization: `Bearer ${process.env.XAI_API_KEY}`,
    },
  },
);

console.log(JSON.stringify(await response.json(), null, 2));
```

```pythonWithoutSDK
import json
import os

import requests

request_id = os.environ["VIDEO_REQUEST_ID"]

response = requests.get(
    f"https://api.x.ai/v1/videos/{request_id}",
    headers={
        "Authorization": f"Bearer {os.environ['XAI_API_KEY']}",
    },
)

print(json.dumps(response.json(), indent=2))
```

\*\*Response example:\*\*

```json
{
  "status": "done",
  "video": {
    "url": "https://vidgen.x.ai/xai-vidgen-bucket/xai-video-{request_id}.mp4",
    "duration": 6,
    "respect_moderation": true
  },
  "model": "grok-imagine-video"
}
```