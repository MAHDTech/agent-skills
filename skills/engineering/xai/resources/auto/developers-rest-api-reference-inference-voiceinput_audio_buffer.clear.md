#### Inference API

# Voice

## POST /v1/realtime/client\_secrets

API endpoint for POST requests to /v1/realtime/client\_secrets.

```
Method: POST
Path: /v1/realtime/client_secrets
```

***

## POST /v2/phone-numbers

API endpoint for POST requests to /v2/phone-numbers.

```
Method: POST
Path: /v2/phone-numbers
```

***

## Realtime

WebSocket endpoint: `wss://api.x.ai/v1/realtime`

Real-time voice conversations with Grok models via WebSocket. The connection begins with an HTTP GET that is upgraded to WebSocket (status 101). Once connected, the client and server exchange JSON messages to configure the session, stream audio, and receive responses. For SIP calls, connect with the \`call\_id\` from a \`realtime.call.incoming\` webhook.

Full schemas and examples: [`/voice-realtime.ws.json`](https://docs.x.ai/voice-realtime.ws.json)

### Query Parameters

* `call_id` (string, optional) — SIP call identifier from a \`realtime.call.incoming\` webhook. When provided, the WebSocket connects to that inbound SIP call. Authenticate with an xAI API key; ephemeral client secrets are not supported for SIP \`call\_id\` sessions.

* `model` (string, optional, default: grok-voice-latest) — Model to use for the session. Ignored when \`call\_id\` is provided because the session is bound to the inbound SIP call. Use grok-voice-latest for the best experience on direct WebSocket sessions.

* `reasoning.effort` (string, optional, default: high) — Controls whether the model uses reasoning. Defaults to \`high\`. Supported only with grok-voice-latest and grok-voice-think-fast-1.0.

### Client Messages

* `session.update` — Update session configuration such as system prompt, voice, audio format, turn detection, and tools.

* `input_audio_buffer.append` — Append chunks of base64-encoded audio data to the input buffer. The server does not send back a corresponding message.

* `input_audio_buffer.commit` — Commit the audio buffer as a user message. Only available when \`turn\_detection\` type is \`null\`. Confirmed by \`input\_audio\_buffer.committed\` from the server.

* `conversation.item.create` — Create a new conversation item. Can be a user text message, an assistant text message for history seeding, a function call for seeding tool-use history, or a function call output.

* `input_audio_buffer.clear` — Clear the input audio buffer. Use this to discard any pending audio data without committing it.

* `conversation.item.delete` — Delete a conversation item by ID. The server confirms deletion with a \`conversation.item.deleted\` event.

* `conversation.item.truncate` — Truncate a previous assistant audio message item. Removes audio and transcript content after the specified duration, keeping only the content up to that point. The server confirms with a \`conversation.item.truncated\` event.

* `response.create` — Request the server to create a new assistant response. This is handled automatically when using server-side VAD.

* `response.cancel` — Cancel an in-progress response. In VAD mode, interruptions are automatic — use this for manual cancel in non-VAD mode.

### Server Messages

* `session.created` — Sent automatically on WebSocket connection. Contains the session configuration.

* `conversation.created` — The first message on connection. Notifies the client that a conversation session has been created.

* `session.updated` — Acknowledges the client's session.update message that the session has been configured.

* `input_audio_buffer.speech_started` — Notifies that the server's VAD detected the start of speech. Only available with server\_vad turn detection.

* `input_audio_buffer.speech_stopped` — Notifies that the server's VAD detected the end of speech. Only available with server\_vad turn detection.

* `input_audio_buffer.committed` — Input audio buffer has been committed as a user message.

* `input_audio_buffer.timeout_triggered` — The \`turn\_detection.idle\_timeout\_ms\` idle timer fired: no user speech was detected for the configured duration after the assistant finished responding. The server commits a silent user turn and generates a proactive check-in.

* `input_audio_buffer.cleared` — Confirms the input audio buffer has been cleared.

* `conversation.item.deleted` — Confirms a conversation item has been deleted.

* `conversation.item.added` — A new user or assistant message has been added to the conversation history.

* `conversation.item.truncated` — Confirms that a conversation item has been truncated. Sent in response to a \`conversation.item.truncate\` client event.

* `conversation.item.input_audio_transcription.completed` — Audio transcription for the user's input has been completed.

* `conversation.item.input_audio_transcription.updated` — Streaming transcription update for the user's audio input. Emitted as the user speaks, providing the cumulative transcript so far before the final \`completed\` event. Note that this is the cumulative transcript which may have corrections to previous updated transcripts — this is different from a transcript delta. Only emitted when \`audio.input.transcription.model\` is set to \`grok-transcribe\` in the session configuration. Useful for displaying live captions.

* `input_audio_buffer.dtmf_event_received` — A DTMF tone (phone keypress) was detected on a SIP session. SIP only — not emitted on direct WebSocket connections. Digits are buffered server-side and flushed as a text message to the model on \`#\` key, 2.5s idle, or when the user begins speaking.

* `response.created` — A new assistant response turn is in progress. Audio deltas from this turn share the same response\_id.

* `response.output_item.added` — A new assistant response item is added to the message history.

* `response.output_item.done` — An output item is complete.

* `response.content_part.added` — A content part starts within an output item.

* `response.content_part.done` — A content part finishes.

* `response.output_audio_transcript.delta` — Streaming text transcript delta of the assistant's audio response.

* `response.output_audio_transcript.done` — The audio transcript for this assistant turn has finished generating.

* `response.output_audio.delta` — Streaming base64-encoded audio delta of the assistant's response.

* `response.output_audio.done` — Audio generation for this assistant turn has finished.

* `response.text.delta` — Text-mode output delta (when using text modality).

* `response.output_text.delta` — Text-mode output delta using the OpenAI GA event name. Functionally identical to \`response.text.delta\`. Clients should handle both event names for maximum compatibility.

* `response.function_call_arguments.delta` — Streaming function call arguments.

* `response.function_call_arguments.done` — A function call has been triggered with complete arguments. Your code should execute the function and return results via \`conversation.item.create\` with type \`function\_call\_output\`.

* `mcp_list_tools.in_progress` — MCP tool discovery has started.

* `mcp_list_tools.completed` — MCP tool discovery succeeded.

* `mcp_list_tools.failed` — MCP tool discovery failed.

* `response.mcp_call_arguments.delta` — MCP call arguments streaming.

* `response.mcp_call_arguments.done` — MCP call arguments finalized.

* `response.mcp_call.in_progress` — MCP server HTTP call starting.

* `response.mcp_call.completed` — MCP tool execution succeeded.

* `response.mcp_call.failed` — MCP tool execution failed.

* `response.done` — The assistant's response is completed. Sent after all audio and transcript deltas. Ready for the client to add a new conversation item.

* `error` — Sent when an error occurs. Contains error code and message. Most errors are recoverable and the session stays open.

### Example Message Flow

1. `session.created` (server)

2. `conversation.created` (server)

3. `session.update` (client)

4. `session.updated` (server)

5. `conversation.item.create` (client)

6. `conversation.item.added` (server)

7. `response.create` (client)

8. `response.created` (server)

9. `response.output_item.added` (server)

10. `response.content_part.added` (server)

11. `response.output_audio.delta` (server)

12. `response.output_audio_transcript.delta` (server)

13. `response.output_audio.done` (server)

14. `response.output_audio_transcript.done` (server)

15. `response.content_part.done` (server)

16. `response.output_item.done` (server)

17. `response.done` (server)

***

## POST /v1/realtime/calls/\{call\_id}/refer

API endpoint for POST requests to /v1/realtime/calls/\{call\_id}/refer.

```
Method: POST
Path: /v1/realtime/calls/{call_id}/refer
```

***

## POST /v1/realtime/calls/\{call\_id}/hangup

API endpoint for POST requests to /v1/realtime/calls/\{call\_id}/hangup.

```
Method: POST
Path: /v1/realtime/calls/{call_id}/hangup
```

***

## POST /v1/tts

API endpoint for POST requests to /v1/tts.

```
Method: POST
Path: /v1/tts
```

***

## Text to speech - Streaming

WebSocket endpoint: `wss://api.x.ai/v1/tts`

