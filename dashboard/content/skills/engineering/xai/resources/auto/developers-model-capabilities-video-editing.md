+++
title = "developers-model-capabilities-video-editing"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

{% raw %}
#### Model Capabilities

# Video Editing

Edit an existing video by providing a source video along with your prompt. The model understands the video content and applies your requested changes.

You can provide the source video as a public URL, a base64-encoded data URI, or a `file_id` from the [Files API](https://docs.x.ai/developers/files). See [Imagine → Files API Integration](https://docs.x.ai/developers/model-capabilities/imagine/files/inputs) for using `file_id` inputs.

> [!WARNING]

The demo below shows video editing in action. `grok-imagine-video` delivers high-fidelity edits with strong scene preservation, modifying only what you ask for while keeping the rest of the video intact:

In the Vercel AI SDK, video editing is triggered by setting `providerOptions.xai.mode` to `"edit-video"` and passing `providerOptions.xai.videoUrl` with a source video URL. The `prompt` describes the desired modifications; `duration`, `aspectRatio`, and `resolution` are ignored because the output inherits these properties from the input video, capped at 720p.

## Concurrent Requests

When you need to apply several edits to the same source video, run requests concurrently. This is useful for branching multiple edits from the same intermediate result.

```python customLanguage="pythonXAI"
import os
import asyncio
import xai_sdk

async def edit_concurrently():
    client = xai_sdk.AsyncClient(api_key=os.getenv("XAI_API_KEY"))

    source_video = "https://data.x.ai/docs/video-generation/portrait-wave.mp4"

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

```javascript customLanguage="javascriptAISDK"
import { xai, type XaiVideoModelOptions } from "@ai-sdk/xai";
import { experimental_generateVideo as generateVideo } from "ai";

const providerOptions = {
    xai: {
        mode: "edit-video",
        videoUrl: "https://example.com/source-video.mp4",
        pollTimeoutMs: 600000,
    } satisfies XaiVideoModelOptions,
};

const step1 = await generateVideo({
    model: xai.video("grok-imagine-video"),
    prompt: "Add a party hat to the person",
    providerOptions,
});

const step1VideoUrl = step1.providerMetadata?.xai?.videoUrl as string;

const [withSunglasses, withScarf] = await Promise.all([
    generateVideo({
        model: xai.video("grok-imagine-video"),
        prompt: "Add sunglasses",
        providerOptions: {
            xai: {
                mode: "edit-video",
                videoUrl: step1VideoUrl,
                pollTimeoutMs: 600000,
            } satisfies XaiVideoModelOptions,
        },
    }),
    generateVideo({
        model: xai.video("grok-imagine-video"),
        prompt: "Add a scarf",
        providerOptions: {
            xai: {
                mode: "edit-video",
                videoUrl: step1VideoUrl,
                pollTimeoutMs: 600000,
            } satisfies XaiVideoModelOptions,
        },
    }),
]);

console.log(withSunglasses.providerMetadata?.xai?.videoUrl);
console.log(withScarf.providerMetadata?.xai?.videoUrl);
```

## Related

* [Video Generation](https://docs.x.ai/developers/model-capabilities/video/generation) — Generate videos from text prompts
* [Image-to-Video](https://docs.x.ai/developers/model-capabilities/video/image-to-video) — Animate a still image
* [Video Extension](https://docs.x.ai/developers/model-capabilities/video/extension) — Extend existing videos
* [API Reference](https://docs.x.ai/developers/rest-api-reference) — Full endpoint documentation
* [Imagine API Landing Page](https://x.ai/api/imagine) — Showcase of the Imagine API in action
{% endraw %}
