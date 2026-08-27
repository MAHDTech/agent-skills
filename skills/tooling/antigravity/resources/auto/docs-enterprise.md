Markdownkeyboard_arrow_down

content_copyCopy Markdown

open_in_newView Markdown

# Antigravity in Gemini Enterprise

To deploy Google Antigravity using models hosted directly within your
organization’s Google Cloud infrastructure, you can integrate with the
Gemini Enterprise app and Gemini Enterprise Agent Platform. Every
session runs under Google Cloud’s enterprise security controls, data
residency guarantees, and the Google Cloud Terms of Service.

Supported products: [Antigravity 2.0](https://antigravity.google/product/antigravity-2)
[Antigravity CLI](https://antigravity.google/product/antigravity-cli) [Visual Studio
Code](https://antigravity.google/docs/ide/extensions/vscode) [Visual Studio
(Preview)](https://antigravity.google/docs/ide/extensions/visual-studio) [JetBrains
(Preview)](https://antigravity.google/docs/ide/extensions/jetbrains) [Zed
(Preview)](https://antigravity.google/docs/ide/extensions/zed) [Xcode
(Preview)](https://antigravity.google/docs/ide/extensions/xcode)

## Overview & Key Benefits

You can use Antigravity in two ways:

- **Gemini Enterprise Agent Platform** - Connect directly to Agent
  Platform API to use Antigravity with consumption-based billing.
- **Gemini Enterprise license** - Connect with your Gemini Enterprise
  license to get access to included quotas, managed overages as well as
  advanced administrative controls.

By connecting Google Antigravity to your Google Cloud project, your
organization gains:

Enterprise Governance

Operates under your existing Google Cloud Terms of Service with
centralized administrative controls.

Data Residency & Security

Satisfies private networking (VPC Service Controls) and regional data
residency constraints. Enterprise prompts, responses, code, and
telemetry are never stored outside your private environments.

## Administrator Setup Guide

### Gemini Enterprise App Setup

To set up Gemini Enterprise subscriptions, follow the official Google
Cloud onboarding guide.

[Gemini Enterprise App
Documentation](https://docs.cloud.google.com/gemini/enterprise/docs/ai-developer-tools-overview)

### Gemini Enterprise Agent Platform API Setup

Complete the following three steps to provision your Google Cloud
project and enable API access.

1.  **Select or Create a Google Cloud Project**: Select an existing
    project or create a dedicated project for your team’s Antigravity
    workloads.

    [Go to GCP Project
    Selector](https://console.cloud.google.com/projectselector2)

2.  **Verify Cloud Billing**: Ensure that Cloud Billing is active for
    your selected Google Cloud project. You can inspect your project’s
    billing status in the Cloud Console.

    [Open Google Cloud Billing
    Console](https://console.cloud.google.com/billing)

3.  **Enable the Agent Platform API**: Enable the Agent Platform API
    (`aiplatform.googleapis.com`) to allow Antigravity clients to
    connect to your project’s model endpoints.

    [Enable Agent Platform
    API](https://console.cloud.google.com/apis/library/aiplatform.googleapis.com)

## Sign In & License Selection

Google Antigravity uses a single sign-on (SSO) flow. When you sign in
with your corporate business account, your license tier is automatically
detected without requiring manual tier selection.

### Sign-In Workflow

1.  Start **Antigravity 2.0**, the **Antigravity CLI**, or your
    supported **[IDE extension](https://antigravity.google/docs/ide/extensions)**.
2.  Select **Sign in** to open the browser authentication flow.
3.  Choose **Business account** *(subject to the Google Cloud Terms of
    Service)*.
4.  Select **Continue with Google Cloud** (or configure Advanced SSO /
    WIF).
5.  Complete authentication in your browser.
6.  Once authenticated, the **License Selector** displays your assigned
    licenses.
7.  Confirm the project linked to your license and select it.
    Alternatively, select **Other** to self-assign a license by entering
    your project ID and selecting a location (`global`, `us`, or `eu`).

## Bring Your Own Identity (BYOID / WIF)

Bring Your Own Identity (BYOID) uses Workforce Identity Federation (WIF)
to let your organization authenticate through an external identity
provider, such as Okta, instead of a standard Google Account.

### Configuring BYOID

1.  In Antigravity, select **Business account**.
2.  Select **Advanced WIF Configuration**.
3.  Enter the **WIF Configuration String** provided by your
    organization’s administrator.
4.  Complete sign-in through your federated identity provider.
5.  Select or self-assign a license from the License Selector.

## Application Default Credentials (ADC) in Antigravity CLI

For headless environments and automated terminal workflows, the
**Antigravity CLI** supports authentication using Google Cloud
Application Default Credentials (ADC).

### Setting Up ADC

1.  Generate local Application Default Credentials for your project
    using the Google Cloud SDK:

    ``` astro-code
    gcloud auth application-default login --project {GCP_PROJECT}
    ```

2.  Verify that your credentials file exists. On Linux and macOS, the
    default path is:

    ``` astro-code
    ~/.config/gcloud/application_default_credentials.json
    ```

3.  Enable ADC authentication by exporting the required environment
    variable:

    ``` astro-code
    export AGY_ADC_AUTH=true
    ```

4.  To sign out of ADC, unset the environment variable and restart your
    terminal session:

    ``` astro-code
    unset AGY_ADC_AUTH
    ```

## Regional Endpoints & Capability Matrix

Antigravity CLI, Antigravity 2.0, and IDE Extensions support
multi-region deployment endpoints to satisfy regional data residency
requirements:

| Endpoint Region | Base Endpoint URI | Supported Capabilities |
|:---|:---|:---|
| **Global** | `global` | Text Generation, Code Inference, Multimodal, Image Generation |
| **US Multi-Region** | `us` | Text Generation, Code Inference, Multimodal |
| **EU Multi-Region** | `eu` | Text Generation, Code Inference, Multimodal |

For full endpoint specifications, consult the [Deployment Endpoints
Documentation](https://docs.cloud.google.com/gemini-enterprise-agent-platform/resources/locations#global).

## Security & Governance

Request & Response Logging

Audit model interactions and maintain enterprise compliance records for
your Gemini Enterprise Agent Platform instance. [Learn
more](https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/capabilities/request-response-logging)

VPC Service Controls (VPC-SC)

Enforce private networking security perimeters by adding the Agent
Platform API to your VPC-SC perimeter. [Learn
more](https://docs.cloud.google.com/gemini-enterprise-agent-platform/machine-learning/general/vpc-service-controls)

## Troubleshooting & Diagnostics

### Common Sign-In & License Issues

- **No Licenses Appear During Setup**: Licenses are assigned by your
  organization’s Google Cloud administrator. If the License Selector is
  empty, contact your administrator to ensure your account has been
  granted access to a Gemini Enterprise Standard or Plus license.
- **Missing BYOID Sign-In Option**: Ensure you are running the latest
  release of **[Antigravity 2.0](https://antigravity.google/download)**, the **[Antigravity
  CLI](https://antigravity.google/docs/cli/install)**, or your **[IDE
  Extension](https://antigravity.google/docs/ide/extensions)**, as enterprise authentication and
  BYOID support are included natively in all recent releases.
- **Browser URL Allowlist Advisory**: When a browser URL allowlist is
  configured in admin controls, allowlisted URLs may still be blocked in
  Antigravity. Admin URL allowlists are currently being integrated and
  are not yet honored.

### Known Limitations

**BYOID / WIF login**:

- When signing in with the Advanced SSO option (BYOID / Workforce
  Identity Federation), a small number of users are unexpectedly logged
  out and must sign in again after restarting Antigravity 2.0 or the
  Antigravity CLI.
- Affected users must re-authenticate on restart.

### Important API Provisioning Advisory

### Sharing Diagnostics with Support

When contacting Google Cloud Support, include the diagnostic log file
from your most recent session:

- **Antigravity CLI (Linux and macOS)**:

  ``` astro-code
  ~/.gemini/antigravity-cli/cli.log
  ```

- **Antigravity 2.0 (macOS)**:

  ``` astro-code
  ~/Library/Logs/Antigravity/language_server.log
  ```

## What’s Next

- Explore supported model architectures in the [Models
  Guide](https://antigravity.google/docs/models).
- Learn more about enterprise privacy and compliance in [Security &
  Governance](#security--governance).
- Check the [Antigravity CLI Reference](https://antigravity.google/docs/cli/reference) for
  headless automation commands.
