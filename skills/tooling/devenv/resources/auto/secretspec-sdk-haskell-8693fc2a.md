# Haskell SDK

The Haskell SDK (`secretspec-hs`) is a thin client over the
`libsecretspec` C ABI, linked at build time via the Haskell FFI.
Resolution happens in the Rust core, so the SDK inherits every provider
with no Haskell-side logic.

## Quick start

``` astro-code
{-# LANGUAGE OverloadedStrings #-}

import qualified Data.Map.Strict as Map
import Data.Function ((&))
import qualified SecretSpec as S

main :: IO ()
main = do
  resolved <-
    S.load
      ( S.builder
          & S.withProvider "keyring://"
          & S.withProfile "production"
          & S.withReason "boot web app"
      )

  print (S.resolvedProvider resolved, S.resolvedProfile resolved)
  case Map.lookup "DATABASE_URL" (S.resolvedSecrets resolved) of
    Just db -> print (S.get db) -- the value, or the file path for as_path secrets
    Nothing -> pure ()
  S.setAsEnv resolved           -- export everything into the process environment
  S.close resolved
```

A missing required secret throws `MissingRequiredError`; any other
failure throws `SecretSpecError` (with a stable `errorKind`).

`as_path` secrets are materialized to temp files that outlive the call;
call `S.close resolved` when done so they do not accumulate in the temp
dir.

## Caller context

```
let caller = S.CallerContext      "git" (Just "2.51.0") (Just "credential_get") (Just "github.com")    configured = S.builder & S.withCaller caller
```

Caller context identifies the invoking integration in audit records but
never satisfies `require_reason`. Do not put credentials or secret
values in it.

## Inline specifications

Use `withInlineSpec spec baseDir` with an Aeson JSON value to resolve
strict inline-spec v1 declarations. `baseDir` resolves relative provider
paths; an older static archive fails to link the versioned call symbol.

## Scopes

Use `withScope "api"` to resolve only a named `[scopes.api]` subset. The
selected name is available through `resolvedScope` and `reportScope`:

``` astro-code
{-# LANGUAGE OverloadedStrings #-}

import Data.Function ((&))
import qualified SecretSpec as S

main :: IO ()
main = do
  resolved <- S.load (S.builder & S.withScope "api")
  S.close resolved
```

## Value-free report

`S.report` returns the inventory/preflight view: per-secret status and
provenance, never a value. Unlike `load`, it does not throw when a
required secret is missing — that secret appears as a `SecretReport`
with `srStatus` `"missing_required"`.

``` astro-code
{-# LANGUAGE OverloadedStrings #-}

import Data.Function ((&))
import qualified SecretSpec as S

main :: IO ()
main = do
  rep <- S.report (S.builder & S.withProfile "production")
  mapM_ (\s -> print (S.srName s, S.srStatus s, S.srRequired s)) (S.reportSecrets rep)
```

## Typed access (codegen)

Generate a typed record with `secretspec schema` plus
[quicktype](https://quicktype.io), then decode `S.fieldsJson resolved`:

```
$ secretspec schema | quicktype -s schema --top-level SecretSpec --lang haskell -o Secrets.hs
```

Terminal window

## Building

The build links the `libsecretspec` archive statically. Stage the `.a`
in a directory of its own (so the linker picks the archive, not the
co-located `.so`) and pass its native dependencies to the linker:

```
$ cargo build -p libsecretspec
$ TARGET="$(cargo metadata --no-deps --format-version 1 \  | grep -o '"target_directory":"[^"]*"' | head -1 | sed 's/.*:"\(.*\)"/\1/')"
# Stage the staticlib alone, and capture its native-static-libs for the linker.$ LIBDIR="$(mktemp -d)"
$ cp "$TARGET/debug/libsecretspec.a" "$LIBDIR/"
$ NATIVE_LIBS="$(cargo rustc -q -p libsecretspec --crate-type staticlib -- \  --print native-static-libs 2>&1 | sed -n 's/^note: native-static-libs: //p' | tail -1)"
$ cabal build --extra-lib-dirs="$LIBDIR" --ghc-options="-optl${NATIVE_LIBS// / -optl}"
$ cabal test  --extra-lib-dirs="$LIBDIR" --ghc-options="-optl${NATIVE_LIBS// / -optl}"
```

Terminal window

### Linking with pkg-config

Install one library type with
[cargo-c](https://github.com/lu-zero/cargo-c):

```
# Use "static" (the default) or "shared"; use separate prefixes for both.$ bash libsecretspec/scripts/cinstall.sh "$PREFIX" static
```

Terminal window

Then use the same Cabal flag for either type:

```
$ cd secretspec-hs
$ PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig" cabal build -f use-pkg-config
$ PKG_CONFIG_PATH="$PREFIX/lib/pkgconfig" cabal test  -f use-pkg-config
```

Terminal window

A shared install in a non-system prefix also requires `PREFIX/lib` in
the platform’s runtime library search path.
