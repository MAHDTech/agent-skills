---
name: cargo-doc
description: Generate, query, and navigate local Rust documentation and docs.rs with zero hallucination. Use when researching Rust crate APIs, verifying struct and trait signatures, inspecting conditionally compiled feature flags, finding methods on types, or searching offline docs when internet or llms.txt is unavailable.
resources:
  - https://raw.githubusercontent.com/rust-lang/cargo/master/doc/man/cargo-doc.md
  - https://raw.githubusercontent.com/rust-lang/cargo/master/doc/man/cargo-rustdoc.md
---

# Cargo Doc and Rustdoc Navigation

Workflow for generating, querying, and verifying Rust crate documentation locally and remotely with zero hallucination.

**The code is the contract:** Rather than guessing API signatures from model pretraining memory, use `cargo doc` to inspect exact types and methods matching your project's `Cargo.lock`.

---

## When to Use This Skill

- Researching unfamiliar, newly added, or rapidly evolving Rust crates.
- Verifying exact method names, arguments, and return types before writing implementation code.
- Checking feature-gated APIs or conditionally compiled modules (`--all-features`).
- Investigating compiler errors when an API appears changed or missing.
- Inspecting documentation offline when crates lack an `llms.txt` file or when internet access is limited.
- Querying `docs.rs` for third-party libraries using deterministic URL patterns.

---

## Reference Files

For practical CLI query recipes:

- `resources/manual/cargo-doc-recipes.md` - Command recipes for filtering and searching local rustdoc HTML and `docs.rs`.

---

## 1. Targeted Documentation Builds

Generating docs for an entire dependency tree is slow. Scope builds strictly to the crates you need using `--no-deps`:

```bash
# Build docs for a specific third-party crate:
cargo doc -p egui --no-deps

# Build docs for multiple target dependencies:
cargo doc -p egui -p bevy_egui --no-deps

# Include all optional features to reveal conditionally compiled APIs:
cargo doc -p serde --all-features --no-deps

# Build only workspace crates:
cargo doc --workspace --no-deps
```

Compiled documentation is written to `target/doc/`.

---

## 2. Querying Local Documentation via File Tools

Once generated, navigate `target/doc/` directly without opening a browser.

### Step 1: Locate the Master Item Index

Every crate contains an `all.html` file listing every public type, trait, function, and macro in that crate:

```bash
# Verify if a type exists in the local documentation:
cat target/doc/egui/all.html | grep -i "CentralPanel"
```

### Step 2: Inspect Specific Structs and Traits

Rustdoc outputs dedicated HTML files for every symbol:

- `target/doc/<crate>/struct.<Name>.html`
- `target/doc/<crate>/trait.<Name>.html`
- `target/doc/<crate>/enum.<Name>.html`

Query methods and function signatures directly using text search:

```bash
# List all public methods on a struct:
grep -in "pub fn " target/doc/egui/struct.Ui.html | head -n 30

# View detailed signatures with surrounding lines:
grep -A 2 -in "pub fn add" target/doc/egui/struct.Ui.html
```

---

## 3. Remote Docs.rs Querying (Pre-Installation)

When exploring a crate before adding it to `Cargo.toml`, query `docs.rs` using deterministic URL conventions:

1. **Master Item Index**:
   `https://docs.rs/<crate>/latest/<crate>/all.html`
   Curl or fetch this page to verify symbols across the whole crate:

   ```bash
   curl -sL "https://docs.rs/bevy_egui/latest/bevy_egui/all.html" | grep -i "EguiContext"
   ```

2. **Deterministic Item URLs**:
   - Struct: `https://docs.rs/<crate>/latest/<crate>/struct.<StructName>.html`
   - Enum: `https://docs.rs/<crate>/latest/<crate>/enum.<EnumName>.html`
   - Trait: `https://docs.rs/<crate>/latest/<crate>/trait.<TraitName>.html`
   - Function: `https://docs.rs/<crate>/latest/<crate>/fn.<FunctionName>.html`

3. **Version Pinned URLs**:
   Replace `latest` with an exact semver tag to view historic or pinned docs:
   `https://docs.rs/egui/0.36.0/egui/struct.Ui.html`

---

## 4. Compiler Diagnostic Loop (`cargo check`)

When uncertain between two similar methods or signatures:

1. Write the tentative implementation.
2. Run `cargo check 2>&1 | head -n 35`.
3. Review the compiler feedback:
   - Method renames: `help: there is a method with a similar name: '...'`
   - Missing traits: `help: the following trait must be implemented: '...'`
   - Missing imports: `help: consider importing this item: 'use ...'`
4. Adjust code based on the compiler's diagnostic output.

---

## 5. Best Practices and Traps

- **Do:** Always pass `--no-deps` when inspecting third-party crates to prevent slow transitive documentation builds.
- **Do:** Pass `--all-features` if the method or type you seek is gated behind an optional Cargo feature.
- **Do:** Use `cat target/doc/<crate>/all.html | grep -i "<symbol>"` as your first step to confirm exact spelling and module paths.
- **Don't:** Run bare `cargo doc` in large workspaces without flags; it compiles documentation for hundreds of dependencies unnecessarily.
- **Don't:** Guess method signatures from memory when the compiler or `cargo doc` can provide the verified ground truth in seconds.
