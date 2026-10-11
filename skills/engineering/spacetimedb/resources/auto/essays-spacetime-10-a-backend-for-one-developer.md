# A backend I can keep in my head

When I picture building an application on my own, the first screen rarely worries me. I worry about coming back to it a month later.

Will I remember which copy of a type is authoritative? Why one route updates a cache and another publishes an event? Whether the mobile client follows the same rules as the web client?

A solo developer needs a system that remains understandable after the excitement of starting it has worn off. That is one of Spacetime's most appealing qualities to me. The main pieces are close enough that I can follow a feature from its data to its behavior to its display.

Take a booking calendar for a small workshop. I would start with sessions, members, and reservations. The reducers express the actions: reserve a place, cancel, and change a session's capacity. Views expose the records a particular member or organizer is allowed to see. The calendar subscribes to the information it needs.

I can explain that design on a walk. That matters.

## A change has somewhere obvious to go

Suppose the organizer asks for a waitlist.

I would add a representation for waiting members, then decide how joining, leaving, and promotion work. When someone cancels, a reducer can update the reservation and the waitlist together. The UI receives the resulting changes through its subscriptions.

The application still needs careful rules. Does the first person in line always get the place? Can someone reserve for a friend? What happens when the organizer reduces capacity? I want to spend my limited attention on those questions.

Spacetime's [transaction model](https://spacetimedb.com/docs/databases/transactions-atomicity/) lets me keep the database changes for an action together. Its [generated client bindings](https://spacetimedb.com/docs/clients/codegen/) give the client an interface derived from the module. After a schema or reducer change, regenerating those bindings and running the type checker helps identify code that still expects the old interface.

That is a development habit I can repeat. Change the module, regenerate, compile, exercise the action, and check a second client.

## Make it easy to return tomorrow

The development tooling also matters. The documented `spacetime dev` workflow watches module changes, rebuilds and republishes them, and can run the client development server. The TypeScript quickstart includes binding generation in its setup. A short feedback loop makes small changes easier to inspect while they are still fresh in my head. [Development workflow](https://spacetimedb.com/docs/databases/developing/), [TypeScript quickstart](https://spacetimedb.com/docs/quickstarts/typescript/).

For the workshop app, I would keep a handful of concrete checks close by: two members competing for the last place, a member trying to cancel someone else's reservation, and an organizer changing a session while another screen is open.

Those checks describe the application. If they fail, I have a fairly direct path through its tables, reducers, and views to investigate.

I would also give development and production separate databases and handle schema changes deliberately. Being able to move quickly is much more enjoyable when tomorrow's work begins with data I understand. Spacetime documents both automatic and incremental migration workflows. [Automatic migrations](https://spacetimedb.com/docs/databases/automatic-migrations/), [incremental migrations](https://spacetimedb.com/docs/databases/incremental-migrations/).

There is real freedom in this shape of backend. One person can own the whole feature without becoming the integration point between a collection of independently evolving systems.

That is the solo-developer story I care about. I want to ship the first version, come back after a week away, read the module, and know where the next change belongs. Spacetime gives me a programming model I would be happy to return to.

