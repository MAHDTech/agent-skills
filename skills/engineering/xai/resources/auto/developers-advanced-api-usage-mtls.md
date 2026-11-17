#### Advanced API Usage

# mTLS Authentication

Mutual TLS (mTLS) lets you lock down API access so that only machines presenting a valid client certificate can make requests on behalf of your team. This is well suited to enterprise environments where API traffic flows through your own gateways and you need cryptographic proof that each request originates from an authorized system.

> [!TIP]
>
> mTLS is an enterprise feature. Contact [support@x.ai](mailto:support@x.ai?subject=mTLS%20Integration%20Request) to enable it for your team.

## Why Use mTLS?

* **Zero-trust security** — Every request must prove its identity with a certificate, not just an API key
* **Gateway-friendly** — Works naturally when your traffic routes through corporate API gateways, proxies, or service meshes
* **No code changes** — Once enabled, you only need to attach your client certificate to requests. All existing API features (models, tools, streaming) work identically

## Quick Start

### 1. Get set up

Contact [support@x.ai](mailto:support@x.ai) with:

* Your team ID (found in the [xAI Console](https://console.x.ai?utm_source=docs\&utm_medium=referral\&utm_campaign=developers-advanced-api-usage-mtls\&utm_content=console-home))
* Your CA certificate in PEM format
* The Common Name (CN) from the client certificates your systems will use

We'll configure your team and confirm when mTLS is active.

### 2. Point to the mTLS endpoint

Use `https://mtls.api.x.ai` instead of `https://api.x.ai`. This is the only change required. All API paths (`/v1/chat/completions`, `/v1/responses`, `/v1/embeddings`, etc.) work the same way.

### 3. Attach your client certificate

Include your client certificate and private key with every request. Here are examples:

```pythonOpenAISDK
import os
import httpx
from openai import OpenAI

# Attach your client certificate to the HTTP transport
http_client = httpx.Client(
    cert=("/path/to/client-cert.pem", "/path/to/client-key.pem")
)

client = OpenAI(
    api_key=os.getenv("XAI_API_KEY"),
    base_url="https://mtls.api.x.ai/v1",
    http_client=http_client,
)

completion = client.chat.completions.create(
    model="grok-4.7",
    messages=[
        {"role": "user", "content": "Hello, world!"}
    ]
)
print(completion.choices[0].message.content)
```

```bash
curl https://mtls.api.x.ai/v1/chat/completions \\
  --cert /path/to/client-cert.pem \\
  --key /path/to/client-key.pem \\
  -H "Content-Type: application/json" \\
  -H "Authorization: Bearer $XAI_API_KEY" \\
  -d '{
    "messages": [
      {
        "role": "user",
        "content": "Hello, world!"
      }
    ],
    "model": "grok-4.7",
    "stream": false
  }'
```

```javascriptOpenAISDK
import OpenAI from 'openai';
import https from 'https';
import fs from 'fs';

const client = new OpenAI({
    apiKey: process.env.XAI_API_KEY,
    baseURL: 'https://mtls.api.x.ai/v1',
    httpAgent: new https.Agent({
        cert: fs.readFileSync('/path/to/client-cert.pem'),
        key: fs.readFileSync('/path/to/client-key.pem'),
    }),
});

const completion = await client.chat.completions.create({
    model: 'grok-4.7',
    messages: [
        { role: 'user', content: 'Hello, world!' }
    ],
});

console.log(completion.choices[0].message.content);
```

> [!NOTE]
>
> You still need a valid API key on every request. mTLS is an **additional** layer of security, not a replacement for API key authentication.

## How Authentication Works

When mTLS is enabled for your team, every request goes through two checks:

1. **Certificate verification** — Your client certificate is validated against the CA certificate you provided during setup. Requests without a valid certificate are rejected with `403 Forbidden`.
2. **API key verification** — Your API key is checked as usual. Invalid or missing keys are rejected with `401 Unauthorized`.

Both checks must pass for the request to proceed. All other behavior (rate limits, billing, model access) is identical to the standard endpoint.

## Rotating Certificates

mTLS is designed so you can rotate certificates without downtime:

| Scenario | What to Do |
|----------|------------|
| **Renewing a client certificate** (same CA, same CN) | Nothing. Just start using the new certificate. |
| **Updating your CA** (e.g., new intermediate) | Contact [support@x.ai](mailto:support@x.ai) to upload the updated CA bundle. |
| **Switching to a different CA entirely** | Contact [support@x.ai](mailto:support@x.ai) to register the new CA certificate. |

## FAQ

### Do I have to use the mTLS endpoint?

If mTLS is enabled as **required** for your team, yes. Requests to `api.x.ai` will be rejected because no client certificate is presented. If you need some API keys to work without mTLS, contact support to discuss your configuration.

### Can I use regional endpoints with mTLS?

mTLS is currently available on the global `mtls.api.x.ai` endpoint. If you need mTLS with [regional endpoints](/developers/advanced-api-usage/regions), contact [support@x.ai](mailto:support@x.ai).

### What certificate format do I need?

X.509 certificates in PEM format. Both the CA certificate (provided during setup) and client certificates must be PEM-encoded.

### Is mTLS configured per API key or per team?

mTLS is configured at the **team level**. All API keys in your team share the same mTLS configuration.

### How do I test my setup?

After setup, make a simple request with your certificate:

```bash
curl -v https://mtls.api.x.ai/v1/api-key \\
  --cert /path/to/client-cert.pem \\
  --key /path/to/client-key.pem \\
  -H "Authorization: Bearer $XAI_API_KEY"
```

A successful response confirms both your certificate and API key are working. If you see `403 Forbidden`, check that your certificate is signed by the CA you provided to SpaceXSI.

