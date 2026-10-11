+++
title = "essays-spacetime-08-react-state-beyond-the-browser"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

# My React app doesn't end at the browser

The part of React I love is the small promise at its center: describe the interface for the current state, then let the framework keep the two in agreement.

I want that feeling to survive contact with the backend.

Consider a shared reading list. A component can render the books, the person who added each one, and whether the group has finished it. Locally, this is an easy state problem. Then someone opens the list on a second laptop, and suddenly I am thinking about requests, stale responses, refetching, and which action invalidates which query.

Spacetime makes that transition much more natural. The shared list lives in database tables. Reducers implement actions such as adding a book or marking it finished. The React components subscribe to the data they display. A change made by another reader reaches the component through the same subscription as a change made from my own screen.

That is the connection I wish more React developers would make: **Spacetime brings the idea of reactive state to the server and database.**

## The component gets to stay a component

Spacetime's React integration provides `SpacetimeDBProvider`, `useTable`, and `useReducer`. The provider supplies the connection, `useTable` subscribes to a table or supported query, and `useReducer` gives the component a way to invoke a server operation. Module-specific bindings supply the table and reducer definitions. [React integration](https://spacetimedb.com/docs/clients/typescript/).

For the reading list, I want the UI code to answer ordinary UI questions. How should a finished book look? Where should an error appear? What should an empty list say?

I don't want each button to carry a second job description: after changing a book, remember to refresh the list, the progress indicator, the sidebar, and the other person's laptop.

Subscriptions take care of distributing the relevant database changes. Once those changes arrive in the client cache, the React hook makes the updated rows available to the component. The connection setup and initial loading state are explicit parts of the application. After that, adding another way to change a book doesn't require inventing another way for every subscribed screen to hear about it.

The difference becomes especially pleasant when an update originates somewhere else. A second user finishes a book. A moderator corrects a title. The screen follows the state either way.

## Local state still has a perfectly good job

I would keep the text in an unfinished input in React state. The same goes for whether a menu is open or which tab someone has selected. The shared reading list belongs in Spacetime.

That division gives each kind of state a home. I can close a menu without involving the database. When I submit a book, a reducer checks the request and changes the shared data. A subscription reflects the result.

The reducer is also where I would enforce the group's rules. Who can remove a book? Can someone edit another reader's note? A disabled button is useful feedback; the server operation still checks the caller. Private tables and caller-aware views let me decide which records each reader can receive. [Views and data visibility](https://spacetimedb.com/docs/functions/views/).

This is why the analogy is useful. It gives a React developer a familiar place to begin, while keeping the shared state and its rules on the server.

Try the [chat tutorial](https://spacetimedb.com/docs/tutorials/chat-app/) in two browser windows. Send something from the second window and watch the first. That small moment explains more than a diagram of backend services ever will.

I love that Spacetime lets me keep thinking about the application as I cross that boundary. The browser is where someone interacts with the reading list. It doesn't have to be where my understanding of its state stops.


