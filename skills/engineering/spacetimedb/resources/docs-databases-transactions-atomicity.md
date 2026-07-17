Version: 2.0.0

On this page

SpacetimeDB provides strong transactional guarantees for all database
operations. Every [reducer](/docs/functions/reducers) runs inside a
database transaction, ensuring your data remains consistent and reliable
even under concurrent load.

## What is a Transaction?

A [database
transaction](https://en.wikipedia.org/wiki/Database_transaction) is a
sequence of operations that execute as a single, indivisible unit of
work. In SpacetimeDB, each reducer invocation is a transaction - either
all of its changes succeed and are committed to the database, or all
changes are rolled back as if the reducer never ran.

## ACID Properties

SpacetimeDB transactions provide the following guarantees:

### Atomicity

**All-or-nothing execution**: A reducer's changes either all succeed or
all fail together. There's no partial state.

- If a reducer completes successfully, all changes (inserts, updates,
  deletes) are **committed** to the database
- If a reducer throws an exception or returns an error, all changes are
  **rolled back** and the database remains unchanged
- It's not possible to keep some changes and discard others from a
  single reducer execution

note

This atomicity guarantee applies to **reducers**, which run in a single
transaction. [Procedures](/docs/functions/procedures) can manually open
multiple separate transactions, where each transaction is individually
atomic, but the procedure as a whole is not. See [Procedures: Manual
Transactions](#procedures-manual-transactions) below.

### Consistency

**Valid states only**: Transactions ensure the database moves from one
valid state to another. All constraints (unique keys, indexes, foreign
key-like relationships in your logic) are enforced.

- Unique constraints are checked before commit
- If a constraint would be violated, the entire transaction is rolled
  back
- The database never enters an invalid state

### Isolation

**Consistent snapshots**: Each reducer sees a consistent snapshot of the
database and doesn't observe partial changes from other reducer
executions.

- A reducer sees the database state as it was at the start of its
  transaction
- A reducer will not observe the effects of other reducers modifying the
  database while it runs
- Each reducer completes fully before its changes are visible to others

This prevents race conditions and ensures predictable behavior.

### Durability

**Persistent changes**: Once a reducer successfully commits, its changes
are permanent and will survive server restarts. SpacetimeDB persists
committed transactions to disk.

## Transaction Scope

### Reducers: Automatic Transactions

Every reducer invocation automatically runs in its own transaction:

- The transaction starts when the reducer is called
- The transaction commits when the reducer returns successfully
- The transaction rolls back if the reducer throws an exception or
  returns an error

You don't need to manually start or commit transactions in reducers -
SpacetimeDB handles this automatically.

### Nested Reducer Calls

When a reducer calls another reducer directly (not via scheduling), they
execute in the **same transaction**:

- TypeScript
- C#
- Rust
- C++

``` codeBlockStandalone_LlrK
export const parent_reducer = spacetimedb.reducer((ctx) => {
    TableA.insert({ /* ... */ });
    
    // This runs in the SAME transaction
    childReducer(ctx);
    
    TableB.insert({ /* ... */ });
    
    // All changes from both parent and child commit together
});

function childReducer(ctx) {
    TableC.insert({ /* ... */ });
    
    // If this throws, the parent's changes also roll back
    if (someCondition) {
        throw new Error('Child failed');
    }
}
```

``` codeBlockStandalone_LlrK
[SpacetimeDB.Reducer]
public static void ParentReducer(ReducerContext ctx)
{
    ctx.Db.TableA.Insert(new RowA { /* ... */ });
    
    // This runs in the SAME transaction
    ChildReducer(ctx);
    
    ctx.Db.TableB.Insert(new RowB { /* ... */ });
    
    // All changes from both parent and child commit together
}

[SpacetimeDB.Reducer]
public static void ChildReducer(ReducerContext ctx)
{
    ctx.Db.TableC.Insert(new RowC { /* ... */ });
    
    // If this throws, the parent's changes also roll back
    if (someCondition)
    {
        throw new Exception("Child failed");
    }
}
```

``` codeBlockStandalone_LlrK
#[reducer]
pub fn parent_reducer(ctx: &ReducerContext) -> Result<(), String> {
    ctx.db.table_a().insert(RowA { /* ... */ });
    
    // This runs in the SAME transaction
    child_reducer(ctx)?;
    
    ctx.db.table_b().insert(RowB { /* ... */ });
    
    // All changes from both parent and child commit together
    Ok(())
}

#[reducer]
pub fn child_reducer(ctx: &ReducerContext) -> Result<(), String> {
    ctx.db.table_c().insert(RowC { /* ... */ });
    
    // If this returns Err, the parent's changes also roll back
    if some_condition {
        return Err("Child failed".to_string());
    }
    
    Ok(())
}
```

``` codeBlockStandalone_LlrK
using namespace SpacetimeDB;

// Forward declare child reducer to allow calling it before its definition
ReducerResult child_reducer(ReducerContext&, bool some_condition);

SPACETIMEDB_REDUCER(parent_reducer, ReducerContext ctx, bool some_condition) {
    ctx.db[table_a].insert(RowA{ /* ... */ });
    
    // This runs in the SAME transaction
    ReducerResult result = child_reducer(ctx, some_condition);
    if (result.is_err()) {
        return result;
    }
    
    ctx.db[table_b].insert(RowB{ /* ... */ });
    
    // All changes from both parent and child commit together
    return Ok();
}

SPACETIMEDB_REDUCER(child_reducer, ReducerContext ctx, bool some_condition) {
    ctx.db[table_c].insert(RowC{ /* ... */ });
    
    // If this returns Err, the parent's changes also roll back
    if (some_condition) {
        return Err("Child failed");
    }
    
    return Ok();
}
```

important

SpacetimeDB does **not** support nested transactions. Nested reducer
calls execute in the same transaction as their parent. If you need
separate transactions, use [scheduled
reducers](/docs/tables/schedule-tables) instead.

### Procedures: Manual Transactions

Unlike reducers, [procedures](/docs/functions/procedures) don't
automatically run in transactions. Procedures can run transactions, but
must manually open them using `with_tx` (Rust) or `withTx` (TypeScript).
This gives procedures more flexibility:

- Procedures can perform operations **outside** transactions (like HTTP
  requests)
- Procedures can open **multiple separate transactions** if needed
- Each `with_tx`/`withTx` call creates a new transaction that commits
  independently

See [Procedures](/docs/functions/procedures) for more details on manual
transaction management.

## Best Practices

### Keep Transactions Short

- Perform only necessary database operations within reducers
- Move external I/O (HTTP requests, etc.) to
  [procedures](/docs/functions/procedures)
- Shorter transactions reduce contention and improve throughput

### Handle Errors Gracefully

- Return descriptive errors to help clients understand failures
- Use `Result<(), String>` in Rust or throw exceptions with clear
  messages
- Remember: any error rolls back ALL changes

## Limitations

### No Nested Transactions

SpacetimeDB does not support nested transactions. When one reducer calls
another, they share the same transaction. If you need separate
transactions, use [scheduled reducers](/docs/tables/schedule-tables) to
trigger the second reducer asynchronously.

### Auto-Increment is Not Transactional

The `#[auto_inc]` sequence generator is not transactional:

- Sequence numbers are allocated even if a transaction rolls back
- This can create gaps in your sequence
- See [Auto-Increment](/docs/tables/auto-increment#crash-recovery) for
  details

## Related Topics

- **[Reducers](/docs/functions/reducers)** - Functions that modify
  database state transactionally
- **[Procedures](/docs/functions/procedures)** - Functions with manual
  transaction control
- **[Schedule Tables](/docs/tables/schedule-tables)** - Schedule
  reducers for separate transactions
- **[Subscriptions](/docs/clients/subscriptions)** - How clients receive
  transactional updates

- [What is a Transaction?](#what-is-a-transaction)
- [ACID Properties](#acid-properties)
  - [Atomicity](#atomicity)
  - [Consistency](#consistency)
  - [Isolation](#isolation)
  - [Durability](#durability)
- [Transaction Scope](#transaction-scope)
  - [Reducers: Automatic Transactions](#reducers-automatic-transactions)
  - [Nested Reducer Calls](#nested-reducer-calls)
  - [Procedures: Manual Transactions](#procedures-manual-transactions)
- [Best Practices](#best-practices)
  - [Keep Transactions Short](#keep-transactions-short)
  - [Handle Errors Gracefully](#handle-errors-gracefully)
- [Limitations](#limitations)
  - [No Nested Transactions](#no-nested-transactions)
  - [Auto-Increment is Not
    Transactional](#auto-increment-is-not-transactional)
- [Related Topics](#related-topics)
