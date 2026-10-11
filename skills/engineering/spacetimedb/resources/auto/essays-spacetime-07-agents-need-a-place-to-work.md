# Spacetime is the foundation I want for the agent era

I think Spacetime could become a foundational platform for agent software, and I want more people building agents to see why.

An agent that participates in a real product needs a place to work. It needs durable tasks, access rules, a record of what happened, and a way for people to see what it is doing. Add multiple agents and those needs become more urgent. This is the application infrastructure underneath the model, and Spacetime's design fits it remarkably well.

Once an agent decides what to do, I start worrying about what comes next. Has another worker already claimed the job? Did the tool call finish before the connection dropped? Is the user still allowed to approve it? Which version of the document did the agent inspect? Can the dashboard tell me what is happening without polling five services?

Those are application-state problems. They are exactly the problems that make me love Spacetime as a foundation for agents.

There are two enormous opportunities here: agents that build software on Spacetime, and software that runs agents with Spacetime as its shared state. I think the combination could change both how applications are made and what they can do.

## A smaller backend is easier for a coding agent to understand

When a coding agent adds a feature to a conventional stack, it may need to coordinate a schema, a migration, a server route, request validation, client types, cache invalidation, and a realtime event handler. Each boundary is another place where individually plausible code can disagree with the rest of the application.

Spacetime gives the agent a more compact loop. Define the data and server operations in a module. Generate client bindings. Connect the UI to the relevant tables or views. Check permissions inside the server operations.

The structure also gives me concrete things to inspect: the schema, the state transitions, the read boundary, and the generated interface. Authorization and business invariants still need review. I can focus that review on a smaller, more coherent application model.

The [task-list example in this series](05-building-a-crud-app.md) is a useful case. Its module and React client were compiled, published locally, and exercised through the generated SDK. Ownership checks and private views were tested. That is the loop I want an agent to close before it tells me a feature is done.

My bet is that architecture will matter even more as agents write more code. Generating a large amount of glue is easy; keeping all of its assumptions aligned is the difficult part. A platform that owns more of those relationships gives the agent a smaller problem and gives me a better chance of understanding the result. That is my engineering judgment rather than a measured model-success claim.

## Running agents requires shared, durable state

Imagine a research assistant with several workers and a person reviewing their output.

I would start with tables for runs, tasks, attempts, messages, artifacts, and approvals. A run records the user's request. A task records a unit of work. An attempt records which worker tried it and how it ended. An approval records exactly what the person authorized.

I would implement these application tables directly on Spacetime. Durable work belongs in tables, and the rules for moving it forward belong in reducers. That is a foundation I am excited to build on.

| Operation I would implement | Invariant the reducer would enforce                                                |
| --------------------------- | ---------------------------------------------------------------------------------- |
| Claim a task                | The task is available, and the claiming identity is an authorized worker.          |
| Record progress             | The attempt belongs to the worker, and its lease or version is still valid.        |
| Request approval            | The proposed action and its inputs are persisted together.                         |
| Approve an action           | The caller can approve this run, and the proposal has not changed.                 |
| Complete a task             | The result belongs to the current attempt, and completion is not already recorded. |

A reducer can check an invariant and change the relevant rows in one transaction. Competing workers cannot both claim the same available task if the claim operation checks and updates that status atomically. Expiration, retries, and fencing rules still need to be designed into the application.

I love how ordinary this becomes. A lot of agent orchestration is a state machine. I want to implement it where state transitions are first-class. Whether a task belongs to a person, a deterministic worker, or an LLM agent, the application still has a precise account of who can act and what the next valid state is.

## The human interface falls out of the same model

The dashboard subscribes to the run it is displaying. As authorized workers commit progress, the person's view updates. An approval request appears because an approval row exists. A completed task appears because the completion transaction succeeded.

The UI and workers can share a durable account of the work, even though their views and permissions differ. I can inspect that account with database tools instead of reconstructing the entire workflow from log fragments. Human oversight becomes a natural feature of the application: the person watches the same work the system is recording.

For streamed model output, I would store bounded chunks or periodic updates rather than blindly turning every token into a separate transaction. Subscription scope, retention, and fan-out still affect cost. The platform gives me the mechanism; I choose a sensible representation.

## Keep uncertain external work outside the transaction

The model call itself is external I/O. It belongs in a worker or an appropriate Spacetime procedure, not inside a reducer. Procedures can call external services and open explicit transactions around database work. A procedure's entire execution is not automatically one transaction. [Procedure semantics](https://spacetimedb.com/docs/functions/procedures/).

I would commit the intent to perform an action, invoke the external service, then record its outcome. I would use stable idempotency keys where the service supports them and design reconciliation for ambiguous failures.

An atomic database transaction cannot make sending an email or charging a card exactly once by itself. Persistent attempts, idempotency, and explicit approvals are application design work. Spacetime gives me a strong place to keep that work coherent.

## I am excited about what comes next

Upcoming inter-database communication will give agent systems a direct way to coordinate across independently executing databases. I picture each workspace owning its tasks and permissions while specialized workers exchange typed messages across the system. Tiered storage will add room for growing task histories and application records. Distributed transactions will support coordinated state changes when work crosses database boundaries. [Spacetime's scaling roadmap](https://spacetimedb.com/blog/how-does-spacetime-scale).

This is where the product vision I call [SpacetimeAgents](01-spacetimedb-became-spacetime.md) becomes especially exciting to me: package the recurring patterns for tasks, attempts, approvals, and live supervision so that each new agent application starts further ahead. I want to spend my time designing what the agents do and how people work with them. I love where this foundation is heading.

## One foundation for people and agents

Convex already offers an agent component and associated tooling. If those abstractions match a project, they are a real advantage. [Convex agent documentation](https://docs.convex.dev/agents/overview).

My default choice would be Spacetime. I want to own the workflow model, support busy shared state, and give people a live view of agents doing useful work. I can keep the LLM provider and orchestration library replaceable while making the application's state durable and inspectable.

The opportunity extends far beyond chat. I can imagine the same foundation supporting research teams, support operations, coding workspaces, simulation environments, and internal business processes. In each case, agents need to change shared state under rules, and people need to understand and influence the outcome.

This is also why the reactive model matters so much. When agents work in the background, the interface should keep up with them. Progress, proposed actions, approvals, and completed results belong in the product experience. Spacetime gives me a direct way to connect those experiences to committed application state.

That is the future I want to build: agents operating inside an application that knows who they are, what they are allowed to change, what has happened, and who needs to see it. I think a common runtime for that work could be as consequential as the models themselves for the quality of the software people actually use.

Spacetime makes me want to build more ambitious agent products because the state model is something I can reason about. I want everyone working on agents to try it. The model supplies the reasoning; Spacetime gives that reasoning somewhere dependable to land.

