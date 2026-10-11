+++
title = "essays-spacetime-09-fewer-backend-decisions"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# I want fewer backend decisions

There is a particular kind of afternoon I would like to stop spending on software.

I have a feature in mind. It is small. Someone should be able to assign a support ticket, and everyone looking at the queue should see the assignment. Before writing it, I find myself deciding where the authoritative record lives, how the browser learns it changed, whether a cache needs invalidating, and which types describe the request in each layer.

None of those decisions is absurd. Together, they can swallow the feature.

This is a large part of why I love Spacetime. It makes several of those decisions together, and the answers fit together.

The ticket is a row. Assigning it is a reducer. The queue subscribes to the records its user is allowed to see. The client interface is generated from the module. I can begin with the thing the person actually asked for.

## Follow the assignment

Suppose two support agents try to claim the same ticket.

I would write the claim reducer to look up the ticket, check the caller's access, check that it is still unassigned, and record the assignee. Those database operations happen in one transaction. The check belongs with the update, so another caller cannot successfully claim an already assigned ticket through that operation. A rejected claim leaves the database changes from that invocation rolled back. [Reducer transactions](https://spacetimedb.com/docs/databases/transactions-atomicity/).

Then the clients subscribed to that ticket receive the relevant change. The queue and ticket detail can both show the assignment without the claim handler maintaining a list of screens to refresh. [Subscriptions](https://spacetimedb.com/docs/clients/subscriptions/).

I still choose the business rule. Perhaps reassignment is allowed for a supervisor. Perhaps a ticket can have several collaborators but only one owner. Those are interesting decisions because they describe the product.

What disappears is a collection of mechanisms I would otherwise have to make agree about the same event.

## The next feature gets the same foundation

Now add an activity record when a ticket is assigned. I can insert that record in the claim reducer alongside the assignment. Both database changes commit together. I don't need a separate repair process for an assignment that succeeded while its activity-row insertion failed within that transaction.

Now add a workload panel. It can use an appropriate query or view over the state the application already maintains. Spacetime views can expose derived results and support subscriptions, so I can keep the panel's read model near the rules that shape its data. [Views](https://spacetimedb.com/docs/functions/views/).

An email notification is a different operation: it involves an external service. I would record the work to be done, perform it through a worker or suitable procedure, and track the outcome. Spacetime procedures support external I/O and explicit database transactions. That gives me a place to integrate the outside world without pretending an email participates in a database rollback. [Procedures](https://spacetimedb.com/docs/functions/procedures/).

I appreciate being able to explain the design in those terms. The important boundaries have a reason to exist.

## Spend the decisions where they matter

Spacetime gives me a strong starting point; I still think about indexes, subscription scope, authorization, and the amount of data the application retains. Those choices have a direct relationship to what the application does and how it behaves under load.

I find that much more satisfying than selecting another service so two existing services can stay in agreement.

For a shared operational tool like this, Spacetime would be my first choice. Persistence, transactions, generated interfaces, and reactive clients are useful individually. Having them work through the same application model makes it easier to keep moving as the requirements change.

The afternoon I want ends with a working assignment button and a queue that updates on my colleague's screen. Spacetime is built around getting me there.


