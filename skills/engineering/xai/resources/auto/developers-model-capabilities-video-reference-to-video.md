#### Model Capabilities

# Reference-to-Video

Provide one or more reference images to incorporate specific people, objects, clothing, or other visual elements into the generated video. The model uses the reference images as a visual guide, producing a video that features the content from those images. This is useful for virtual try-on, product placement, and character-consistent storytelling.

Unlike [image-to-video](https://docs.x.ai/developers/model-capabilities/video/image-to-video), where the source image becomes the starting frame, reference images influence what appears in the video without locking in the first frame.

Each reference image can be provided as a public HTTPS URL, a base64-encoded data URI, or a `file_id` from the [Files API](https://docs.x.ai/developers/files) — and you can mix kinds within a single request. See [Imagine → Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs) for `file_id` details and examples.

In the Vercel AI SDK, set `providerOptions.xai.mode` to `"reference-to-video"` and pass the images with `providerOptions.xai.referenceImageUrls`.

> [!WARNING]

```python customLanguage="pythonXAI"
import os
import xai_sdk

client = xai_sdk.Client(api_key=os.getenv("XAI_API_KEY"))

response = client.video.generate(
    prompt="slow zoom in on the white fashion runway stage. then, the model from <IMAGE_1> walks in from the back of the shot from the white opening, and gracefully walk out onto the front of the white stage platform. they wear the shirt from <IMAGE_2> and black flared jeans. they look dramatically at the camera. high quality slow motion shot. fun, playful. skin pores. highly detailed faces. perfect shot. they reach the end of the runway and look at the camera as the camera slowly zooms. subtle smile.",
    model="grok-imagine-video",
    reference_image_urls=[
        "<IMAGE_URL_1>",
        "<IMAGE_URL_2>",
        "<IMAGE_URL_3>",
    ],
    duration=10,
    aspect_ratio="16:9",
    resolution="720p",
)

print(response.url)
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
    "https://api.x.ai/v1/videos/generations",
    headers=headers,
    json={
        "model": "grok-imagine-video",
        "prompt": "slow zoom in on the white fashion runway stage. then, the model from <IMAGE_1> walks in from the back of the shot from the white opening, and gracefully walk out onto the front of the white stage platform. they wear the shirt from <IMAGE_2> and black flared jeans. they look dramatically at the camera. high quality slow motion shot. fun, playful. skin pores. highly detailed faces. perfect shot. they reach the end of the runway and look at the camera as the camera slowly zooms. subtle smile.",
        "reference_images": [
            {"url": "<IMAGE_URL_1>"},
            {"url": "<IMAGE_URL_2>"},
            {"url": "<IMAGE_URL_3>"},
        ],
        "duration": 10,
        "aspect_ratio": "16:9",
        "resolution": "720p",
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
    elif data["status"] == "expired":
        print("Request expired")
        break
    time.sleep(5)
```

```javascript customLanguage="javascriptAISDK"
import { xai } from "@ai-sdk/xai";
import { experimental_generateVideo as generateVideo } from "ai";

const result = await generateVideo({
    model: xai.video("grok-imagine-video"),
    prompt: "slow zoom in on the white fashion runway stage. then, the model from <IMAGE_1> walks in from the back of the shot from the white opening, and gracefully walk out onto the front of the white stage platform. they wear the shirt from <IMAGE_2> and black flared jeans. they look dramatically at the camera. high quality slow motion shot. fun, playful. skin pores. highly detailed faces. perfect shot. they reach the end of the runway and look at the camera as the camera slowly zooms. subtle smile.",
    duration: 10,
    aspectRatio: "16:9",
    providerOptions: {
        xai: {
            mode: "reference-to-video",
            referenceImageUrls: [
                "<IMAGE_URL_1>",
                "<IMAGE_URL_2>",
                "<IMAGE_URL_3>",
            ],
            resolution: "720p",
            pollTimeoutMs: 600000,
        },
    },
});

const videoUrl = result.providerMetadata?.xai?.videoUrl;
console.log(videoUrl);
```

```bash
# Start the reference-to-video request
REQUEST_ID=$(curl -s -X POST https://api.x.ai/v1/videos/generations \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -d '{
    "model": "grok-imagine-video",
    "prompt": "slow zoom in on the white fashion runway stage. then, the model from <IMAGE_1> walks in from the back of the shot from the white opening, and gracefully walk out onto the front of the white stage platform. they wear the shirt from <IMAGE_2> and black flared jeans. they look dramatically at the camera. high quality slow motion shot. fun, playful. skin pores. highly detailed faces. perfect shot. they reach the end of the runway and look at the camera as the camera slowly zooms. subtle smile.",
    "reference_images": [
      {"url": "<IMAGE_URL_1>"},
      {"url": "<IMAGE_URL_2>"},
      {"url": "<IMAGE_URL_3>"}
    ],
    "duration": 10,
    "aspect_ratio": "16:9",
    "resolution": "720p"
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

## Related

* [Video Generation](https://docs.x.ai/developers/model-capabilities/video/generation) — Generate videos from text prompts
* [Image-to-Video](https://docs.x.ai/developers/model-capabilities/video/image-to-video) — Animate a still image
* [Video Editing](https://docs.x.ai/developers/model-capabilities/video/editing) — Edit existing videos
* [API Reference](https://docs.x.ai/developers/rest-api-reference) — Full endpoint documentation
* [Imagine API Landing Page](https://x.ai/api/imagine) — Showcase of the Imagine API in action