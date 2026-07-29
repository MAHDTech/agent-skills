+++
title = "reference-javascript-sql-12136da7"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

{% raw %}
# @tauri-apps/plugin-sql

## Classes

### default

**Database**

The `Database` class serves as the primary interface for communicating
with the rust side of the sql plugin.

#### Constructors

##### new default()

```
new default(path): default
```

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

###### Returns

[`default`](https://v2.tauri.app/reference/javascript/sql/#default)

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L29>

#### Properties

| Property | Type | Defined in |
|----|----|----|
|  `path` | `string` | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L28> |

#### Methods

##### close()

```
close(db?): Promise<boolean>
```

**close**

Closes the database connection pool.

###### Parameters

| Parameter | Type | Description |
|----|----|----|
| `db`? | `string` | Optionally state the name of a database if you are managing more than one. Otherwise, all database pools will be in scope. |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`boolean`\>

###### Example

```
const success = await db.close()
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L162>

##### execute()

```
execute(query, bindValues?): Promise<QueryResult>
```

**execute**

Passes a SQL expression to the database for execution.

###### Parameters

| Parameter     | Type          |
|---------------|---------------|
| `query`       | `string`      |
| `bindValues`? | `unknown`\[\] |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`QueryResult`](https://v2.tauri.app/reference/javascript/sql/#queryresult)\>

###### Example

```
// for sqlite & postgres// INSERT exampleconst result = await db.execute(   "INSERT into todos (id, title, status) VALUES ($1, $2, $3)",   [ todos.id, todos.title, todos.status ]);// UPDATE exampleconst result = await db.execute(   "UPDATE todos SET title = $1, completed = $2 WHERE id = $3",   [ todos.title, todos.status, todos.id ]);
// for mysql// INSERT exampleconst result = await db.execute(   "INSERT into todos (id, title, status) VALUES (?, ?, ?)",   [ todos.id, todos.title, todos.status ]);// UPDATE exampleconst result = await db.execute(   "UPDATE todos SET title = ?, completed = ? WHERE id = ?",   [ todos.title, todos.status, todos.id ]);
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L108>

##### select()

```
select<T>(query, bindValues?): Promise<T>
```

**select**

Passes in a SELECT query to the database for execution.

###### Type Parameters

| Type Parameter |
|----------------|
| `T`            |

###### Parameters

| Parameter     | Type          |
|---------------|---------------|
| `query`       | `string`      |
| `bindValues`? | `unknown`\[\] |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<`T`\>

###### Example

```
// for sqlite & postgresconst result = await db.select(   "SELECT * from todos WHERE id = $1", [ id ]);
// for mysqlconst result = await db.select(   "SELECT * from todos WHERE id = ?", [ id ]);
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L141>

##### get()

```
static get(path): default
```

**get**

A static initializer which synchronously returns an instance of the
Database class while deferring the actual database connection until the
first invocation or selection on the database.

# Sqlite

The path is relative to `tauri::path::BaseDirectory::App` and must start
with `sqlite:`.

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

###### Returns

[`default`](https://v2.tauri.app/reference/javascript/sql/#default)

###### Example

```
const db = Database.get("sqlite:test.db");
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L72>

##### load()

```
static load(path): Promise<default>
```

**load**

A static initializer which connects to the underlying database and
returns a `Database` instance once a connection to the database is
established.

# Sqlite

The path is relative to `tauri::path::BaseDirectory::App` and must start
with `sqlite:`.

###### Parameters

| Parameter | Type     |
|-----------|----------|
| `path`    | `string` |

###### Returns

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)\<[`default`](https://v2.tauri.app/reference/javascript/sql/#default)\>

###### Example

```
const db = await Database.load("sqlite:test.db");
```

**Source**:
<https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L48>

## Interfaces

### QueryResult

#### Properties

| Property | Type | Description | Defined in |
|----|----|----|----|
|  `lastInsertId?` | `number` | The last inserted `id`. This value is not set for Postgres databases. If the last inserted id is required on Postgres, the `select` function must be used, with a `RETURNING` clause (`INSERT INTO todos (title) VALUES ($1) RETURNING id`). | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L18> |
|  `rowsAffected` | `number` | The number of rows affected by the query. | **Source**: <https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/guest-js/index.ts#L9> |

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

{% endraw %}
