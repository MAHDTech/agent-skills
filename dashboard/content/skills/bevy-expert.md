+++
title = "bevy-expert"
description = "Expert developer guidance on Bevy, Rust's data-driven game engine, covering ECS structure, input, assets, events, states, system scheduling, and performance tuning."
date = 2026-06-04
[extra]
triggers = ["bevy","bevy game engine","bevy-expert","Rust game engine","Rust gamedev"]
mermaid = false
is_command = false
+++


# Bevy Game Engine Expert

Expert knowledge and patterns for developing high-performance, modular games with Bevy—the data-driven game engine built in Rust.

## When to Use This Skill

- Developing 2D or 3D games and simulations in Rust using Bevy.
- Structuring ECS data layouts, writing queries, and designing systems.
- Ordering, grouping, and scheduling systems (including fixed timesteps and state transitions).
- Handling input, asset loading, event-driven communication, and UI development.
- Optimizing Bevy game performance (parallel queries, sparse sets, change detection, batching).

---

## 1. ECS Fundamentals

Bevy uses a data-oriented Entity Component System (ECS) architecture.

### Defining Components

Components are plain data structs. Derive `Component` and `Reflect` to support reflection and editor tooling.

```rust
use bevy::prelude::*;

// Table storage (default) - best for stable components accessed regularly
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

// Marker components (Zero-Sized Types) - used to filter queries
#[derive(Component, Default)]
pub struct Player;
```

### Writing Systems

Systems are normal Rust functions that read/write resources and query components.

```rust
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}
```

### Managing Resources

Resources represent global, unique singleton data. Access them via `Res` (read-only) or `ResMut` (read-write).

```rust
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameScore {
    pub value: u32,
}

fn score_display_system(score: Res<GameScore>) {
    if score.is_changed() {
        println!("Current Score: {}", score.value);
    }
}
```

---

## 2. App & Plugin Structure

Organize your game logic using modular plugins. This keeps compile times manageable and code decoupled.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        // Default plugins supply windowing, rendering, inputs, etc.
        .add_plugins(DefaultPlugins)
        // Custom game plugins
        .add_plugins(PlayerPlugin)
        // Global resources
        .init_resource::<GameScore>()
        // Startup systems run once at launch
        .add_systems(Startup, setup_camera)
        // Systems run on every frame update
        .add_systems(Update, score_display_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Organize features inside standalone plugins
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, movement_system);
    }
}
```

---

## 3. System Scheduling & Ordering

By default, Bevy runs systems in parallel if their resource/query accesses do not conflict. Use ordering constraint utilities to enforce execution dependencies.

### Ordering & Chains

Enforce execution sequences using `.before()`, `.after()`, or `.chain()`.

```rust
fn configure_system_order(app: &mut App) {
    // Execution chain (runs input -> movement -> collision -> cleanup sequentially)
    app.add_systems(Update, (
        read_input,
        apply_movement,
        check_collisions,
        cleanup_dead_entities,
    ).chain());

    // Single dependencies
    app.add_systems(Update, render_particles.after(check_collisions));
}
```

### System Sets

Group related systems together to apply conditions or orderings to the entire set.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSet {
    Movement,
    Collision,
}

fn configure_system_sets(app: &mut App) {
    // Configure set execution order
    app.configure_sets(Update, PhysicsSet::Collision.after(PhysicsSet::Movement));

    // Register systems into sets
    app.add_systems(Update, (
        apply_gravity.in_set(PhysicsSet::Movement),
        apply_velocity.in_set(PhysicsSet::Movement),
        resolve_collisions.in_set(PhysicsSet::Collision),
    ));
}
```

### Run Conditions

Gate system execution based on states, resources, or custom conditions.

```rust
fn configure_conditions(app: &mut App) {
    app.add_systems(Update, (
        gameplay_logic.run_if(in_state(GameState::Playing)),
        debug_grid.run_if(resource_exists::<DebugConfig>),
        spawn_enemies.run_if(should_spawn_enemy),
    ));
}

fn should_spawn_enemy(time: Res<Time>, enemies: Query<&Enemy>) -> bool {
    enemies.iter().count() < 10 && time.elapsed_seconds() > 5.0
}
```

### Fixed Timestep

Use `FixedUpdate` for gameplay simulations (e.g., physics, networking) that require a consistent update interval independent of frame rate.

```rust
fn configure_fixed_timestep(app: &mut App) {
    // Set fixed frequency (60Hz)
    app.insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0));

    app.add_systems(FixedUpdate, (
        physics_step,
        integrate_positions,
    ));
}
```

---

## 4. Query & Change Detection Patterns

Optimize iteration patterns and resolve data access conflicts with query tools.

### Query Filters

Use `With` and `Without` to filter queried entities efficiently without reading their data.

```rust
fn enemy_targeting(
    mut attackers: Query<&mut Target, With<Enemy>>,
    targets: Query<&Transform, (With<Player>, Without<Dead>)>,
) {
    // Queries only alive players
}
```

### Conflict Resolution with ParamSet

Use `ParamSet` when multiple queries in the same system request conflicting mutable access to components.

```rust
fn combat_system(
    mut param_set: ParamSet<(
        Query<&mut Health, With<Player>>,
        Query<&mut Health, With<Enemy>>,
    )>,
) {
    // Access player health mutably
    for mut health in param_set.p0().iter_mut() {
        health.current += 1.0;
    }

    // Access enemy health mutably
    for mut health in param_set.p1().iter_mut() {
        health.current -= 10.0;
    }
}
```

