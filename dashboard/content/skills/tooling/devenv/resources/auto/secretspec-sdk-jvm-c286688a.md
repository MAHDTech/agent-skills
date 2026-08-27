+++
title = "secretspec-sdk-jvm-c286688a"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# JVM SDK

> **Version compatibility:** Available since SecretSpec 0.20.

The JVM SDK (`org.cachix.secretspec-jvm`) is a thin client over the same
Rust resolver as the CLI. Every provider, fallback chain, profile,
generator, reference, and `as_path` secret therefore works without
JVM-side resolution logic.

## Install (0.20+)

### Using gradle

In your `build.gradle.kts` dependencies:

```
implementation("org.cachix:secretspec-jvm:$secretspecVersion")
```

### Using Maven

In your `pom.xml` dependencies:

```
<dependency>  <groupId>org.cachix</groupId>  <artifactId>secretspec-jvm</artifactId>  <version>${secretspec.version}</version></dependency>
```

### Native libraries

The package targets JDK 11 and includes native resolvers for glibc and
musl Linux x64/Arm64, macOS x64/Arm64, and Windows x64/Arm64. Windows
assets statically include the C runtime. No separate SecretSpec CLI,
native library, Visual C++ Redistributable, or system `libdbus`
installation is needed.

## Quick start

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;

public class QuickStartExample {

    public static void main(String[] args) {
        try (var resolved = SecretSpec.builder()
            .withProvider("keyring://")
            .withProfile("production")
            .withReason("boot web app")
            .load()
        ) {
            System.out.println(resolved.provider() + " (" + resolved.profile() + ")");
            System.out.println(resolved.secret("DATABASE_URL").get());
            resolved.setAsSystemProperties();
        }
    }
}
```

`get()` returns the inline value, or the readable file path for an
`as_path` secret. A missing required secret throws
`MissingRequiredException`; its `missing()` method returns the secret
names. Other failures throw `SecretSpecException`, whose `kind()` method
returns a stable error category.

## Scopes (0.17+)

Use `withScope("api")` to resolve only a named `[scopes.api]` subset.
The selected name is available as `Resolved.scope()` and
`ResolutionReport.scope()`:

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;

public class ScopeExample {

    public static void main(String[] args) {
        try (var resolved = SecretSpec.builder().withScope("api").load()) {
            resolved.setAsSystemProperties();
        }
    }
}
```

## Value-free preflight

`report()` returns the inventory view exposed by
`secretspec check --json`. It never carries values. Missing required
secrets appear with `status().equals("missing_required")` rather than
throwing, so incomplete deployments can still be inspected.

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;

public class ReportExample {

    public static void main(String[] args) {
        var report = SecretSpec.builder()
            .withProfile("production")
            .withReason("deployment preflight")
            .report();

        for (var secret : report.secrets())
            System.out.println(secret.name() + ": " + secret.status());
    }
}
```

## Typed access

Generate an idiomatic language model from the manifest schema:

```
secretspec schema |  quicktype -s schema --top-level AppSecrets --lang java -o AppSecrets.java
```

Terminal window

Then deserialize the SDK’s flat field map:

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;
import io.quicktype.AppSecrets;
import io.quicktype.Converter;

import java.io.IOException;

public class TypedAccessExample {

    public static void main(String[] args) throws IOException {
        try (var resolved = SecretSpec.builder().load()) {
            AppSecrets typed = Converter.fromJsonString(resolved.fieldsJson());
            System.out.println(typed.getDatabaseURL());
        }
    }
}
```

The schema models successful resolution: required, defaulted, and
generated secrets are non-nullable, and profile-specific schemas include
inherited default-profile fields.

## Files (`as_path`)

File-shaped secrets are materialized as mode-0400 temporary files. The
returned path must remain valid after `load()`, so the caller owns its
lifetime. `Resolved` implements `AutoCloseable`; use a
`try-with-resources` declaration or call `close()` to remove these files
deterministically:

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;

public class AsPathExample {

    public static void main(String[] args) {
        try (var resolved = SecretSpec.builder().withReason("TLS boot").load()) {
            var secrets = resolved.secrets();
            var certificatePath = secrets.get("TLS_CERT").get();
            // Use the certificate before resolved is disposed.
            System.out.println(certificatePath);
        }
    }
}
```

## Caller (0.20+)

Caller context answers *what* invoked SecretSpec (for example, `git`).
It is deliberately separate from the user-supplied access reason, which
answers *why* the access is happening and may be required by a project’s
`require_reason` policy. Caller context never satisfies that policy.

The context is caller-asserted metadata, not an authenticated identity.
It is included in audit events and forwarded to providers that choose to
consume it. Do not put credentials or secret values in any field.

``` astro-code
package org.cachix.examples;

import org.cachix.secretspec.SecretSpec;
import org.cachix.secretspec.Caller;

public class CallerExample {

    public static void main(String[] args) {
        try (var resolved = SecretSpec.builder()
            .withProvider("keyring://")
            .withProfile("production")
            .withCaller(Caller.named("caller name")
                .withVersion("optional caller version")
                .withOperation("optional operation")
                .withResource("optional resource")
            )
            .withReason("boot web app")
            .load()
        ) {
            System.out.println(resolved.provider() + " (" + resolved.profile() + ")");
            System.out.println(resolved.secret("DATABASE_URL").get());
        }
    }
}
```

## Native loading

The Jar runtime asset is selected automatically. For local SDK
development, `SECRETSPEC_FFI_LIB` can point to a particular
`libsecretspec_ffi` build. From a SecretSpec source checkout, the SDK
also searches an ancestor Cargo `target/debug` or `target/release`
directory.

