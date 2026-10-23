---
name: spacetime
description: Develop, publish, and debug Spacetime (formerly SpacetimeDB) backend modules in Rust, C#, TypeScript, or C++ and connect real-time clients. Use when defining tables, reducers, procedures, views, HTTP handlers, schedules, or subscriptions, using the spacetime CLI, generating client bindings, or migrating SpacetimeDB applications.
resources:
  - https://spacetimedb.com/docs/llms.txt
---

# Spacetime development

Spacetime combines a relational database, backend logic, and real-time subscriptions. TypeScript modules run on V8; Rust, C#, and C++ modules compile to WebAssembly. Clients call module functions and subscribe to SQL queries over WebSocket.

The platform is now called Spacetime. Technical identifiers still include `spacetimedb` (Rust crate and TypeScript package), `SpacetimeDB` (C# namespace), and `spacetimedb.com`. Preserve those identifiers when writing code; the CLI executable is `spacetime`.

## Start with the installed version

1. Inspect the module's dependency versions, client SDK versions, generated bindings, and `spacetime.json`. Run `spacetime version` and check command flags with `spacetime help <command>`.
2. Find the relevant page in the [downloaded documentation index](resources/auto/docs-llms.txt). Search the [full documentation](resources/auto/docs-llms-full.txt) by page heading or source URL for readable examples; individual downloaded pages can contain HTML.
3. Match examples to the project's language and version. The current documentation targets 2.x; read its upgrade notes before changing a 1.x project. Check the [upstream index](https://spacetimedb.com/docs/llms.txt) when local resources lack an API or appear stale.

Use these documentation sections for the task at hand:

| Task                           | Sections to find in the full documentation                                    |
| :----------------------------- | :---------------------------------------------------------------------------- |
| Module setup and language APIs | Quickstarts, Building and Publishing, Language Support                        |
| Schemas and authorization      | Tables, Access Permissions, Authentication, Row-Level Security                |
| Backend functions              | Reducers, Reducer Context, Procedures, Views, HTTP Handlers                   |
| Timers and notifications       | Schedule Tables, Event Tables                                                 |
| Client integration             | Generating Client Bindings, Connection, Subscriptions, Subscription Semantics |
| Schema changes                 | Automatic Migrations, Incremental Migrations, 1.x to 2.0 Upgrade Notes        |
| Reusable backend components    | Submodules                                                                    |
| Verification and operations    | Unit Test Modules, CLI Reference, Troubleshooting                             |

## Choose the execution boundary

| Function     | Database access                             | Transaction boundary                          | Use for                                            |
| :----------- | :------------------------------------------ | :-------------------------------------------- | :------------------------------------------------- |
| Reducer      | Read and write                              | One automatic transaction per call            | Validated state changes                            |
| Procedure    | Read and write inside explicit transactions | Each transaction commits independently        | External HTTP calls and workflows returning values |
| View         | Read-only                                   | Evaluated by the host; supports subscriptions | Derived or caller-filtered data                    |
| HTTP handler | Follow the language's handler context API   | Open explicit transactions for database work  | HTTP routes and webhooks                           |

- Keep reducers deterministic. Use their context for caller identity, timestamps, and randomness. A returned error or panic rolls back the transaction.
- Procedures can perform external I/O. Keep blocking I/O outside transactions. Transaction callbacks may be retried, so keep external side effects outside those callbacks as well.
- Implement authorization inside every callable mutation. A private table restricts client reads; it does not prevent a reducer from exposing or changing its rows on a caller's behalf.
- HTTP handlers receive an anonymous sender; user-defined routes do not automatically authenticate an `Authorization` header. Validate credentials explicitly, or use a procedure when the host must supply verified JWT claims.
- Use the documented lifecycle hooks: `init`, `client_connected`, and `client_disconnected` in Rust. Initialization runs on first publish or after clearing the database; module updates require the documented migration path.

## Model durable state and transient events

- Tables are private by default. Mark a table public only when every connected client may read its rows, or expose a view with the required filtering.
- Declare primary keys, uniqueness, and indexes around the actual lookup and subscription queries. An auto-increment column receives a generated value when inserted as zero.
- Event rows exist for the creating transaction and are broadcast on commit. Their inserts are recorded in the commitlog, but the table is empty between transactions. Use ordinary tables when reconnecting clients need retained history, such as chat messages.
- Scheduled tables need the documented scheduling attribute, primary key, and `ScheduleAt` column. Enforce the documented caller check so clients cannot invoke scheduled work without authorization.
- Check every schema change against Automatic Migrations before publishing. Additive changes can be supported, but compatibility depends on constraints, defaults, canonical names, and existing clients. Regenerate bindings after schema changes.

## Minimal Rust module

Keep the `spacetimedb` import even though the skill and product use Spacetime.

```rust
use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

#[table(accessor = user_profile, public)]
pub struct UserProfile {
    #[primary_key]
    pub identity: Identity,
    pub username: String,
}

#[reducer]
pub fn register_user(ctx: &ReducerContext, username: String) -> Result<(), String> {
    let username = username.trim().to_owned();
    if username.is_empty() {
        return Err("Username cannot be empty".to_owned());
    }

    if let Some(mut profile) = ctx.db.user_profile().identity().find(ctx.sender()) {
        profile.username = username;
        ctx.db.user_profile().identity().update(profile);
    } else {
        ctx.db.user_profile().try_insert(UserProfile {
            identity: ctx.sender(),
            username,
        })?;
    }
    Ok(())
}
```

## Build, publish, and connect

Run commands from the project directory. Resolve module paths, database names, and servers from the project configuration, and confirm flags against the installed CLI.

| Command                                                                                    | Purpose                                                                          |
| :----------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------- |
| `spacetime init --lang rust my-app`                                                        | Scaffold a project; other module languages are `typescript`, `csharp`, and `cpp` |
| `spacetime start`                                                                          | Run a local standalone host                                                      |
| `spacetime build --module-path <module-dir>`                                               | Compile a module                                                                 |
| `spacetime dev`                                                                            | Rebuild, publish, and regenerate bindings during development                     |
| `spacetime publish --server local --module-path <module-dir> <database>`                   | Publish to the local host                                                        |
| `spacetime generate --lang typescript --out-dir <bindings-dir> --module-path <module-dir>` | Generate bindings from a local module                                            |
| `spacetime logs --server local <database>`                                                 | Inspect server execution logs                                                    |
| `spacetime mcp`                                                                            | Expose the documented agent tools over stdio                                     |

1. Build and publish the module locally, then generate client bindings for that schema. Keep generated files generated.
2. Build the connection with the generated `DbConnection` API and the intended server, database name, and authentication token. Persist identity tokens using the client's documented mechanism.
3. Register row callbacks and subscribe to the required tables or views. Wait for subscription application before treating the local cache as initialized. The cache contains subscribed data, not the whole database.
4. Invoke reducers or procedures through generated bindings. Handle connection failures, subscription errors, rejected mutations, and disconnects using the SDK's callbacks or promises.

## Verify the change

- Run the module and client build commands used by the project. For module unit tests, consult Unit Test Modules for the installed language and version's harness.
- On a local database, exercise a successful mutation and an invalid or unauthorized mutation. Verify the failure leaves stored state unchanged.
- Connect two clients, subscribe, mutate from one, and confirm the other receives the expected committed update. Check filtered reads with distinct identities when access rules change.
- For schema changes, republish against representative existing data and regenerate bindings. Confirm retained data and client subscriptions still work.
- Inspect `spacetime logs` for reducer errors, transaction failures, and subscription failures before reporting completion.