### Parent-Child Hierarchies

Establish nested entity structures. Parents propagate their transforms to children.

```rust
fn spawn_hierarchical_entity(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), Name::new("Parent")))
        .with_children(|parent| {
            parent.spawn((SpriteBundle::default(), Name::new("Child Offset")));
        });
}

// Queries child relations
fn child_lookup_system(
    children_query: Query<(&Children, &Transform)>,
    transforms: Query<&Transform>,
) {
    for (children, parent_transform) in &children_query {
        for &child in children.iter() {
            if let Ok(child_transform) = transforms.get(child) {
                // Process child transform
            }
        }
    }
}
```

### Change Detection

Query only entities with components that have recently been added or modified.

```rust
fn on_transform_change(
    query: Query<&Transform, Changed<Transform>>,
) {
    for transform in &query {
        // Triggered only if Transform changed this frame
    }
}

fn on_new_enemy(
    query: Query<Entity, Added<Enemy>>,
) {
    for entity in &query {
        // Runs only once when Enemy component is added
    }
}

// Read changes manually with Ref<T>
fn manual_change_tracking(query: Query<Ref<Transform>, With<Player>>) {
    for transform in &query {
        if transform.is_changed() {
            // Transform changed
        }
    }
}
```

---

## 5. Input & Asset Handling

### Input Handling

Read keyboard, mouse, and gamepads via resources.

```rust
fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    // Keyboard buttons
    if keyboard.pressed(KeyCode::KeyW) { /* move up */ }
    if keyboard.just_pressed(KeyCode::Space) { /* jump */ }

    // Mouse buttons & position
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = windows.single().cursor_position() {
            println!("Click pos: {:?}", cursor_pos);
        }
    }
}
```

### Asset Management

Asynchronously load textures, audio, and configurations via the `AssetServer`.

```rust
#[derive(Resource)]
pub struct GameAssets {
    pub player_texture: Handle<Image>,
    pub font: Handle<Font>,
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        player_texture: asset_server.load("sprites/player.png"),
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    });
}
```

---

## 6. Performance Patterns

### Parallel Iteration

Leverage parallel execution on multi-core systems when updating large counts of independent entities.

```rust
fn parallel_update_system(mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}
```

### SparseSet Storage Hint

For components added or removed frequently (like statuses/buffs), use `SparseSet` storage to prevent archetype fragmentation.

```rust
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Burning;
```

### Batch Spawning

Spawning entities in batches is significantly faster than individual `spawn` commands.

```rust
fn spawn_asteroids(mut commands: Commands) {
    commands.spawn_batch((0..1000).map(|i| (
        Asteroid,
        Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0),
        Velocity(Vec2::new(0.0, -10.0)),
    )));
}
```

### Custom Entity Commands via Traits

Extend the capabilities of `Commands` to cleanly spawn styled assets or complex hierarchies.

```rust
pub trait SpawnHelpersExt {
    fn spawn_player(&mut self, position: Vec3) -> Entity;
}

impl<'w, 's> SpawnHelpersExt for Commands<'w, 's> {
    fn spawn_player(&mut self, position: Vec3) -> Entity {
        self.spawn((
            Player,
            Health { current: 100.0, max: 100.0 },
            Transform::from_translation(position),
            GlobalTransform::default(),
            InheritedVisibility::default(),
        )).id()
    }
}

// Usage in systems:
// commands.spawn_player(Vec3::ZERO);
```

---

## 7. Developer Commands & Agentic Optimizations

Use these shell commands to build, profile, and search Bevy codebases.

### Compile & Run Configs

Configure Cargo to build fast and run with debugging capabilities.

```bash
# Run project in debug mode with dynamic linking (vastly speeds up incremental compilation times)
cargo run --features bevy/dynamic_linking

# Build project with release optimizations
cargo run --release
```

### Agentic Grep Cheatsheet

| Task                                 | Command                                                                          |
| ------------------------------------ | -------------------------------------------------------------------------------- |
| Check for compile or syntax errors   | `cargo check 2>&1 \| head -30`                                                   |
| List all component definitions       | `grep -rn "derive(Component)" src/ --include="*.rs"`                             |
| Find systems using queries           | `grep -rn "fn.*Query<" src/ --include="*.rs"`                                    |
| Identify state machine declarations  | `grep -rn "derive.*States" src/ --include="*.rs"`                                |
| Spot system ordering hooks           | `grep -rn "\.add_systems\|\.chain()\|\.after(\|\.before(" src/ --include="*.rs"` |
| Find `ParamSet` conflict workarounds | `grep -rn "ParamSet" src/ --include="*.rs"`                                      |
| List crate dependencies              | `cargo metadata --format-version=1 \| jq -r '.packages[0].dependencies[].name'`  |

---

## 8. Best Practices

- 🟢 **Do:** Minimize resource locking. Access singletons with `Res<T>` instead of `ResMut<T>` unless modification is necessary, allowing Bevy's scheduler to execute systems concurrently.
- 🟢 **Do:** Use marker components and Zero-Sized Types (ZSTs) to filter queries quickly.
- 🟢 **Do:** Apply the `Changed<T>` filter to reactive systems (e.g., UI updates, animation triggers) to avoid processing stable data.
- 🔴 **Don't:** Place heavy nested loops or complex algorithms inside systems without using `par_iter()` or caching queries when handling >1,000 entities.
- 🔴 **Don't:** Run heavy resource initialization or loading operations inside standard update frames; offload load checks to dedicated loading states or use standard asset events.

