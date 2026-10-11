+++
title = "essays-spacetime-02-a-distributed-operating-system"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# Spacetime is the distributed operating system I want to program

I think Spacetime is pursuing one of the most exciting ideas in systems software: make the infrastructure underneath an application behave like a computer I can actually program.

That is why I call it a distributed operating system for applications. The category matters. If we keep evaluating it only as a database, we miss the opportunity to move a much larger part of backend engineering into a common runtime.

I mean that literally at the application layer. It loads programs, hosts their state, mediates access to that state, handles client communication, and, in its cloud deployment, places application databases across machines. Those are operating-system responsibilities, raised to the level where I spend my time as a backend engineer.

The host operating system still handles devices and processes. Spacetime supplies the higher-level application environment: durable state, transactional execution, identity-aware operations, and communication with live clients. That is the layer I want to spend my working day programming.

Once I started looking at it that way, the family resemblance became obvious: Plan 9's ambition, Erlang's state ownership, Linux's role as a shared runtime, and Postgres's respect for structured data. I love this combination. It brings some of the best ideas in computing to the work of building an ordinary application.

## Plan 9: make the network part of the computer

Plan 9 took the idea of a distributed computing environment seriously. Its namespaces and file protocol made resources on different machines available through a common model. The delightful part is the ambition: programmers should see useful resources, not spend every interaction negotiating which box owns them. [Plan 9's namespace paper](https://9p.io/sys/doc/names.html).

Spacetime gives me a related feeling. My client connects to an application database and works with its interface. In Maincloud, routing and placement are platform concerns. I think about a named application with tables and callable functions.

Spacetime's common language is application state and operations on that state, rather than Plan 9's files and 9P protocol. That choice feels especially natural for software whose users are spread across browsers, phones, and game clients. A live application becomes the thing I connect to; the machines that host it become an implementation concern.

## Erlang: give state an owner

Erlang made independent processes and message passing practical tools for building concurrent systems. A process owns its state; other processes communicate with it. That boundary makes a large system easier to reason about. [Erlang's concurrency documentation](https://www.erlang.org/doc/system/conc_prog.html).

A Spacetime database is a larger-grained version of that idea in my mental model. It owns tables and the code that changes them. Reducer calls ask it to perform state transitions. Each successful reducer commits atomically.

The durable, relational state is the twist I love. I can inspect it, query it, and subscribe to it. I do not have to turn an opaque in-memory object into a persistence system before the application can survive a restart. The same state that drives execution can also drive a dashboard, a game client, or an agent's view of its work.

The connection to Erlang is the ownership model; Spacetime has its own runtime and lifecycle mechanisms. Upcoming inter-database messaging takes that idea further, and I am excited to build systems out of independently executing databases that communicate directly.

## Linux: provide the environment programs need

A useful operating system supplies more than a place to execute instructions. It gives programs conventions for resources, permissions, communication, and lifecycle.

Spacetime's module boundary does something similar for application code. I publish a module containing schema and functions. The host supplies the database interfaces and execution environment. Clients use generated bindings rather than negotiating a bespoke protocol for every project. The platform provides the common machinery that makes those programs useful. [Spacetime's architecture](https://spacetimedb.com/docs/intro/key-architecture/).

This changes what “deploy the backend” means to me. I am installing an application into a runtime that understands its data. I want that to become as ordinary as running a program on a laptop. It is an ambitious standard, and I think it is the right one.

## Postgres: make state structured and changes transactional

The Postgres part keeps the whole idea grounded.

Applications have facts. Facts belong in a model I can inspect. Related changes should succeed together or fail together. Spacetime brings tables, indexes, SQL queries, and transactional reducers into the application runtime.

Postgres already supports executing logic beside data through database functions. Spacetime makes that arrangement central to the developer experience and connects it to live clients. SQL support does not mean complete PostgreSQL compatibility; extensions and arbitrary existing queries still need evaluation before a migration.

The resulting model is unusually coherent: a running program owns relational state, exposes controlled operations, and publishes relevant changes to interested clients. Persistence and communication become part of the environment the program inhabits. That could reshape how we teach backend development, how teams structure applications, and how coding agents generate them.

## The next chapter is the part I cannot stop thinking about

Today, Spacetime distributes independent databases across a cluster while keeping each database's execution local to its leader. Each database owns its state, and independent databases can work in parallel. That gives the platform a fast, understandable building block for larger applications.

The roadmap brings **inter-database communication, tiered storage, distributed transactions, and partitions** into that model. I am excited about every one of them because they expand what I can build while preserving the local execution model I already love. [Spacetime's scaling roadmap](https://spacetimedb.com/blog/how-does-spacetime-scale).

**Asynchronous inter-database communication** will let databases call into one another through a typed interface. I think about regions in a game, teams in a collaboration product, and workers coordinating across an agent system. Each owns its state and communicates with the others. That is a beautiful way to organize a distributed application.

**Tiered storage** will extend tables across memory, disk, and object storage. I want the hot working set close to execution and room for the rest of the application's data to grow. This makes me excited about keeping more of a product's state in the same programming model as it matures.

**Distributed transactions** will bring atomic operations across databases. Most work can stay local; operations that need a shared commit will have an explicit way to get one. I love the idea of choosing that coordination where the business operation calls for it.

**Partitions within a logical database** will let independently executing pieces share a deployment artifact and coordinated schema changes. That is exactly the kind of platform work I want: make the application easier to operate as its execution spreads across machines.

Together, these features will make the distributed-computer idea feel increasingly natural. Keep the local unit of execution fast and understandable, then make those units easier to compose into a larger system. Many applications, many workers, and many interfaces can share a coherent way of working with the state they own. I cannot wait to build on that.

I see room here for a common foundation across business software, games, simulations, and agent systems. Those domains have very different interfaces, but each needs computation with state and rules. An application operating system is a powerful place to unify that work.

I want this idea to win. I want programming a distributed application to feel like programming against a thoughtfully designed system. Spacetime makes that future concrete enough to build on, and ambitious enough that I want every systems engineer I know to look at it.