Bidirectional streaming text-to-speech via WebSocket. Send text incrementally and receive audio chunks in real time. Shares the \`/v1/tts\` path with the batch POST endpoint — a GET with \`Upgrade: websocket\` activates streaming mode. Configuration is done via query parameters at connection time. Supports multi-utterance: after \`audio.done\`, send another stream of \`text.delta\` messages on the same connection.

Full schemas and examples: [`/tts-streaming.ws.json`](https://docs.x.ai/tts-streaming.ws.json)

### Query Parameters

* `voice` (string, optional, default: eve) — Voice identifier. Use a built-in voice from \`GET /v1/tts/voices\` (e.g. \`eve\`, \`ara\`) or a custom voice ID.

* `language` (string, required) — BCP-47 language code (e.g. \`en\`, \`zh\`, \`pt-BR\`) or \`auto\` for automatic language detection. Case-insensitive.

* `codec` (string, optional, default: mp3) — Audio codec for the output.

* `sample_rate` (integer, optional, default: 24000) — Sample rate in Hz.

* `bit_rate` (integer, optional, default: 128000) — Bit rate in bps. Only applies when \`codec\` is \`mp3\`.

* `optimize_streaming_latency` (integer, optional, default: 0) — Latency optimization level. \`0\` (default): No optimization — best audio quality. \`1\`: Reduced first-chunk size for lower time-to-first-audio, with minor quality tradeoff at chunk boundaries.

* `speed` (number, optional, default: 1.0) — Speech speed multiplier. \`1.0\` is normal speed. Values below \`1.0\` slow down speech, values above \`1.0\` speed it up. Range: \`0.7\` to \`1.5\`.

* `text_normalization` (boolean, optional, default: false) — Enable text normalization before synthesis. When enabled, the model normalizes written-form text (e.g. numbers, abbreviations, symbols) into spoken-form before generating audio.

* `with_timestamps` (boolean, optional, default: false) — Return per-character timing metadata on each \`audio.delta\` event. When \`true\`, every \`audio.delta\` carries \`audio\_timestamps\`.

### Client Messages

* `text.delta` — Send a chunk of text to be synthesized. Text is processed incrementally — audio generation begins as soon as enough text is buffered. Individual deltas are capped at 15,000 characters.

* `text.done` — Signal that all text for this utterance has been sent. The server will finish generating audio and send \`audio.done\`. After receiving \`audio.done\`, you can start a new utterance with another \`text.delta\`.

### Server Messages

* `audio.delta` — A chunk of base64-encoded audio data. Decode and append to your audio buffer or pipe directly to playback. The format matches the \`codec\` and \`sample\_rate\` specified in the query parameters. When the connection was opened with \`with\_timestamps=true\`, the event also carries \`audio\_timestamps\` and \`audio\_duration\` for the characters that fall inside this chunk.

* `audio.done` — Audio generation for this utterance is complete. The connection remains open for multi-utterance — send another \`text.delta\` to start a new synthesis, or close the connection.

* `error` — An error occurred during synthesis. The connection may be closed after this message.

### Example Message Flow

1. `text.delta` (client)

2. `text.delta` (client)

3. `text.done` (client)

4. `audio.delta` (server)

5. `audio.delta` (server)

6. `audio.delta` (server)

7. `audio.done` (server)

***

## GET /v1/tts/voices

API endpoint for GET requests to /v1/tts/voices.

```
Method: GET
Path: /v1/tts/voices
```

***

## GET /v1/tts/voices/\{voice\_id}

API endpoint for GET requests to /v1/tts/voices/\{voice\_id}.

```
Method: GET
Path: /v1/tts/voices/{voice_id}
```

***

## POST /v1/stt

API endpoint for POST requests to /v1/stt.

```
Method: POST
Path: /v1/stt
```

***

## Speech to text - Streaming

WebSocket endpoint: `wss://api.x.ai/v1/stt`

