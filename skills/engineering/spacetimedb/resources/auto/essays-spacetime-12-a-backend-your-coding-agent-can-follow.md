# Give your coding agent a backend it can understand

The instruction “add an archive button” sounds small until you trace it through an application.

There is a field to store, a rule about who can change it, an operation to call, a list that should stop showing the archived item, and perhaps another list where it should appear. An agent can write plausible code for each piece and still leave the feature wrong.

This is why I care so much about the architecture underneath AI-assisted development. I want the agent to have a short, inspectable path from the request to a working result.

Spacetime gives me that path. The module defines the data and the operations that change it. Views and subscriptions define what the client sees. Generated bindings connect the module's interface to the client. I can ask an agent to work through those pieces and check each one.

## Give it a complete feature, with a clear finish line

For the archive example, my instruction would describe the behavior in plain language:

> Add archiving to the task list. Only the owner can archive a task. Keep archived tasks in the database, remove them from the active list, and show them in an archive view. Check that a second connected client sees the change and that another identity cannot read or archive the task.

That gives the agent something concrete to implement and me something concrete to review.

The interesting part is how naturally the request maps onto Spacetime. The task has a stored archive state. The reducer checks the caller and changes it. The read model separates active and archived records. Subscriptions carry the resulting updates to the clients that are allowed to see them. [Reducers](https://spacetimedb.com/docs/functions/reducers/), [views](https://spacetimedb.com/docs/functions/views/).

I can inspect those rules in the module. I can regenerate the bindings and compile the client. I can exercise the operation with two identities. The architecture gives the agent useful feedback at several points before it declares success.

## Make the interface evidence available

I would start the agent with the [Spacetime setup guide](https://spacetimedb.com/agent-setup.md), the project's installed SDK version, and its generated bindings. The bindings are particularly valuable: they describe the interface this application actually has. An agent should generate them from the module and use them as evidence when wiring the client. [Binding generation](https://spacetimedb.com/docs/clients/codegen/).

Then I want it to close the loop. Build the module. Generate the bindings. Run the type checker. Call the operation. Confirm the subscribed result. Try the forbidden operation as a different identity.

The [CRUD example in this series](05-building-a-crud-app.md) was checked through that kind of loop, including a separate identity's rejected writes and private-table access. That is the standard I want for an agent-built feature.

The agent has fewer relationships to keep aligned, and I have a repeatable way to check its work. A feature is finished when it runs and those checks pass.

## A smaller review is a better review

I like being able to ask precise questions. Does the reducer check ownership before changing the row? Does the view expose only the intended records? Does the client use the generated operation? What happens when the operation fails?

Those questions are still necessary. Spacetime makes them easier to locate in the code.

The gain also carries into the next request. Restoring an archived task can follow the same pattern. Adding a team permission means revisiting the write rules and read boundary. The agent can build on the application model it already has.

I want AI-assisted development to feel more dependable as a project grows. Spacetime is the backend I would reach for because I can explain its main pieces to an agent, follow the resulting change myself, and verify the behavior from another client. That is a much more useful kind of speed than producing a large diff quickly.

