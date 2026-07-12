# ECS Reference (Bevy 0.19)

Deep reference for components, required components, storage, relationships/hierarchy,
and spawning. See `../SKILL.md` for the skimmable core.

## Components in Depth

Components are plain data. Derive `Component`; add `Reflect` + `#[reflect(Component)]`
to support the editor, scene serialization, and runtime type inspection.

```rust
use bevy::prelude::*;

// Table storage (default) — best for stable components read every frame.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

// Marker components (Zero-Sized Types) — used purely to filter queries.
#[derive(Component, Default)]
pub struct Player;
```

### SparseSet storage hint

For components that are added and removed frequently (statuses, buffs, tags), use
`SparseSet` storage to avoid archetype fragmentation and expensive table moves.

```rust
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Burning;
```

## Required Components (replaces bundles)

Bundles (`*Bundle`) were deprecated in Bevy 0.15 in favour of **required components**.
Instead of grouping components in a bundle, a component declares the components it needs
with `#[require(...)]`; Bevy inserts them automatically (using their `Default`, or a
provided constructor) when the component is spawned.

```rust
// Spawning `Enemy` also inserts Transform, Visibility, and Health automatically.
#[derive(Component)]
#[require(Transform, Visibility, Health)]
pub struct Enemy;
```

Common migrations from the old bundle world:

| Old bundle (removed/deprecated) | Spawn this instead                              |
| ------------------------------- | ----------------------------------------------- |
| `Camera2dBundle::default()`     | `Camera2d`                                      |
| `Camera3dBundle::default()`     | `Camera3d::default()`                           |
| `SpriteBundle { .. }`           | `Sprite::from_image(handle)` / `Sprite { .. }`  |
| `SpatialBundle::default()`      | `(Transform::default(), Visibility::default())` |
| `AudioBundle { .. }`            | `AudioPlayer(handle)`                           |
| `Text2dBundle { .. }`           | `Text2d::new("..")`                             |

`Transform` requires `GlobalTransform`, and `Visibility` requires the inherited/view
visibility components, so you never insert those by hand — spawning `Transform` or
`Visibility` pulls them in.

```rust
fn spawn_things(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("sprites/player.png")));
    // Old SpatialBundle equivalent — just the components you actually want:
    commands.spawn((Transform::from_xyz(0.0, 5.0, 0.0), Visibility::default()));
}
```

## Resources

Resources are global singletons. (Internally, Bevy 0.19 stores them as components on a
singleton entity, but the `Res` / `ResMut` API is unchanged.)

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

Prefer `Res<T>` over `ResMut<T>` unless you actually mutate — read-only access lets the
scheduler run more systems in parallel.

## Relationships & Hierarchy

The relationships system was overhauled in Bevy 0.16. Key renames:

- The `Parent` component is now `ChildOf`. Read the parent with `child_of.parent()`.
- `Children::iter()` yields `Entity` **by value** (not `&Entity`).
- `with_children`'s closure now receives `ChildSpawnerCommands` (was `ChildBuilder`).
- `despawn_recursive()` is gone — plain `despawn()` now despawns descendants too.

```rust
fn spawn_hierarchical_entity(mut commands: Commands) {
    commands
        .spawn((Transform::default(), Visibility::default(), Name::new("Parent")))
        .with_children(|parent| {
            parent.spawn((Transform::from_xyz(10.0, 0.0, 0.0), Name::new("Child Offset")));
        });
}

fn child_lookup_system(
    parents: Query<(&Children, &Transform)>,
    transforms: Query<&Transform>,
) {
    for (children, _parent_transform) in &parents {
        for child in children.iter() {
            if let Ok(child_transform) = transforms.get(child) {
                let _ = child_transform;
            }
        }
    }
}

fn read_parent(query: Query<&ChildOf>) {
    for child_of in &query {
        let _parent: Entity = child_of.parent();
    }
}
```

Set a parent directly by inserting `ChildOf`:

```rust
commands.entity(child).insert(ChildOf(parent));
```

## Spawning Patterns

### Batch spawning

Spawning many entities at once with `spawn_batch` is far faster than a loop of `spawn`.

```rust
fn spawn_asteroids(mut commands: Commands) {
    commands.spawn_batch((0..1000).map(|i| (
        Asteroid,
        Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0),
        Velocity(Vec2::new(0.0, -10.0)),
    )));
}
```

### Custom spawn helpers on `Commands`

Extend `Commands` with an extension trait to keep spawn sites tidy. With required
components you no longer list `GlobalTransform` / visibility by hand.

```rust
pub trait SpawnHelpersExt {
    fn spawn_player(&mut self, position: Vec3) -> Entity;
}

impl SpawnHelpersExt for Commands<'_, '_> {
    fn spawn_player(&mut self, position: Vec3) -> Entity {
        self.spawn((
            Player,
            Health { current: 100.0, max: 100.0 },
            Transform::from_translation(position),
        ))
        .id()
    }
}

// Usage: commands.spawn_player(Vec3::ZERO);
```
