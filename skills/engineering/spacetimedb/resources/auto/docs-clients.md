Version: 2.0.0

On this page

The SpacetimeDB Client SDKs provide a comprehensive interface for
building applications that connect to SpacetimeDB
[databases](https://spacetimedb.com/docs/databases). Client applications can query data, invoke
server-side functions, and receive real-time updates as the database
state changes.

## Available SDKs

SpacetimeDB provides client SDKs for multiple languages:

- [Rust](https://spacetimedb.com/docs/clients/rust) - [(Quickstart)](https://spacetimedb.com/docs/quickstarts/rust)
- [C#](https://spacetimedb.com/docs/clients/c-sharp) -
  [(Quickstart)](https://spacetimedb.com/docs/quickstarts/c-sharp)
- [TypeScript](https://spacetimedb.com/docs/clients/typescript) -
  [(Quickstart)](https://spacetimedb.com/docs/quickstarts/typescript)
- [Unreal](https://spacetimedb.com/docs/clients/unreal) - [(Tutorial)](https://spacetimedb.com/docs/tutorials/unreal)

## Getting Started

To build a client application with SpacetimeDB:

1.  **[Generate client bindings](https://spacetimedb.com/docs/clients/codegen)** - Use
    `spacetime generate` to create type-safe bindings for your
    [database](https://spacetimedb.com/docs/databases)
2.  **[Connect to your database](https://spacetimedb.com/docs/clients/connection)** - Establish
    a WebSocket connection to SpacetimeDB
3.  **[Use the SDK API](https://spacetimedb.com/docs/clients/api)** - Subscribe to data, invoke
    functions, and register callbacks

## Core Capabilities

### Connection Management

The SDKs handle establishing and maintaining WebSocket connections to
SpacetimeDB servers. Connections support authentication via tokens (for
example, from
[SpacetimeAuth](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/)) and
provide lifecycle callbacks for connect, disconnect, and error events.

See [Connecting to SpacetimeDB](https://spacetimedb.com/docs/clients/connection) for details.

### Client-Side Data Cache

Each client maintains a local cache of database rows through
[subscriptions](https://spacetimedb.com/docs/clients/subscriptions). Clients define which data
they need using typed query builders (or raw SQL when needed), and
SpacetimeDB automatically synchronizes changes to the subscribed data.
The local cache can be queried without network round-trips, providing
fast access to frequently-read data.

### Real-Time Updates

Clients receive automatic updates when subscribed data changes. The SDKs
provide callbacks for observing:

- **Subscription updates** - When subscription queries are applied or
  fail
- **Row changes** - When rows are inserted, updated, or deleted in the
  local cache
- **Reducer invocations** - When [reducers](https://spacetimedb.com/docs/functions/reducers)
  run on the server
- **Procedure results** - When [procedures](https://spacetimedb.com/docs/functions/procedures)
  are called the results are returned via a callback

### Invoking Server Functions

Clients can invoke server-side functions to modify data or perform
operations:

- **[Reducers](https://spacetimedb.com/docs/functions/reducers)** - Transactional functions
  that modify database state
- **[Procedures](https://spacetimedb.com/docs/functions/procedures)** - Functions that can
  perform external operations like HTTP requests

### Type Safety

The [generated client bindings](https://spacetimedb.com/docs/clients/codegen) provide
compile-time type safety between your client and server code. Table
schemas, function signatures, and return types are all reflected in the
generated code, catching errors before runtime.

## Choosing a Language

When selecting a language for your client application, consider these
factors:

### Team Expertise

Choose a language your development team is comfortable with to maximize
productivity and reduce development time.

### Application Type

- **Web applications** - TypeScript integrates seamlessly with browser
  and Node.js environments
- **Desktop applications** - Rust or C# depending on your platform and
  requirements
- **Performance-critical applications** - Rust offers the best
  performance and memory efficiency
- **Unity games** - C# is required for Unity development
- **Unreal games** - C++ and Blueprint are both supported for Unreal
  clients

### Platform and Ecosystem

Each language has its own ecosystem of libraries and tools. If your
application depends on specific libraries or frameworks, that may
influence your choice.

The functionality of the SDKs remains consistent across languages, so
transitioning between them primarily involves syntax changes rather than
architectural changes. You can even use multiple languages in the same
project - for example, C# for a Unity game client and TypeScript for a
web administration panel.

## Learning Path

New to SpacetimeDB client development? Follow this progression:

1.  **[Generate Client Bindings](https://spacetimedb.com/docs/clients/codegen)** - Create
    type-safe interfaces from your module
2.  **[Connect to SpacetimeDB](https://spacetimedb.com/docs/clients/connection)** - Establish a
    connection and understand the lifecycle
3.  **[Use the SDK API](https://spacetimedb.com/docs/clients/api)** - Learn about
    subscriptions, reducers, and callbacks
4.  **Language Reference** - Dive into language-specific details:
    [Rust](https://spacetimedb.com/docs/clients/rust), [C#](https://spacetimedb.com/docs/clients/c-sharp),
    [TypeScript](https://spacetimedb.com/docs/clients/typescript), and [Unreal
    Engine](https://spacetimedb.com/docs/clients/unreal)

## Next Steps

- To build your first client, follow a **Quickstart guide** for
  [Rust](https://spacetimedb.com/docs/quickstarts/rust), [C#](https://spacetimedb.com/docs/quickstarts/c-sharp), or
  [TypeScript](https://spacetimedb.com/docs/quickstarts/typescript), or use the [Unreal
  tutorial](https://spacetimedb.com/docs/tutorials/unreal)
- Learn about [Databases](https://spacetimedb.com/docs/databases) to understand what you're
  connecting to
- Explore [Subscriptions](https://spacetimedb.com/docs/clients/subscriptions) for efficient
  data synchronization
- Review [Reducers](https://spacetimedb.com/docs/functions/reducers) to understand server-side
  state changes

- [Available SDKs](#available-sdks)
- [Getting Started](#getting-started)
- [Core Capabilities](#core-capabilities)
  - [Connection Management](#connection-management)
  - [Client-Side Data Cache](#client-side-data-cache)
  - [Real-Time Updates](#real-time-updates)
  - [Invoking Server Functions](#invoking-server-functions)
  - [Type Safety](#type-safety)
- [Choosing a Language](#choosing-a-language)
  - [Team Expertise](#team-expertise)
  - [Application Type](#application-type)
  - [Platform and Ecosystem](#platform-and-ecosystem)
- [Learning Path](#learning-path)
- [Next Steps](#next-steps)
