+++
title = "essays-spacetime-13-install-the-boring-parts-of-your-backend"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# I want to install the boring parts of my backend

Every application seems to rediscover a few of the same problems.

Someone needs an invitation. An action needs an audit record. A resource needs a reservation. Eventually, someone asks whether the code from the last project can be reused, and the answer involves copying some tables, renaming a few functions, and hoping all the assumptions made the trip.

I want backend reuse to include the state a feature owns and the operations that make that state useful.

That is why Spacetime's TypeScript submodules get my attention.

A submodule packages database tables and functions so another module can include them under a chosen namespace. The consumer can give the library a home without negotiating every table name with every other dependency. The same module can be used on its own or included in a larger application. [Submodules](https://spacetimedb.com/docs/submodules/).

This is a very promising unit of reuse: a working piece of the backend.

## Reuse the feature's structure

Consider an audit-history library I would like to build.

It would define the records it keeps, helpers for appending entries, and views for reading the history. The application would supply the meaning of an action and enforce who is allowed to perform it. The library would provide a consistent representation of what happened.

In a workshop booking app, I could include that library under an `audit` namespace. A cancellation reducer could update the reservation and call the library's helper to record the action in the same reducer transaction. Calling a helper directly with the current transaction keeps those database writes together; failure rolls them back together. [Transaction behavior](https://spacetimedb.com/docs/databases/transactions-atomicity/).

That beats a collection of copied snippets. The table definition, the append operation, and the read interface can evolve as a library with a clear contract.

## The client can see the composition too

The current submodule support is for TypeScript. A consumer imports the module's full namespace and registers it in its schema under an alias. Exported functions and tables are registered through that composition. Generated client bindings expose nested table, reducer, and procedure interfaces under the alias. [Registration and client access](https://spacetimedb.com/docs/submodules/).

That means the boundary is visible throughout the application. A developer can follow an audit feature from the server-side namespace into the generated client interface. The UI can subscribe to the exposed audit data using the same general pattern it uses for the application's own data.

A namespace gives names somewhere to live; access rules still belong in the library and application. I would design an audit package's visibility deliberately, since histories often contain information that should remain private.

There are also useful ownership rules to understand when composing modules. Lifecycle reducers belong to the root module, and the root's HTTP router explicitly exposes any submodule handlers it wants to serve. Those responsibilities give the application a clear place to control startup behavior and its HTTP surface. [Submodule integration rules](https://spacetimedb.com/docs/submodules/).

## Let useful work travel

I would love to see small, well-explained libraries for invitations, reservations, approvals, and other recurring application patterns. Each could carry its tables, operations, documentation, and tests. A team could improve one implementation and use it across its projects.

The application would still decide its policies. An invitation expires when the product says it expires. A reservation is valid under the product's rules. Reuse becomes valuable when those choices are explicit and the machinery around them is dependable.

This is one of the most exciting consequences of putting application logic and data in the same programming model. More of a feature can travel together.

Spacetime gives that idea a practical shape. I want the next project to start with useful work already done, and I want the work I do on that project to make the one after it easier.


