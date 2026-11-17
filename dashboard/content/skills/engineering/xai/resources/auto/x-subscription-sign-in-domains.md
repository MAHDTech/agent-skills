+++
title = "x-subscription-sign-in-domains"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "xai"
+++

#### Sign-in

# Which domains does Cursor sign-in need?

On October 30, 2026, Cursor sign-in will move from `authenticator.cursor.sh` to `accounts.spacex.ai` and `accounts.x.ai`. Until then, `authenticator.cursor.sh` remains the current sign-in page. After the transition, it will no longer be the primary sign-in page or fallback. Your Cursor app, site, and SSO tiles will keep working, and existing sessions will stay signed in.

If your company uses a firewall or proxy allowlist (for example, Zscaler), add the domains below before the transition. If `accounts.spacex.ai` or `accounts.x.ai` is blocked after the transition, you and your users will not be able to log in.

Add these domains before October 30, 2026, so your users can reach the new sign-in page after the transition.

If your team has bookmarks or internal docs that point to `authenticator.cursor.sh`, update them to `accounts.spacex.ai` and `accounts.x.ai`.

## Prefer wildcards

Use wildcards when you can, so you do not have to update the list later:

* `*.spacex.ai` (covers `accounts.spacex.ai` and related auth hosts)
* `*.x.ai` (covers `accounts.x.ai`, `auth.x.ai`, and related auth hosts)
* `*.grok.com` (covers `auth.grok.com` and `grok.com`)
* `*.grokusercontent.com`
* `*.grokipedia.com`

## Minimum hosts

If your policy will not allow wildcards, allow at least:

* `accounts.x.ai`
* `accounts.spacex.ai`
* `auth.x.ai`
* `auth.grok.com`
* `auth.grokusercontent.com`
* `auth.grokipedia.com`
* `grok.com`

## Still required for Cursor itself

These are unchanged:

* Keep `*.cursor.sh` allowlisted for Cursor services through the transition. This covers `authenticator.cursor.sh`.
* `*.cursor-cdn.com`
* `*.cursorapi.com`
* `*.cursorvm.com`

## Notes for IT admins

* Existing Cursor sessions stay signed in. This is not a forced logout.
* SSO IdP configuration, including Okta tiles and ACS URLs, does not need to change.
* Cursor app, website, and API hosts are not changing.

If login fails after the transition and your network uses an allowlist, have IT add the domains above, then retry from `accounts.spacex.ai`, `accounts.x.ai`, or your existing IdP.


