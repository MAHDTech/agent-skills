# egui Quick Reference and Pattern Catalog

A concise recipe catalog for immediate-mode GUI development in pure Rust using `egui`.

---

## 1. Top-Level Containers

Immediate-mode layouts run once per frame. Panels must be declared before the central panel.

```rust
use egui::{CentralPanel, SidePanel, TopBottomPanel, Window};

fn render_layout(ctx: &egui::Context) {
    // Top bar for menus or headers:
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Application Title");
            ui.separator();
            if ui.button("Settings").clicked() {
                // handle settings toggle
            }
        });
    });

    // Left navigation or tools sidebar:
    SidePanel::left("left_sidebar")
        .resizable(true)
        .default_width(200.0)
        .width_range(150.0..=400.0)
        .show(ctx, |ui| {
            ui.label("Sidebar Controls");
        });

    // Central panel fills remaining space:
    CentralPanel::default().show(ctx, |ui| {
        ui.label("Main canvas or viewport content");
    });

    // Floating, draggable window:
    Window::new("Inspector")
        .default_open(true)
        .resizable(true)
        .vscroll(true)
        .show(ctx, |ui| {
            ui.label("Window contents");
        });
}
```

---

## 2. Layouts and Alignment

```rust
// Horizontal row of widgets:
ui.horizontal(|ui| {
    ui.label("Status:");
    ui.colored_label(egui::Color32::GREEN, "Online");
});

// Vertical column:
ui.vertical(|ui| {
    ui.label("Item 1");
    ui.label("Item 2");
});

// Multi-column layout:
ui.columns(2, |columns| {
    columns[0].label("Left Column");
    columns[1].label("Right Column");
});

// Tabular grid:
egui::Grid::new("properties_grid")
    .num_columns(2)
    .spacing([20.0, 8.0])
    .striped(true)
    .show(ui, |ui| {
        ui.label("Resolution");
        ui.label("1920x1080");
        ui.end_row();

        ui.label("Framerate");
        ui.label("60 FPS");
        ui.end_row();
    });

// Scrollable container:
egui::ScrollArea::vertical()
    .max_height(300.0)
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for i in 0..100 {
            ui.label(format!("Row {i}"));
        }
    });

// Collapsible section:
ui.collapsing("Advanced Settings", |ui| {
    ui.label("Hidden options here");
});
```

---

## 3. Common Widgets

```rust
// Text & Labels:
ui.heading("Heading Text");
ui.label("Normal text");
ui.colored_label(egui::Color32::RED, "Warning");

// Buttons & Actions:
if ui.button("Click Me").clicked() {
    // Action performed
}

// Input Fields:
ui.text_edit_singleline(&mut string_var);
ui.text_edit_multiline(&mut text_content);

// Sliders and Drag Values:
ui.add(egui::Slider::new(&mut float_val, 0.0..=1.0).text("Volume"));
ui.add(egui::DragValue::new(&mut int_val).speed(1));

// Checkbox and Radio:
ui.checkbox(&mut is_active, "Enable Feature");
ui.radio_value(&mut selected_mode, Mode::Fast, "Fast");
ui.radio_value(&mut selected_mode, Mode::Accurate, "Accurate");

// Combo Box / Dropdown:
egui::ComboBox::from_label("Quality")
    .selected_text(format!("{:?}", current_quality))
    .show_ui(ui, |ui| {
        ui.selectable_value(&mut current_quality, Quality::Low, "Low");
        ui.selectable_value(&mut current_quality, Quality::High, "High");
    });
```

---

## 4. Custom Widget & Painter Canvas

```rust
use egui::{Color32, Pos2, Rect, Sense, Stroke, Vec2};

fn draw_custom_canvas(ui: &mut egui::Ui) -> egui::Response {
    let desired_size = Vec2::new(200.0, 100.0);
    // Allocate space on screen and receive a Response for mouse interaction:
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);

        // Draw background rectangle:
        painter.rect_filled(rect, 4.0, Color32::from_gray(30));

        // Draw outline:
        painter.rect_stroke(rect, 4.0, Stroke::new(1.0, Color32::from_gray(80)));

        // Draw interactive circle at pointer position when dragging:
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            painter.circle_filled(mouse_pos, 8.0, Color32::GOLD);
        } else {
            painter.circle_filled(rect.center(), 6.0, Color32::LIGHT_BLUE);
        }
    }

    response
}
```

---

## 5. State Retention Across Frames

Immediate-mode GUIs do not keep widget structs alive across frames. Store your application state in your own structs, or use `egui::Id` to store transient UI state in egui's memory cache.

```rust
// Generate stable unique IDs:
let my_id = ui.make_persistent_id("my_custom_state");

// Read or write transient state in egui memory:
let mut is_expanded: bool = ui.data_mut(|d| d.get_temp(my_id).unwrap_or(false));
if ui.button("Toggle Internal").clicked() {
    is_expanded = !is_expanded;
    ui.data_mut(|d| d.insert_temp(my_id, is_expanded));
}
```

---

## 6. Standalone Application (`eframe`)

Minimal template for building desktop tools without a game engine:

```rust
use eframe::egui;

#[derive(Default)]
struct DemoApp {
    counter: i32,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Standalone eframe Tool");
            if ui.button("Increment").clicked() {
                self.counter += 1;
            }
            ui.label(format!("Count: {}", self.counter));
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Demo Tool",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}
```
