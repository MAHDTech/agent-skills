# Cargo doc query recipes

Extra CLI recipes and flag reference beyond the workflow in `SKILL.md`.

In local `target/doc/<lib_name>/` paths and `docs.rs` module URLs, `<lib_name>` is the crate's library identifier with hyphens replaced by underscores (for example, package `tower-http` maps to `tower_http`).

## List item links in a crate

```bash
grep -o -E 'href="[^"]+"' target/doc/<lib_name>/all.html | head -n 30
```

## Search rustdoc HTML reliably

Rustdoc wraps keywords and identifiers in `<span>` elements, so raw `pub fn ` searches can miss signatures. Match anchors, CSS classes, or stripped text instead:

```bash
# List method anchors on a struct or trait:
grep -o -E 'id="method\.[^"]+"' target/doc/<lib_name>/struct.<Name>.html | sort -u

# List function links in a module index or all.html:
grep -o -E 'class="fn"[^>]*>[^<]+<' target/doc/<lib_name>/all.html

# Strip HTML tags to inspect a full signature with context:
sed -E 's/<[^>]+>//g' target/doc/<lib_name>/struct.<Name>.html | grep -n -A 3 "pub fn <method>"
```

## Module index URL on docs.rs

```text
https://docs.rs/<package>/latest/<lib_name>/<module>/index.html
```

## Key `cargo doc` flags

Documentation builds are cumulative under `target/doc/`; clear prior output with `cargo clean --doc`.

| Flag | Purpose |
| ---- | ------- |
| `-p <spec>`, `--package <spec>` | Document one or more specific packages (repeatable). |
| `--workspace` | Document all workspace members. |
| `--no-deps` | Skip building documentation for dependencies. |
| `--all-features` | Activate all optional features to include feature-gated APIs. |
| `--no-default-features` / `-F <features>` | Document a specific feature set. |
| `--document-private-items` | Include non-public items (default for binary targets). |
| `--open` | Open the generated docs in the default browser after building. |
| `--lib` | Document only the package's library target. |
| `--bin <name>` / `--bins` | Document a named binary or all binary targets. |
| `--example <name>` / `--examples` | Document a named example or all example targets. |

When no target selection flag is passed, `cargo doc` documents all library and binary targets (skipping a binary whose name matches the library target, or binaries missing `required-features`).

## Custom rustdoc flags with `cargo rustdoc`

Use `cargo rustdoc [options] -- <rustdoc-args>` to pass extra flags directly to the final `rustdoc` invocation for one target (dependencies are not documented):

```bash
# Pass custom flags to rustdoc for the library target:
cargo rustdoc -p <package> --lib -- --document-private-items

# Include custom HTML in the generated header:
cargo rustdoc -p <package> --lib -- --html-in-header head.html
```

Run `cargo doc --help` or `cargo rustdoc --help` for the complete flag set of the installed Cargo version.
