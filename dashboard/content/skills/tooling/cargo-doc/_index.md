+++
title = "cargo-doc"
description = "Verify Rust crate APIs with local cargo doc output or docs.rs. Use to inspect exact signatures, methods, traits, and feature-gated items for the project version."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "tooling"
mermaid = false
+++


# Cargo doc and rustdoc navigation

Generate, query, and verify Rust crate documentation locally in `target/doc/` or remotely on `docs.rs` so every type, trait, and signature matches the project's `Cargo.lock`.

For extra CLI recipes and the `cargo doc` / `cargo rustdoc` flag reference, read [resources/manual/cargo-doc-recipes.md](@/skills/tooling/cargo-doc/resources/manual/cargo-doc-recipes.md).

## 1. Build targeted local docs

Bare `cargo doc` documents every dependency. Scope builds to the packages you need with `-p` and `--no-deps`, and add `--all-features` when an API may be feature-gated:

```bash
# Build docs for a specific dependency:
cargo doc -p tower-http --no-deps

# Build docs for multiple dependencies:
cargo doc -p egui -p bevy_egui --no-deps

# Include optional features to reveal conditionally compiled APIs:
cargo doc -p serde --all-features --no-deps

# Build only workspace crates:
cargo doc --workspace --no-deps
```

Cargo writes HTML to `target/doc/<lib_name>/`, where `<lib_name>` replaces hyphens in the Cargo `<package>` name with underscores (for example, package `tower-http` writes to `target/doc/tower_http/`).

## 2. Query local documentation

Navigate `target/doc/<lib_name>/` directly with search tools:

1. **Search the crate item index** (`all.html`) first to confirm exact item names and module paths:

   ```bash
   grep -i "TraceLayer" target/doc/tower_http/all.html
   ```

2. **Inspect item HTML files** (`struct.<Name>.html`, `trait.<Name>.html`, `enum.<Name>.html`, `fn.<Name>.html`). Rustdoc wraps keywords and identifiers in `<span>` elements, so match method anchors (`id="method.`), function links (`class="fn"`), or strip HTML tags when reading signatures:

   ```bash
   # List method anchors on a struct or trait:
   grep -o -E 'id="method\.[^"]+"' target/doc/egui/struct.Ui.html | sort -u | head -n 30

   # Search stripped HTML text for a method signature:
   sed -E 's/<[^>]+>//g' target/doc/egui/struct.Ui.html | grep -n -A 2 "pub fn add"
   ```

## 3. Query docs.rs before adding a crate

When checking a crate before adding it to `Cargo.toml`, distinguish the hyphenated Cargo `<package>` in the URL prefix from the underscored `<lib_name>` in the module path:

- **Item index**: `https://docs.rs/<package>/latest/<lib_name>/all.html`
- **Item pages**: `https://docs.rs/<package>/latest/<lib_name>/{struct,enum,trait,fn}.<Name>.html`
- **Pinned version**: replace `latest` with an exact version (`https://docs.rs/<package>/<version>/<lib_name>/all.html`)

```bash
curl -sL "https://docs.rs/tower-http/latest/tower_http/all.html" | grep -i "TraceLayer"
```

## 4. Verify with the compiler

When choosing between similar methods or trait bounds, write the call and run `cargo check 2>&1 | head -n 35`. Use rustc's suggestions for renamed methods, missing trait imports, and type mismatches to confirm the exact signature.

## Completion criteria

- Every external API used in the change is confirmed in `target/doc/<lib_name>/` or on `docs.rs` for the version in `Cargo.lock`.
- `cargo check` exits 0.

