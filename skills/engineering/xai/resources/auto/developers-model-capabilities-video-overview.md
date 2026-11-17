#### Model Capabilities

# Video Overview

Grok video models turn a set of references, a still image, an existing clip, or a prompt into video with generated audio. `grok-imagine-video-1.5` generates lip-synced speech, renders text-to-video and image-to-video at native 1080p, takes up to 14 reference images and 3 voice references, and pins first, last, and mid-video frames, in clips up to 15 seconds. `grok-imagine-video-1.5-lite` does text-to-video and image-to-video with lip-synced speech at the lowest price per second, reaching 1080p by upscaling 720p. We recommend `grok-imagine-video-1.5-lite` for simple videos, and `grok-imagine-video-1.5` when you need references, voices, pinned frames, or native 1080p.

Requests are asynchronous: submit, poll with the returned `request_id`, then download the clip. The xAI SDK and the Vercel AI SDK poll for you ([how it works](/developers/model-capabilities/video/generation#how-it-works)).

## Capabilities and specifications

| Capability | `grok-imagine-video-1.5` | `grok-imagine-video-1.5-lite` | `grok-imagine-video` |
| --- | --- | --- | --- |
| [Text-to-video](/developers/model-capabilities/video/generation) | Yes: Up to 1080p, native | Yes: Up to 1080p, upscaled from 720p | Yes: Up to 720p |
| [Image-to-video](/developers/model-capabilities/video/image-to-video) | Yes: Up to 1080p, native | Yes: Up to 1080p, upscaled from 720p | Yes: Up to 720p |
| [Reference images](/developers/model-capabilities/video/reference-to-video) | Yes: Up to 14 images, 15 s, 720p | Not supported | Yes: Up to 7 images, 10 s, 720p |
| [Voice references](/developers/model-capabilities/video/reference-to-video#reference-audio) | Yes: Up to 3 voices | Not supported | Not supported |
| [First & last frame](/developers/model-capabilities/video/reference-to-video#first--last-frame) | Yes | Not supported | Not supported |
| [Keyframes](/developers/model-capabilities/video/reference-to-video#keyframes) | Yes: Up to 4 mid-video frames | Not supported | Not supported |
| [Video editing](/developers/model-capabilities/video/editing) | Not supported | Not supported | Yes: Source up to 8.7 s |
| [Video extension](/developers/model-capabilities/video/extension) | Not supported | Not supported | Yes: Adds 2–10 s |
| Duration | Yes: 1–15 s | Yes: 1–15 s | Yes: 1–15 s |
| [Resolution](/developers/model-capabilities/video/generation#resolution) | Yes: 480p, 720p, 1080p | Yes: 480p, 720p, 1080p | Yes: 480p, 720p |
| [Aspect ratio](/developers/model-capabilities/video/generation#aspect-ratio) | Yes: 1:1, 16:9, 9:16, 4:3, 3:4, 3:2, 2:3, 21:9, 5:2 | Yes: 1:1, 16:9, 9:16, 4:3, 3:4, 3:2, 2:3, 21:9, 5:2 | Yes: 1:1, 16:9, 9:16, 4:3, 3:4, 3:2, 2:3, 21:9, 5:2 |
| [Image-to-video aspect ratio](/developers/model-capabilities/video/generation#aspect-ratio) | Yes: Automatic: matches the input image | Yes: Automatic: matches the input image | Yes: Automatic: matches the input image |
| [Audio](/developers/model-capabilities/video/generation#audio) | Yes: Native audio with lip-synced speech | Yes: Native audio with lip-synced speech | Yes: Generated audio |

Per-second prices by resolution are listed on the [Models](/developers/models) page.

## Reference-to-video

Reference images supply the characters, products, and locations without fixing the first frame: up to 14 per request on `grok-imagine-video-1.5`, 7 on `grok-imagine-video`. On `grok-imagine-video-1.5`, `reference_audios` gives up to three characters a preset voice, with lip-synced speech. Voice references from your own audio files are available to trusted partners [on request](https://x.ai/contact-sales?interest=imagine).

## First, key, and last frames

On `grok-imagine-video-1.5`, `image` sets the first frame, `last_frame` the last, and up to four `keyframes` pin frames at chosen timestamps; the model fills in the motion. Pass the same image as `image` and `last_frame` to make a loop.

## Image-to-video

The still you pass as `image` becomes the first frame, and the prompt describes what happens next. The output always matches the still's aspect ratio; `aspect_ratio` is ignored.

## Text-to-video

Describe the shot, including camera movement, lighting, and sound. On `grok-imagine-video-1.5` and `grok-imagine-video-1.5-lite`, the model renders a first frame from the prompt, then animates it.

## Video editing

Send a clip and an instruction to `grok-imagine-video`. It applies the change and keeps everything else as close to the source as it can; say what must stay the same, as these prompts do.

## Related

* [Video Generation](/developers/model-capabilities/video/generation): Configuration, polling, and error handling
* [Image-to-Video](/developers/model-capabilities/video/image-to-video): Animate a still image
* [Reference-to-Video](/developers/model-capabilities/video/reference-to-video): References, voices, and pinned frames
* [Video Editing](/developers/model-capabilities/video/editing) and [Video Extension](/developers/model-capabilities/video/extension): Change or continue an existing clip
* [Models](/developers/models): Pricing and rate limits for every video model

