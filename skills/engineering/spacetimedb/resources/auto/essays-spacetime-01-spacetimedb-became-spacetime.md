# SpacetimeDB became Spacetime because the ambition is the whole backend

I love SpacetimeDB, and I think the industry is still dramatically underestimating what it represents.

We have become very good at assembling backends out of pieces. We have databases, application servers, authentication services, caches, queues, and synchronization layers. We have entire categories of tooling devoted to making those pieces agree. Spacetime makes me question how much of that assembly should have been my job in the first place.

Yes, it stores rows. It also runs my application logic, gives that logic transactional access to those rows, and keeps subscribed clients synchronized when they change. I can publish a module and connect a frontend directly to it. The application server I would normally put between those two things has become part of the platform.

The scope of that idea is enormous. It reaches into how we build SaaS products, internal tools, games, collaboration software, and applications where agents work alongside people. All of them need a place for state, rules, identity, and communication to come together.

That's why the move from **SpacetimeDB to Spacetime** feels inevitable to me. Spacetime is the platform. SpacetimeDB is the database engine at its center. The shorter name gives the whole system room to be what it already wants to be: a place where applications live. I think that is big enough to become a foundational category of software.

The public website already uses Spacetime, while the CLI, SDK package, repository, and technical documentation retain SpacetimeDB names. That is a perfectly useful distinction. A brand can grow without forcing me to rename every import in my application. [Spacetime](https://spacetimedb.com/), [SDK source](https://github.com/clockworklabs/SpacetimeDB/tree/master/crates/bindings-typescript).

## A family of products around one application

There are already several concrete pieces to this story.

**SpacetimeDB** is where my schema, durable state, and transactional application code come together. A module describes both what the application remembers and what it can do. That is a much more useful unit of deployment than a database connection string.

**SpacetimeAuth** handles sign-in. It is an OIDC provider, so its tokens fit into a familiar authentication model, and its projects can serve multiple databases. I love having identity beside the application runtime. Users sign in, their identity reaches my application, and I write the authorization rules that make sense for my data. [SpacetimeAuth documentation](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/).

**Maincloud** operates the backend for me. **Standalone** lets me run the engine myself. Those are different operational choices around the same core programming model. I can work locally without treating development as a miniature cloud migration, then choose how I want to operate the application. [Products and pricing](https://spacetimedb.com/pricing).

Put those pieces together and I can start thinking in terms of an application again. I have somewhere to put its data, execute its rules, identify its users, and run the result. That is the level of abstraction I want from a backend platform.

## The product family I am excited about

Here is how I imagine the expanding Spacetime family, with the names I would give its pieces. I get excited just writing this list, because each piece brings more of the application into one coherent environment.

| Name in this vision  | What excites me                                                                                                                                          |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **SpacetimeSync**    | Subscriptions and client caches that make every interface a live view of the application.                                                                |
| **SpacetimeCompute** | Transactional reducers, procedures for external work, and scheduled execution, all organized around the application's state.                             |
| **SpacetimeStudio**  | A complete workspace for understanding a running application: data, schema, logs, permissions, and connected users, growing from the existing dashboard. |
| **SpacetimeFlow**    | Durable jobs, retries, approvals, and coordination becoming a natural part of the platform.                                                              |
| **SpacetimeAgents**  | Reusable tools for durable tasks, scoped actions, shared memory, and live human supervision.                                                             |

The requirement I would put above all five names is simple: they should feel like parts of one application. I want the product family to make a small engineering team feel unusually capable.

I am especially excited about workflows and agents. I want to start a job, watch its progress, pause for approval, and let an agent pick up the next task through the same application model. Putting those capabilities together will make the platform feel bigger while making my job feel smaller.

When someone signs in, I want that identity to flow into my business rules. When a job changes a record, I want the relevant screen to update. When an agent takes an action, I want the resulting state to be available to the user and the next worker through the same data model.

Naming those capabilities helps explain them. Integrating them is what makes them valuable. My bet is that a coherent platform can change the economics of building software: more of a team's effort reaches the product, and less goes into maintaining the machinery between its parts. That is a much bigger prize than winning a feature checklist.

## The feature I care about is continuity

Imagine building a small support tool. Customers sign in through SpacetimeAuth. Tickets live in SpacetimeDB. Reducers enforce who can assign or close them. Subscriptions keep the queue current. Maincloud runs the backend.

Later, I add an agent that drafts replies. The interesting work is deciding when a draft is useful and who can approve it. I can represent those decisions in the same application state. I do not have to introduce a second, competing account of what a ticket is doing just because an LLM has joined the workflow.

That continuity is what makes me excited about the rebrand. Most platform diagrams make me count integrations. This one makes me think about features I could build.

And I would carry the same model into an operations dashboard, a multiplayer editor, a booking system, or an agent workspace. Their business rules differ. The need for durable state, controlled changes, identity, and live clients keeps coming back. Spacetime has a plausible claim on the common foundation underneath all of them.

I want infrastructure to disappear from my daily attention in the same way a good compiler does. I still care deeply that it works. I just want to spend my afternoon on the application. Spacetime is one of the clearest expressions of that ambition I have seen.

SpacetimeDB earned my attention by making the database fast. Spacetime has me telling other engineers to rethink their default stack. I want everyone building a backend to spend an afternoon with it, because I think this is where a huge amount of software development is going.