Real-time streaming speech-to-text via WebSocket. Stream raw audio as binary frames and receive JSON transcript events as the audio is processed. Configuration is done via query parameters at connection time.

Full schemas and examples: [`/stt-streaming.ws.json`](https://docs.x.ai/stt-streaming.ws.json)

### Query Parameters

* `sample_rate` (integer, optional, default: 16000) — Audio sample rate in Hz. Supported values: \`8000\`, \`16000\`, \`22050\`, \`24000\`, \`44100\`, \`48000\`.

* `encoding` (string, optional, default: pcm) — Audio encoding format. \`pcm\` — signed 16-bit little-endian (2 bytes/sample). \`mulaw\` — G.711 µ-law (1 byte/sample). \`alaw\` — G.711 A-law (1 byte/sample).

* `interim_results` (boolean, optional, default: false) — When \`true\`, the server emits partial transcript events (\`is\_final=false\`) approximately every 500 ms while audio is being processed. When \`false\` (default), only finalized results are sent.

* `endpointing` (integer, optional, default: 10) — Silence duration in milliseconds before the server fires a \`speech\_final=true\` event, indicating the speaker stopped talking. Range: 0–5000. Set to \`0\` for no delay (fire on any VAD silence boundary). Default: 10ms.

* `language` (string, optional, default: ) — Language code (e.g. \`en\`, \`fr\`, \`de\`, \`ja\`). When set, enables Inverse Text Normalization — spoken-form numbers, currencies, and units are converted to their written form.

* `multichannel` (boolean, optional, default: false) — When \`true\`, enables per-channel transcription for interleaved multichannel audio. Requires \`channels\` to be set to ≥ 2.

* `channels` (integer, optional, default: 1) — Number of interleaved audio channels. Required when \`multichannel=true\`. Min: 2, Max: 8.

* `diarize` (boolean, optional, default: false) — When \`true\`, enables speaker diarization. Words in \`transcript.partial\` and \`transcript.done\` events include a \`speaker\` field (integer) identifying the detected speaker.

* `keyterm` (string (repeatable), optional) — A key term to bias transcription toward (e.g. product names, proper nouns). Repeat the parameter for each term (e.g. \`keyterm=Understand+The+Universe\`). Max 100 terms, each up to 50 characters.

* `filler_words` (boolean, optional, default: false) — When \`true\`, filler words (e.g. \`uh\`, \`um\`, \`er\`) are included in the transcript. When \`false\` (default), filler words are automatically removed from the transcript text and the \`words\` array.

* `smart_turn` (number, optional) — Enable Smart Turn end-of-turn detection. Set to a confidence threshold between \`0.0\` and \`1.0\`. When the model's end-of-turn probability exceeds this threshold at a VAD silence boundary, \`speech\_final\` fires immediately. When confidence is below the threshold, \`speech\_final\` is suppressed and the event is demoted to \`chunk\_final\`. Every \`transcript.partial\` event includes an \`end\_of\_turn\_confidence\` field (0.0–1.0) when Smart Turn is enabled. Example: \`smart\_turn=0.7\`.

* `smart_turn_timeout` (integer, optional) — Maximum silence duration in milliseconds before forcing \`speech\_final\`, even when the Smart Turn model predicts the speaker hasn't finished. Acts as a safety net to prevent sessions from hanging during extended silence. Only applies when \`smart\_turn\` is enabled. Range: 1–5000. Example: \`smart\_turn\_timeout=3000\`.

* `vad_threshold` (number, optional, default: 0.08) — Speech-probability threshold for the voice-activity gate (0.0–1.0). Audio in chunks scoring below the threshold is treated as non-speech and skipped for transcription. Lower values transcribe quieter or noisier speech (e.g. narrowband telephony) but may produce spurious text for background noise; \`0\` disables the gate entirely. Does not affect endpointing or \`speech\_final\` timing. Default: \`0.08\`.

### Client Messages

* `Binary frame (audio)` — Send raw audio as binary WebSocket frames in the encoding specified by the \`encoding\` query parameter. Audio should be streamed in real-time-paced chunks (e.g. 100 ms at a time). No base64 encoding — send raw bytes directly.

* `finalize` — Force the current utterance to finalize as \`speech\_final\` immediately, without waiting for VAD endpointing or Smart Turn. The session stays open so you can continue streaming audio. Accepts \`finalize\` or \`Finalize\` as the type value. When \`multichannel=true\`, optional \`channel\` (0-based) limits the finalize to that channel; omit \`channel\` to finalize every channel.

* `audio.done` — Signal that all audio has been sent. The server flushes any remaining buffered audio, emits final transcript events, and sends a \`transcript.done\` event. The connection closes after \`transcript.done\`.

### Server Messages

* `transcript.created` — Sent immediately after the WebSocket connection is established and the server is ready to receive audio. \*\*Wait for this event before sending audio\*\* — the server needs to initialize its ASR backend.

* `transcript.partial` — A transcript result for a portion of the audio stream. Two boolean fields convey state: interim (\`is\_final=false\`) means text may still change, chunk final (\`is\_final=true\`, \`speech\_final=false\`) means the chunk is locked, and utterance final (\`is\_final=true\`, \`speech\_final=true\`) means the speaker stopped talking.

* `transcript.done` — Final transcript after \`audio.done\`. \`duration\` always present. One per channel when \`multichannel=true\`. Connection closes after this event.

* `error` — An error occurred during the session. Most errors (pipeline failures, stream timeouts) close the connection. Only client message parse errors keep the connection open.

### Example Message Flow

1. `transcript.created` (server)

2. `Binary frame (audio)` (client)

3. `Binary frame (audio)` (client)

4. `transcript.partial` (server)

5. `Binary frame (audio)` (client)

6. `transcript.partial` (server)

7. `Binary frame (audio)` (client)

8. `transcript.partial` (server)

9. `audio.done` (client)

10. `transcript.done` (server)

***

## POST /v1/custom-voices

API endpoint for POST requests to /v1/custom-voices.

```
Method: POST
Path: /v1/custom-voices
```

***

## GET /v1/custom-voices

API endpoint for GET requests to /v1/custom-voices.

```
Method: GET
Path: /v1/custom-voices
```

***

## GET /v1/custom-voices/\{voice\_id}

API endpoint for GET requests to /v1/custom-voices/\{voice\_id}.

```
Method: GET
Path: /v1/custom-voices/{voice_id}
```

***

## PATCH /v1/custom-voices/\{voice\_id}

API endpoint for PATCH requests to /v1/custom-voices/\{voice\_id}.

```
Method: PATCH
Path: /v1/custom-voices/{voice_id}
```

***

## DELETE /v1/custom-voices/\{voice\_id}

API endpoint for DELETE requests to /v1/custom-voices/\{voice\_id}.

```
Method: DELETE
Path: /v1/custom-voices/{voice_id}
```

***

## GET /v1/custom-voices/\{voice\_id}/audio

API endpoint for GET requests to /v1/custom-voices/\{voice\_id}/audio.

```
Method: GET
Path: /v1/custom-voices/{voice_id}/audio
```