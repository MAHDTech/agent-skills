Version: 2.0.0

On this page

This guide covers the breaking changes between SpacetimeDB 1.0 and 2.0
and how to update your code.

## Overview of Changes

SpacetimeDB 2.0 introduces a new WebSocket protocol (v2) and SDK with
several breaking changes aimed at simplifying the programming model and
improving security:

1.  **Reducer callbacks removed** -- replaced by event tables and
    per-call `_then()` callbacks
2.  **`light_mode` removed** -- no longer necessary since reducer events
    are no longer broadcast
3.  **`CallReducerFlags` removed** -- `NoSuccessNotify` and
    `set_reducer_flags()` are gone
4.  **Event tables introduced** -- a new table type for publishing
    transient events to subscribers
5.  **Confirmed reads enabled by default** -- subscription updates and
    SQL results are only sent after the transaction is confirmed durable

## Reducer Callbacks

### What changed

In 1.0, you could register global callbacks on reducers that would fire
whenever *any* client called that reducer and you were subscribed to
affected rows:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
conn.reducers.onDealDamage((ctx, { target, amount }) => {
  console.log(`Someone called dealDamage with args: (${target}, ${amount})`);
});
```

``` codeBlockStandalone_LlrK
// 1.0-style global reducer callback semantics (no longer true in 2.0)
conn.Reducers.OnDealDamage += (ctx, target, amount) =>
{
    Console.WriteLine($"Someone called DealDamage with args: ({target}, {amount})");
};
```

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
conn.reducers.on_deal_damage(|ctx, target, amount| {
    println!("Someone called deal_damage with args: ({target}, {amount})");
});
```

``` codeBlockStandalone_LlrK
// 1.0-style global reducer callback semantics (no longer true in 2.0)
UFUNCTION()
void OnDealDamage(const FReducerEventContext& Context, const FSpacetimeDBIdentity& Target, int32 Amount)
{
    UE_LOG(LogTemp, Log, TEXT("Someone called DealDamage with args: (%s, %d)"), *Target.ToString(), Amount);
}

Conn->Reducers->OnDealDamage.AddDynamic(this, &AMyActor::OnDealDamage);
```

In 2.0, global reducer callbacks no longer exist. The server does not
broadcast reducer argument data to other clients. Instead, you have two
options:

### Option A: Per-call result callbacks (`_then()`)

If you only need to know the result of a reducer *you* called, you can
await the result or use the `_then()` variant:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
try {
  await ctx.reducers.dealDamage({ target, amount });
  console.log(`You called dealDamage with args: (${target}, ${amount})`);
} catch (err) {
  if (err instanceof SenderError) {
    console.log(`You made an error: ${err}`)
  } else if (err instanceof InternalError) {
    console.log(`The server had an error: ${err}`);
  }
}
```

``` codeBlockStandalone_LlrK
// 2.0 -- per-call callback on the calling connection
conn.Reducers.OnDealDamage += (ctx, _, _) =>
{
    if (ctx.Event.Status is Status.Committed)
    {
        Console.WriteLine("Reducer succeeded");
    }
    else if (ctx.Event.Status is Status.Failed(var reason))
    {
        Console.WriteLine($"Reducer failed: {reason}");
    }
    else if (ctx.Event.Status is Status.OutOfEnergy)
    {
        Console.WriteLine("Reducer failed: out of energy");
    }
};

conn.Reducers.DealDamage(target, amount);
```

``` codeBlockStandalone_LlrK
// 2.0 -- per-call callback
ctx.reducers.deal_damage_then(target, amount, |ctx, result| {
    match result {
        Ok(Ok(())) => println!("Reducer succeeded"),
        Ok(Err(err)) => println!("Reducer failed: {err}"),
        Err(internal) => println!("Internal error: {internal:?}"),
    }
}).unwrap();
```

The fire-and-forget form still works:

``` codeBlockStandalone_LlrK
// 2.0 -- fire and forget (unchanged)
ctx.reducers.deal_damage(target, amount).unwrap();
```

``` codeBlockStandalone_LlrK
// 2.0 -- per-call callback on the calling connection
UFUNCTION()
void OnDealDamage(const FReducerEventContext& Context, const FSpacetimeDBIdentity& Target, int32 Amount)
{
    if (Context.Event.Status.IsCommitted())
    {
        UE_LOG(LogTemp, Log, TEXT("Reducer succeeded"));
    }
    else if (Context.Event.Status.IsFailed())
    {
        UE_LOG(LogTemp, Error, TEXT("Reducer failed: %s"), *Context.Event.Status.GetAsFailed());
    }
    else if (Context.Event.Status.IsOutOfEnergy())
    {
        UE_LOG(LogTemp, Error, TEXT("Reducer failed: out of energy"));
    }
}

Conn->Reducers->OnDealDamage.AddDynamic(this, &AMyActor::OnDealDamage);
Conn->Reducers->DealDamage(Target, Amount);
```

In Unreal, there is no `_then()` method. The generated `On<Reducer>`
delegate is correlated by `request_id` and only fires on the connection
that called the reducer.

### Option B: Event tables (recommended for most use cases)

If you need *other* clients to observe that something happened (the
primary use case for 1.0 reducer callbacks), create an event table and
insert into it from your reducer.

- TypeScript
- C#
- Rust
- C++

**Server (module) -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (reducer args were automatically broadcast)
spacetimedb.reducer('deal_damage', { target: t.identity(), amount: t.u32() }, (ctx, { target, amount }) => {
  // update game state
});
```

**Server (module) -- after:**

``` codeBlockStandalone_LlrK
// 2.0 server -- explicitly publish events via an event table
const damageEvent = table({ name: 'damage_event', event: true }, {
    target: t.identity(),
    amount: t.u32(),
})
// schema() takes an object: schema({ damageEvent }), never schema(damageEvent)
const spacetimedb = schema({ damageEvent });

export const dealDamage = spacetimedb.reducer({ target: t.identity(), amount: t.u32() }, (ctx, { target, amount }) => {
  ctx.db.damageEvent.insert({ target, amount });
});
```

**Server (module) -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (reducer args were automatically broadcast)
[SpacetimeDB.Reducer]
public static void DealDamage(ReducerContext ctx, Identity target, uint amount)
{
    // update game state...
}
```

**Server (module) -- after:**

``` codeBlockStandalone_LlrK
// 2.0 server -- explicitly publish events via an event table
[SpacetimeDB.Table(Accessor = "DamageEvent", Public = true, Event = true)]
public partial struct DamageEvent
{
    public Identity Target;
    public uint Amount;
}

