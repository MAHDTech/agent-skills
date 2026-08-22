+++
title = "docs-core-concepts"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

Version: 2.0.0

On this page

This section covers the fundamental concepts you need to understand to
build applications with SpacetimeDB.

## Databases

Learn how SpacetimeDB databases work, including modules, publishing, and
transactions.

- [What is a Database?](https://spacetimedb.com/docs/databases) - Understanding SpacetimeDB
  databases and modules
- [Building & Publishing](https://spacetimedb.com/docs/databases/building-publishing) - Deploy
  your module to SpacetimeDB
- [Transactions](https://spacetimedb.com/docs/databases/transactions-atomicity) - How atomicity
  and rollback work
- [Migrations](https://spacetimedb.com/docs/databases/automatic-migrations) - Evolving your
  schema over time

## Tables

Define your data model with tables, columns, and indexes.

- [Tables Overview](https://spacetimedb.com/docs/tables) - Declaring and using tables
- [Column Types](https://spacetimedb.com/docs/tables/column-types) - Supported column types
- [Indexes](https://spacetimedb.com/docs/tables/indexes) - Optimizing queries with indexes
- [Access Permissions](https://spacetimedb.com/docs/tables/access-permissions) - Public vs
  private tables
- [Schedule Tables](https://spacetimedb.com/docs/tables/schedule-tables) - Time-based
  operations

## Functions

Implement your application logic with reducers, procedures, and views.

- [Reducers](https://spacetimedb.com/docs/functions/reducers) - Transactional functions that
  modify state
- [Procedures](https://spacetimedb.com/docs/functions/procedures) - Functions that can make
  external HTTP calls
- [Views](https://spacetimedb.com/docs/functions/views) - Read-only computed queries

## Authentication

Secure your application with SpacetimeAuth.

- [Authentication](https://spacetimedb.com/docs/core-concepts/authentication) - Using OpenID
  Connect (OIDC) for identity and access control
- [SpacetimeAuth
  Overview](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/) - Managed
  authentication service
- [Auth Claims](https://spacetimedb.com/docs/core-concepts/authentication/usage) - Using
  identity and roles

## Clients

Connect your frontend to SpacetimeDB.

- [SDK Overview](https://spacetimedb.com/docs/clients) - Available client SDKs
- [Code Generation](https://spacetimedb.com/docs/clients/codegen) - Generate type-safe bindings
- [Connecting to SpacetimeDB](https://spacetimedb.com/docs/clients/connection) - Establish and
  manage client connections
- [SDK API Overview](https://spacetimedb.com/docs/clients/api) - Core API concepts shared
  across SDKs
- [Subscriptions](https://spacetimedb.com/docs/clients/subscriptions) - Subscribe to data and
  keep a local cache in sync
- [Subscription Semantics](https://spacetimedb.com/docs/clients/subscriptions/semantics) -
  Understand subscription consistency and ordering guarantees
- [TypeScript](https://spacetimedb.com/docs/clients/typescript), [Rust](https://spacetimedb.com/docs/clients/rust),
  [C#](https://spacetimedb.com/docs/clients/c-sharp), [Unreal](https://spacetimedb.com/docs/clients/unreal) -
  Language-specific references

