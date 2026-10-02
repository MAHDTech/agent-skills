# Bevy egui Integration Guide

A technical reference for integrating `egui` into Bevy 0.19 using `bevy_egui` 0.40-0.42.

---

## 1. Version Compatibility Matrix

`bevy_egui` is tightly bound to Bevy releases. Always match your crate versions according to this table:

| Bevy Version | Compatible bevy_egui | Embedded egui Version |
| :--- | :--- | :--- |
| **0.19** | **0.40 - 0.42** | **0.36** |
| 0.18 | 0.39 | 0.35 |
| 0.17 | 0.37 - 0.38 | 0.33 - 0.34 |
| 0.16 | 0.34 - 0.36 | 0.31 - 0.32 |
| 0.15 | 0.31 - 0.33 | 0.29 - 0.30 |

In `Cargo.toml`:

```toml
[dependencies]
bevy = "0.19.0"
bevy_egui = "0.42.0"
```

---

## 2. Scheduling: EguiPrimaryContextPass

In Bevy 0.19, `bevy_egui` schedules primary window UI passes into the dedicated `EguiPrimaryContextPass` schedule (or system set):

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(EguiPrimaryContextPass, render_ui_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn render_ui_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Debug Panel").show(contexts.ctx_mut()?, |ui| {
        ui.label("Running in EguiPrimaryContextPass");
    });
    Ok(())
}
```

Returning `Result` from the UI system allows using the `?` operator on `contexts.ctx_mut()?`, cleanly handling frames before window initialization.

---

## 3. Input Absorption (Preventing Click-Through)

A common issue in game development is "click-through": clicking a button in an egui window triggers an in-game weapon fire or unit selection in Bevy.

`bevy_egui` provides run conditions to cleanly gate gameplay systems whenever egui consumes pointer or keyboard focus.

```rust
use bevy::prelude::*;
use bevy_egui::input::{egui_wants_any_keyboard_input, egui_wants_any_pointer_input};

pub struct GameplayInputPlugin;

impl Plugin for GameplayInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // Only process player movement if egui is not focused on an input field:
                player_movement_system.run_if(not(egui_wants_any_keyboard_input)),
                // Only process mouse clicks/aiming if pointer is outside egui windows:
                weapon_fire_system.run_if(not(egui_wants_any_pointer_input)),
                camera_orbit_system.run_if(not(egui_wants_any_pointer_input)),
            ),
        );
    }
}
```

---

## 4. Texture and Asset Integration

Render Bevy `Handle<Image>` assets inside egui widgets:

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

#[derive(Resource)]
struct IconAssets {
    logo: Handle<Image>,
}

fn display_bevy_texture_in_ui(
    mut contexts: EguiContexts,
    icons: Res<IconAssets>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    // Register the Bevy image handle with egui (returns egui::TextureId):
    let texture_id = contexts.add_image(icons.logo.clone());

    egui::Window::new("Asset Viewer").show(ctx, |ui| {
        ui.label("Bevy Asset rendered in egui:");
        // SizedTexture takes TextureId and dimensions:
        ui.image(egui::load::SizedTexture::new(texture_id, [128.0, 128.0]));
    });

    Ok(())
}
```

---

## 5. Game State Gating (Pause Menu and HUD)

Connect egui interfaces to Bevy `States`:

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                EguiPrimaryContextPass,
                (
                    hud_system.run_if(in_state(GameState::Playing)),
                    pause_menu_system.run_if(in_state(GameState::Paused)),
                ),
            );
    }
}

fn hud_system(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;
    egui::Area::new(egui::Id::new("hud_top_left"))
        .fixed_pos([16.0, 16.0])
        .show(ctx, |ui| {
            ui.heading("Health: 100%");
        });
    Ok(())
}

fn pause_menu_system(
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    egui::Window::new("Game Paused")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            if ui.button("Resume").clicked() {
                next_state.set(GameState::Playing);
            }
            if ui.button("Quit to Desktop").clicked() {
                // handle quit
            }
        });
    Ok(())
}
```

---

## 6. Multi-Camera and Split-Screen Viewports

In Bevy 0.19, distinct `EguiContext` components can be attached directly to Camera entities. This enables split-screen multiplayer where each player has their own independent UI context rendered over their camera viewport.

```rust
use bevy::prelude::*;
use bevy_egui::{EguiContext, EguiPlugin, egui};

fn setup_split_screen_cameras(mut commands: Commands) {
    // Player 1 Camera with independent EguiContext:
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        EguiContext::default(),
    ));

    // Player 2 Camera with independent EguiContext:
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        EguiContext::default(),
    ));
}
```
