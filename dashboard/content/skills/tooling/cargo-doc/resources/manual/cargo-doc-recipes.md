+++
title = "cargo-doc-recipes"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "cargo-doc"
+++

# Cargo doc query recipes

Extra recipes beyond the workflow in `SKILL.md`.

## List every item link in a crate

```bash
grep -o -E 'href="[^"]+"' target/doc/<crate>/all.html | head -n 30
```

## Module index URL on docs.rs

`https://docs.rs/<crate>/latest/<crate>/<module>/index.html`

## Rustdoc options

`cargo doc` and `cargo rustdoc` flags (`--document-private-items`, `--open`, target selection) are in `resources/auto/rust-lang-cargo-master-doc-man-cargo-doc.md` and `resources/auto/rust-lang-cargo-master-doc-man-cargo-rustdoc.md`.

