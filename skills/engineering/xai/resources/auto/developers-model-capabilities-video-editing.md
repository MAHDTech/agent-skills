#### Model Capabilities

# Video Editing

Edit an existing video by providing a source video along with your prompt. The model understands the video content and applies your requested changes.

You can provide the source video as a public URL, a base64-encoded data URI, or a `file_id` from the [Files API](/developers/files). See [Files API integration for image & video requests](/developers/model-capabilities/imagine/files/inputs) for using `file_id` inputs.

| Requirement | Value |
|-------------|-------|
| Model | `grok-imagine-video` |
| Source video | `.mp4` with a supported codec such as H.264, H.265, or AV1 |
| Source length | Up to **8.7 seconds** |
| Output | Same duration, aspect ratio, and resolution as the source, capped at **720p** |
| `duration`, `aspect_ratio`, `resolution` | Not supported |

`grok-imagine-video` applies the change you describe and keeps everything else as close to the source as it can. Say what must stay the same, as these prompts do:

In the Vercel AI SDK, video editing is triggered by setting `providerOptions.xai.mode` to `"edit-video"` and passing `providerOptions.xai.videoUrl` with a source video URL. The `prompt` describes the desired modifications; `duration`, `aspectRatio`, and `resolution` are ignored because the output inherits these properties from the input video, capped at 720p.

## Concurrent Requests

When you need to apply several edits to the same source video, run requests concurrently.

```javascript customLanguage="javascriptAISDK"
import { xai, type XaiVideoModelOptions } from "@ai-sdk/xai";
import { experimental_generateVideo as generateVideo } from "ai";

const sourceVideo = "https://media.x.ai/v1/docs/i2v-portrait-wave-4cb4c17c.mp4";

const prompts = [
    "Give the woman a silver necklace",
    "Change the color of the woman's outfit to red",
    "Give the woman a wide-brimmed black hat",
];

const results = await Promise.all(
    prompts.map(prompt =>
        generateVideo({
            model: xai.video("grok-imagine-video"),
            prompt,
            providerOptions: {
                xai: {
                    mode: "edit-video",
                    videoUrl: sourceVideo,
                    pollTimeoutMs: 600000,
                } satisfies XaiVideoModelOptions,
            },
        }),
    ),
);

prompts.forEach((prompt, i) => {
    console.log(`${prompt}: ${results[i].providerMetadata?.xai?.videoUrl}`);
});
```

```python customLanguage="pythonXAI"
import os
import asyncio
import xai_sdk

async def edit_concurrently():
    client = xai_sdk.AsyncClient(api_key=os.getenv("XAI_API_KEY"))

    source_video = "https://media.x.ai/v1/docs/i2v-portrait-wave-4cb4c17c.mp4"

    prompts = [
        "Give the woman a silver necklace",
        "Change the color of the woman's outfit to red",
        "Give the woman a wide-brimmed black hat",
    ]

    tasks = [
        client.video.generate(
            prompt=prompt,
            model="grok-imagine-video",
            video_url=source_video,
        )
        for prompt in prompts
    ]

    results = await asyncio.gather(*tasks)

    for prompt, result in zip(prompts, results):
        print(f"{prompt}: {result.url}")

asyncio.run(edit_concurrently())
```

## Related

* [Video Generation](/developers/model-capabilities/video/generation) — Generate videos from text prompts
* [Image-to-Video](/developers/model-capabilities/video/image-to-video) — Animate a still image
* [Video Extension](/developers/model-capabilities/video/extension) — Extend existing videos
* [API Reference](/developers/rest-api-reference) — Full endpoint documentation
* [Image & Video API Landing Page](https://x.ai/api/imagine) — Showcase of the image & video API in action

