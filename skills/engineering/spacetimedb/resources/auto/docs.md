Version: 2.0.0

On this page

## Installation

You can get started by first installing the `spacetime` CLI tool. The
`spacetime` CLI tool makes it extremely easy to manage your databases
and deployments.

[](https://spacetimedb.com/install)

###### Install the SpacetimeDB CLI tool

## Log in to SpacetimeDB

SpacetimeDB authenticates users using a GitHub or Google login, to
prevent unauthorized access (e.g. somebody else publishing over your
module). Log in to SpacetimeDB using:

``` codeBlockStandalone_LlrK
spacetime login
```

This will open a browser and ask you to log in via GitHub or Google. If
you forget this step, any commands that require login (like
`spacetime publish`) will ask you to log in when you run them.

## Quickstart Guides

You are now ready to start developing SpacetimeDB modules. Choose your
favorite language and follow one of our quickstart guides to get started
building your first app with SpacetimeDB.

[](https://spacetimedb.com/docs/quickstarts/react)

###### React

Get a SpacetimeDB React app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/nextjs)

###### Next.js

Get a SpacetimeDB Next.js app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/vue)

###### Vue

Get a SpacetimeDB Vue app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/nuxt)

###### Nuxt

Get a SpacetimeDB Nuxt app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/svelte)

###### Svelte

Get a SpacetimeDB Svelte app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/angular)

###### Angular

Get a SpacetimeDB Angular app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/tanstack)

###### TanStack Start

Get a SpacetimeDB app with TanStack Start running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/remix)

###### Remix

Get a SpacetimeDB Remix app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/browser)

###### Browser

Get a SpacetimeDB app running in the browser with inline JavaScript.

[](https://spacetimedb.com/docs/quickstarts/bun)

###### Bun

Get a SpacetimeDB Bun app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/deno)

###### Deno

Get a SpacetimeDB Deno app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/nodejs)

###### Node.js

Get a SpacetimeDB Node.js app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/typescript)

###### TypeScript

Get a SpacetimeDB TypeScript app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/rust)

###### Rust

Get a SpacetimeDB Rust app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/c-sharp)

###### C#

Get a SpacetimeDB C# app running in under 5 minutes.

[](https://spacetimedb.com/docs/quickstarts/c-plus-plus)

###### C++

Get a SpacetimeDB C++ app running in under 5 minutes.

## Running SpacetimeDB Locally

To develop SpacetimeDB databases locally, you will need to run the
Standalone version of the server.

After installing the SpacetimeDB CLI, run the start command:

``` codeBlockStandalone_LlrK
spacetime start
```

The server listens on port `3000` by default, customized via
`--listen-addr`.

💡 Standalone mode will run in the foreground. ⚠️ SSL is not supported
in standalone mode.

## Next Steps: Learn SpacetimeDB

After completing a quickstart guide, explore these core concepts to
deepen your understanding:

### Core Concepts

- **[Databases](https://spacetimedb.com/docs/databases)** - Understand database lifecycle,
  publishing, and management
- **[Tables](https://spacetimedb.com/docs/tables)** - Define your data structure with tables,
  columns, and indexes
- **[Functions](https://spacetimedb.com/docs/functions)** - Write reducers, procedures, and
  views to implement your server logic
- **[Subscriptions](https://spacetimedb.com/docs/clients/subscriptions)** - Enable real-time
  data synchronization with clients
- **[Client SDKs](https://spacetimedb.com/docs/clients)** - Connect your client applications to
  SpacetimeDB
