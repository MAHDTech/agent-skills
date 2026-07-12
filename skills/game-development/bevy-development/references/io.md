# Input, Assets & Messaging Reference (Bevy 0.19)

Deep reference for input, asset loading, buffered messages, and observer events. See
`../SKILL.md` for the skimmable core.

## Input Handling

Read keyboard and mouse buttons through the `ButtonInput<T>` resource (renamed from
`Input<T>` back in 0.13). Note that `Query::single()` now returns a `Result` (0.16), so
window access must handle the error/empty case.

```rust
fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    if keyboard.pressed(KeyCode::KeyW) { /* held this frame */ }
    if keyboard.just_pressed(KeyCode::Space) { /* pressed this frame */ }

    if mouse.just_pressed(MouseButton::Left) {
        // single() -> Result in Bevy 0.16+; handle the empty/multiple cases.
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                println!("Click at {cursor_pos:?}");
            }
        }
    }
}
```

## Asset Management

Load assets asynchronously through the `AssetServer`. `load` returns a `Handle<T>`
immediately; the data streams in on a background thread.

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

Gate "assets loaded?" checks behind a loading state rather than polling every frame in
gameplay systems.

## Messages (buffered events)

Bevy 0.17 split the old "events" into two concepts. **Buffered events are now
"messages".** What used to be `EventReader` / `EventWriter` / `#[derive(Event)]` for the
buffered, poll-every-frame pattern is now `MessageReader` / `MessageWriter` /
`#[derive(Message)]`.

Rename table (0.16 -> 0.17):

| Old                           | New                           |
| ----------------------------- | ----------------------------- |
| `#[derive(Event)]` (buffered) | `#[derive(Message)]`          |
| `EventReader<E>`              | `MessageReader<M>`            |
| `EventWriter<E>`              | `MessageWriter<M>`            |
| `Events<E>`                   | `Messages<M>`                 |
| `App::add_event::<E>()`       | `App::add_message::<M>()`     |
| `EventWriter::send(..)`       | `MessageWriter::write(..)`    |
| `World::send_event(..)`       | `World::write_message(..)`    |
| `Commands::send_event(..)`    | `Commands::write_message(..)` |

```rust
#[derive(Message)]
struct DamageEvent {
    target: Entity,
    amount: f32,
}

fn register(app: &mut App) {
    app.add_message::<DamageEvent>();
}

fn deal_damage(mut writer: MessageWriter<DamageEvent>, target: Entity) {
    writer.write(DamageEvent { target, amount: 10.0 });
}

fn apply_damage(mut reader: MessageReader<DamageEvent>, mut healths: Query<&mut Health>) {
    for msg in reader.read() {
        if let Ok(mut health) = healths.get_mut(msg.target) {
            health.current -= msg.amount;
        }
    }
}
```

## Observer events

The `Event` trait is now reserved for **observable** events — things that are triggered
and reacted to immediately by observers, rather than buffered and polled. Reach for these
when you want an instant, targeted reaction instead of once-per-frame draining.

- Derive `#[derive(Event)]`.
- Trigger with `commands.trigger(MyEvent)` (or on the `World`).
- Observe with `app.add_observer(..)` or `commands.entity(e).observe(..)`.
- The observer's system parameter is `On<E>` (renamed from `Trigger<E>` in 0.17).

```rust
#[derive(Event)]
struct GameOver;

fn setup(app: &mut App) {
    app.add_observer(|_trigger: On<GameOver>| {
        // Runs the moment GameOver is triggered.
    });
}

fn end_game(mut commands: Commands) {
    commands.trigger(GameOver);
}
```

A type can implement both `Message` and `Event`, but most types want only one.
