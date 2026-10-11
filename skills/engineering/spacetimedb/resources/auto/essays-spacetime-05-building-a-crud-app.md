# I built a CRUD app with Spacetime, and I want to build everything this way

I built a small task app to put Spacetime's promise under a microscope: create a task, read the list, rename it, mark it done, and delete it. I also wanted two connections using the same identity to stay synchronized, and a different user to see none of those tasks.

That is a deliberately ordinary app. Ordinary apps are where backend complexity has the least excuse to get in the way.

I loved how closely the implementation followed the problem. A task is a row. A write is a reducer. A user's task list is a view. React renders the subscribed result. I could spend my attention on ownership and behavior instead of designing the transport between every layer.

This is the part of Spacetime I want every web developer to experience. The backend code looks like the feature. The live behavior comes from the same state I am already maintaining. I can read the implementation and understand why the application works.

I built and tested this local prototype for the article. The exact example targets **SpacetimeDB CLI/server/SDK 2.9.0**, with TypeScript 5.9.2 and React 19.1.1. The product is Spacetime; the package and CLI retain their existing names.

## The whole data model fits in the feature

Here is the complete module, saved as `spacetimedb/src/index.ts`.

The base table is private. The view chooses rows using the server-provided caller identity. Every mutation checks ownership on the server. A client-side filter would not provide that protection.

<!-- file: spacetimedb/src/index.ts -->

```ts
import { schema, table, t, SenderError } from 'spacetimedb/server';

const task = table(
    { public: false },
    {
        id: t.u64().primaryKey().autoInc(),
        owner: t.identity().index(),
        title: t.string(),
        done: t.bool(),
    }
);
const spacetimedb = schema({ task });
export default spacetimedb;

function cleanTitle(title: string): string {
    const trimmed = title.trim();
    if (trimmed.length === 0 || trimmed.length > 200) {
        throw new SenderError('Title must contain 1 to 200 characters.');
    }
    return trimmed;
}

export const myTasks = spacetimedb.view({ public: true }, t.array(task.rowType), ctx => [
    ...ctx.db.task.owner.filter(ctx.sender),
]);

export const createTask = spacetimedb.reducer({ title: t.string() }, (ctx, { title }) => {
    ctx.db.task.insert({
        id: 0n,
        owner: ctx.sender,
        title: cleanTitle(title),
        done: false,
    });
});

export const renameTask = spacetimedb.reducer(
    { id: t.u64(), title: t.string() },
    (ctx, { id, title }) => {
        const row = ctx.db.task.id.find(id);
        if (!row || !row.owner.isEqual(ctx.sender)) {
            throw new SenderError('Task not found.');
        }
        ctx.db.task.id.update({ ...row, title: cleanTitle(title) });
    }
);

export const setTaskDone = spacetimedb.reducer(
    { id: t.u64(), done: t.bool() },
    (ctx, { id, done }) => {
        const row = ctx.db.task.id.find(id);
        if (!row || !row.owner.isEqual(ctx.sender)) {
            throw new SenderError('Task not found.');
        }
        ctx.db.task.id.update({ ...row, done });
    }
);

export const deleteTask = spacetimedb.reducer({ id: t.u64() }, (ctx, { id }) => {
    const row = ctx.db.task.id.find(id);
    if (!row || !row.owner.isEqual(ctx.sender)) {
        throw new SenderError('Task not found.');
    }
    ctx.db.task.id.delete(id);
});
```

The auto-incrementing ID starts with the `0n` sentinel on insert. IDs are `bigint` in TypeScript, which is why the frontend uses `toString()` for React keys. The owner comes from `ctx.sender`; there is no caller-supplied owner field to forge.

The view is public in the sense that clients can invoke it. Its contents are still scoped by the server to the caller. This example returns an array from a procedural view; it does not claim the same execution cost as an incrementally maintained query expression. For a very large task list, I would revisit the subscription scope and view implementation.

I deliberately made completion a “set this value” operation. Retrying a request to set `done` to true expresses a clearer intention than blindly toggling the current value. The create operation does not have a request-deduplication key, so it is not an exactly-once API across arbitrary client retries.

## The UI follows the accepted state

After generating the bindings, this is the complete `src/App.tsx`. The hooks receive the actual generated table and reducer definitions. The component waits for connection and subscription readiness, displays reducer errors, and disables controls while its current request is pending.

<!-- file: src/App.tsx -->