[SpacetimeDB.Reducer]
public static void DealDamage(ReducerContext ctx, Identity target, uint amount)
{
    // update game state...

    ctx.Db.DamageEvent.Insert(new DamageEvent
    {
        Target = target,
        Amount = amount,
    });
}
```

**Server (module) -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (reducer args were automatically broadcast)
#[spacetimedb::reducer]
fn deal_damage(ctx: &ReducerContext, target: Identity, amount: u32) {
    // update game state...
}
```

**Server (module) -- after:**

``` codeBlockStandalone_LlrK
// 2.0 server -- explicitly publish events via an event table
use spacetimedb::{table, reducer, ReducerContext, Table, Identity};

#[spacetimedb::table(accessor = damage_event, public, event)]
pub struct DamageEvent {
    pub target: Identity,
    pub amount: u32,
}

#[reducer]
fn deal_damage(ctx: &ReducerContext, target: Identity, amount: u32) {
    // update game state...

    ctx.db.damage_event().insert(DamageEvent { target, amount });
}
```

**Server (module) -- before:**

``` codeBlockStandalone_LlrK
// 1.0-style reducer arguments were effectively observable through reducer callbacks
SPACETIMEDB_REDUCER(deal_damage, ReducerContext ctx, Identity target, uint32_t amount) {
    // update game state...
    return Ok();
}
```

**Server (module) -- after:**

``` codeBlockStandalone_LlrK
// 2.0 server -- explicitly publish events via an event table
#include <spacetimedb.h>
using namespace SpacetimeDB;

struct DamageEvent {
    Identity target;
    uint32_t amount;
};
SPACETIMEDB_STRUCT(DamageEvent, target, amount);
SPACETIMEDB_TABLE(DamageEvent, damage_event, Public);

SPACETIMEDB_REDUCER(deal_damage, ReducerContext ctx, Identity target, uint32_t amount) {
    // update game state...

    ctx.db[damage_event].insert(DamageEvent{target, amount});
    return Ok();
}
```

- TypeScript
- C#
- Rust
- Unreal C++

**Client -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (global reducer callback)
conn.reducers.onDealDamage((ctx, { target, amount }) => {
    playDamageAnimation(target, amount);
});
```

**Client -- after:**

``` codeBlockStandalone_LlrK
// 2.0 client -- event table callback
// Note that although this callback fires, the `damageEvent`
// table will never have any rows present in the client cache
conn.db.damageEvent().onInsert((ctx, { target, amount }) => {
    playDamageAnimation(target, amount);
});
```

**Client -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (global reducer callback)
conn.Reducers.OnDealDamage += (ctx, target, amount) =>
{
    PlayDamageAnimation(target, amount);
};
```

**Client -- after:**

``` codeBlockStandalone_LlrK
// 2.0 client -- event table callback
conn.Db.DamageEvent.OnInsert += (ctx, damageEvent) =>
{
    PlayDamageAnimation(damageEvent.Target, damageEvent.Amount);
};
```

**Client -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (global reducer callback)
conn.reducers.on_deal_damage(|ctx, target, amount| {
    play_damage_animation(target, amount);
});
```

**Client -- after:**

``` codeBlockStandalone_LlrK
// 2.0 client -- event table callback
// Note that although this callback fires, the `damage_event`
// table will never have any rows present in the client cache
conn.db.damage_event().on_insert(|ctx, event| {
    play_damage_animation(event.target, event.amount);
});
```

**Client -- before:**

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0 (global reducer callback)
UFUNCTION()
void OnDealDamage(const FReducerEventContext& Context, const FSpacetimeDBIdentity& Target, int32 Amount)
{
    PlayDamageAnimation(Target, Amount);
}

Conn->Reducers->OnDealDamage.AddDynamic(this, &AMyActor::OnDealDamage);
```

**Client -- after:**

``` codeBlockStandalone_LlrK
// 2.0 client -- event table callback
// Note that although this callback fires, the `DamageEvent`
// table will never have any rows present in the client cache
UFUNCTION()
void OnDamageEvent(const FEventContext& Context, const FDamageEventType& DamageEvent)
{
    PlayDamageAnimation(DamageEvent.Target, DamageEvent.Amount);
}

Conn->Db->DamageEvent->OnInsert.AddDynamic(this, &AMyActor::OnDamageEvent);

Conn->SubscriptionBuilder()
    ->OnApplied(OnAppliedDelegate)
    ->OnError(OnErrorDelegate)
    ->AddQuery([](https://spacetimedb.com/docs/const%20FQueryBuilder&%20Q)
    {
        return Q.From.DamageEvent();
    })
    ->Subscribe();
```

### Why event tables are better

- **Security**: You control exactly what data is published. In 1.0,
  reducer arguments were broadcast to any subscriber of affected rows,
  which could accidentally leak sensitive data.
- **Flexibility**: Multiple reducers can insert the same event type. In
  1.0, events were tied 1:1 to a specific reducer.
- **Transactional**: Events are only published if the transaction
  commits. In 1.0, workarounds using scheduled reducers were not
  transactional.
- **Row-level security**: RLS rules apply to event tables, so you can
  control which clients see which events.
- **Queryable**: Event tables can be subscribed to with query builders
  (or SQL), and can be filtered per-client.

### Event table details

- Event tables are always empty outside of a transaction. They don't
  accumulate rows.
- On the client, `count()` always returns 0 and `iter()` is always
  empty.
- Only `on_insert` callbacks are generated (no `on_delete` or
  `on_update`).
- The `event` keyword in `#[table(..., event)]` marks the table as
  transient.
- Event tables can be subscribed to with subscribe-all helpers or
  explicit typed queries, but clients observe them only through insert
  callbacks.

## Event Type Changes

### What changed

- TypeScript
- C#
- Rust
- Unreal C++

In 1.0, table callbacks received
`{ tag: 'Reducer'; value: ReducerEvent\<Reducer\> }` with full reducer
information when a reducer caused a table change. Non-callers could also
receive `{ tag: 'UnknownTransaction' }`.

In 2.0, the event model is simplified:

- **The caller** sees `{ tag: 'Reducer'; value: ReducerEvent\<Reducer\> }`
  with `type ReducerEvent = { timestamp, status, reducer }` in response
  to their own reducer calls.
