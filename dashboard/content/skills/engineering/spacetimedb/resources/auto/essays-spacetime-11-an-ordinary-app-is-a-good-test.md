+++
title = "essays-spacetime-11-an-ordinary-app-is-a-good-test"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# A very ordinary app is a very good test

A repair queue is a good way to explain why I like Spacetime.

Picture a bicycle shop. A customer drops off a bike. Someone records the problem. A mechanic claims the job, adds a note, and marks the repair ready. The person at the counter needs to see what changed. A customer should see their own repair's status, without receiving the shop's internal notes or anyone else's records.

There is nothing exotic about this application. I would happily choose Spacetime for it.

People sometimes encounter Spacetime through a multiplayer game and assume they need a similarly demanding project to justify it. The properties that make shared game state useful also make the repair queue a pleasant application to build: durable records, authoritative rules, transactions, and clients that stay current.

## Start with the shop's language

For this example, I would define repairs, assignments, notes, and customer access. I would make internal records private and expose the appropriate information through views. The customer's view might contain a repair identifier, its public status, and a message intended for them. The workshop's view would have more detail. Spacetime's caller-aware views provide a way to express that read boundary in module code. [Views](https://spacetimedb.com/docs/functions/views/).

The actions are just as recognizable: create a repair, assign a mechanic, add a note, and mark it ready. Each reducer checks who is acting and whether the transition is allowed.

Marking a repair ready could update its status and insert an activity record in the same transaction. If validation fails, that invocation's database changes roll back together. [Transactions and atomicity](https://spacetimedb.com/docs/databases/transactions-atomicity/).

Now connect the screens. The mechanic sees the jobs they can work on. The counter sees the queue. The customer sees their own status. Each subscribes to the data it is permitted to receive. [Subscriptions](https://spacetimedb.com/docs/clients/subscriptions/).

This is already enough to explain the appeal. One action changes the application state, and the relevant screens learn about it through the platform.

## The second screen is where it becomes convincing

I would demonstrate this application with two windows open.

Mark a repair ready in the workshop window. Watch it move in the counter's queue. Then sign in as the customer and check that the public status is visible while the internal note remains inaccessible.

That demonstration tests something more useful than whether a form can submit. It follows the feature through the write rules, shared data, read permissions, and interface.

It also gives the next feature somewhere to fit. Add an estimated completion date. Add a reason for a delay. Give the counter a way to leave a message for the mechanic. These changes extend the same model.

The React side can stay focused on the experience of using the queue. Spacetime's hooks connect components to subscribed data and reducer calls. The person building the app gets to spend time on a readable status, a useful empty state, and a clear explanation when an action is rejected. [React integration](https://spacetimedb.com/docs/clients/typescript/).

## This is enough reason to be excited

I don't need to turn the bicycle shop into a benchmark to find this compelling. The shop needs a dependable account of its work, and its people need to see that work changing.

For a runnable example of the same basic programming model, the [task app in this series](05-building-a-crud-app.md) includes a module, generated bindings, a React client, and validation of ownership and live updates.

Small applications deserve a good foundation. Their requirements grow, their users open a second tab, and their owners eventually ask for something the original form never anticipated.

Spacetime makes me excited about that ordinary work. I can begin with the shop's language and carry it surprisingly far into the implementation.