```tsx
import { useState } from 'react';
import { useReducer, useSpacetimeDB, useTable } from 'spacetimedb/react';
import { reducers, tables } from './module_bindings';

export default function App() {
    const { isActive, connectionError } = useSpacetimeDB();
    const [tasks, ready] = useTable(tables.myTasks);
    const createTask = useReducer(reducers.createTask);
    const renameTask = useReducer(reducers.renameTask);
    const setTaskDone = useReducer(reducers.setTaskDone);
    const deleteTask = useReducer(reducers.deleteTask);
    const [title, setTitle] = useState('');
    const [error, setError] = useState('');
    const [pending, setPending] = useState(false);
    const [editingId, setEditingId] = useState<bigint | null>(null);
    const [editTitle, setEditTitle] = useState('');

    async function run(action: () => Promise<void>) {
        setPending(true);
        setError('');
        try {
            await action();
        } catch (err) {
            setError(err instanceof Error ? err.message : String(err));
        } finally {
            setPending(false);
        }
    }

    if (connectionError) return <p role="alert">{connectionError.message}</p>;
    if (!isActive || !ready) return <p>Connecting and loading tasks…</p>;

    return (
        <main>
            <h1>My live task list</h1>
            {error && <p role="alert">{error}</p>}
            <form
                onSubmit={event => {
                    event.preventDefault();
                    void run(async () => {
                        await createTask({ title });
                        setTitle('');
                    });
                }}
            >
                <fieldset disabled={pending}>
                    <label>
                        New task{' '}
                        <input
                            value={title}
                            maxLength={200}
                            onChange={event => setTitle(event.target.value)}
                        />
                    </label>
                    <button disabled={!title.trim()}>Add</button>
                </fieldset>
            </form>
            <ul>
                {tasks.map(task => (
                    <li key={task.id.toString()}>
                        <label>
                            <input
                                type="checkbox"
                                checked={task.done}
                                disabled={pending}
                                onChange={event => {
                                    const done = event.target.checked;
                                    void run(() => setTaskDone({ id: task.id, done }));
                                }}
                            />
                            {task.title}
                        </label>{' '}
                        <button
                            disabled={pending}
                            onClick={() => {
                                setEditingId(task.id);
                                setEditTitle(task.title);
                            }}
                        >
                            Rename
                        </button>{' '}
                        <button
                            disabled={pending}
                            onClick={() => void run(() => deleteTask({ id: task.id }))}
                        >
                            Delete
                        </button>
                        {editingId === task.id && (
                            <form
                                onSubmit={event => {
                                    event.preventDefault();
                                    void run(async () => {
                                        await renameTask({ id: task.id, title: editTitle });
                                        setEditingId(null);
                                    });
                                }}
                            >
                                <fieldset disabled={pending}>
                                    <label>
                                        Task title{' '}
                                        <input
                                            value={editTitle}
                                            maxLength={200}
                                            onChange={event => setEditTitle(event.target.value)}
                                        />
                                    </label>
                                    <button disabled={!editTitle.trim()}>Save</button>
                                    <button type="button" onClick={() => setEditingId(null)}>
                                        Cancel
                                    </button>
                                </fieldset>
                            </form>
                        )}
                    </li>
                ))}
            </ul>
        </main>
    );
}
```

There is no fetch-after-save call here. The committed state arrives through the subscription. There is also no optimistic update pretending the server has already accepted a write. That keeps this example small and makes its behavior easy to inspect.

Renaming uses an inline editor with save and cancel controls. This is a small functional UI, ready for a product-specific design.

The connection belongs above the component. Save this as `src/main.tsx`:

<!-- file: src/main.tsx -->

```tsx
import { createRoot } from 'react-dom/client';
import { SpacetimeDBProvider } from 'spacetimedb/react';
import { DbConnection } from './module_bindings';
import App from './App';

const uri = 'ws://127.0.0.1:3918';
const database = 'essay-tasks';
const tokenKey = `${uri}/${database}/token`;
const connectionBuilder = DbConnection.builder()
    .withUri(uri)
    .withDatabaseName(database)
    .withToken(localStorage.getItem(tokenKey) ?? undefined)
    .onConnect((_connection, _identity, token) => {
        localStorage.setItem(tokenKey, token);
    });

createRoot(document.getElementById('root')!).render(
    <SpacetimeDBProvider connectionBuilder={connectionBuilder}>
        <App />
    </SpacetimeDBProvider>
);
```

This demo stores the server-issued anonymous identity token in local storage. Open the second tab **after the first tab connects**, using the same origin, to reuse that identity. An incognito window gets another identity and an empty task list. Clearing the token loses access to that anonymous user's tasks; this is not a recoverable account system. For a real sign-in flow I would use SpacetimeAuth or another supported OIDC provider. [Authentication documentation](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/).

## Run the exact example

Use Node.js 22.12 or newer and SpacetimeDB 2.9.0 for this pinned example. Create an empty project directory, then save the three source files above and these supporting files at the indicated paths.

`package.json`:

<!-- file: package.json -->

```json
{
    "name": "spacetime-essay-crud",
    "private": true,
    "type": "module",
    "scripts": {
        "build": "tsc --noEmit && vite build",
        "dev": "vite --host 127.0.0.1 --port 5179"
    },
    "dependencies": {
        "spacetimedb": "2.9.0",
        "react": "19.1.1",
        "react-dom": "19.1.1"
    },
    "devDependencies": {
        "typescript": "5.9.2",
        "vite": "7.1.5",
        "@types/react": "19.1.13",
        "@types/react-dom": "19.1.9",
        "tsx": "4.20.5"
    }
}
```

