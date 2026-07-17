Version: 2.0.0

On this page

This section covers the fundamental concepts you need to understand to
build applications with SpacetimeDB.

## Databases

Learn how SpacetimeDB databases work, including modules, publishing, and
transactions.

- [What is a Database?](/docs/databases) - Understanding SpacetimeDB
  databases and modules
- [Building & Publishing](/docs/databases/building-publishing) - Deploy
  your module to SpacetimeDB
- [Transactions](/docs/databases/transactions-atomicity) - How atomicity
  and rollback work
- [Migrations](/docs/databases/automatic-migrations) - Evolving your
  schema over time

## Tables

Define your data model with tables, columns, and indexes.

- [Tables Overview](/docs/tables) - Declaring and using tables
- [Column Types](/docs/tables/column-types) - Supported column types
- [Indexes](/docs/tables/indexes) - Optimizing queries with indexes
- [Access Permissions](/docs/tables/access-permissions) - Public vs
  private tables
- [Schedule Tables](/docs/tables/schedule-tables) - Time-based
  operations

## Functions

Implement your application logic with reducers, procedures, and views.

- [Reducers](/docs/functions/reducers) - Transactional functions that
  modify state
- [Procedures](/docs/functions/procedures) - Functions that can make
  external HTTP calls
- [Views](/docs/functions/views) - Read-only computed queries

## Authentication

Secure your application with SpacetimeAuth.

- [Authentication](/docs/core-concepts/authentication) - Using OpenID
  Connect (OIDC) for identity and access control
- [SpacetimeAuth
  Overview](/docs/core-concepts/authentication/spacetimeauth/) - Managed
  authentication service
- [Auth Claims](/docs/core-concepts/authentication/usage) - Using
  identity and roles

## Clients

Connect your frontend to SpacetimeDB.

- [SDK Overview](/docs/clients) - Available client SDKs
- [Code Generation](/docs/clients/codegen) - Generate type-safe bindings
- [Connecting to SpacetimeDB](/docs/clients/connection) - Establish and
  manage client connections
- [SDK API Overview](/docs/clients/api) - Core API concepts shared
  across SDKs
- [Subscriptions](/docs/clients/subscriptions) - Subscribe to data and
  keep a local cache in sync
- [Subscription Semantics](/docs/clients/subscriptions/semantics) -
  Understand subscription consistency and ordering guarantees
- [TypeScript](/docs/clients/typescript), [Rust](/docs/clients/rust),
  [C#](/docs/clients/c-sharp), [Unreal](/docs/clients/unreal) -
  Language-specific references

- [Databases](#databases)
- [Tables](#tables)
- [Functions](#functions)
- [Authentication](#authentication)
- [Clients](#clients)
