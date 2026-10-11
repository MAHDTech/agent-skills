+++
title = "egui"
description = "Build immediate-mode GUIs in Rust using egui - widgets, layout, containers, styling, custom painting, state retention, eframe apps, and testing. Use when building desktop or web interfaces in Rust, creating custom immediate-mode widgets or canvas painters, styling egui visuals, or querying docs.rs rustdoc without hallucination."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "engineering"
mermaid = false
+++


# egui Immediate-Mode GUI

Patterns for building performant, reactive graphical interfaces in pure Rust using `egui` (0.36+).

**Immediate mode means code is layout:** UI elements are evaluated and drawn each frame from scratch. There is no persistent widget object tree in memory. Application state lives in your data models; UI reacts to and mutates that data directly.

---

## When to Use This Skill

- Building desktop, web (Wasm), or embedded user interfaces in pure Rust.
- Designing layout panels, toolbars, sidebars, grids, and scroll areas.
- Implementing custom interactive widgets, charts, or canvas painters.
- Customizing themes, dark/light visuals, styles, strokes, and typography.
- Managing immediate-mode state retention and deterministic ID generation.
- Developing standalone desktop utilities with `eframe`.
- Looking up exact `egui` signatures and methods on `docs.rs` with zero hallucination.

---

## Reference Files

For in-depth recipes and documentation lookup protocols:

- `resources/manual/egui-cheatsheet.md` - Ready-to-use widget, layout, and painting recipes.
- `resources/manual/rustdoc-lookup-guide.md` - Protocol for searching `docs.rs` and local `cargo doc` outputs without guessing.

---

## 1. Mental Model and Frame Cycle

In `egui`, every frame follows a three-step cycle:

1. **Input Phase**: `egui` receives raw platform events (cursor position, clicks, keypresses, scroll delta).
2. **Logic & Drawing Phase**: Your application code runs. You invoke `ui.button()`, `ui.label()`, etc. Widgets query the input state, calculate their size, submit draw shapes to the painter, and return an `egui::Response`.
3. **Output Phase**: `egui` tessellates submitted shapes into textured triangles and dispatches them to the graphics backend.

```rust
// Basic frame function:
fn render_ui(ctx: &egui::Context, state: &mut MyAppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("My Dashboard");

        if ui.button("Increment Counter").clicked() {
            state.counter += 1;
        }

        ui.label(format!("Current count: {}", state.counter));
    });
}
```

---

## 2. Containers and Paneling Order

Containers define screen topology. Order matters: declare outer panels before inner panels.

1. **`TopBottomPanel::top` / `bottom`**: Docked headers and status bars spanning full width.
2. **`SidePanel::left` / `right`**: Sidebars docked between top and bottom panels.
3. **`CentralPanel::default()`**: Consumes all remaining viewport space. Must be called after outer panels.
4. **`Window::new()`**: Floating, resizable, movable windows rendered above docked panels.
5. **`Area::new()`**: Arbitrary floating layout elements without window frames or title bars.

```rust
use egui::{CentralPanel, SidePanel, TopBottomPanel, Window};

fn build_workspace(ctx: &egui::Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.label("Header");
    });

    SidePanel::left("nav_panel").resizable(true).show(ctx, |ui| {
        ui.label("Navigation");
    });

    // CentralPanel takes the remaining area:
    CentralPanel::default().show(ctx, |ui| {
        ui.label("Main Work Area");
    });

    // Floating window over top:
    Window::new("Debug Tool").resizable(true).show(ctx, |ui| {
        ui.label("Debug info");
    });
}
```

---

## 3. Layout Control

Use layout builders to structure contents within containers:

- **Horizontal Row**: `ui.horizontal(|ui| { ... })`
- **Vertical Column**: `ui.vertical(|ui| { ... })`
- **Centered / Alignment**: `ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ... })`
- **Multi-column Split**: `ui.columns(3, |columns| { ... })`
- **Tabular Grids**: `egui::Grid::new("id").striped(true).show(ui, |ui| { ... ui.end_row(); })`
- **Scrollable Area**: `egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| { ... })`

---

## 4. Responses and Interaction

Every widget returns an `egui::Response`. Inspect it to handle interactions:

