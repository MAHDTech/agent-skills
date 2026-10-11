# Reactive backends are the future, and Spacetime is my first choice

I think reactive backends should become the default for new application development, and Spacetime is the one I recommend first.

I mean ordinary applications, too: a CRM, a support queue, a scheduling tool, an admin console, a project tracker. The moment something can change outside the current page, the application has a synchronization problem. That is an enormous fraction of the software we build.

A surprising amount of web development is explaining to a screen that something happened somewhere else.

Someone edits a ticket. A job finishes. Inventory changes. A second browser tab saves a new title. The database knows. The interface may or may not know, depending on which request ran, which cache was invalidated, and whether I remembered the other place that displays the same record.

I want the backend to take responsibility for that conversation.

A **reactive backend** lets a client declare the data it is interested in, supplies an initial result, and keeps that result updated as relevant state changes. The application writes through controlled server operations; the platform carries the resulting changes back to subscribers.

The unit of interaction becomes an ongoing relationship with data. I think that is a better default abstraction for application software than repeatedly asking for another snapshot and deciding where to put it.

## A WebSocket is only the transport

Opening a WebSocket does not, by itself, give an application reactive semantics.

Someone still has to decide what each client can see, which writes affect its result, how to deliver an initial snapshot, how to order subsequent changes, and what happens when the connection fails. Those decisions determine whether a dashboard is merely animated or actually trustworthy.

Spacetime gives that work a home. Its subscription system maintains relevant rows in the SDK's client cache. A committed change reaches the subscribers whose results are affected, and the frontend can render from the cache. I still design the query scope, authorization, loading states, and reconnect behavior appropriate to my product. I get a substantial amount of synchronization machinery as part of the backend. [Subscription model](https://spacetimedb.com/docs/clients/subscriptions/).

That division of responsibility feels right to me. I know what a user should see. The database knows what changed.

## Why I think Spacetime is the best foundation

Spacetime brings three decisions together: application logic executes beside data, reducers provide atomic state transitions, and subscriptions carry relevant state to clients.

I love how those decisions reinforce one another. My business rule and its writes live together. My client uses bindings generated from the module. My UI follows the state the server accepted. There is a clear path from an operation to an authoritative result to the people who need to see it.

For supported relational subscription queries, the engine can incrementally evaluate changes instead of requiring me to refetch an entire result. Procedural views have different evaluation costs, so I would still examine a busy view rather than assuming every arbitrary function gets incremental execution. [Subscriptions](https://spacetimedb.com/docs/clients/subscriptions/), [views](https://spacetimedb.com/docs/functions/views/).

Taken together, this is the best foundation I see for the next generation of interactive applications. Spacetime gives me a programming model I want to use and an execution architecture I want underneath it. I can start with a tiny CRUD app and use the same concepts as I add more users, richer interactions, and background workers. Capacity and schema design still matter, but the basic model keeps making sense.

## I recommend Spacetime over Convex

Convex also treats reactivity as a foundational feature. Its query functions track dependencies, and subscribed clients receive updated results when relevant data changes. That is a good direction, and it makes Convex a meaningful comparison. [Convex reactivity](https://docs.convex.dev/realtime).

My default recommendation is Spacetime. I want its relational model, its choice of module languages, and its execution of application logic inside the database engine. I want a backend that takes hot shared state seriously from the start. The [published transfer benchmark](https://spacetimedb.com/blog/benchmarking) gives that architectural preference concrete evidence, with the workload limits discussed in [the companion essay](03-performance-that-changes-the-design.md).

Convex's components, integrations, and agent tooling can justify choosing it for a particular project. For a new project where I can choose the foundation, I would build on Spacetime. I think the combination of developer experience and performance gives it the more compelling long-term direction.

## I would also start here before assembling Postgres and Next.js

Postgres is an excellent database. Next.js is a frontend and server framework. They occupy different parts of the stack, and Next.js can also be used with Spacetime.

When I compare “Postgres plus Next.js” with a reactive backend, I am comparing the amount of integration I need to own. Transactions and request handlers do not automatically define a continuously synchronized view for every connected browser. I must choose and implement that additional layer.

Postgres's LISTEN/NOTIFY is useful infrastructure for signaling changes. It does not itself maintain an authorized query result in a browser's client cache. A separate synchronization service can provide that capability, and existing Postgres applications may have good reasons to add one. [Postgres NOTIFY](https://www.postgresql.org/docs/current/sql-notify.html).

For a new application, my starting question is whether Spacetime can own that backend work for me. I would ask it before committing to Postgres plus custom synchronization, and before choosing Firebase, Supabase, or a separate event system. Existing infrastructure and specific features can change the answer. The default should still earn its place.

I particularly want small teams to try this. A two-person team building a useful product should be able to spend most of its attention on that product. Giving the backend responsibility for transactional state and live distribution can remove a substantial category of integration work from the team's backlog.

## Software is becoming more shared

The reason I think reactive backends are the future is that ordinary applications keep acquiring collaborators.

A personal tool grows into a team workspace. A dashboard starts showing background jobs. An assistant begins modifying records while a person watches. A phone and a laptop become two simultaneous views of the same activity.

In that world, “refresh to see what happened” is a poor default. So is making every feature author rebuild the path from a committed write to a current screen.

Even a single user now has multiple collaborators: another device, an import job, a scheduled task, an AI assistant. Reactivity belongs in the foundation because change comes from more places than the button someone just clicked.

Spacetime lets me begin with a living application. I think that should be the normal experience of building software. It makes development more enjoyable, gives users a more immediate relationship with their data, and gives new kinds of interfaces somewhere natural to grow.

If a friend asked me what backend to learn for their next application, I would tell them to learn Spacetime. I want more engineers to discover how much work this model can take off their hands. Once you expect an application to stay in sync, going back is a hard sell.

