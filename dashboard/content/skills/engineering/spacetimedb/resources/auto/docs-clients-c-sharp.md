+++
title = "docs-clients-c-sharp"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

{% raw %}
Version: 2.0.0

On this page

The SpacetimeDB client for C# contains all the tools you need to build
native clients for SpacetimeDB modules using C#.

## Server module quick reference

If you are **writing a SpacetimeDB module** (tables and reducers), use
these patterns:

- **Module class**: `public static partial class Module`
- **Tables**:
  `[SpacetimeDB.Table(Accessor = "TableName", Public = true)]` on
  `partial struct` (or `partial class`) — `Accessor` controls generated
  API names, and canonical SQL names are derived unless `Name` is
  explicitly set
- **Primary key**: Define `[SpacetimeDB.PrimaryKey]` on one column when
  you need key-based lookups or updates
- **Reducers**: `[SpacetimeDB.Reducer]` on static methods with
  `ReducerContext ctx` as first parameter
- **Required**: `using SpacetimeDB;` and `partial` on all table structs
  and the Module class
- **Index**: Always use `SpacetimeDB.Index.BTree` (never bare `Index`).
  Bare `Index` is ambiguous with `System.Index`. For multi-column:
  `Columns = new[] { nameof(Col1), nameof(Col2) }`, not collection
  expressions `[nameof(X)]`
- **Sum types**: Use `TaggedEnum<(VariantA A, VariantB B)>` with
  `partial record`, not `partial class`
- **Scheduled tables**: `ScheduledAt` should reference a field of type
  `ScheduleAt` in the schedule table

