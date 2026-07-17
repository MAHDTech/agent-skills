+++
title = "docs-clients-api"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

Version: 2.0.0

On this page

The SpacetimeDB client SDKs provide a comprehensive API for interacting
with your [database](/docs/databases). After [generating client
bindings](/docs/clients/codegen) and [establishing a
connection](/docs/clients/connection), you can query data, invoke server
functions, and observe real-time changes.

This page describes the core concepts and patterns that apply across all
client SDKs. For language-specific details and complete API
documentation, see the reference pages for [Rust](/docs/clients/rust),
[C#](/docs/clients/c-sharp), [TypeScript](/docs/clients/typescript), or
[Unreal Engine](/docs/clients/unreal).

## Prerequisites

Before using the SDK API, you must:

1.  [Generate client bindings](/docs/clients/codegen) using
    `spacetime generate`
2.  [Create a connection](/docs/clients/connection) to your database

## Subscriptions

Subscriptions replicate a subset of the database to your client,
maintaining a local cache that automatically updates as the server state
changes. Clients should subscribe to the data they need, then query the
local cache.

Typical flow:

1.  Create a subscription with the SDK builder API
2.  Wait for `onApplied`/`OnApplied` to know initial rows are present
3.  Read from the local cache and register callbacks
4.  Unsubscribe when the data is no longer needed

For lifecycle guarantees and semantics, see
[Subscriptions](/docs/clients/subscriptions) and [Subscription
Semantics](/docs/clients/subscriptions/semantics).

### Example

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
import { tables } from './module_bindings';

const handle = conn
  .subscriptionBuilder()
  .onApplied(ctx => {
    console.log(`Ready with ${ctx.db.user.count()} users`);
  })
  .onError((ctx, error) => {
    console.error(`Subscription failed: ${error}`);
  })
  .subscribe([tables.user.where(r => r.online.eq(true))]);
```

``` codeBlockStandalone_LlrK
var handle = conn
    .SubscriptionBuilder()
    .OnApplied(ctx =>
    {
        Console.WriteLine($"Ready with {ctx.Db.User.Count} users");
    })
    .OnError((ctx, error) =>
    {
        Console.WriteLine($"Subscription failed: {error}");
    })
    .AddQuery(q => q.From.User())
    .Subscribe();
```

``` codeBlockStandalone_LlrK
let handle = conn
    .subscription_builder()
    .on_applied(|ctx| {
        println!("Ready with {} users", ctx.db().user().count());
    })
    .on_error(|_ctx, error| {
        eprintln!("Subscription failed: {}", error);
    })
    .add_query(|q| q.from.user())
    .subscribe();
```

``` codeBlockStandalone_LlrK
TArray<FString> Queries = { TEXT("SELECT * FROM user") };

USubscriptionHandle* Handle = Conn->SubscriptionBuilder()
    ->OnApplied(AppliedDelegate)
    ->OnError(ErrorDelegate)
    ->Subscribe(Queries);
```

## Querying the Local Cache

After a subscription is applied, reads are local and do not require
network round-trips.

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
const userCount = conn.db.user.count();
const user = conn.db.user.name.find('Alice');
```

``` codeBlockStandalone_LlrK
var userCount = conn.Db.User.Count;
var user = conn.Db.User.Name.Find("Alice");
```

``` codeBlockStandalone_LlrK
let user_count = conn.db().user().count();
let user = conn.db().user().name().find("Alice");
```

``` codeBlockStandalone_LlrK
int32 UserCount = Conn->Db->User->Count();
FUserType User = Conn->Db->User->Name->Find(TEXT("Alice"));
```

## Reacting to Cache Changes

Use row callbacks to react when subscribed rows are inserted, updated,
or deleted.

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
conn.db.user.onInsert((ctx, row) => {});
conn.db.user.onUpdate((ctx, oldRow, newRow) => {});
conn.db.user.onDelete((ctx, row) => {});
```

``` codeBlockStandalone_LlrK
conn.Db.User.OnInsert += (ctx, row) => {};
conn.Db.User.OnUpdate += (ctx, oldRow, newRow) => {};
conn.Db.User.OnDelete += (ctx, row) => {};
```

``` codeBlockStandalone_LlrK
conn.db().user().on_insert(|ctx, row| {});
conn.db().user().on_update(|ctx, old_row, new_row| {});
conn.db().user().on_delete(|ctx, row| {});
```

``` codeBlockStandalone_LlrK
Conn->Db->User->OnInsert.AddDynamic(this, &AMyActor::OnUserInsert);
Conn->Db->User->OnUpdate.AddDynamic(this, &AMyActor::OnUserUpdate);
Conn->Db->User->OnDelete.AddDynamic(this, &AMyActor::OnUserDelete);
```

## Canonical API References

- [Subscriptions](/docs/clients/subscriptions) - Lifecycle, usage
  patterns, and semantics
- [Subscription Semantics](/docs/clients/subscriptions/semantics) -
  Detailed consistency and ordering behavior
- [TypeScript
  Reference](/docs/clients/typescript#subscribe-to-queries) -
  `SubscriptionBuilder`, `SubscriptionHandle`, query builder API
- [C# Reference](/docs/clients/c-sharp#subscribe-to-queries) -
  `SubscriptionBuilder`, `SubscriptionHandle`
- [C# Query Builder API](/docs/clients/c-sharp#query-builder-api) -
  Typed subscription query builder
- [Rust Reference](/docs/clients/rust#subscribe-to-queries) -
  `SubscriptionBuilder`, `SubscriptionHandle`
- [Rust Query Builder API](/docs/clients/rust#query-builder-api) - Typed
  subscription query builder
- [Unreal Reference](/docs/clients/unreal#subscriptions) - Unreal
  subscription APIs

- [Prerequisites](#prerequisites)
- [Subscriptions](#subscriptions)
  - [Example](#example)
- [Querying the Local Cache](#querying-the-local-cache)
- [Reacting to Cache Changes](#reacting-to-cache-changes)
- [Canonical API References](#canonical-api-references)