- **Other clients** see `{ tag: 'Transaction' }` (no reducer details).
- `{ tag: 'UnknownTransaction' }` is removed.

``` codeBlockStandalone_LlrK
// 2.0 -- checking who caused a table change
conn.db.myTable().onInsert((ctx, row) => {
  if (ctx.event.tag === 'Reducer') {
    // This client called the reducer that caused this insert.
    console.log(`Our reducer: ${ctx.event.value.reducer}`);
  }
  if (ctx.event.tag === 'Transaction') {
    // Another client's action caused this insert.
  }
});
```

In 1.0, table callbacks could receive `Event\<Reducer\>.Reducer` with
reducer information when a reducer caused a table change. Non-callers
could also receive `Event\<Reducer\>.UnknownTransaction`.

In 2.0, for known reducer updates:

- **The caller** sees `Event\<Reducer\>.Reducer` with
  `ReducerEvent { Timestamp, Status, Reducer }`.
- **Other clients** see `Event\<Reducer\>.Transaction` (no reducer
  details).

``` codeBlockStandalone_LlrK
// 2.0 -- checking who caused a table change
conn.Db.Person.OnInsert += (ctx, row) =>
{
    if (ctx.Event is Event<Reducer>.Reducer(var reducerEvent))
    {
        // This client called the reducer that caused this insert.
        Console.WriteLine($"Our reducer: {reducerEvent.Reducer}");
    }
    else if (ctx.Event is Event<Reducer>.Transaction)
    {
        // Another client's action caused this insert.
    }
};
```

In 1.0, table callbacks received `Event::Reducer` with full reducer
information when a reducer caused a table change. Non-callers could also
receive `Event::UnknownTransaction`.

In 2.0, the event model is simplified:

- **The caller** sees `Event::Reducer` with
  `ReducerEvent { timestamp, status, reducer }` in response to their own
  reducer calls.
- **Other clients** see `Event::Transaction` (no reducer details).
- `Event::UnknownTransaction` is removed.

``` codeBlockStandalone_LlrK
// 2.0 -- checking who caused a table change
conn.db.my_table().on_insert(|ctx, row| {
    match &ctx.event {
        Event::Reducer(reducer_event) => {
            // This client called the reducer that caused this insert.
            println!("Our reducer: {:?}", reducer_event.reducer);
        }
        Event::Transaction => {
            // Another client's action caused this insert.
        }
        _ => {}
    }
});
```

In 1.0, table callbacks could receive a reducer event with full reducer
information when a reducer caused a table change. In 2.0:

- **The caller** sees `Context.Event.IsReducer()` with
  `FReducerEvent { Timestamp, Status, Reducer }`.
- **Other clients** see `Context.Event.IsTransaction()` with no reducer
  details.

``` codeBlockStandalone_LlrK
// 2.0 -- checking who caused a table change
UFUNCTION()
void OnPersonInsert(const FEventContext& Context, const FPersonType& Row)
{
    if (Context.Event.IsReducer())
    {
        const auto ReducerEvent = Context.Event.GetAsReducer();
        UE_LOG(LogTemp, Log, TEXT("Our reducer: %s"), *ReducerEvent.ReducerName);
    }
    else if (Context.Event.IsTransaction())
    {
        // Another client's action caused this insert.
    }
}
```

If you need metadata about reducers invoked by other clients, update
your reducer code to emit an event using an event table.

## Subscription API

In 2.0, the subscription API is largely the same, but you can now
subscribe to the database with a typed query builder:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0
ctx.subscriptionBuilder()
  .onApplied(ctx => { /* ... */ })
  .onError((ctx, err) => { /* ... */ })
  .subscribe(["SELECT * FROM my_table"]);
```

``` codeBlockStandalone_LlrK
// 2.0 -- Typed query builder
import { tables } from './module_bindings';
ctx.subscriptionBuilder()
  .onApplied(ctx => { /* ... */ })
  .onError((ctx, err) => { /* ... */ })
  .subscribe([tables.myTable]);
```

``` codeBlockStandalone_LlrK
// 2.0 -- same as 1.0
conn.SubscriptionBuilder()
    .OnApplied(_ => { /* ... */ })
    .OnError((_, error) => { /* ... */ })
    .AddQuery(q => q.From.Person())
    .Subscribe();
```

``` codeBlockStandalone_LlrK
// 2.0 -- same as 1.0
ctx.subscription_builder()
    .on_applied(|ctx| { /* ... */ })
    .on_error(|ctx, error| { /* ... */ })
    .add_query(|q| q.from.my_table())
    .subscribe();
