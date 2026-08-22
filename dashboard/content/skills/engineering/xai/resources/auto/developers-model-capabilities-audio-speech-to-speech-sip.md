+++
title = "developers-model-capabilities-audio-speech-to-speech-sip"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Speech to Speech API

# SIP Phone Calls

SIP lets you route PSTN, contact-center, or PBX calls into a Speech to Speech API session.

### 1. Register the phone number

Create a Direct SIP phone number and include the webhook details that should receive incoming-call events. Use `origin: "byo_trunk"` for a customer-owned number. Provisioning xAI phone numbers via API is not supported. xAI returns the webhook signing secret in the response.

Choose one SIP authentication method.

The response includes a signing secret after you register the phone number. Store it securely; xAI returns it only once.

Configure your carrier or PBX to route calls to:



If you provide `allowed_addresses`, make sure the list contains your provider's SIP signaling CIDR ranges. If you provide SIP digest credentials, configure your carrier with the same username and password; xAI never returns the password after creation.

### 2. Handle the incoming-call webhook

When a caller dials the number, xAI sends a signed `realtime.call.incoming` webhook to the webhook URL. Verify the `webhook-id`, `webhook-timestamp`, and `webhook-signature` headers using the signing secret returned after you register the phone number, then read `data.call_id` from the payload.

The webhook has this shape:

```json
{
  "object": "event",
  "id": "evt_123",
  "type": "realtime.call.incoming",
  "created_at": 1750000000,
  "data": {
    "call_id": "00000000-0000-0000-0000-000000000000",
    "sip_headers": [
      { "name": "From", "value": "+14155550100" },
      { "name": "To", "value": "+18005550199" }
    ],
    "metadata": {}
  }
}
```

### 3. Join the call over WebSocket

Open `wss://api.x.ai/v1/realtime?call_id={call_id}` with your xAI API key. Then send `session.update` to configure the voice agent for this call, followed by `response.create` when the agent should begin speaking.

After connecting, the WebSocket behaves like any other Speech to Speech API session. The SIP caller's audio is bridged into the session, and assistant audio is played back to the caller.

```python customLanguage="pythonWithoutSDK"
import asyncio
import json
import os
import websockets

async def handle_sip_call(call_id: str):
    async with websockets.connect(
        f"wss://api.x.ai/v1/realtime?call_id={call_id}",
        additional_headers={"Authorization": f"Bearer {os.environ['XAI_API_KEY']}"},
    ) as ws:
        await ws.send(json.dumps({
            "type": "session.update",
            "session": {
                "voice": "eve",
                "instructions": "You are a helpful phone support agent.",
                "turn_detection": {"type": "server_vad"},
            },
        }))
        await ws.send(json.dumps({"type": "response.create"}))

        async for msg in ws:
            event = json.loads(msg)
            print(event["type"])

asyncio.run(handle_sip_call("00000000-0000-0000-0000-000000000000"))
```

```javascript customLanguage="javascriptWithoutSDK"
import WebSocket from "ws";

const callId = "00000000-0000-0000-0000-000000000000";
const ws = new WebSocket(`wss://api.x.ai/v1/realtime?call_id=${callId}`, {
  headers: { Authorization: `Bearer ${process.env.XAI_API_KEY}` },
});

ws.on("open", () => {
  ws.send(JSON.stringify({
    type: "session.update",
    session: {
      voice: "eve",
      instructions: "You are a helpful phone support agent.",
      turn_detection: { type: "server_vad" },
    },
  }));
  ws.send(JSON.stringify({ type: "response.create" }));
});

ws.on("message", data => {
  const event = JSON.parse(data.toString());
  console.log(event.type);
});
```

## Call control

Use `refer` to transfer the caller to another PSTN or SIP destination:

```bash customLanguage="bash"
curl -X POST "https://api.x.ai/v1/realtime/calls/$CALL_ID/refer" \
  -H "Authorization: Bearer $XAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"target_uri": "sip:agent@example.com"}'
```

Use `hangup` when your application should end the call:

```bash customLanguage="bash"
curl -X POST "https://api.x.ai/v1/realtime/calls/$CALL_ID/hangup" \
  -H "Authorization: Bearer $XAI_API_KEY"
```

## DTMF phone keypresses

When using the Speech to Speech API over SIP, phone keypresses (DTMF tones) are automatically buffered and flushed to the model as text input. The client receives `input_audio_buffer.dtmf_event_received` events as an audit trail of each keypress.

### Flush triggers

Buffered digits are submitted to the model when any of the following occurs:

* The user presses `#` (submit key)
* 2.5 seconds of idle time after the last keypress
* The user begins speaking (preempts the digit buffer)

### Audit event

Each keypress is reported to the client WebSocket:

```json customLanguage="json"
{
  "type": "input_audio_buffer.dtmf_event_received",
  "event": "5",
  "received_at": 1730000000
}
```

> [!NOTE]
>
> DTMF is only available on SIP sessions — it is not emitted on direct WebSocket connections.

## Telephony providers

In every provider, the destination is the xAI SIP URI for your registered number:



Replace `{number}` with your Direct SIP phone number. If you configured `allowed_addresses` when registering the number, include your provider's SIP signaling CIDR ranges.

### Twilio

1. In the Twilio Console, go to **Voice** → **Elastic SIP Trunking** and create a trunk.
2. Open the trunk's **Origination** settings and add this origination URI: `sip:{number}@sip.voice.x.ai;transport=tls`.
3. Assign a Twilio phone number to the trunk, or purchase a new number and attach it.
4. If your application transfers calls mid-session, enable call transfer on the trunk.

### Telnyx

1. In the Telnyx Portal, go to **Voice Suite** → **SIP Trunking** and create an FQDN SIP Connection.
2. In **Authentication and Routing**, add `sip.voice.x.ai` as the primary FQDN on port `5060` with record type `A`.
3. In **Inbound settings**, set the destination number format to **E.164**.
4. Enable at least one supported codec: G.711 μ-law, G.711 A-law, or G.722.
5. Assign a phone number to the SIP Connection.

### Plivo

1. In the Plivo Console, go to **SIP Trunking** and create a SIP trunk.
2. Choose **Inbound**, then create a new URI with FQDN `sip.voice.x.ai`.
3. Link an existing phone number to the trunk, or buy a new number and attach it.

### Bring Your Own SIP Provider

1. In your carrier, contact center, or PBX, create an outbound route or SIP trunk.
2. Set the destination to `sip:{number}@sip.voice.x.ai;transport=tls`.
