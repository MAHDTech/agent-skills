# Systems, Scheduling & State Reference (Bevy 0.19)

Deep reference for ordering, system sets, run conditions, the fixed timestep, states,
query conflict resolution, change detection, and parallel iteration. See `../SKILL.md`
for the skimmable core.

By default Bevy runs systems in parallel whenever their component/resource accesses do
not conflict. Add ordering only where a real data dependency exists.

## Ordering & Chains

```rust
fn configure_system_order(app: &mut App) {
    // Run these in sequence: input -> movement -> collision -> cleanup.
    app.add_systems(Update, (
        read_input,
        apply_movement,
        check_collisions,
        cleanup_dead_entities,
    ).chain());

    // A single explicit dependency.
    app.add_systems(Update, render_particles.after(check_collisions));
}
```

## System Sets

Group systems under a `SystemSet` to order or gate them all at once.

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSet {
    Movement,
    Collision,
}

fn configure_system_sets(app: &mut App) {
    app.configure_sets(Update, PhysicsSet::Collision.after(PhysicsSet::Movement));

    app.add_systems(Update, (
        apply_gravity.in_set(PhysicsSet::Movement),
        apply_velocity.in_set(PhysicsSet::Movement),
        resolve_collisions.in_set(PhysicsSet::Collision),
    ));
}
```

## Run Conditions

Gate a system on state, resource presence, or any custom `-> bool` system.

```rust
fn configure_conditions(app: &mut App) {
    app.add_systems(Update, (
        gameplay_logic.run_if(in_state(GameState::Playing)),
        debug_grid.run_if(resource_exists::<DebugConfig>),
        spawn_enemies.run_if(should_spawn_enemy),
    ));
}

fn should_spawn_enemy(time: Res<Time>, enemies: Query<&Enemy>) -> bool {
    // Note: elapsed_seconds() was renamed to elapsed_secs() in Bevy 0.16.
    enemies.iter().count() < 10 && time.elapsed_secs() > 5.0
}
```

## Fixed Timestep

Put deterministic simulation (physics, networking) in `FixedUpdate`, which runs at a
fixed rate independent of frame rate. The default rate is 64 Hz.

```rust
fn configure_fixed_timestep(app: &mut App) {
    // Set the rate by frequency...
    app.insert_resource(Time::<Fixed>::from_hz(64.0));
    // ...or by period: Time::<Fixed>::from_seconds(1.0 / 64.0)

    app.add_systems(FixedUpdate, (
        physics_step,
        integrate_positions,
    ).chain());
}
```

Inside `FixedUpdate` systems, `Res<Time>` reports the fixed delta. Use `time.delta_secs()`
(renamed from `delta_seconds()` in 0.16).

## States

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

fn configure_states(app: &mut App) {
    app.init_state::<GameState>()
        // OnEnter / OnExit schedules fire on transitions.
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(OnExit(GameState::Playing), teardown_level)
        .add_systems(Update, gameplay.run_if(in_state(GameState::Playing)));
}

// Request a transition by setting NextState; it applies at the next transition point.
fn pause_game(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Paused);
}
```

Use `app.insert_state(GameState::Menu)` instead of `init_state` when the initial state
is not the `Default`.

## Query Conflict Resolution with ParamSet

When one system needs multiple queries with overlapping mutable access, wrap them in a
`ParamSet` and access one at a time via `p0()`, `p1()`, ...

```rust
fn combat_system(
    mut param_set: ParamSet<(
        Query<&mut Health, With<Player>>,
        Query<&mut Health, With<Enemy>>,
    )>,
) {
    for mut health in param_set.p0().iter_mut() {
        health.current += 1.0;
    }
    for mut health in param_set.p1().iter_mut() {
        health.current -= 10.0;
    }
}
```

## Change Detection

Query only what changed this tick with the `Changed<T>` and `Added<T>` filters, or read
change flags manually with `Ref<T>`.

```rust
fn on_transform_change(query: Query<&Transform, Changed<Transform>>) {
    for _transform in &query {
        // Runs only for entities whose Transform changed this tick.
    }
}

fn on_new_enemy(query: Query<Entity, Added<Enemy>>) {
    for _entity in &query {
        // Runs once, the tick the Enemy component is added.
    }
}

fn manual_change_tracking(query: Query<Ref<Transform>, With<Player>>) {
    for transform in &query {
        if transform.is_changed() {
            // Inspect the flag directly.
        }
    }
}
```

## Parallel Iteration

For large numbers of independent entities, spread work across cores with
`par_iter_mut().for_each(..)`.

```rust
fn parallel_update_system(mut query: Query<(&mut Transform, &Velocity)>) {
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0);
    });
}
```
