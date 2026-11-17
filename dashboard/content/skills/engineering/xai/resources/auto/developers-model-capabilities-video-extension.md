+++
title = "developers-model-capabilities-video-extension"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Model Capabilities

# Video Extension

Extend an existing video by providing a source video and a text prompt describing what should happen next. The result is a single video that picks up seamlessly from the last frame of the input and continues with the generated content.

You can provide the source video as a public URL, a base64-encoded data URI, or a `file_id` from the [Files API](/developers/files). See [Files API integration for image & video requests](/developers/model-capabilities/imagine/files/inputs) for using `file_id` inputs.

| Requirement | Value |
|-------------|-------|
| Model | `grok-imagine-video` |
| Source video | `.mp4` with a supported codec such as H.264, H.265, or AV1 |
| Source length | **2–15 seconds** |
| Extension length (`duration`) | **2–10 seconds**; default 6 |
| Output | Same aspect ratio and resolution as the source, capped at **720p** |
| `aspect_ratio`, `resolution` | Not supported |

The `duration` parameter controls the length of the **extended portion only**, not the total output. For example, if your input video is 10 seconds and you set `duration` to 5, the returned video will be 15 seconds long (10s original + 5s extension).

```javascript customLanguage="javascriptAISDK"
import { xai } from "@ai-sdk/xai";
import { experimental_generateVideo as generateVideo } from "ai";

const source = await generateVideo({
    model: xai.video("grok-imagine-video-1.5"),
    prompt: "A cat sitting on a sunlit windowsill, tail gently swishing.",
    duration: 5,
    aspectRatio: "16:9",
    providerOptions: {
        xai: {
            pollTimeoutMs: 600000,
        },
    },
});

const sourceUrl = source.providerMetadata?.xai?.videoUrl;

const extended = await generateVideo({
    model: xai.video("grok-imagine-video"),
    prompt: "The cat turns its head, notices a butterfly, and leaps off.",
    duration: 6,
    providerOptions: {
        xai: {
            mode: "extend-video",
            videoUrl: sourceUrl,
            pollTimeoutMs: 600000,
        },
    },
});

const extendedVideoUrl = extended.providerMetadata?.xai?.videoUrl;
console.log(extendedVideoUrl);
```

```python customLanguage="pythonRequests"
import os
import time
import requests

headers = {
    "Content-Type": "application/json",
    "Authorization": f"Bearer {os.environ['XAI_API_KEY']}",
}

response = requests.post(
    "https://api.x.ai/v1/videos/extensions",
    headers=headers,
    json={
        "model": "grok-imagine-video",
        "prompt": "The shot pans to an over the shoulder perspective. Calm controlled scene.",
        "duration": 10,
        "video": {"url": "<VIDEO_URL>"},
    },
)

request_id = response.json()["request_id"]

while True:
    result = requests.get(
        f"https://api.x.ai/v1/videos/{request_id}",
        headers={"Authorization": headers["Authorization"]},
    )
    data = result.json()
    if data["status"] == "done":
        print(data["video"]["url"])
        break
    elif data["status"] in ("expired", "failed"):
        print(f"Request {data['status']}")
        break
    time.sleep(5)
```

```bash
# Start the video extension request
REQUEST_ID=$(curl -s -X POST https://api.x.ai/v1/videos/extensions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-imagine-video",
    "prompt": "The shot pans to an over the shoulder perspective. Calm controlled scene.",
    "duration": 10,
    "video": {"url": "<VIDEO_URL>"}
  }' | jq -r '.request_id')

# Poll until the video is ready
while true; do
  RESULT=$(curl -s https://api.x.ai/v1/videos/$REQUEST_ID \
    -H "Authorization: Bearer $XAI_API_KEY")
  STATUS=$(echo "$RESULT" | jq -r '.status')
  if [ "$STATUS" = "done" ]; then
    echo "$RESULT" | jq -r '.video.url'
    break
  elif [ "$STATUS" = "failed" ] || [ "$STATUS" = "expired" ]; then
    echo "Request $STATUS"; echo "$RESULT" | jq .
    break
  fi
  sleep 5
done
```

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.video.extend(
    prompt="The shot pans to an over the shoulder perspective. Calm controlled scene.",
    model="grok-imagine-video",
    video_url="<VIDEO_URL>",
    duration=10,
)

print(response.url)
```

Extension runs asynchronously:

* **REST API** — `POST /v1/videos/extensions` returns a `request_id`. Poll `GET /v1/videos/{request_id}` until `status` is `done`.
* **Python SDK** — `client.video.extend()` polls for you and returns the finished video.
* **AI SDK** — Set `providerOptions.xai.mode` to `"extend-video"` and pass the source in `providerOptions.xai.videoUrl`. The result URL is in `providerMetadata.xai.videoUrl`.

To change what happens inside a video instead of continuing it, see [Video Editing](/developers/model-capabilities/video/editing).

## Related

* [Video Generation](/developers/model-capabilities/video/generation) — Generate videos from text prompts
* [Video Editing](/developers/model-capabilities/video/editing) — Edit existing videos
* [Image-to-Video](/developers/model-capabilities/video/image-to-video) — Animate a still image
* [API Reference](/developers/rest-api-reference) — Full endpoint documentation
* [Image & Video API Landing Page](https://x.ai/api/imagine) — Showcase of the image & video API in action


