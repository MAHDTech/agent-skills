# Fly.io provider name

> The Fly.io provider uses the fly name and URI scheme in SecretSpec 0.20+.

**Changed in version 0.20**

The pre-release [`flyctl`](https://fly.io/docs/flyctl/) provider name was replaced by `fly`. Use the [`fly` provider guide](/providers/fly/) and configure Fly.io application secrets with a `fly://APP` URI.

The provider still invokes the `flyctl` executable internally. Only the SecretSpec provider name and URI scheme changed.

# The [Fly.io](https://fly.io/) provider is named `fly`