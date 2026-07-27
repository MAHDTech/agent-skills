- side_navigation
- Antigravity IDE
  \>
- Browser
  \>
- Allowlist / Denylist

# Allowlist / Denylist[link](#allowlist-denylist)

The browser uses a two-layer security system to control which URLs can
be accessed:

- **Denylist** - Deny dangerous/malicious URLs
- **Allowlist** - Explicitly allow trusted URLs

## How It Works[link](#how-it-works)

### Denylist[link](#denylist-2)

The denylist is maintained and enforced using the Google Superroots’s
BadUrlsChecker service (See documentation). When the browser attempts to
navigate to a URL, the hostname is checked against the server-side
denylist via RPC.

**NOTE:** If the server is unavailable, access is denied by default.

### Allowlist[link](#allowlist-2)

The allowlist is a local text file that you can edit to explicitly trust
specific URLs.

![Allowlist](https://antigravity.google/assets/image/docs/browser-allowlist.png)

The allowlist is initialized with just localhost, and can be edited at
anytime.

When the browser attempts to navigate to a non-allowlisted URL, it will
prompt you with an “always allow” button, which if clicked will add the
URL to the allowlist and enable the browser to open and interact with
the web page. An example situation is shown below:

![Always Allow](https://antigravity.google/assets/image/docs/always-allow-url.png)

You can also add/remove URLS from the allowlist manually. However, the
denylist always takes precedence: you cannot allowlist a URL that
appears on the denylist.

On this Page