See [Tables](https://spacetimedb.com/docs/tables), [Reducers](https://spacetimedb.com/docs/functions/reducers), and
[Column Types](https://spacetimedb.com/docs/tables/column-types) for full server-side
documentation. The rest of this page covers the **client SDK**
(connections, subscriptions, callbacks).

------------------------------------------------------------------------

Before diving into the reference, you may want to review:

- [Generating Client Bindings](https://spacetimedb.com/docs/clients/codegen) - How to generate
  C# bindings from your module
- [Connecting to SpacetimeDB](https://spacetimedb.com/docs/clients/connection) - Establishing
  and managing connections (important: C# requires manual connection
  advancement!)
- [SDK API Reference](https://spacetimedb.com/docs/clients/api) - Core concepts that apply
  across all SDKs

| Name | Description |
|----|----|
| [Project setup](#project-setup) | Configure a C# project to use the SpacetimeDB C# client SDK. |
| [Generate module bindings](#generate-module-bindings) | Use the SpacetimeDB CLI to generate module-specific types and interfaces. |
| [`DbConnection` type](#type-dbconnection) | A connection to a remote database. |
| [`IDbContext` interface](#interface-idbcontext) | Methods for interacting with the remote database. |
| [`EventContext` type](#type-eventcontext) | Implements [`IDbContext`](#interface-idbcontext) for [row callbacks](#callback-oninsert). |
| [`ReducerEventContext` type](#type-reducereventcontext) | Implements [`IDbContext`](#interface-idbcontext) for [reducer callbacks](#observe-and-invoke-reducers). |
| [`SubscriptionEventContext` type](#type-subscriptioneventcontext) | Implements [`IDbContext`](#interface-idbcontext) for [subscription callbacks](#subscribe-to-queries). |
| [`ErrorContext` type](#type-errorcontext) | Implements [`IDbContext`](#interface-idbcontext) for subscription error callbacks. |
| [Query Builder API](#query-builder-api) | Type-safe query builder for typed subscription queries. |
| [Access the client cache](#access-the-client-cache) | Access to your local view of the database. |
| [Observe and invoke reducers](#observe-and-invoke-reducers) | Send requests to the database to run reducers, and register callbacks to run when notified of reducers. |
| [Identify a client](#identify-a-client) | Types for identifying users and client connections. |

## Project setup

### Using the `dotnet` CLI tool

If you would like to create a console application using .NET, you can
create a new project using `dotnet new console` and add the SpacetimeDB
SDK to your dependencies:

``` codeBlockStandalone_LlrK
dotnet add package SpacetimeDB.ClientSDK
```

(See also the [CSharp Quickstart](https://spacetimedb.com/docs/quickstarts/c-sharp) for an
in-depth example of such a console application.)

### Using Unity

Add the SpacetimeDB Unity Package using the Package Manager. Open the
Package Manager window by clicking on Window -\> Package Manager. Click
on the + button in the top left corner of the window and select "Add
package from git URL". Enter the following URL and click Add.

``` codeBlockStandalone_LlrK
https://github.com/clockworklabs/com.clockworklabs.spacetimedbsdk.git
```

(See also the [Unity Tutorial](https://spacetimedb.com/docs/tutorials/unity/part-1))

## Generate module bindings

Each SpacetimeDB client depends on some bindings specific to your
module. Create a `module_bindings` directory in your project's directory
and generate the C# interface files using the Spacetime CLI. From your
project directory, run:

``` codeBlockStandalone_LlrK
mkdir -p module_bindings
spacetime generate --lang csharp --out-dir module_bindings --module-path PATH-TO-MODULE-DIRECTORY
```

Replace `PATH-TO-MODULE-DIRECTORY` with the path to your SpacetimeDB
module.

## Type `DbConnection`

A connection to a remote database is represented by the `DbConnection`
class. This class is generated per module and contains information about
the types, tables, and reducers defined by your module.

| Name | Description |
|----|----|
| [Connect to a database](#connect-to-a-database) | Construct a `DbConnection` instance. |
| [Advance the connection](#advance-the-connection-and-process-messages) | Poll the `DbConnection` or run it in the background. |
| [Access tables and reducers](#access-tables-and-reducers) | Access the client cache, request reducer invocations, and register callbacks. |

### Connect to a database

``` codeBlockStandalone_LlrK
class DbConnection
{
    public static DbConnectionBuilder<DbConnection> Builder();
}
```

Construct a `DbConnection` by calling `DbConnection.Builder()`, chaining
configuration methods, and finally calling `.Build()`. At a minimum, you
must specify `WithUri` to provide the URI of the SpacetimeDB instance,
and `WithDatabaseName` to specify the database's name or identity.

| Name | Description |
|----|----|
| [WithUri method](#method-withuri) | Set the URI of the SpacetimeDB instance hosting the remote database. |
| [WithDatabaseName method](#method-withdatabasename) | Set the name or identity of the remote database. |
| [WithConfirmedReads method](#method-withconfirmedreads) | Enable or disable confirmed reads. |
| [OnConnect callback](#callback-onconnect) | Register a callback to run when the connection is successfully established. |
| [OnConnectError callback](#callback-onconnecterror) | Register a callback to run if the connection is rejected or the host is unreachable. |
| [OnDisconnect callback](#callback-ondisconnect) | Register a callback to run when the connection ends. |
| [WithToken method](#method-withtoken) | Supply a token to authenticate with the remote database. |
| [Build method](#method-build) | Finalize configuration and open the connection. |

#### Method `WithUri`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnectionBuilder<DbConnection> WithUri(string uri);
}
```

Configure the URI of the SpacetimeDB instance or cluster which hosts the
remote module and database.

#### Method `WithDatabaseName`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder
{
    public DbConnectionBuilder<DbConnection> WithDatabaseName(string nameOrIdentity);
}
```

Configure the SpacetimeDB domain name or `Identity` of the remote
database which identifies it within the SpacetimeDB instance or cluster.

#### Method `WithConfirmedReads`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder
{
    public DbConnectionBuilder<DbConnection> WithConfirmedReads(bool confirmedReads);
}
```

Configure the connection to request confirmed reads.

When enabled, the server will send query results only after they are
confirmed to be durable, i.e. persisted to disk on one or more replicas
depending on the replication settings of the database. When set to
`false`, the server will send results as soon as transactions are
committed in memory.

If this method is not called, the server chooses the default.

#### Callback `OnConnect`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnectionBuilder<DbConnection> OnConnect(Action<DbConnection, Identity, string> callback);
}
```

Chain a call to `.OnConnect(callback)` to your builder to register a
callback to run when your new `DbConnection` successfully initiates its
connection to the remote database. The callback accepts three arguments:
a reference to the `DbConnection`, the `Identity` by which SpacetimeDB
identifies this connection, and a private access token which can be
saved and later passed to [`WithToken`](#method-withtoken) to
authenticate the same user in future connections.

#### Callback `OnConnectError`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnectionBuilder<DbConnection> OnConnectError(Action<Exception> callback);
}
```

Chain a call to `.OnConnectError(callback)` to your builder to register
a callback to run when your connection fails.

#### Callback `OnDisconnect`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnectionBuilder<DbConnection> OnDisconnect(Action<DbConnection, Exception?> callback);
}
```

Chain a call to `.OnDisconnect(callback)` to your builder to register a
callback to run when your `DbConnection` disconnects from the remote
database, either as a result of a call to
[`Disconnect`](#method-disconnect) or due to an error.

#### Method `WithToken`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnectionBuilder<DbConnection> WithToken(string? token);
}
```

Chain a call to `.WithToken(token)` to your builder to provide an OpenID
Connect compliant JSON Web Token to authenticate with, or to explicitly
select an anonymous connection. If this method is not called or `null`
is passed, SpacetimeDB will generate a new `Identity` and sign a new
private access token for the connection.

#### Method `Build`

``` codeBlockStandalone_LlrK
class DbConnectionBuilder<DbConnection>
{
    public DbConnection Build();
}
```

After configuring the connection and registering callbacks, attempt to
open the connection.

### Advance the connection and process messages

In the interest of supporting a wide variety of client applications with
different execution strategies, the SpacetimeDB SDK allows you to choose
when the `DbConnection` spends compute time and processes messages. If
you do not arrange for the connection to advance by calling one of these
methods, the `DbConnection` will never advance, and no callbacks will
ever be invoked.

| Name | Description |
|----|----|
| [`FrameTick` method](#method-frametick) | Process messages on the main thread without blocking. |

#### Method `FrameTick`

``` codeBlockStandalone_LlrK
class DbConnection {
    public void FrameTick();
}
```

`FrameTick` will advance the connection until no work remains or until
it is disconnected, then return rather than blocking. Games might
arrange for this message to be called every frame.

It is not advised to run `FrameTick` on a background thread, since it
modifies [`dbConnection.Db`](#property-db). If main thread code is also
accessing the `Db`, it may observe data races when `FrameTick` runs on
another thread.

(Note that the SDK already does most of the work for parsing messages on
a background thread. `FrameTick()` does the minimal amount of work
needed to apply updates to the `Db`.)

### Access tables and reducers

#### Property `Db`

``` codeBlockStandalone_LlrK
class DbConnection
{
    public RemoteTables Db;
    /* other members */
}
```

The `Db` property of the `DbConnection` provides access to the
subscribed view of the remote database's tables. See [Access the client
cache](#access-the-client-cache).

#### Property `Reducers`

``` codeBlockStandalone_LlrK
class DbConnection
{
    public RemoteReducers Reducers;
    /* other members */
}
```

The `Reducers` field of the `DbConnection` provides access to reducers
exposed by the module of the remote database. See [Observe and invoke
reducers](#observe-and-invoke-reducers).

### Interface `IDbContext`

``` codeBlockStandalone_LlrK
interface IDbContext<DbView, RemoteReducers, ..>
{
    /* methods */
}
```

[`DbConnection`](#type-dbconnection),
[`EventContext`](#type-eventcontext),
[`ReducerEventContext`](#type-reducereventcontext),
[`SubscriptionEventContext`](#type-subscriptioneventcontext) and
[`ErrorContext`](#type-errorcontext) all implement `IDbContext`.
`IDbContext` has methods for inspecting and configuring your connection
to the remote database.

The `IDbContext` interface is implemented by connections and contexts to
*every* module - hence why it takes [`DbView`](#method-db) and
[`RemoteReducers`](#method-reducers) as type parameters.

| Name | Description |
|----|----|
| [`IRemoteDbContext` interface](#interface-iremotedbcontext) | Module-specific `IDbContext`. |
| [`Db` method](#method-db) | Provides access to the subscribed view of the remote database's tables. |
| [`Reducers` method](#method-reducers) | Provides access to reducers exposed by the remote module. |
| [`Disconnect` method](#method-disconnect) | End the connection. |
| [Subscribe to queries](#subscribe-to-queries) | Register subscription queries to receive updates about matching rows. |
| [Read connection metadata](#read-connection-metadata) | Access the connection's `Identity` and `ConnectionId` |

### Interface `IRemoteDbContext`

Each module's `module_bindings` exports an interface `IRemoteDbContext`
which inherits from `IDbContext`, with the type parameters `DbView` and
`RemoteReducers` bound to the types defined for that module. This can be
more convenient when creating functions that can be called from any
callback for a specific module, but which access the database or invoke
reducers, and so must know the type of the `DbView` or `Reducers`.

#### Method `Db`

``` codeBlockStandalone_LlrK
interface IRemoteDbContext
{
    public DbView Db { get; }
}
```

`Db` will have methods to access each table defined by the module.

##### Example

``` codeBlockStandalone_LlrK
var conn = ConnectToDB();

// Get a handle to the User table
var tableHandle = conn.Db.User;
```

#### Method `Reducers`

``` codeBlockStandalone_LlrK
interface IRemoteDbContext
{
    public RemoteReducers Reducers { get; }
}
```

`Reducers` will have methods to invoke each reducer defined by the
module, plus methods for adding and removing callbacks on each of those
reducers.

##### Example

``` codeBlockStandalone_LlrK
var conn = ConnectToDB();

// Register a callback to be run every time the SendMessage reducer is invoked
conn.Reducers.OnSendMessage += Reducer_OnSendMessageEvent;
```

#### Method `Disconnect`

``` codeBlockStandalone_LlrK
interface IRemoteDbContext
{
    public void Disconnect();
}
```

Gracefully close the `DbConnection`. Throws an error if the connection
is already closed.

### Subscribe to queries

| Name | Description |
|----|----|
| [`SubscriptionBuilder` type](#type-subscriptionbuilder) | Builder-pattern constructor to register subscribed queries. |
| [`TypedSubscriptionBuilder` type](#type-typedsubscriptionbuilder) | Builder for typed query subscriptions. |
| [`SubscriptionHandle` type](#type-subscriptionhandle) | Manage an active subscription. |

#### Type `SubscriptionBuilder`

| Name | Description |
|----|----|
| [`ctx.SubscriptionBuilder()` constructor](#constructor-ctxsubscriptionbuilder) | Begin configuring a new subscription. |
| [`OnApplied` callback](#callback-onapplied) | Register a callback to run when matching rows become available. |
| [`OnError` callback](#callback-onerror) | Register a callback to run if the subscription fails. |
| [`Subscribe` method](#method-subscribe) | Finish configuration and subscribe to one or more queries. |
| [`AddQuery` method](#method-addquery) | Build a typed subscription query without writing query strings. |
| [`SubscribeToAllTables` method](#method-subscribetoalltables) | Convenience method to subscribe to the entire database. |

##### Constructor `ctx.SubscriptionBuilder()`

``` codeBlockStandalone_LlrK
interface IRemoteDbContext
{
    public SubscriptionBuilder SubscriptionBuilder();
}
```

Subscribe to queries by calling `ctx.SubscriptionBuilder()` and chaining
configuration methods, then calling `.Subscribe(queries)`.

##### Callback `OnApplied`

``` codeBlockStandalone_LlrK
class SubscriptionBuilder
{
    public SubscriptionBuilder OnApplied(Action<SubscriptionEventContext> callback);
}
```

Register a callback to run when the subscription is applied and the
matching rows are inserted into the client cache.

##### Callback `OnError`

``` codeBlockStandalone_LlrK
class SubscriptionBuilder
{
    public SubscriptionBuilder OnError(Action<ErrorContext, Exception> callback);
}
```

Register a callback to run if the subscription is rejected or
unexpectedly terminated by the server. This is most frequently caused by
passing an invalid query to [`Subscribe`](#method-subscribe).

##### Method `Subscribe`

``` codeBlockStandalone_LlrK
class SubscriptionBuilder
{
    public SubscriptionHandle Subscribe(string[] querySqls);
}
```

Subscribe to a set of queries. `queries` should be an array of SQL query
strings.

See [the SpacetimeDB SQL Reference](https://spacetimedb.com/docs/reference/sql#subscriptions)
for information on the queries SpacetimeDB supports as subscriptions.

For typed query subscriptions, use [`AddQuery`](#method-addquery).

##### Method `AddQuery`

``` codeBlockStandalone_LlrK
class SubscriptionBuilder
{
    public TypedSubscriptionBuilder AddQuery<TRow>(
        Func<QueryBuilder, IQuery<TRow>> build
    );
}
```

Start a typed query subscription. Once a typed query is added, continue
with typed queries on `TypedSubscriptionBuilder` and finish with
`Subscribe()`.

``` codeBlockStandalone_LlrK
var handle = conn
    .SubscriptionBuilder()
    .AddQuery(q => q.From.User())
    .AddQuery(q => q.From.Message())
    .Subscribe();
```

##### Method `SubscribeToAllTables`

``` codeBlockStandalone_LlrK
class SubscriptionBuilder
{
    public void SubscribeToAllTables();
}
```

Subscribe to all rows from all public tables. This method is provided as
a convenience for simple clients. The subscription initiated by
`SubscribeToAllTables` cannot be canceled after it is initiated. You
should [`subscribe` to specific queries](#method-subscribe) if you need
fine-grained control over the lifecycle of your subscriptions.

#### Type `TypedSubscriptionBuilder`

| Name | Description |
|----|----|
| [`AddQuery` method](#method-addquery-typedsubscriptionbuilder) | Add another typed query to the same subscription. |
| [`Subscribe` method](#method-subscribe-typedsubscriptionbuilder) | Subscribe to all typed queries added so far. |

##### Method `AddQuery` (TypedSubscriptionBuilder)

``` codeBlockStandalone_LlrK
class TypedSubscriptionBuilder
{
    public TypedSubscriptionBuilder AddQuery<TRow>(
        Func<QueryBuilder, IQuery<TRow>> build
    );
}
```

Add another typed query. This keeps all added queries grouped under one
returned `SubscriptionHandle`.

##### Method `Subscribe` (TypedSubscriptionBuilder)

``` codeBlockStandalone_LlrK
class TypedSubscriptionBuilder
{
    public SubscriptionHandle Subscribe();
}
```

Subscribe to the set of typed queries that were added to the builder.

## Query Builder API

The C# SDK provides a type-safe query builder for subscriptions. You use
it through `SubscriptionBuilder.AddQuery(...)` and
`TypedSubscriptionBuilder.AddQuery(...)`.

### Entry Point

Typed query builders are created from generated table accessors under
`QueryBuilder.From`.

``` codeBlockStandalone_LlrK
var handle = conn
    .SubscriptionBuilder()
    .AddQuery(q => q.From.User())
    .Subscribe();
```

### Building Queries with `Where` / `Filter`

Each generated table accessor supports both `Where(...)` and
`Filter(...)`. They are equivalent. Chaining multiple `Where`/`Filter`
calls combines conditions with logical `AND`.

``` codeBlockStandalone_LlrK
// All users
q.From.User()

// Filtered users
q.From.User().Where(u => u.Online.Eq(true))
q.From.User().Filter(u => u.Name.Neq("Anonymous"))

// Chained filters (AND semantics)
q.From.User()
    .Where(u => u.Score.Gte(1000UL))
    .Filter(u => u.Level.Gte(10U))
```

### Comparison Operators

| Operator | Description              | Example               |
|----------|--------------------------|-----------------------|
| `Eq`     | Equal to                 | `u.Online.Eq(true)`   |
| `Neq`    | Not equal to             | `u.Name.Neq("BOT")`   |
| `Lt`     | Less than                | `u.Level.Lt(10U)`     |
| `Lte`    | Less than or equal to    | `u.Level.Lte(10U)`    |
| `Gt`     | Greater than             | `u.Score.Gt(1000UL)`  |
| `Gte`    | Greater than or equal to | `u.Score.Gte(1000UL)` |

### Boolean Combinators

Combine conditions with `And`, `Or`, and `Not`:

``` codeBlockStandalone_LlrK
q.From.User().Where(u => u.Level.Gte(5U).And(u.Level.Lt(10U)))
q.From.User().Where(u => u.Online.Eq(true).Or(u.Name.Eq("Admin")))
q.From.User().Where(u => u.Banned.Eq(true).Not())
```

### Semijoins

Semijoins match rows across two tables and return rows from one side:

- `LeftSemijoin(...)` returns rows from the left side that match at
  least one row on the right.
- `RightSemijoin(...)` returns rows from the right side that match at
  least one row on the left.
- The join predicate uses indexed columns (`IxCols`) and must compare
  one indexed column from each side with `Eq`.
- Filters before a semijoin apply to the pre-join source side. Filters
  after a semijoin apply to the returned side.

``` codeBlockStandalone_LlrK
var handle = conn
    .SubscriptionBuilder()
    .AddQuery(q => q.From.Player()
        .Where(p => p.Score.Gte(1000UL))
        .LeftSemijoin(q.From.PlayerLevel(), (p, pl) => p.Id.Eq(pl.PlayerId))
        .Where(p => p.Online.Eq(true)))
    .AddQuery(q => q.From.Player()
        .Where(p => p.Score.Gte(1000UL))
        .RightSemijoin(q.From.PlayerLevel(), (p, pl) => p.Id.Eq(pl.PlayerId))
        .Where(pl => pl.Level.Gte(10U)))
    .Subscribe();
```

### Using Query Builders with Subscriptions

`AddQuery` accepts a builder function that returns an `IQuery<TRow>`.
You can add multiple typed queries and subscribe once.

``` codeBlockStandalone_LlrK
var handle = conn
    .SubscriptionBuilder()
    .AddQuery(q => q.From.User().Where(u => u.Online.Eq(true)))
    .AddQuery(q => q.From.Message().Where(m => m.ChannelId.Eq(1U)))
    .Subscribe();
```

#### Type `SubscriptionHandle`

A `SubscriptionHandle` represents a subscribed query or a group of
subscribed queries.

The `SubscriptionHandle` does not contain or provide access to the
subscribed rows. Subscribed rows of all subscriptions by a connection
are contained within that connection's [`ctx.Db`](#property-db). See
[Access the client cache](#access-the-client-cache).

| Name | Description |
|----|----|
| [`IsEnded` property](#property-isended) | Determine whether the subscription has ended. |
| [`IsActive` property](#property-isactive) | Determine whether the subscription is active and its matching rows are present in the client cache. |
| [`Unsubscribe` method](#method-unsubscribe) | Discard a subscription. |
| [`UnsubscribeThen` method](#method-unsubscribethen) | Discard a subscription, and register a callback to run when its matching rows are removed from the client cache. |

##### Property `IsEnded`

``` codeBlockStandalone_LlrK
class SubscriptionHandle
{
    public bool IsEnded;
}
```

True if this subscription has been terminated due to an unsubscribe call
or an error.

##### Property `IsActive`

``` codeBlockStandalone_LlrK
class SubscriptionHandle
{
    public bool IsActive;
}
```

True if this subscription has been applied and has not yet been
unsubscribed.

##### Method `Unsubscribe`

``` codeBlockStandalone_LlrK
class SubscriptionHandle
{
    public void Unsubscribe();
}
```

Terminate this subscription, causing matching rows to be removed from
the client cache. Any rows removed from the client cache this way will
have [`OnDelete` callbacks](#callback-ondelete) run for them.

Unsubscribing is an asynchronous operation. Matching rows are not
removed from the client cache immediately. Use
[`UnsubscribeThen`](#method-unsubscribethen) to run a callback once the
unsubscribe operation is completed.

Returns an error if the subscription has already ended, either due to a
previous call to `Unsubscribe` or
[`UnsubscribeThen`](#method-unsubscribethen), or due to an error.

##### Method `UnsubscribeThen`

``` codeBlockStandalone_LlrK
class SubscriptionHandle
{
    public void UnsubscribeThen(Action<SubscriptionEventContext>? onEnded);
}
```

Terminate this subscription, and run the `onEnded` callback when the
subscription is ended and its matching rows are removed from the client
cache. Any rows removed from the client cache this way will have
[`OnDelete` callbacks](#callback-ondelete) run for them.

Returns an error if the subscription has already ended, either due to a
previous call to [`Unsubscribe`](#method-unsubscribe) or
`UnsubscribeThen`, or due to an error.

### Read connection metadata

#### Property `Identity`

``` codeBlockStandalone_LlrK
interface IDbContext
{
    public Identity? Identity { get; }
}
```

Get the `Identity` with which SpacetimeDB identifies the connection.
This method returns null if the connection was initiated anonymously and
the newly-generated `Identity` has not yet been received, i.e. if called
before the [`OnConnect` callback](#callback-onconnect) is invoked.

#### Property `ConnectionId`

``` codeBlockStandalone_LlrK
interface IDbContext
{
    public ConnectionId ConnectionId { get; }
}
```

Get the [`ConnectionId`](#type-connectionid) with which SpacetimeDB
identifies the connection.

#### Property `IsActive`

``` codeBlockStandalone_LlrK
interface IDbContext
{
    public bool IsActive { get; }
}
```

`true` if the connection has not yet disconnected. Note that a
connection `IsActive` when it is constructed, before its [`OnConnect`
callback](#callback-onconnect) is invoked.

## Type `EventContext`

An `EventContext` is an [`IDbContext`](#interface-idbcontext) augmented
with an [`Event`](#record-event) property. `EventContext`s are passed as
the first argument to row callbacks [`OnInsert`](#callback-oninsert),
[`OnDelete`](#callback-ondelete) and [`OnUpdate`](#callback-onupdate).

| Name | Description |
|----|----|
| [`Event` property](#property-event) | Enum describing the cause of the current row callback. |
| [`Db` property](#property-db) | Provides access to the client cache. |
| [`Reducers` property](#property-reducers) | Allows requesting reducers run on the remote database. |
| [`Event` record](#record-event) | Possible events which can cause a row callback to be invoked. |

### Property `Event`

``` codeBlockStandalone_LlrK
class EventContext {
    public readonly Event<Reducer> Event;
    /* other fields */
}
```

The [`Event`](#record-event) contained in the `EventContext` describes
what happened to cause the current row callback to be invoked.

### Property `Db`

``` codeBlockStandalone_LlrK
class EventContext {
    public RemoteTables Db;
    /* other fields */
}
```

The `Db` property of the context provides access to the subscribed view
of the remote database's tables. See [Access the client
cache](#access-the-client-cache).

### Field `Reducers`

``` codeBlockStandalone_LlrK
class EventContext {
    public RemoteReducers Reducers;
    /* other fields */
}
```

The `Reducers` property of the context provides access to reducers
exposed by the remote module. See [Observe and invoke
reducers](#observe-and-invoke-reducers).

### Record `Event`

| Name | Description |
|----|----|
| [`Reducer` variant](#variant-reducer) | A reducer ran in the remote database. |
| [`SubscribeApplied` variant](#variant-subscribeapplied) | A new subscription was applied to the client cache. |
| [`UnsubscribeApplied` variant](#variant-unsubscribeapplied) | A previous subscription was removed from the client cache after a call to [`Unsubscribe`](#method-unsubscribe). |
| [`SubscribeError` variant](#variant-subscribeerror) | A previous subscription was removed from the client cache due to an error. |
| [`UnknownTransaction` variant](#variant-unknowntransaction) | A transaction ran in the remote database, but was not attributed to a known reducer. |
| [`ReducerEvent` record](#record-reducerevent) | Metadata about a reducer run. Contained in a [`Reducer` event](#variant-reducer) and [`ReducerEventContext`](#type-reducereventcontext). |
| [`Status` record](#record-status) | Completion status of a reducer run. |
| [`Reducer` record](#record-reducer) | Module-specific generated record with a variant for each reducer defined by the module. |

#### Variant `Reducer`

``` codeBlockStandalone_LlrK
record Event<R>
{
    public record Reducer(ReducerEvent<R> ReducerEvent) : Event<R>;
}
```

Event when we are notified that a reducer ran in the remote database.
The [`ReducerEvent`](#record-reducerevent) contains metadata about the
reducer run, including its arguments and termination
[`Status`](#record-status).

This event is passed to row callbacks resulting from modifications by
the reducer.

#### Variant `SubscribeApplied`

``` codeBlockStandalone_LlrK
record Event<R>
{
    public record SubscribeApplied : Event<R>;
}
```

Event when our subscription is applied and its rows are inserted into
the client cache.

This event is passed to [row `OnInsert` callbacks](#callback-oninsert)
resulting from the new subscription.

#### Variant `UnsubscribeApplied`

``` codeBlockStandalone_LlrK
record Event<R>
{
    public record UnsubscribeApplied : Event<R>;
}
```

Event when our subscription is removed after a call to
[`SubscriptionHandle.Unsubscribe`](#method-unsubscribe) or
[`SubscriptionHandle.UnsubscribeTthen`](#method-unsubscribethen) and its
matching rows are deleted from the client cache.

This event is passed to [row `OnDelete` callbacks](#callback-ondelete)
resulting from the subscription ending.

#### Variant `SubscribeError`

``` codeBlockStandalone_LlrK
record Event<R>
{
    public record SubscribeError(Exception Exception) : Event<R>;
}
```

Event when a subscription ends unexpectedly due to an error.

This event is passed to [row `OnDelete` callbacks](#callback-ondelete)
resulting from the subscription ending.

#### Variant `UnknownTransaction`

``` codeBlockStandalone_LlrK
record Event<R>
{
    public record UnknownTransaction : Event<R>;
}
```

Event when we are notified of a transaction in the remote database which
we cannot associate with a known reducer. This may be an ad-hoc SQL
query or a reducer for which we do not have bindings.

This event is passed to [row callbacks](#callback-oninsert) resulting
from modifications by the transaction.

### Record `ReducerEvent`

``` codeBlockStandalone_LlrK
record ReducerEvent<R>(
    Timestamp Timestamp,
    Status Status,
    Identity CallerIdentity,
    ConnectionId? CallerConnectionId,
    U128? EnergyConsumed,
    R Reducer
)
```

A `ReducerEvent` contains metadata about a reducer run.

### Record `Status`

``` codeBlockStandalone_LlrK
record Status : TaggedEnum<(
    Unit Committed,
    string Failed,
    Unit OutOfEnergy
)>;
```

| Name | Description |
|----|----|
| [`Committed` variant](#variant-committed) | The reducer ran successfully. |
| [`Failed` variant](#variant-failed) | The reducer errored. |
| [`OutOfEnergy` variant](#variant-outofenergy) | The reducer was aborted due to insufficient energy. |

#### Variant `Committed`

The reducer returned successfully and its changes were committed into
the database state. An [`Event.Reducer`](#variant-reducer) passed to a
row callback must have this status in its
[`ReducerEvent`](#record-reducerevent).

#### Variant `Failed`

The reducer returned an error, panicked, or threw an exception. The
record payload is the stringified error message. Formatting of the error
message is unstable and subject to change, so clients should use it only
as a human-readable diagnostic, and in particular should not attempt to
parse the message.

#### Variant `OutOfEnergy`

The reducer was aborted due to insufficient energy balance of the module
owner.

### Record `Reducer`

The module bindings contains an record `Reducer` with a variant for each
reducer defined by the module. Each variant has a payload containing the
arguments to the reducer.

## Type `ReducerEventContext`

A `ReducerEventContext` is an [`IDbContext`](#interface-idbcontext)
augmented with an [`Event`](#record-reducerevent) property.
`ReducerEventContext`s are passed as the first argument to [reducer
callbacks](#observe-and-invoke-reducers).

| Name | Description |
|----|----|
| [`Event` property](#property-event) | [`ReducerEvent`](#record-reducerevent) containing reducer metadata. |
| [`Db` property](#property-db) | Provides access to the client cache. |
| [`Reducers` property](#property-reducers) | Allows requesting reducers run on the remote database. |

### Property `Event`

``` codeBlockStandalone_LlrK
class ReducerEventContext {
    public readonly ReducerEvent<Reducer> Event;
    /* other fields */
}
```

The [`ReducerEvent`](#record-reducerevent) contained in the
`ReducerEventContext` has metadata about the reducer which ran.

### Property `Db`

``` codeBlockStandalone_LlrK
class ReducerEventContext {
    public RemoteTables Db;
    /* other fields */
}
```

The `Db` property of the context provides access to the subscribed view
of the remote database's tables. See [Access the client
cache](#access-the-client-cache).

### Property `Reducers`

``` codeBlockStandalone_LlrK
class ReducerEventContext {
    public RemoteReducers Reducers;
    /* other fields */
}
```

The `Reducers` property of the context provides access to reducers
exposed by the remote module. See [Observe and invoke
reducers](#observe-and-invoke-reducers).

## Type `SubscriptionEventContext`

A `SubscriptionEventContext` is an
[`IDbContext`](#interface-idbcontext). Unlike the other context types,
`SubscriptionEventContext` doesn't have an `Event` property.
`SubscriptionEventContext`s are passed to subscription
[`OnApplied`](#callback-onapplied) and
[`UnsubscribeThen`](#method-unsubscribethen) callbacks.

| Name | Description |
|----|----|
| [`Db` property](#property-db) | Provides access to the client cache. |
| [`Reducers` property](#property-reducers) | Allows requesting reducers run on the remote database. |

### Property `Db`

``` codeBlockStandalone_LlrK
class SubscriptionEventContext {
    public RemoteTables Db;
    /* other fields */
}
```

The `Db` property of the context provides access to the subscribed view
of the remote database's tables. See [Access the client
cache](#access-the-client-cache).

### Property `Reducers`

``` codeBlockStandalone_LlrK
class SubscriptionEventContext {
    public RemoteReducers Reducers;
    /* other fields */
}
```

The `Reducers` property of the context provides access to reducers
exposed by the remote module. See [Observe and invoke
reducers](#observe-and-invoke-reducers).

## Type `ErrorContext`

An `ErrorContext` is an [`IDbContext`](#interface-idbcontext) augmented
with an `Event` property. `ErrorContext`s are passed to subscriptions'
[`OnError`](#callback-onerror) callbacks.

| Name | Description |
|----|----|
| [`Event` property](#property-event) | The error which caused the current error callback. |
| [`Db` property](#property-db) | Provides access to the client cache. |
| [`Reducers` property](#property-reducers) | Allows requesting reducers run on the remote database. |

### Property `Event`

``` codeBlockStandalone_LlrK
class SubscriptionEventContext {
    public readonly Exception Event;
    /* other fields */
}
```

### Property `Db`

``` codeBlockStandalone_LlrK
class ErrorContext {
    public RemoteTables Db;
    /* other fields */
}
```

The `Db` property of the context provides access to the subscribed view
of the remote database's tables. See [Access the client
cache](#access-the-client-cache).

### Property `Reducers`

``` codeBlockStandalone_LlrK
class ErrorContext {
    public RemoteReducers Reducers;
    /* other fields */
}
```

The `Reducers` property of the context provides access to reducers
exposed by the remote database. See [Observe and invoke
reducers](#observe-and-invoke-reducers).

## Access the client cache

All [`IDbContext`](#interface-idbcontext) implementors, including
[`DbConnection`](#type-dbconnection) and
[`EventContext`](#type-eventcontext), have `.Db` properties, which in
turn have methods for accessing tables in the client cache.

Each table defined by a module has an accessor method on this `.Db`
property, generated from the table accessor name using C# naming
conventions (for example, `player_score` becomes `PlayerScore`). The
table accessor methods return table handles which inherit from
[`RemoteTableHandle`](#type-remotetablehandle) and have methods for
searching by index.

| Name | Description |
|----|----|
| [`RemoteTableHandle`](#type-remotetablehandle) | Provides access to subscribed rows of a specific table within the client cache. |
| [Unique constraint index access](#unique-constraint-index-access) | Seek a subscribed row by the value in its unique or primary key column. |
| [BTree index access](#btree-index-access) | Seek subscribed rows by the value in its indexed column. |

### Type `RemoteTableHandle`

Implemented by all table handles.

| Name | Description |
|----|----|
| [`Row` type parameter](#type-row) | The type of rows in the table. |
| [`Count` property](#property-count) | The number of subscribed rows in the table. |
| [`Iter` method](#method-iter) | Iterate over all subscribed rows in the table. |
| [`OnInsert` callback](#callback-oninsert) | Register a callback to run whenever a row is inserted into the client cache. |
| [`OnDelete` callback](#callback-ondelete) | Register a callback to run whenever a row is deleted from the client cache. |
| [`OnUpdate` callback](#callback-onupdate) | Register a callback to run whenever a subscribed row is replaced with a new version. |

#### Type `Row`

``` codeBlockStandalone_LlrK
class RemoteTableHandle<EventContext, Row>
{
    /* members */
}
```

The type of rows in the table.

#### Property `Count`

``` codeBlockStandalone_LlrK
class RemoteTableHandle
{
    public int Count;
}
```

The number of rows of this table resident in the client cache, i.e. the
total number which match any subscribed query.

#### Method `Iter`

``` codeBlockStandalone_LlrK
class RemoteTableHandle
{
    public IEnumerable<Row> Iter();
}
```

An iterator over all the subscribed rows in the client cache, i.e. those
which match any subscribed query.

#### Callback `OnInsert`

``` codeBlockStandalone_LlrK
class RemoteTableHandle
{
    public delegate void RowEventHandler(EventContext context, Row row);
    public event RowEventHandler? OnInsert;
}
```

The `OnInsert` callback runs whenever a new row is inserted into the
client cache, either when applying a subscription or being notified of a
transaction. The passed [`EventContext`](#type-eventcontext) contains an
[`Event`](#record-event) which can identify the change which caused the
insertion, and also allows the callback to interact with the connection,
inspect the client cache and invoke reducers. Newly registered or
canceled callbacks do not take effect until the following event.

See [the quickstart](https://spacetimedb.com/docs/quickstarts/c-sharp) for examples of
registering and unregistering row callbacks.

#### Callback `OnDelete`

``` codeBlockStandalone_LlrK
class RemoteTableHandle
{
    public delegate void RowEventHandler(EventContext context, Row row);
    public event RowEventHandler? OnDelete;
}
```

The `OnDelete` callback runs whenever a previously-resident row is
deleted from the client cache. Newly registered or canceled callbacks do
not take effect until the following event.

See [the quickstart](https://spacetimedb.com/docs/quickstarts/c-sharp) for examples of
registering and unregistering row callbacks.

#### Callback `OnUpdate`

``` codeBlockStandalone_LlrK
class RemoteTableHandle
{
    public delegate void RowEventHandler(EventContext context, Row row);
    public event RowEventHandler? OnUpdate;
}
```

The `OnUpdate` callback runs whenever an already-resident row in the
client cache is updated, i.e. replaced with a new row that has the same
primary key. The handle must have a known primary key for callbacks to
be triggered. This includes tables with primary keys, query builder
views with inferred primary keys, and procedural views declared with
`PrimaryKey`. Newly registered or canceled callbacks do not take effect
until the following event.

See [the quickstart](https://spacetimedb.com/docs/quickstarts/c-sharp) for examples of
registering and unregistering row callbacks.

### Unique constraint index access

For each unique constraint on a table, its table handle has a property
which is a unique index handle and whose name is the unique column name.
This unique index handle has a method `.Find(Column value)`. If a `Row`
with `value` in the unique column is resident in the client cache,
`.Find` returns it. Otherwise it returns null.

#### Example

Given the following module-side `User` definition:

``` codeBlockStandalone_LlrK
[Table(Accessor = "User", Public = true)]
public partial class User
{
    [Unique] // Or [PrimaryKey]
    public Identity Identity;
    ..
}
```

a client would lookup a user as follows:

``` codeBlockStandalone_LlrK
User? FindUser(RemoteTables tables, Identity id) => tables.User.Identity.Find(id);
```

### BTree index access

For each btree index defined on a remote table, its corresponding table
handle has a property which is a btree index handle and whose name is
the name of the index. This index handle has a method
`IEnumerable<Row> Filter(Column value)` which will return `Row`s with
`value` in the indexed `Column`, if there are any in the cache.

#### Example

Given the following module-side `Player` definition:

``` codeBlockStandalone_LlrK
[Table(Accessor = "Player", Public = true)]
public partial class Player
{
    [PrimaryKey]
    public Identity id;

    [SpacetimeDB.Index.BTree(Accessor = "Level")]
    public uint level;
    ..
}
```

a client would count the number of `Player`s at a certain level as
follows:

``` codeBlockStandalone_LlrK
int CountPlayersAtLevel(RemoteTables tables, uint level) => tables.Player.Level.Filter(level).Count();
```

## Observe and invoke reducers

All [`IDbContext`](#interface-idbcontext) implementors, including
[`DbConnection`](#type-dbconnection) and
[`EventContext`](#type-eventcontext), have a `.Reducers` property, which
in turn has methods for invoking reducers defined by the module and
registering callbacks on it.

Each reducer defined by the module has three methods on the `.Reducers`:

- An invoke method, whose name is the reducer's name converted to snake
  case, like `set_name`. This requests that the module run the reducer.
- A callback registation method, whose name is prefixed with `on_`, like
  `on_set_name`. This registers a callback to run whenever we are
  notified that the reducer ran, including successfully committed runs
  and runs we requested which failed. This method returns a callback id,
  which can be passed to the callback remove method.
- A callback remove method, whose name is prefixed with `remove_on_`,
  like `remove_on_set_name`. This cancels a callback previously
  registered via the callback registration method.

## Identify a client

### Type `Identity`

A unique public identifier for a client connected to a database. See the
[module docs](https://spacetimedb.com/docs/intro/key-architecture#identity) for more details.

### Type `ConnectionId`

An opaque identifier for a client connection to a database, intended to
differentiate between connections from the same
[`Identity`](#type-identity). See the [module
docs](https://spacetimedb.com/docs/intro/key-architecture#connectionid) for more details.

### Type `Timestamp`

A point in time, measured in microseconds since the Unix epoch. See the
[module docs](https://spacetimedb.com/docs/tables/column-types) for more details.

### Type `TaggedEnum`

A [tagged union](https://en.wikipedia.org/wiki/Tagged_union) type. When
defining TaggedEnum types in a module, use `partial record`, not
`partial class`. See the [module docs](https://spacetimedb.com/docs/tables/column-types) for
more details.

- [Server module quick reference](#server-module-quick-reference)
- [Project setup](#project-setup)
  - [Using the `dotnet` CLI tool](#using-the-dotnet-cli-tool)
  - [Using Unity](#using-unity)
- [Generate module bindings](#generate-module-bindings)
- [Type `DbConnection`](#type-dbconnection)
  - [Connect to a database](#connect-to-a-database)
    - [Method `WithUri`](#method-withuri)
    - [Method `WithDatabaseName`](#method-withdatabasename)
    - [Method `WithConfirmedReads`](#method-withconfirmedreads)
    - [Callback `OnConnect`](#callback-onconnect)
    - [Callback `OnConnectError`](#callback-onconnecterror)
    - [Callback `OnDisconnect`](#callback-ondisconnect)
    - [Method `WithToken`](#method-withtoken)
    - [Method `Build`](#method-build)
  - [Advance the connection and process
    messages](#advance-the-connection-and-process-messages)
    - [Method `FrameTick`](#method-frametick)
  - [Access tables and reducers](#access-tables-and-reducers)
    - [Property `Db`](#property-db)
    - [Property `Reducers`](#property-reducers)
  - [Interface `IDbContext`](#interface-idbcontext)
  - [Interface `IRemoteDbContext`](#interface-iremotedbcontext)
    - [Method `Db`](#method-db)
      - [Example](#example)
    - [Method `Reducers`](#method-reducers)
      - [Example](#example-1)
    - [Method `Disconnect`](#method-disconnect)
  - [Subscribe to queries](#subscribe-to-queries)
    - [Type `SubscriptionBuilder`](#type-subscriptionbuilder)
      - [Constructor
        `ctx.SubscriptionBuilder()`](#constructor-ctxsubscriptionbuilder)
      - [Callback `OnApplied`](#callback-onapplied)
      - [Callback `OnError`](#callback-onerror)
      - [Method `Subscribe`](#method-subscribe)
      - [Method `AddQuery`](#method-addquery)
      - [Method `SubscribeToAllTables`](#method-subscribetoalltables)
    - [Type `TypedSubscriptionBuilder`](#type-typedsubscriptionbuilder)
      - [Method `AddQuery`
        (TypedSubscriptionBuilder)](#method-addquery-typedsubscriptionbuilder)
      - [Method `Subscribe`
        (TypedSubscriptionBuilder)](#method-subscribe-typedsubscriptionbuilder)
- [Query Builder API](#query-builder-api)
  - [Entry Point](#entry-point)
  - [Building Queries with `Where` /
    `Filter`](#building-queries-with-where--filter)
  - [Comparison Operators](#comparison-operators)
  - [Boolean Combinators](#boolean-combinators)
  - [Semijoins](#semijoins)
  - [Using Query Builders with
    Subscriptions](#using-query-builders-with-subscriptions)
    - [Type `SubscriptionHandle`](#type-subscriptionhandle)
      - [Property `IsEnded`](#property-isended)
      - [Property `IsActive`](#property-isactive)
      - [Method `Unsubscribe`](#method-unsubscribe)
      - [Method `UnsubscribeThen`](#method-unsubscribethen)
  - [Read connection metadata](#read-connection-metadata)
    - [Property `Identity`](#property-identity)
    - [Property `ConnectionId`](#property-connectionid)
    - [Property `IsActive`](#property-isactive-1)
- [Type `EventContext`](#type-eventcontext)
  - [Property `Event`](#property-event)
  - [Property `Db`](#property-db-1)
  - [Field `Reducers`](#field-reducers)
  - [Record `Event`](#record-event)
    - [Variant `Reducer`](#variant-reducer)
    - [Variant `SubscribeApplied`](#variant-subscribeapplied)
    - [Variant `UnsubscribeApplied`](#variant-unsubscribeapplied)
    - [Variant `SubscribeError`](#variant-subscribeerror)
    - [Variant `UnknownTransaction`](#variant-unknowntransaction)
  - [Record `ReducerEvent`](#record-reducerevent)
  - [Record `Status`](#record-status)
    - [Variant `Committed`](#variant-committed)
    - [Variant `Failed`](#variant-failed)
    - [Variant `OutOfEnergy`](#variant-outofenergy)
  - [Record `Reducer`](#record-reducer)
- [Type `ReducerEventContext`](#type-reducereventcontext)
  - [Property `Event`](#property-event-1)
  - [Property `Db`](#property-db-2)
  - [Property `Reducers`](#property-reducers-1)
- [Type `SubscriptionEventContext`](#type-subscriptioneventcontext)
  - [Property `Db`](#property-db-3)
  - [Property `Reducers`](#property-reducers-2)
- [Type `ErrorContext`](#type-errorcontext)
  - [Property `Event`](#property-event-2)
  - [Property `Db`](#property-db-4)
  - [Property `Reducers`](#property-reducers-3)
- [Access the client cache](#access-the-client-cache)
  - [Type `RemoteTableHandle`](#type-remotetablehandle)
    - [Type `Row`](#type-row)
    - [Property `Count`](#property-count)
    - [Method `Iter`](#method-iter)
    - [Callback `OnInsert`](#callback-oninsert)
    - [Callback `OnDelete`](#callback-ondelete)
    - [Callback `OnUpdate`](#callback-onupdate)
  - [Unique constraint index access](#unique-constraint-index-access)
    - [Example](#example-2)
  - [BTree index access](#btree-index-access)
    - [Example](#example-3)
- [Observe and invoke reducers](#observe-and-invoke-reducers)
- [Identify a client](#identify-a-client)
  - [Type `Identity`](#type-identity)
  - [Type `ConnectionId`](#type-connectionid)
  - [Type `Timestamp`](#type-timestamp)
  - [Type `TaggedEnum`](#type-taggedenum)

{% endraw %}
