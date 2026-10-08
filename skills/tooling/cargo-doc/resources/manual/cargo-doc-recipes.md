# Cargo Doc and Rustdoc Query Recipes

A practical reference of commands and shell workflows for querying local `rustdoc` outputs and remote `docs.rs` pages with zero hallucination.

---

## 1. Targeted Local Documentation Builds

Building documentation for the entire dependency tree can take minutes and consume gigabytes of disk space. Always target specific packages with `--no-deps`.

```bash
# Build docs for a single dependency without transitive dependencies:
cargo doc -p egui --no-deps

# Build docs for multiple specific crates:
cargo doc -p egui -p bevy_egui --no-deps

# Build docs with all feature flags enabled to reveal conditional APIs:
cargo doc -p serde --all-features --no-deps

# Build docs exclusively for workspace crates:
cargo doc --workspace --no-deps
```

---

## 2. Navigating Local HTML Documentation via CLI

Local docs are located in `target/doc/<crate_name>/`.

### Finding the Master Item Index

Every crate contains an `all.html` file listing all public types, traits, functions, and macros:

```bash
# Locate all items in crate:
cat target/doc/egui/all.html | grep -o -E 'href="[^"]+"' | head -n 30
```

### Inspecting Methods on a Struct or Trait

Inspect the generated struct file directly to view method signatures and doc comments:

```bash
# Search for public methods on struct Ui:
grep -in "pub fn " target/doc/egui/struct.Ui.html | head -n 25

# View methods including return types:
grep -A 2 -in "pub fn " target/doc/egui/struct.Ui.html | head -n 40
```

---

## 3. Remote Docs.rs Querying (Pre-dependency Research)

When evaluating a crate before adding it to `Cargo.toml`, query `docs.rs` directly using predictable URL patterns.

### The `all.html` Catalog

Fetch and filter the complete index of items:

```bash
# Find all structs matching 'Context':
curl -sL "https://docs.rs/bevy_egui/latest/bevy_egui/all.html" | grep -i "EguiContext"

# Verify trait implementations on a type:
curl -sL "https://docs.rs/egui/latest/egui/all.html" | grep -i "Widget"
```

### Direct Item URLs

- **Structs**: `https://docs.rs/<crate>/latest/<crate>/struct.<Name>.html`
- **Traits**: `https://docs.rs/<crate>/latest/<crate>/trait.<Name>.html`
- **Enums**: `https://docs.rs/<crate>/latest/<crate>/enum.<Name>.html`
- **Functions**: `https://docs.rs/<crate>/latest/<crate>/fn.<name>.html`
- **Modules**: `https://docs.rs/<crate>/latest/<crate>/<module>/index.html`

---

## 4. Compiler Diagnostic Feedback Loop (`cargo check`)

When uncertain about an exact method signature or renamed symbol, write an approximate implementation and execute `cargo check`:

```bash
cargo check 2>&1 | head -n 35
```

`rustc` diagnostics deliver precise corrections:
- Similar method names (`help: there is a method with a similar name: '...'`).
- Trait requirements (`help: the following trait must be implemented: '...'`).
- Required imports (`help: consider importing this struct: 'use egui::...'`).
