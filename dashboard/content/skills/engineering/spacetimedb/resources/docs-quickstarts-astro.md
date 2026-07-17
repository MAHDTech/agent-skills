+++
title = "docs-quickstarts-astro"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

Version: 2.0.0

Get a SpacetimeDB Astro app running in under 5 minutes.

## Prerequisites

- [Node.js](https://nodejs.org/) 18+ installed
- [SpacetimeDB CLI](https://spacetimedb.com/install) installed

[](https://spacetimedb.com/install)

###### Install the SpacetimeDB CLI tool

------------------------------------------------------------------------

**Create your project**

Run the `spacetime dev` command to create a new project with a
SpacetimeDB module and Astro client.

This will start the local SpacetimeDB server, publish your module,
generate TypeScript bindings, and start the Astro development server.

``` codeBlockStandalone_LlrK
spacetime dev --template astro-ts
```

**Open your app**

Navigate to [http://localhost:4321](http://localhost:4321) to see your
app running.

The Astro app reads `SPACETIMEDB_*` variables on the server and
`PUBLIC_SPACETIMEDB_*` variables in the client, so `.env.local` can
configure both sides of the app.

**Explore the project structure**

Your project contains both server and client code using Astro SSR and a
live React island for real-time updates.

Edit `spacetimedb/src/index.ts` to add tables and reducers. Edit
`src/pages/index.astro` and `src/components/PersonList.tsx` to build
your UI.

``` codeBlockStandalone_LlrK
my-astro-app/
├── spacetimedb/              # Your SpacetimeDB module
│   └── src/
│       └── index.ts          # SpacetimeDB module logic
├── src/
│   ├── components/
│   │   ├── PersonList.tsx
│   │   ├── SpacetimeApp.tsx
│   │   └── DeferredPeopleSnapshot.astro
│   ├── lib/
│   │   └── spacetimedb-server.ts
│   ├── module_bindings/      # Auto-generated types
│   ├── layouts/
│   │   └── Layout.astro
│   ├── pages/
│   │   └── index.astro
│   └── styles/
│       └── global.css
└── package.json
```

**Understand tables and reducers**

Open `spacetimedb/src/index.ts` to see the module code. The template
includes a `person` table and two reducers: `add` to insert a person,
and `sayHello` to greet everyone.

Tables store your data. Reducers are functions that modify data and are
the only way to write to the database.

``` codeBlockStandalone_LlrK
import { schema, table, t } from 'spacetimedb/server';

const spacetimedb = schema({
  person: table(
    { public: true },
    {
      name: t.string(),
    }
  ),
});
export default spacetimedb;

export const add = spacetimedb.reducer(
  { name: t.string() },
  (ctx, { name }) => {
    ctx.db.person.insert({ name });
  }
);

export const sayHello = spacetimedb.reducer(ctx => {
  for (const person of ctx.db.person.iter()) {
    console.info(`Hello, ${person.name}!`);
  }
  console.info('Hello, World!');
});
```

**Test with the CLI**

Open a new terminal and navigate to your project directory. Then use the
SpacetimeDB CLI to call reducers and query your data directly.

``` codeBlockStandalone_LlrK
cd my-spacetime-app

# Call the add reducer to insert a person
spacetime call add Alice

# Query the person table
spacetime sql "SELECT * FROM person"
 name
---------
 "Alice"

# Call sayHello to greet everyone
spacetime call say_hello

# View the module logs
spacetime logs
2025-01-13T12:00:00.000000Z  INFO: Hello, Alice!
2025-01-13T12:00:00.000000Z  INFO: Hello, World!
```

**Understand Astro SSR and real-time hydration**

The SpacetimeDB SDK works both server-side and client-side. The template
uses a hybrid approach:

- **Astro page** (`src/pages/index.astro`): Fetches initial data on the
  server for a fast first render
- **React island** (`src/components/SpacetimeApp.tsx`): Hydrates on the
  client and provides the SpacetimeDB connection
- **Live table UI** (`src/components/PersonList.tsx`): Uses `useTable()`
  and `useReducer()` for real-time updates

``` prism-code
---
import Layout from '../layouts/Layout.astro';
import SpacetimeApp from '../components/SpacetimeApp';
import { fetchPeople } from '../lib/spacetimedb-server';

const initialPeople = await fetchPeople();
---

<Layout title="astro-ts">
  <h1>astro-ts</h1>
  <SpacetimeApp client:load initialPeople={initialPeople} />
</Layout>
```

**Understand Astro server islands**

The template also includes a deferred Astro-only section to demonstrate
`server:defer`.

`src/components/DeferredPeopleSnapshot.astro` fetches its own
server-rendered snapshot, and `src/pages/index.astro` renders it as a
server island without changing the main real-time client flow.

``` prism-code
<DeferredPeopleSnapshot server:defer>
  <div slot="fallback">Loading deferred people snapshot…</div>
</DeferredPeopleSnapshot>
```

