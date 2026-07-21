#### Connectors

# Custom MCP Server Tunneling

When you add a [custom MCP connector](https://docs.x.ai/grok/connectors#custom-mcp-connectors), Grok's servers need to reach your MCP server over the public internet. Servers running on `localhost` or a private network address (such as `127.0.0.1`, `10.x.x.x`, `172.16.x.x`, or `192.168.x.x`) are not directly reachable, and Grok will reject these URLs.

A **tunneling service** solves this by exposing your local server through a public URL that Grok can connect to.

## How tunneling works

A tunnel creates a secure, temporary public URL that forwards traffic to a port on your machine. When Grok calls your MCP server, the request travels through the tunnel provider's infrastructure and arrives at your local process as if it were a local request.

```text
Grok ──► https://your-tunnel.example.com ──► tunnel provider ──► localhost:3001
```

Your MCP server code requires no changes; only the URL you provide to Grok differs.

## Setting up a tunnel

Several tunneling services work well with MCP servers. Below are two popular options.

### ngrok

[ngrok](https://ngrok.com) provides stable URLs and a dashboard for inspecting traffic. ngrok is a third-party service and is not affiliated with xAI or Grok.

1. Install ngrok and authenticate:

```bash customLanguage="bash"
# macOS
brew install ngrok

# Windows
winget install ngrok -s msstore

# Authenticate (free account required)
ngrok config add-authtoken YOUR_AUTH_TOKEN
```

2. Start a tunnel pointing to your MCP server's port:

```bash customLanguage="bash"
ngrok http 3001
```

3. Copy the `Forwarding` URL from the output (e.g., `https://a1b2c3d4.ngrok-free.app`) and use it as the server URL in Grok's custom connector dialog.

### Cloudflare Tunnel

[Cloudflare Tunnel](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/) is a free alternative that does not require account signup for quick tunnels.

```bash customLanguage="bash"
# macOS
brew install cloudflared

# Windows
winget install --id Cloudflare.cloudflared

# Start a quick tunnel
cloudflared tunnel --url http://localhost:3001
```

Copy the generated `*.trycloudflare.com` URL and use it as the server URL in Grok.

> [!CAUTION]
>
> &#x20;Cloudflare quick tunnels do not support Server-Sent Events (SSE). If your MCP server uses the SSE transport, use ngrok instead. Servers using the newer Streamable HTTP transport work fine with Cloudflare.

## Things to keep in mind

* **Tunnels are temporary.** Most free-tier tunnel URLs change each time you restart the tunnel. If you restart, you will need to remove the old connector in Grok and add a new one with the updated URL.
* **Keep your MCP server running.** Grok calls your server on demand during conversations. If the tunnel or the local server is stopped, tool calls will fail.
* **Authentication still applies.** If your MCP server requires OAuth or API keys, you will still complete that flow in Grok after providing the tunnel URL. The tunnel only handles network reachability.