`tsconfig.json`:

<!-- file: tsconfig.json -->

```json
{
    "compilerOptions": {
        "target": "ES2022",
        "module": "ESNext",
        "moduleResolution": "Bundler",
        "lib": ["ES2022", "DOM"],
        "jsx": "react-jsx",
        "strict": true,
        "skipLibCheck": true,
        "noEmit": true,
        "allowImportingTsExtensions": true
    },
    "include": ["src", "spacetimedb/src"]
}
```

`spacetimedb/package.json`:

<!-- file: spacetimedb/package.json -->

```json
{
    "name": "task-module",
    "private": true,
    "type": "module",
    "dependencies": {
        "spacetimedb": "2.9.0"
    },
    "devDependencies": {
        "typescript": "5.9.2"
    }
}
```

`spacetimedb/tsconfig.json`:

<!-- file: spacetimedb/tsconfig.json -->

```json
{
    "compilerOptions": {
        "target": "ES2022",
        "module": "ESNext",
        "moduleResolution": "Bundler",
        "lib": ["ES2022", "DOM"],
        "jsx": "react-jsx",
        "strict": true,
        "skipLibCheck": true,
        "noEmit": true,
        "allowImportingTsExtensions": true
    },
    "include": ["src"]
}
```

`index.html`:

<!-- file: index.html -->

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>Spacetime tasks</title>
    </head>
    <body>
        <div id="root"></div>
        <script type="module" src="/src/main.tsx"></script>
    </body>
</html>
```

Install the dependencies from the project root:

```bash
npm install
npm install --prefix spacetimedb
```

In one terminal, start the local server. Keep it running:

```bash
spacetime start --listen-addr 127.0.0.1:3918 --data-dir ./server-data --non-interactive
```

In another terminal at the project root, publish to that local server and generate the bindings:

```bash
spacetime publish --server http://127.0.0.1:3918 --module-path spacetimedb --anonymous --yes essay-tasks
spacetime generate --lang typescript --module-path spacetimedb --out-dir src/module_bindings --yes
npm run build
npm run dev
```

Open [the local app](http://127.0.0.1:5179). The generated `src/module_bindings` directory supplies the imports used by both client files; it should be regenerated after changing the module's interface. The publish command above is for a fresh local database. Anonymous publishing does not retain an owner token for later module updates; use an authenticated CLI workflow for ongoing development. For a managed starter workflow, see the [React quickstart](https://spacetimedb.com/docs/quickstarts/react/).

## What I actually checked

I type-checked the module and client, built the production frontend, published the module to a local server, and generated bindings from that module. Through real SDK connections I checked create, read, rename, completion, and deletion; rejected blank and oversized titles; rejected unauthorized writes; confirmed a different identity could not read the private base table; and verified live changes on a second connection and state on reconnect.

I also opened the actual React app in two browser tabs and checked creation, completion, an inline rename reaching both tabs, and persistence after reload. Finally, I extracted the source files from this Markdown and rebuilt them, so the copyable example is the code that was checked.

Those checks establish the behavior of this small example. They are not a production load test, a full accessibility review, or evidence of automatic offline editing support.

## The small app makes the bigger argument

The reason I loved building it is how little work existed between the business rule and the screen. I still had to decide whose tasks were visible and which changes were allowed. Those are the right decisions to spend time on.

The task list is tiny, but the pattern reaches much further. Tickets, bookings, inventory, project records, approval requests, and agent jobs all need controlled state transitions and a current view of the result. Each domain adds its own rules. I want to add those rules directly, while keeping the machinery for communicating accepted changes in the platform.

That is why I think Spacetime belongs in the conversation about everyday web development. You should not need a multiplayer game or a spectacular benchmark workload to benefit from a backend that handles synchronization. A form and a list already demonstrate the value.

Consider the next feature. I might add a due date, an assignment, a background worker, or a second interface. Each one raises real product questions. Spacetime gives me a common place to express the state and operations behind them. The more directions the application grows, the more I value that consistency.

My broader bet is that this changes what a small team can reasonably attempt. Features become easier to understand when less of their implementation consists of keeping different layers in agreement. That matters when I am writing the code myself, reviewing a teammate's work, or asking an agent to make a change.

To me, this is the future of web development: application features built directly on transactional, reactive state. I want the framework to own more of the machinery, so the code I write looks more like the thing the user asked for.

I would start my next CRUD app on Spacetime. I would recommend it for the internal tool someone keeps putting off, the SaaS idea a small team wants to ship, and the live dashboard attached to a growing product. This little task list is enough to make the principle tangible. Build it, open the second tab, and watch your data arrive. That is the moment I want more developers to have.

