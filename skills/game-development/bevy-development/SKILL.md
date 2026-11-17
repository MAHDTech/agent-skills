---
name: bevy-development
description: Build or modernize Bevy games and simulations in Rust. Use for ECS design, scheduling, input, assets, messages, performance, or migration between Bevy APIs.
resources:
  - https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/learn/migration-guides/0.18-to-0.19.md
  - https://raw.githubusercontent.com/bevyengine/bevy-website/main/content/learn/migration-guides/0.19-to-0.20.md
---

# Bevy game development

Patterns for building modular games and simulations with Bevy, the data-driven ECS game engine built in Rust.

These snippets target Bevy 0.20 (the cheat sheet and examples were checked against the 0.20.1 crates). Bevy breaks APIs between releases, so read the project's pinned `bevy` version in `Cargo.toml` first; for any other version, or when upgrading, check the migration guides before applying these snippets: the bundled `resources/auto/` copies cover 0.18 to 0.20, and `https://bevy.org/learn/migration-guides/` has the rest.

## Migration cheat sheet (older Bevy to 0.20)

Old code you will meet in stale projects, and its current form:

| Old (deprecated / removed)    | Current (0.20)                                  |
| ----------------------------- | ----------------------------------------------- |
| `time.delta_seconds()`        | `time.delta_secs()`                             |
| `time.elapsed_seconds()`      | `time.elapsed_secs()`                           |
| `*Bundle` types               | required components (`#[require(..)]`)          |
| `Camera2dBundle::default()`   | `Camera2d`                                      |
| `SpriteBundle { .. }`         | `Sprite::from_image(handle)`                    |
| `SpatialBundle::default()`    | `(Transform::default(), Visibility::default())` |
| `Parent` component            | `ChildOf` (read via `child_of.parent()`)        |
| `despawn_recursive()`         | `despawn()`                                     |
| `Query::single()` panics      | `single()` returns `Result`                     |
| `EventReader` / `EventWriter` | `MessageReader` / `MessageWriter`               |
| `#[derive(Event)]` (buffered) | `#[derive(Message)]`                            |
| `App::add_event` / `.send()`  | `App::add_message` / `.write()`                 |
| `Trigger<E>` (observers)      | `On<E>`                                         |
| `Input<T>`                    | `ButtonInput<T>`                                |

## 1. ECS fundamentals

Components are plain data structs; systems are functions; resources are global
singletons. Use required components for default setups, child-parent relationships
for hierarchy, and query filters for optimizing storage access.

```rust
use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Default)]
pub struct Player; // marker (ZST) for filtering queries

fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in &mut query {
        // delta_secs() - renamed from delta_seconds() in 0.15.
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

## 2. App and plugin structure

Split game logic into plugins to decouple features and keep incremental compiles fast.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // windowing, rendering, input, ...
        .add_plugins(PlayerPlugin)
        .init_resource::<GameScore>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, score_display_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d); // was Camera2dBundle::default()
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, movement_system);
    }
}
```

## 3. Scheduling and ordering

Systems run in parallel unless their accesses conflict. Add ordering only for real data
dependencies. Use system sets, states, and run conditions to organize and gate
systems.

```rust
// Sequential chain.
app.add_systems(Update, (read_input, apply_movement, check_collisions).chain());

// Gate on state or resource presence.
app.add_systems(Update, gameplay.run_if(in_state(GameState::Playing)));

// Deterministic simulation at a fixed rate (default 64 Hz).
app.insert_resource(Time::<Fixed>::from_hz(64.0));
app.add_systems(FixedUpdate, physics_step);
```

## 4. Queries and change detection

Filter with `With` / `Without`; react to changes with `Changed<T>` / `Added<T>`. Use
`ParamSet` to resolve system query conflicts, and `Ref<T>` to inspect component
metadata and change ticks.

```rust
fn enemy_targeting(
    mut attackers: Query<&mut Target, With<Enemy>>,
    targets: Query<&Transform, (With<Player>, Without<Dead>)>,
) { /* only alive players */ }

fn on_transform_change(query: Query<&Transform, Changed<Transform>>) {
    for _transform in &query { /* only entities whose Transform changed */ }
}
```

Read input via `ButtonInput<T>`; load assets via `AssetServer`; use messages (buffered
events, renamed from `EventReader`/`EventWriter` in 0.17) for decoupled communication.
Use observer events for immediate reaction to trigger events.

```rust
fn player_input(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) { /* jump */ }
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("sprites/player.png");
    commands.spawn(Sprite::from_image(texture)); // was SpriteBundle
}

#[derive(Message)] // was #[derive(Event)] for buffered events
struct Scored(u32);
```

## 5. Performance patterns

- **Parallel iteration:** for large independent workloads, use `par_iter_mut()`.
- **SparseSet storage:** for frequently added/removed components (buffs, statuses) -
  use `#[component(storage = "SparseSet")]` to optimize storage access patterns.
- **Batch spawning:** `commands.spawn_batch(..)` beats a loop of individual `spawn`
  calls.

```rust
fn parallel_update(mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}
```

## 6. Developer commands

```bash
# Fast iterative dev builds via dynamic linking (much faster incremental compiles).
cargo run --features bevy/dynamic_linking

# Optimized build.
cargo run --release
```

## 7. Practices

- Access singletons with `Res<T>`, and take `ResMut<T>` only in systems that mutate;
  read-only access lets the scheduler run more systems concurrently.
- Use marker components (ZSTs) to filter queries cheaply.
- Apply `Changed<T>` to reactive systems (UI updates, animation) to skip stable data.
- Use required components instead of bundle-shaped wrapper structs.
- Iterate workloads over roughly 1,000 entities with `par_iter_mut()`.
- Gate asset-loaded checks behind a loading state so gameplay frames never poll them.
