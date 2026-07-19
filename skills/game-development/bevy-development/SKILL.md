---
name: bevy-development
description: Expert guidance for building 2D and 3D games in Bevy 0.19, Rust's data-driven ECS game engine — components, systems, scheduling, states, queries, input, assets, messages, and performance. Use when writing or modernizing Bevy code, laying out ECS data and systems, scheduling or gating systems, handling input/assets/messages, or migrating off deprecated Bevy APIs like bundles, delta_seconds, EventReader, or Parent.
resources:
  - https://r.jina.ai/https://bevyengine.org/learn/migration-guides/
---

# Bevy Game Engine Expert

Patterns for building high-performance, modular games with Bevy — the data-driven ECS
game engine built in Rust.

**Targets Bevy 0.19** (latest stable). Bevy moves fast and breaks APIs between releases;
if a project pins an older version, verify against that version's docs before applying
these snippets. The migration cheat sheet below covers the churn since ~0.14.

## When to Use This Skill

- Developing 2D or 3D games and simulations in Rust with Bevy.
- Structuring ECS data layouts, writing queries, and designing systems.
- Ordering, grouping, and scheduling systems (fixed timesteps, state transitions).
- Handling input, asset loading, and message/event-driven communication.
- Optimizing Bevy performance (parallel queries, sparse sets, change detection, batching).
- Modernizing a stale Bevy codebase off deprecated APIs.

## Reference Files

Load these for depth:

- `resources/learn-migration-guides.md` — Bevy migration guides and release notes.

## Migration Cheat Sheet (older Bevy -> 0.19)

Old code you will meet in stale projects, and its current form:

| Old (deprecated / removed)    | Current (0.19)                                  |
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

## 1. ECS Fundamentals

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
        // delta_secs() — renamed from delta_seconds() in 0.16.
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

## 2. App & Plugin Structure

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

## 3. Scheduling & Ordering

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

## 4. Queries & Change Detection

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

## 5. Performance Patterns

- **Parallel iteration:** for large independent workloads, use `par_iter_mut()`.
- **SparseSet storage:** for frequently added/removed components (buffs, statuses) —
  use `#[component(storage = "SparseSet")]` to optimize storage access patterns.
- **Batch spawning:** `commands.spawn_batch(..)` beats a loop of individual `spawn`
  calls.
- **Change filters:** put `Changed<T>` on reactive systems (UI, animation) to skip
  stable data.

```rust
fn parallel_update(mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}
```

## 6. Developer Commands

```bash
# Fast iterative dev builds via dynamic linking (much faster incremental compiles).
cargo run --features bevy/dynamic_linking

# Optimized build.
cargo run --release
```

| Task                                | Command                                                                          |
| ----------------------------------- | -------------------------------------------------------------------------------- |
| Check for compile errors            | `cargo check 2>&1 \| head -30`                                                   |
| List component definitions          | `grep -rn "derive(Component)" src/ --include="*.rs"`                             |
| Find systems using queries          | `grep -rn "fn.*Query<" src/ --include="*.rs"`                                    |
| Identify state machine declarations | `grep -rn "derive.*States" src/ --include="*.rs"`                                |
| Spot system ordering hooks          | `grep -rn "\.add_systems\|\.chain()\|\.after(\|\.before(" src/ --include="*.rs"` |
| Find deprecated bundle usage        | `grep -rn "Bundle" src/ --include="*.rs"`                                        |
| Find deprecated Time calls          | `grep -rn "delta_seconds\|elapsed_seconds" src/ --include="*.rs"`                |

## 7. Best Practices

- **Do:** access singletons with `Res<T>` rather than `ResMut<T>` unless you mutate —
  read-only access lets the scheduler run more systems concurrently.
- **Do:** use marker components / ZSTs to filter queries cheaply.
- **Do:** apply `Changed<T>` to reactive systems (UI updates, animation) to skip stable
  data.
- **Do:** prefer required components over reintroducing bundle-shaped wrapper structs.
- **Don't:** run heavy nested loops over >1,000 entities without `par_iter_mut()` or a
  cached query.
- **Don't:** poll asset-loaded checks inside gameplay frames — gate them behind a
  loading state.