```rust
let response = ui.button("Submit");

if response.clicked() {
    // Left-clicked this frame
}
if response.secondary_clicked() {
    // Right-clicked this frame (context menu trigger)
}
if response.hovered() {
    // Cursor is currently over the widget
}
if response.changed() {
    // Value was edited (applicable to sliders, text inputs, checkboxes)
}

// Attach a tooltip on hover:
response.on_hover_text("Click to execute operation");
```

---

## 5. Custom Painting and Canvas

When standard widgets do not suffice, allocate exact screen real estate and paint custom 2D primitives:

```rust
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

fn draw_radar(ui: &mut egui::Ui) -> egui::Response {
    let size = Vec2::splat(150.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        let center = rect.center();
        let radius = rect.width() / 2.0;

        // Background:
        painter.circle_filled(center, radius, Color32::from_rgb(10, 25, 10));
        // Grid ring:
        painter.circle_stroke(center, radius * 0.66, Stroke::new(1.0, Color32::GREEN));
        // Reticle lines:
        painter.line_segment([Pos2::new(rect.left(), center.y), Pos2::new(rect.right(), center.y)], Stroke::new(1.0, Color32::GREEN));
        painter.line_segment([Pos2::new(center.x, rect.top()), Pos2::new(center.x, rect.bottom())], Stroke::new(1.0, Color32::GREEN));
    }

    response
}
```

---

## 6. Deterministic IDs and State Retention

`egui` relies on unique `egui::Id` values to track which windows are open, scroll positions, and collapsing headers.

- **Automatic Hashing**: Widgets compute IDs based on parent IDs and label strings.
- **Explicit IDs**: Use `ui.push_id("unique_key", |ui| { ... })` or `Id::new("salt")` when dynamically generating widgets in loops to prevent ID collisions.
- **Storing UI State**: Store transient UI state (e.g. selection or drag offset) in egui memory via `ui.data_mut()`:

```rust
let persistent_id = ui.make_persistent_id("my_selected_item");
let mut selected: usize = ui.data_mut(|d| d.get_temp(persistent_id).unwrap_or(0));

if ui.button("Select Next").clicked() {
    selected += 1;
    ui.data_mut(|d| d.insert_temp(persistent_id, selected));
}
```

---

## 7. Theming and Styles

Set visual themes globally on `egui::Context`:

```rust
// Quick switch between dark and light themes:
ctx.set_visuals(egui::Visuals::dark());
ctx.set_visuals(egui::Visuals::light());

// Custom style tweaks:
let mut style = (*ctx.style()).clone();
style.spacing.item_spacing = egui::vec2(10.0, 8.0);
style.spacing.button_padding = egui::vec2(12.0, 6.0);
ctx.set_style(style);
```

---

## 8. Standalone Apps with `eframe`

For non-game desktop and web tools, use `eframe`:

```toml
[dependencies]
egui = "0.36"
eframe = "0.36"
```

```rust
use eframe::egui;

struct Application {
    name: String,
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Desktop Tool");
            ui.text_edit_singleline(&mut self.name);
            ui.label(format!("Greetings, {}", self.name));
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Application",
        native_options,
        Box::new(|_cc| Ok(Box::new(Application { name: "Ferris".into() }))),
    )
}
```

---

## 9. Zero-Hallucination API Verification

When implementing with `egui`, verify method signatures before assuming:

1. **Verify via `all.html`**:
   Inspect `https://docs.rs/egui/latest/egui/all.html` to confirm whether a struct or function exists.
2. **Inspect Local Docs**:
   Run `cargo doc -p egui --no-deps` and check `target/doc/egui/struct.Ui.html`.
3. **Compiler Suggestions**:
   Run `cargo check 2>&1 | head -n 30` to let the Rust compiler pinpoint valid method names and type signatures.

---

## 10. Best Practices and Traps

- **Do:** Keep heavy calculations, network requests, or file I/O out of the UI update function. Spawn background threads or async tasks and poll results.
- **Do:** Wrap dynamically generated list rows in `ui.push_id(item_id, |ui| { ... })` to prevent layout jumps or ID collisions.
- **Do:** Use `ScrollArea::vertical().auto_shrink([false, false])` inside panels so scrollbars expand properly.
- **Don't:** Retain `Ui` references across frames or store them in structs. `Ui` is only valid during the immediate render closure.
- **Don't:** Allocate large strings or collections inside the render loop every frame. Pre-allocate or format into scratch buffers.

