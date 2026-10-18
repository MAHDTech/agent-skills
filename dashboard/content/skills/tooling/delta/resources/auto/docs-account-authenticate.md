+++
title = "docs-account-authenticate"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
+++

---
title: "Sign In"
url: "https://delta.dev/docs/account/authenticate"
description: "How to sign in to and out of Delta and manage your account email address."
---

1.  Account & Billing
2.  Sign In

# Sign In

How to sign in to and out of Delta and manage your account email address.

On this page

1.  [Signing in](#signing-in)
2.  [Signing out](#signing-out)
3.  [Sign in from your terminal](#sign-in-from-your-terminal)
4.  [Create an organization](#create-an-organization)
5.  [Email addresses](#email-addresses)

You must sign in with a Zed account to use Delta. Sign in or create an account on the [Zed dashboard](https://dashboard.zed.dev/delta-beta), then accept the Delta Early Access Agreement. You can use Delta as soon as you accept.

## [Signing in](#signing-in)

Delta uses GitHub’s OAuth flow to authenticate you. It requests the `read:user` and `user:email` GitHub scopes to read your profile and email addresses.

1.  Open Delta. On first launch it shows a sign-in screen; later, select **Delta > Sign In** from the application menu.
2.  Your default web browser opens the Zed sign-in page.
3.  Authenticate with your GitHub account when prompted.
4.  After authentication, your browser displays a confirmation and Delta signs you in automatically.

Delta stores your session in your operating system keychain.

NOTE

If you’re behind a corporate firewall, ensure that connections to `zed.dev`, `dashboard.zed.dev`, `cloud.zed.dev`, and `delta.zed.dev` are allowed.

## [Signing out](#signing-out)

To sign out, select **Delta > Sign Out** from the application menu.

## [Sign in from your terminal](#sign-in-from-your-terminal)

After [installing the command-line tool](/docs/installation), use `delta auth` to check or change the desktop app’s account. These commands use the same session and saved credentials as the app.

Check the current account:

```
delta auth status
```

Sign in through your browser:

```
delta auth login
```

The command opens the sign-in page and waits for you to finish. Complete sign-in in a browser on the same machine as Delta.

Sign out and delete the app’s saved credentials:

```
delta auth logout
```

## [Create an organization](#create-an-organization)

Open the user menu and choose **Create Organization** to open the Zed dashboard’s organization-creation form. You can do this after onboarding if you skipped creating a team during setup. You can also run **Create Organization** from the desktop command palette.

To invite teammates later, select your team organization in the desktop user menu and choose **Invite Members…**. This requires sign-in and an organization that supports email invitations; it is unavailable for personal organizations.

## [Email addresses](#email-addresses)

Your Zed account uses the email address provided by GitHub OAuth. If your GitHub profile has a public email address, Zed uses it. Otherwise, Zed uses your primary GitHub email address. To sync changes you make to your email address on GitHub, sign in to [zed.dev](https://zed.dev).

Stripe uses your Zed account’s email address when you start a subscription. Changing your Zed account email address does not currently update the address used by Stripe. See [Updating Billing Information](https://zed.dev/docs/account/billing#updating-billing-info) to change that address.