```

``` codeBlockStandalone_LlrK
// 2.0 -- typed query builder
Conn->SubscriptionBuilder()
    ->OnApplied(OnAppliedDelegate)
    ->OnError(OnErrorDelegate)
    ->AddQuery([](https://spacetimedb.com/docs/const%20FQueryBuilder&%20Q)
    {
        return Q.From.Person();
    })
    ->Subscribe();
```

Unreal 2.0 now supports typed query-builder subscriptions in C++. Use
`AddQuery(...)` as the default for table and event-table subscriptions.

Use explicit queries when you want to subscribe to event tables without
subscribing to every public table:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// Subscribe explicitly to an event table:
import { tables } from "./module_bindings";
ctx.subscriptionBuilder()
    .onApplied((ctx) => { /* ... */ })
    .subscribe([tables.damageEvent]);
```

``` codeBlockStandalone_LlrK
// Subscribe explicitly to an event table:
conn.SubscriptionBuilder()
    .OnApplied(_ => { /* ... */ })
    .AddQuery(q => q.From.DamageEvent())
    .Subscribe();
```

``` codeBlockStandalone_LlrK
// Subscribe explicitly to an event table:
ctx.subscription_builder()
    .on_applied(|ctx| { /* ... */ })
    .add_query(|q| q.from.damage_event())
    .subscribe();
```

``` codeBlockStandalone_LlrK
// Subscribe explicitly to an event table:
Conn->SubscriptionBuilder()
    ->OnApplied(OnAppliedDelegate)
    ->OnError(OnErrorDelegate)
    ->AddQuery([](https://spacetimedb.com/docs/const%20FQueryBuilder&%20Q)
    {
        return Q.From.DamageEvent();
    })
    ->Subscribe();
```

## Table Name Canonicalization

### What changed

SpacetimeDB 2.0 no longer equates the canonical name of your tables and
indexes with the accessor method you use in module or client code. The
canonical name is largely an internal detail, but you may encounter it
when making SQL queries, or in the migration plans printed by
`spacetime publish`.

### Updating source code: Change `name` to `accessor` in table definitions

- TypeScript
- C#
- Rust
- C++

The `name` option for table definitions is now used to overwrite the
canonical name, and is optional. The name of the key passed to the
`schema` function controls the method names you write in your module and
client source code.

By default, the canonical name is derived from the accessor by
converting it to snake case.

To migrate a 1.0 table definition to 2.0, pass an object to the `schema`
function. Always use `schema({ table1 })` or `schema({ t1, t2 })` —
never pass a single table directly.

TypeScript: `schema()` takes exactly one argument — an object

Use `schema({ table })` or `schema({ t1, t2 })`. **Never** use
`schema(table)` or `schema(t1, t2, t3)`.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0
const myTable = table({ name: "my_table", public: true });
const spacetimedb = schema(myTable); // NO LONGER VALID in 2.0
```

``` codeBlockStandalone_LlrK
// 2.0
const myTable = table({ public: true });
const spacetimedb = schema({ myTable }); // NOTE! We are passing `{ myTable }`, not `myTable`
export default spacetimedb; // You must now also export the schema from your module.
```

The `Name` argument on table and index attributes is now used to
override the canonical SQL name and is optional. The `Accessor` argument
controls the generated API names you use in module and client code.

By default, the canonical name is derived from the accessor using the
module's case-conversion policy.

To migrate a 1.0 table definition to 2.0, replace `Name =` with
`Accessor =` in table and index definitions. Always use
`SpacetimeDB.Index.BTree` (never bare `Index` — it conflicts with
`System.Index`):

``` codeBlockStandalone_LlrK
// 1.0 style -- NO LONGER VALID in 2.0
[SpacetimeDB.Table(Name = "MyTable", Public = true)]
[SpacetimeDB.Index.BTree(Name = "Position", Columns = new[] { nameof(X), nameof(Y) })]
public partial struct MyTable
{
    [SpacetimeDB.PrimaryKey]
    [SpacetimeDB.AutoInc]
    public uint Id;
    public uint X;
    public uint Y;
}

// 2.0
[SpacetimeDB.Table(Accessor = "MyTable", Public = true)]
[SpacetimeDB.Index.BTree(Accessor = "Position", Columns = new[] { nameof(X), nameof(Y) })]
public partial struct MyTable
{
    [SpacetimeDB.PrimaryKey]
    [SpacetimeDB.AutoInc]
    public uint Id;
    public uint X;
    public uint Y;
}
```

The `name` argument to table definitions is now used to overwrite the
canonical name, and is optional. The `accessor` argument controls the
method names you write in your module and client source code.

By default, the canonical name is derived from the accessor by
converting it to snake case.

To migrate a 1.0 table definition to 2.0, replace `name =` with
`accessor =` in the table and index definitions:

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0
#[spacetimedb::table(
    name = my_table,
    public,
    index(
        name = position,
        btree(columns = [x, y]),
    )
)]
struct MyTable {
    #[primary_key]
    #[auto_inc]
    id: u32,
    x: u32,
    y: u32,
}

// 2.0
#[spacetimedb::table(
    accessor = my_table,
    public,
    index(
        accessor = position,
        btree(columns = [x, y]),
    )
)]
struct MyTable {
    #[primary_key]
    #[auto_inc]
    id: u32,
    x: u32,
    y: u32,
}
```

The C++ module API does not have a direct `name -> accessor` rename like
C# and Rust.

In C++, the second argument to `SPACETIMEDB_TABLE(...)` is already the
table accessor you use in module code, and the second argument to
`FIELD_NamedMultiColumnIndex(...)` is already the index accessor.

So for this migration step, keep using the accessor names you want in
code:

``` codeBlockStandalone_LlrK
struct MyTable {
    uint32_t id;
    uint32_t x;
    uint32_t y;
};
SPACETIMEDB_STRUCT(MyTable, id, x, y);

// 2.0
SPACETIMEDB_TABLE(MyTable, my_table, Public);
FIELD_PrimaryKeyAutoInc(my_table, id);
FIELD_NamedMultiColumnIndex(my_table, position, x, y);
```

If you also need to preserve or override the canonical SQL names that
appear in migrations or SQL, use the explicit-name forms described in
Option 2 below.

### Auto-migrating existing databases

The new default behavior for canonicalizing names may not be compatible
with existing 1.0 databases, as it may change the casing of table names,
which would require a manual migration.

#### Option 1: Disable case conversion

To avoid this manual migration, configure the case conversion policy in
your module to not convert, which will result in the same table names as
a 1.0 module:

- TypeScript
- C#
- Rust
- C++

``` codeBlockStandalone_LlrK
export const moduleSettings: ModuleSettings = {
  caseConversionPolicy: CaseConversionPolicy.None,
};
```

``` codeBlockStandalone_LlrK
[SpacetimeDB.Settings]
public const SpacetimeDB.CaseConversionPolicy CASE_CONVERSION_POLICY =
    SpacetimeDB.CaseConversionPolicy.None;
```

``` codeBlockStandalone_LlrK
use spacetimedb::CaseConversionPolicy;

#[spacetimedb::settings]
const CASE_CONVERSION_POLICY: CaseConversionPolicy = CaseConversionPolicy::None;
```

``` codeBlockStandalone_LlrK
SPACETIMEDB_SETTING_CASE_CONVERSION(SpacetimeDB::CaseConversionPolicy::None)
```

Use `SPACETIMEDB_SETTING_CASE_CONVERSION(...)` to preserve 1.0-style
canonical names when migrating a C++ module.

#### Option 2: overwrite the name of individual tables

Alternatively, manually specify the correct canonical name of each
table:

- TypeScript
- C#
- Rust
- C++

``` codeBlockStandalone_LlrK
import { table, schema, t } from 'spacetimedb/server';

const myTable = table(
  {
    name: 'MyTable',
    public: true,
    indexes: [{ accessor: 'position', columns: ['x', 'y'] }],
  },
  {
    id: t.u32().primaryKey().autoInc(),
    x: t.u32(),
    y: t.u32(),
  }
);
```

Always use `SpacetimeDB.Index.BTree` (never bare `Index` — it conflicts
with `System.Index`):

``` codeBlockStandalone_LlrK
[SpacetimeDB.Table(Accessor = "MyTable", Name = "MyTable", Public = true)]
[SpacetimeDB.Index.BTree(Accessor = "Position", Columns = new[] { nameof(X), nameof(Y) })]
public partial struct MyTable
{
    [SpacetimeDB.PrimaryKey]
    [SpacetimeDB.AutoInc]
    public uint Id;
    public uint X;
    public uint Y;
}
```

``` codeBlockStandalone_LlrK
#[spacetimedb::table(
    accessor = my_table,
    name = "MyTable",
    public,
    index(
        accessor = position,
        btree(columns = [x, y]),
    )
)]
struct MyTable {
    #[primary_key]
    #[auto_inc]
    id: u32,
    x: u32,
    y: u32,
}
```

If you need to preserve an existing canonical SQL name during migration,
pass it directly to `SPACETIMEDB_TABLE(...)`. Use the `_NAMED`
field/index macros only for index canonical names.

``` codeBlockStandalone_LlrK
struct MyTable {
    uint32_t id;
    uint32_t x;
    uint32_t y;
};
SPACETIMEDB_STRUCT(MyTable, id, x, y);
SPACETIMEDB_TABLE(MyTable, my_table, Public, "MyTable");
FIELD_PrimaryKeyAutoInc(my_table, id);
FIELD_MultiColumnIndex_NAMED(my_table, position, "Position", x, y);
```

## Clients connect with database name

- TypeScript
- C#
- Rust
- Unreal C++

When constructing a `DbConnection` to a remote database, you now use
`withDatabaseName` to provide the database name, rather than
`withModuleName`. This is a more accurate terminology.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER CORRECT
const conn = DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withModuleName("my-database")
    // other options...
    .build();

// 2.0
const conn = DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withDatabaseName("my-database")
    // other options...
    .build()
```

When constructing a `DbConnection` to a remote database, you now use
`WithDatabaseName` to provide the database name, rather than
`WithModuleName`. This is a more accurate terminology.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER CORRECT
var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithModuleName("my-database")
    // other options...
    .Build();

// 2.0
var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my-database")
    // other options...
    .Build();
```

When constructing a `DbConnection` to a remote database, you now use
`with_database_name` to provide the database name, rather than
`with_module_name`. This is a more accurate terminology.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER CORRECT
let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_module_name("my-database")
    // other options...
    .build()
    .expect("Failed to connect");

// 2.0
let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my-database")
    // other options...
    .build()
    .expect("Failed to connect");
```

When constructing a `UDbConnection` to a remote database, use
`WithDatabaseName` in 2.0. The Unreal 2.0 builder does not expose
`WithModuleName`.

``` codeBlockStandalone_LlrK
// 1.0 terminology in other SDKs / older docs
// UDbConnection::Builder()
//     ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
//     ->WithModuleName(TEXT("my-database"))
//     ->Build();

// 2.0
UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
    ->WithDatabaseName(TEXT("my-database"))
    // other options...
    ->Build();
```

## `sender` Is Now A Method, Not A Field

- TypeScript
- C#
- Rust
- C++

This change does not apply to TypeScript, where properties are
indistinguishable from fields.

This change does not apply to C#, where properties are indistinguishable
from fields.

In Rust modules, the sender of a request is no longer exposed via a
field `ctx.sender` on `ReducerContext`, `ProcedureContext`,
`ViewContext` or `AnonymousViewContext`. Instead, each of these types
has a method `ctx.sender()` which returns the sender's identity.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER CORRECT
#[spacetimedb::reducer]
fn my_reducer(ctx: &ReducerContext) {
    let sender_identity = ctx.sender;
    // Do stuff with `sender_identity`...
}

// 2.0
#[spacetimedb::reducer]
fn my_reducer(ctx: &ReducerContext) {
    let sender_identity = ctx.sender();
    // Do stuff with `sender_identity`...
}
```

In C++ modules, the sender is now accessed with `ctx.sender()` rather
than a `ctx.sender` field.

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER CORRECT
SPACETIMEDB_REDUCER(my_reducer, ReducerContext ctx) {
    auto sender_identity = ctx.sender;
    // Do stuff with `sender_identity`...
    return Ok();
}

// 2.0
SPACETIMEDB_REDUCER(my_reducer, ReducerContext ctx) {
    auto sender_identity = ctx.sender();
    // Do stuff with `sender_identity`...
    return Ok();
}
```

## Only Primary Keys Have Update Methods

- TypeScript
- C#
- Rust
- C++

In 2.0 modules, only columns with a `.primaryKey()` constraint expose an
`update` method, whereas previously, `.unique()` constraints also
provided that method. The previous behavior led to confusion, as only
updates which preserved the value in the primary key column resulted in
`onUpdate` callbacks being invoked on the client.

``` codeBlockStandalone_LlrK
const myTable = table({ name: 'my_table' }, {
    id: t.u32().unique(),
    name: t.string(),
}) 

// 1.0 -- REMOVED in 2.0 
spacetimedb.reducer('my_reducer', ctx => {
    ctx.db.myTable.id.update({
        id: 1,
        name: "Foobar",
    });
})

// 2.0 -- Perform a delete followed by an insert
// OR change the `.unique()` constraint into `.primaryKey()` constraint
spacetimedb.reducer(ctx => {
    ctx.db.myTable.id.delete(1);
    ctx.db.myTable.insert({
        id: 1,
        name: "Foobar"
    });
})
```

In 2.0 modules, only `[SpacetimeDB.PrimaryKey]` indexes expose an
`Update` method, whereas previously, `[SpacetimeDB.Unique]` indexes also
provided that method. The previous behavior led to confusion, as only
updates which preserved the primary key value resulted in `OnUpdate`
callbacks being invoked on the client.

### Updates which preserve the primary key - update with the primary key index

``` codeBlockStandalone_LlrK
[SpacetimeDB.Table(Accessor = "User")]
public partial struct User
{
    [SpacetimeDB.PrimaryKey]
    public Identity Identity;

    [SpacetimeDB.Unique]
    public string Name;

    public uint ApplesOwned;
}

// 1.0 -- REMOVED in 2.0
[SpacetimeDB.Reducer]
public static void AddAppleOld(ReducerContext ctx, string name)
{
    var user = ctx.Db.User.Name.Find(name).Value;
    ctx.Db.User.Name.Update(new User
    {
        ApplesOwned = user.ApplesOwned + 1,
        Identity = user.Identity,
        Name = user.Name,
    });
}

// 2.0
[SpacetimeDB.Reducer]
public static void AddApple(ReducerContext ctx, string name)
{
    var user = ctx.Db.User.Name.Find(name).Value;
    ctx.Db.User.Identity.Update(new User
    {
        ApplesOwned = user.ApplesOwned + 1,
        Identity = user.Identity,
        Name = user.Name,
    });
}
```

### Updates which change the primary key - explicitly delete and insert

``` codeBlockStandalone_LlrK
[SpacetimeDB.Table(Accessor = "User")]
public partial struct User
{
    [SpacetimeDB.PrimaryKey]
    public Identity Identity;

    [SpacetimeDB.Unique]
    public string Name;

    public uint ApplesOwned;
}

// 1.0 -- REMOVED in 2.0
[SpacetimeDB.Reducer]
public static void ChangeUserIdentityOld(ReducerContext ctx, string name, Identity identity)
{
    var user = ctx.Db.User.Name.Find(name).Value;
    ctx.Db.User.Name.Update(new User
    {
        Identity = identity,
        Name = user.Name,
        ApplesOwned = user.ApplesOwned,
    });
}

// 2.0
[SpacetimeDB.Reducer]
public static void ChangeUserIdentity(ReducerContext ctx, string name, Identity identity)
{
    var user = ctx.Db.User.Name.Find(name).Value;
    ctx.Db.User.Delete(user);
    ctx.Db.User.Insert(new User
    {
        Identity = identity,
        Name = user.Name,
        ApplesOwned = user.ApplesOwned,
    });
}
```

In 2.0 modules, only `#[primary_key]` constraints expose an `update`
method, whereas previously, `#[unique]` constraints also provided that
method. The previous behavior led to confusion, as only updates which
preserved the value in the primary key column resulted in `on_update`
callbacks being invoked on the client.

### Updates which preserve the primary key - update with the primary key index

``` codeBlockStandalone_LlrK
#[spacetimedb::table(accessor = user)]
struct User {
    #[primary_key]
    identity: Identity,

    #[unique]
    name: String,

    apples_owned: u32,
}

// 1.0 -- REMOVED in 2.0
#[spacetimedb::reducer]
fn add_apple(ctx: &ReducerContext, name: String) {
    let user = ctx.db.user().name().find(&name).unwrap();
    ctx.db.user().name().update(User {
        apples_owned: user.apples_owned + 1,
        ..user
    });
}

// 2.0
#[spacetimedb::reducer]
fn add_apple(ctx: &ReducerContext, name: String) {
    let user = ctx.db.user().name().find(&name).unwrap();
    ctx.db.user().identity().update(User {
        apples_owned: user.apples_owned + 1,
        ..user
    });
}
```

### Updates which change the primary key - explicitly delete and insert

``` codeBlockStandalone_LlrK
#[spacetimedb::table(accessor = user)]
#[derive(Clone)]
struct User {
    #[primary_key]
    identity: Identity,

    #[unique]
    name: String,

    apples_owned: u32,
}

// 1.0 -- REMOVED in 2.0
#[spacetimedb::reducer]
fn change_user_identity(ctx: &ReducerContext, name: String, identity: Identity) {
    let user = ctx.db.user().name().find(&name).unwrap();
    ctx.db.user().name().update(User {
        identity,
        ..user
    });
}

// 2.0
#[spacetimedb::reducer]
fn change_user_identity(ctx: &ReducerContext, name: String, identity: Identity) {
    let user = ctx.db.user().name().find(&name).unwrap();
    ctx.db.user().delete(user.clone());
    ctx.db.user().insert(User {
        identity,
        ..user
    });
}
```

In 2.0 modules, updates should go through the primary key index. If you
were previously treating another unique index like an update handle,
migrate to primary-key updates or an explicit delete-plus-insert.

### Updates which preserve the primary key - update with the primary key index

``` codeBlockStandalone_LlrK
struct User {
    Identity identity;
    std::string name;
    uint32_t apples_owned;
};
SPACETIMEDB_STRUCT(User, identity, name, apples_owned);
SPACETIMEDB_TABLE(User, user, Public);
FIELD_PrimaryKey(user, identity);
FIELD_Unique(user, name);

// 2.0
SPACETIMEDB_REDUCER(add_apple, ReducerContext ctx, std::string name) {
    auto user_row = ctx.db[user_name].find(name);
    if (!user_row) {
        return Err("User not found");
    }

    user_row->apples_owned += 1;
    ctx.db[user_identity].update(*user_row);
    return Ok();
}
```

### Updates which change the primary key - explicitly delete and insert

``` codeBlockStandalone_LlrK
struct User {
    Identity identity;
    std::string name;
    uint32_t apples_owned;
};
SPACETIMEDB_STRUCT(User, identity, name, apples_owned);
SPACETIMEDB_TABLE(User, user, Public);
FIELD_PrimaryKey(user, identity);
FIELD_Unique(user, name);

// 2.0
SPACETIMEDB_REDUCER(change_user_identity, ReducerContext ctx, std::string name, Identity identity) {
    auto user_row = ctx.db[user_name].find(name);
    if (!user_row) {
        return Err("User not found");
    }

    User updated = *user_row;
    ctx.db[user_identity].delete_by_key(updated.identity);
    updated.identity = identity;
    ctx.db[user].insert(updated);
    return Ok();
}
```

## Scheduled Functions Are Now Private

Scheduled reducers and procedures are now private by default, meaning
that only the database owner and team collaborators can bypass the
schedule table to invoke them manually.

### Remove authorization logic from scheduled functions

Because scheduled reducers and procedures are now private, it's no
longer necessary to explicitly check that the sender is the database
itself.

- TypeScript
- C#
- Rust
- C++

``` codeBlockStandalone_LlrK
// 1.0 -- NO LONGER VALID in 2.0
const myTimer = table({ name: "my_timer", scheduled: 'runMyTimer' }, {
  scheduledId: t.u64(),
  scheduledAt: t.scheduleAt(),
});
const spacetimedb = schema(myTimer);

// 1.0 - SUPERFLUOUS IN 2.0
spacetimedb.reducer('runMyTimer', myTimer.rowType, (ctx, timer) => {
  if (ctx.sender != ctx.identity) {
    throw SenderError(`'runMyTimer' should only be invoked by the database!`);
  }
  // Do stuff
})
```

``` codeBlockStandalone_LlrK
const myTimer = table({ name: 'my_timer', scheduled: (): any => runMyTimer }, {
  scheduledId: t.u64().primaryKey().autoInc(),
  scheduledAt: t.scheduleAt(),
});
const spacetimedb = schema({ myTimer }); // schema({ table }), never schema(table)

// 2.0 -- Can only be called by the database
export const runMyTimer = spacetimedb.reducer({ arg: myTimer.rowType }, (ctx, { arg }) => {
  // Do stuff
})
```

``` codeBlockStandalone_LlrK
[SpacetimeDB.Table(Accessor = "MyTimer", Scheduled = nameof(RunMyTimer))]
public partial struct MyTimer
{
    [SpacetimeDB.PrimaryKey]
    [SpacetimeDB.AutoInc]
    public ulong ScheduledId;
    public ScheduleAt ScheduledAt;
}

// 1.0 - SUPERFLUOUS
[SpacetimeDB.Reducer]
public static void RunMyTimer(ReducerContext ctx, MyTimer timer)
{
    if (ctx.Sender != ctx.Identity)
    {
        throw new Exception("`RunMyTimer` should only be invoked by the database!");
    }
    // Do stuff...
}

// 2.0
[SpacetimeDB.Reducer]
public static void RunMyTimer(ReducerContext ctx, MyTimer timer)
{
    // Do stuff...
}
```

``` codeBlockStandalone_LlrK
#[spacetimedb::table(accessor = my_timer, scheduled(run_my_timer))]
struct MyTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
}

// 1.0 - SUPERFLUOUS IN 2.0
#[spacetimedb::reducer]
fn run_my_timer(ctx: &ReducerContext, timer: MyTimer) -> Result<(), String> {
    if ctx.sender() != ctx.identity() {
        return Err("`run_my_timer` should only be invoked by the database!".to_string());
    }
    // Do stuff...
    Ok(())
}

// 2.0 -- Can only be called by the database
#[spacetimedb::reducer]
fn run_my_timer(ctx: &ReducerContext, timer: MyTimer) {
    // Do stuff...
}
```

``` codeBlockStandalone_LlrK
struct MyTimer {
    uint64_t scheduled_id;
    ScheduleAt scheduled_at;
};
SPACETIMEDB_STRUCT(MyTimer, scheduled_id, scheduled_at);
SPACETIMEDB_TABLE(MyTimer, my_timer, Private);
FIELD_PrimaryKeyAutoInc(my_timer, scheduled_id);
SPACETIMEDB_SCHEDULE(my_timer, 1, run_my_timer);

// 1.0 - SUPERFLUOUS IN 2.0
SPACETIMEDB_REDUCER(run_my_timer, ReducerContext ctx, MyTimer timer) {
    if (ctx.sender() != ctx.identity()) {
        return Err("`run_my_timer` should only be invoked by the database!");
    }
    // Do stuff...
    return Ok();
}

// 2.0
SPACETIMEDB_REDUCER(run_my_timer, ReducerContext ctx, MyTimer timer) {
    // Do stuff...
    return Ok();
}
```

### Define wrappers for functions that are both scheduled and invoked by clients

In the rare event that you have a reducer or procedure which is intended
to be invoked by both clients and a schedule table, define a new public
reducer or procedure which wraps the scheduled function.

- TypeScript
- C#
- Rust
- C++

``` codeBlockStandalone_LlrK
const myTimer = table({ name: 'my_timer', scheduled: (): any => runMyTimerPrivate }, {
  scheduledId: t.u64().primaryKey().autoInc(),
  scheduledAt: t.scheduleAt(),
});
const spacetimedb = schema({ myTimer }); // schema({ table }), never schema(table)

export const runMyTimerPrivate = spacetimedb.reducer({ arg: myTimer.rowType }, (ctx, { arg }) => {
  // Do stuff...
});

export const runMyTimer = spacetimedb.reducer({ arg: myTimer.rowType }, (ctx, { arg }) => {
  // Same logic as runMyTimerPrivate — extract to a helper if needed
});
```

``` codeBlockStandalone_LlrK
[SpacetimeDB.Table(Accessor = "MyTimer", Scheduled = nameof(RunMyTimerPrivate))]
public partial struct MyTimer
{
    [SpacetimeDB.PrimaryKey]
    [SpacetimeDB.AutoInc]
    public ulong ScheduledId;
    public ScheduleAt ScheduledAt;
}

[SpacetimeDB.Reducer]
public static void RunMyTimerPrivate(ReducerContext ctx, MyTimer timer)
{
    // Do stuff...
}

[SpacetimeDB.Reducer]
public static void RunMyTimer(ReducerContext ctx, MyTimer timer)
{
    RunMyTimerPrivate(ctx, timer);
}
```

``` codeBlockStandalone_LlrK
#[spacetimedb::table(accessor = my_timer, scheduled(run_my_timer_private))]
struct MyTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: spacetimedb::ScheduleAt,
}

#[spacetimedb::reducer]
fn run_my_timer_private(ctx: &ReducerContext, timer: MyTimer) {
    // Do stuff...
}

#[spacetimedb::reducer]
fn run_my_timer(ctx: &ReducerContext, timer: MyTimer) {
    run_my_timer_private(ctx, timer)
}
```

``` codeBlockStandalone_LlrK
struct MyTimer {
    uint64_t scheduled_id;
    ScheduleAt scheduled_at;
};
SPACETIMEDB_STRUCT(MyTimer, scheduled_id, scheduled_at);
SPACETIMEDB_TABLE(MyTimer, my_timer, Private);
FIELD_PrimaryKeyAutoInc(my_timer, scheduled_id);
SPACETIMEDB_SCHEDULE(my_timer, 1, run_my_timer_private);

SPACETIMEDB_REDUCER(run_my_timer_private, ReducerContext ctx, MyTimer timer) {
    // Do stuff...
    return Ok();
}

SPACETIMEDB_REDUCER(run_my_timer, ReducerContext ctx, MyTimer timer) {
    return run_my_timer_private(ctx, timer);
}
```

## Private Items Are Not Code-Generated By Default

Starting in SpacetimeDB 2.0, `spacetime generate` will not generate
bindings for private tables or functions by default. These bindings were
confusing, as only clients authenticated as the database owner or a
collaborator could access them, with most clients seeing an error when
trying to subscribe to a private table or invoke a private function.

### Pass `--include-private` to `spacetime generate`

For clients which rely on generated bindings to private tables or
functions, pass the `--include-private` flag to the `spacetime generate`
CLI command.

## Light Mode

### What changed

In 1.0, `light_mode` prevented the server from sending reducer event
data to a client (unless that client was the caller):

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
DbConnection.builder()
    .withLightMode(true)
    // ...
```

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
DbConnection.Builder()
    .WithLightMode(true)
    // ...
```

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
DbConnection::builder()
    .with_light_mode(true)
    // ...
```

``` codeBlockStandalone_LlrK
// 1.0 equivalent removed in 2.0
// The Unreal 2.0 builder does not expose WithLightMode(...)
```

In 2.0, the server never broadcasts reducer argument data to any client,
so `light_mode` is no longer necessary. Simply remove the call:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// 2.0
DbConnection.builder()
    .withUri(uri)
    .withDatabaseName(name)
    // no withLightMode needed
    .build()
```

``` codeBlockStandalone_LlrK
// 2.0
DbConnection.Builder()
    .WithUri(uri)
    .WithDatabaseName(name)
    // no WithLightMode needed
    .Build();
```

``` codeBlockStandalone_LlrK
// 2.0
DbConnection::builder()
    .with_uri(uri)
    .with_database_name(name)
    // no with_light_mode needed
    .build()
```

``` codeBlockStandalone_LlrK
// 2.0
UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(Uri)
    ->WithDatabaseName(DatabaseName)
    // no WithLightMode needed
    ->Build();
```

This migration item does not apply directly to Unreal. The 2.0 Unreal
builder has no public `WithLightMode(...)` API.

## CallReducerFlags

### What changed

In 1.0, you could suppress success notifications for individual reducer
calls:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
ctx.setReducerFlags(CallReducerFlags.NoSuccessNotify);
ctx.reducers.myReducer(args);
```

In 2.0, the success notification is lightweight (just `requestId` and
`timestamp`, no reducer args or full event data), so there is no need to
suppress it. Remove any `setReducerFlags` calls and `CallReducerFlags`
imports.

This migration item does not apply to C#. Before recent SpacetimeDB
changes, C# had no public `CallReducerFlags` or `set_reducer_flags`
equivalent.

``` codeBlockStandalone_LlrK
// 1.0 -- REMOVED in 2.0
ctx.set_reducer_flags(CallReducerFlags::NoSuccessNotify);
ctx.reducers.my_reducer(args).unwrap();
```

In 2.0, the success notification is lightweight (just `request_id` and
`timestamp`, no reducer args or full event data), so there is no need to
suppress it. Remove any `set_reducer_flags` calls and `CallReducerFlags`
imports.

This migration item does not apply to Unreal. The 2.0 Unreal SDK has no
public `CallReducerFlags` or `setReducerFlags` equivalent.

## Confirmed Reads Enabled by Default

### What changed

In 1.0, subscription updates and SQL query results were sent to the
client immediately, before the underlying transaction was confirmed to
be durable. This meant a client could observe a row that was later lost
if the server crashed before persisting it.

In 2.0, **confirmed reads are enabled by default**. The server waits
until a transaction is confirmed durable before sending updates to
clients. This ensures that any data a client receives will survive a
server restart.

### Impact

- **Slightly higher latency**: Subscription updates and SQL results may
  arrive a few milliseconds later, as the server waits for durability
  confirmation before sending them.
- **Stronger consistency**: Clients will never observe data that could
  be lost due to a crash.
- **No code changes required**: This is a server-side default change.
  Existing client code works without modification.

### Opting out

If your application prioritizes low latency over durability guarantees
(for example, a real-time game where occasional data loss on crash is
acceptable), you can opt out by passing `confirmed=false` in the
connection URL:

- TypeScript
- C#
- Rust
- Unreal C++

``` codeBlockStandalone_LlrK
DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withDatabaseName("my-database")
    .withConfirmedReads(false) // opt out of confirmed reads
    .build()
```

``` codeBlockStandalone_LlrK
DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my-database")
    .WithConfirmedReads(false) // opt out of confirmed reads
    .Build();
```

``` codeBlockStandalone_LlrK
DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my-database")
    .with_confirmed_reads(false) // opt out of confirmed reads
    .build()
    .expect("Failed to connect");
```

The Unreal SDK gets confirmed reads by default in 2.0. There is
currently no Unreal builder method for opting out.

For the CLI:

``` codeBlockStandalone_LlrK
# SQL without confirmed reads
spacetime sql <database> "SELECT * FROM my_table"
# The --confirmed flag is no longer needed (it is the default)
```

## Quick Migration Checklist

Remove all `ctx.reducers.on_\<reducer\>()` calls

- Replace with `_then()` callbacks for your own reducer calls
- Unreal: replace with generated `On<Reducer>` delegates on the calling
  connection
- Replace with event tables + `on_insert` for cross-client notifications

Migrate Unreal subscription SQL strings to typed queries where
appropriate

- Use `Conn->SubscriptionBuilder()->AddQuery(...)->Subscribe()` instead
  of `Subscribe({ TEXT("SELECT ...") })`

Update `Event::UnknownTransaction` matches to `Event::Transaction`

For each reducer whose args you were observing from other clients:

1.  Create an `#[table(..., event)]` on the server
2.  Insert into it from the reducer
3.  Subscribe to it on the client
4.  Use `on_insert` instead of the old reducer callback

Replace `name =` with `accessor =` in table and index definitions

**TypeScript:** Use `schema({ table })` or `schema({ t1, t2 })` — never
`schema(table)` or `schema(t1, t2, t3)`

Set your module's case conversion policy to `None`

Change `with_module_name` to `with_database_name`

Change `ctx.sender` to `ctx.sender()`

- Only necessary in Rust and C++ modules.

Remove `update` calls on non-primary key unique indexes

- When leaving the primary key value unchanged, update using the primary
  key index
- When altering the primary key value, delete and insert

Remove superfluous auth logic from scheduled functions which are not
called by clients

Define wrappers around scheduled functions which are called by clients

Use `spacetime generate --include-private` if you rely on bindings for
private tables or functions

Remove `with_light_mode()` from `DbConnectionBuilder`

- Unreal: no action if you are already on the 2.0 Unreal SDK; there is
  no public `WithLightMode(...)`

Remove `set_reducer_flags()` calls and `CallReducerFlags` imports

- Unreal: no action if you are already on the 2.0 Unreal SDK; there is
  no public reducer-flags API

Remove `unstable::CallReducerFlags` from imports

Note that confirmed reads are now enabled by default (no action needed
unless you want to opt out with `.withConfirmedReads(false)`)
