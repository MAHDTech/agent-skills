---
name: bevy-egui
description: Integrate egui immediate-mode GUIs into Bevy 0.19 using bevy_egui 0.40-0.42 - plugin setup, EguiPrimaryContextPass scheduling, EguiContexts, input absorption, viewport cameras, Bevy image textures, and inspector integration. Use when adding UI to Bevy games, creating debug overlays or HUDs, preventing click-through into game entities, or rendering egui to textures.
resources:
  - https://raw.githubusercontent.com/vladbat00/bevy_egui/master/README.md
---

# Bevy egui Integration

Patterns for building in-game interfaces, debug overlays, and HUDs by integrating `egui` into Bevy 0.19 using `bevy_egui` (0.40-0.42).

**Targets Bevy 0.19 and bevy_egui 0.42.0** (with egui 0.36). Older Bevy versions (0.14-0.15) scheduled UI systems into `Update` with `contexts.ctx_mut()`; Bevy 0.19 uses the dedicated `EguiPrimaryContextPass` schedule and returns `Result`.

---

## When to Use This Skill

- Integrating immediate-mode UI panels, HUDs, or debug windows into a Bevy 0.19 game or simulation.
- Scheduling UI passes cleanly inside `EguiPrimaryContextPass`.
- Preventing click-through into gameplay worlds via input absorption run conditions (`egui_wants_any_pointer_input`).
- Rendering Bevy asset textures (`Handle<Image>`) inside egui widgets.
- Managing multi-window contexts, split-screen cameras, or rendering egui directly onto 3D in-game surfaces.
- Gating UI display behind Bevy `States` (e.g. gameplay HUD vs pause menu).
- Connecting `bevy-inspector-egui` for runtime ECS entity/component reflection.

---

## Reference Files

For expanded recipes and compatibility details:

- `resources/manual/bevy-egui-guide.md` - Complete architectural guide, split-screen viewports, and asset texture mapping.

---

## 1. Minimal Setup (Bevy 0.19)

Add `bevy_egui` to your dependencies and register `EguiPlugin`.

```toml
[dependencies]
bevy = "0.19.0"
bevy_egui = "0.42.0"
```

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn ui_example_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Diagnostics").show(contexts.ctx_mut()?, |ui| {
        ui.label("Bevy 0.19 running with bevy_egui 0.42");
    });
    Ok(())
}
```

---

## 2. Scheduling and Context Access

In Bevy 0.19:

- **Schedule**: Add primary window UI systems to `EguiPrimaryContextPass`.
- **System Parameter**: Inject `mut contexts: EguiContexts`.
- **Result Return Type**: Declare systems returning `-> Result` so you can use `contexts.ctx_mut()?` without explicit pattern matching.
- **Window & Camera Queries**: When querying primary windows or cameras alongside egui, use `Single<&mut Camera, Without<EguiContext>>` or `Single<&mut Window, With<PrimaryWindow>>`.

```rust
fn debug_hud_system(
    mut contexts: EguiContexts,
    time: Res<Time>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Area::new(egui::Id::new("fps_counter"))
        .fixed_pos([10.0, 10.0])
        .show(ctx, |ui| {
            ui.colored_label(
                egui::Color32::GREEN,
                format!("Delta: {:.2}ms", time.delta_secs() * 1000.0),
            );
        });

    Ok(())
}
```

---

## 3. Input Absorption (Preventing Click-Through)

Without input absorption, clicking on an egui button also clicks through to whatever game object or camera control sits behind the window.

Protect your gameplay systems using `bevy_egui::input` run conditions:

```rust
use bevy::prelude::*;
use bevy_egui::input::{egui_wants_any_keyboard_input, egui_wants_any_pointer_input};

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Block player movement / typing when egui is focused on a text box:
                keyboard_movement_system.run_if(not(egui_wants_any_keyboard_input)),
                // Block weapon fire / unit selection when cursor is over an egui window:
                mouse_interaction_system.run_if(not(egui_wants_any_pointer_input)),
            ),
        );
    }
}
```

---

## 4. Displaying Bevy Images and Textures

Convert Bevy `Handle<Image>` assets into egui textures:

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

fn render_character_portrait(
    mut contexts: EguiContexts,
    portrait_handle: Res<Handle<Image>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let texture_id = contexts.add_image(portrait_handle.clone());

    egui::Window::new("Character Profile").show(ctx, |ui| {
        ui.image(egui::load::SizedTexture::new(texture_id, [96.0, 96.0]));
        ui.label("Hero Unit");
    });

    Ok(())
}
```

---

## 5. Game State Gating

Toggle UI visibility based on Bevy `States`:

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
}

pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                EguiPrimaryContextPass,
                (
                    in_game_hud.run_if(in_state(GameState::InGame)),
                    pause_overlay.run_if(in_state(GameState::Paused)),
                ),
            );
    }
}

fn in_game_hud(mut contexts: EguiContexts) -> Result {
    egui::Area::new(egui::Id::new("hud")).show(contexts.ctx_mut()?, |ui| {
        ui.label("Score: 1250");
    });
    Ok(())
}

fn pause_overlay(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    egui::Window::new("Paused")
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(contexts.ctx_mut()?, |ui| {
            if ui.button("Resume").clicked() {
                next_state.set(GameState::InGame);
            }
        });
    Ok(())
}
```

---

## 6. Rendering egui to 3D Surfaces

To render an egui interface onto an in-game object (such as a computer terminal or holographic monitor):

1. Allocate a render target `Image` with appropriate `TextureUsages` (`TEXTURE_BINDING | RENDER_ATTACHMENT`).
2. Spawn a 2D camera with its `RenderTarget::Image` pointing to this texture.
3. Attach `EguiContext::default()` to that camera entity.
4. Draw egui widgets using the camera's specific `EguiContext`.
5. Apply the render target image as the `base_color_texture` of a 3D `StandardMaterial`.

---

## 7. Developer and Verification Commands

```bash
# Verify crate compatibility and compilation:
cargo check 2>&1 | head -n 30

# Inspect local docs for exact bevy_egui 0.42 API:
cargo doc -p bevy_egui --no-deps
grep -rn "EguiPrimaryContextPass" target/doc/bevy_egui/
```

| Task                          | Command                                  |
| :---------------------------- | :--------------------------------------- |
| Check for compile errors      | `cargo check 2>&1 \| head -n 30`         |
| Search for egui systems       | `grep -rn "EguiPrimaryContextPass" src/` |
| Check input gating conditions | `grep -rn "egui_wants_any" src/`         |
| Find image texture bindings   | `grep -rn "add_image" src/`              |

---

## 8. Best Practices and Traps

- **Do:** Schedule primary UI systems into `EguiPrimaryContextPass`.
- **Do:** Protect pointer and keyboard gameplay systems with `run_if(not(egui_wants_any_pointer_input))`.
- **Do:** Return `Result` from your egui systems to use the `contexts.ctx_mut()?` question mark operator cleanly.
- **Don't:** Run heavy ECS queries with mutable world access inside the UI rendering loop unless needed; pass specific query references.
- **Don't:** Forget to spawn a camera (`Camera2d` or `Camera3d`), or egui will have no target to render onto.
