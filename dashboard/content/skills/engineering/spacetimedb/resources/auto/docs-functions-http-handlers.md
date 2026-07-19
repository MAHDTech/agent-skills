+++
title = "docs-functions-http-handlers"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

Version: 2.0.0

On this page

HTTP handlers allow a SpacetimeDB database to expose an HTTP API.
External clients can make HTTP requests to routes nested under
[`/v1/database/:name_or_address/route`](https://spacetimedb.com/docs/http/database#any-v1databasename_or_identityroutepath);
these requests are resolved to routes defined by the database and then
passed to the corresponding HTTP handler.

warning

***HTTP handlers are currently in beta, and their API may change in
upcoming SpacetimeDB releases.***

## Defining HTTP Handlers

- TypeScript
- Rust
- C++
- C#

Define an HTTP handler with `spacetimedb.httpHandler`.

The function must accept exactly two arguments:

1.  A `HandlerContext`.
2.  A `Request`.

The function must return a `SyncResponse`.

``` codeBlockStandalone_LlrK
import { schema, SyncResponse } from "spacetimedb/server";

const spacetimedb = schema({});
export default spacetimedb;

export const say_hello = spacetimedb.httpHandler((_ctx, _req) => {
    return new SyncResponse("Hello!");
});
```

Because HTTP handlers are unstable, Rust modules that define them must
opt in to the `unstable` feature in their `Cargo.toml`:

``` codeBlockStandalone_LlrK
[dependencies]
spacetimedb = { version = "2.*", features = ["unstable"] }
```

Define an HTTP handler by annotating a function with
`#[spacetimedb::http::handler]`.

The function must accept exactly two arguments:

1.  A `&mut spacetimedb::http::HandlerContext`.
2.  A `spacetimedb::http::Request`.

The function must return a `spacetimedb::http::Response`.

``` codeBlockStandalone_LlrK
use spacetimedb::http::{Body, handler, HandlerContext, Request, Response};

#[handler]
fn say_hello(_ctx: &mut HandlerContext, _req: Request) -> Response {
    Response::new(Body::from_bytes("Hello!"))
}
```

Because HTTP handlers are unstable, C++ modules that define them must
enable `SPACETIMEDB_UNSTABLE_FEATURES` when compiling.

Define an HTTP handler with `SPACETIMEDB_HTTP_HANDLER`.

The function must accept exactly two arguments:

1.  A `SpacetimeDB::HandlerContext`.
2.  A `SpacetimeDB::HttpRequest`.

The function must return a `SpacetimeDB::HttpResponse`.

``` codeBlockStandalone_LlrK
#include "spacetimedb.h"

using namespace SpacetimeDB;

SPACETIMEDB_HTTP_HANDLER(say_hello, HandlerContext ctx, HttpRequest request) {
    return HttpResponse{
        200,
        HttpVersion::Http11,
        { HttpHeader{"content-type", "text/plain; charset=utf-8"} },
        HttpBody::from_string("Hello!"),
    };
}
```

HTTP handlers in C# are currently unstable. To use them, add
`#pragma warning disable STDB_UNSTABLE` at the top of your file.

Define an HTTP handler by annotating a method with
`[SpacetimeDB.HttpHandler]`.

The method must accept exactly two arguments:

1.  A `SpacetimeDB.HandlerContext`.
2.  A `SpacetimeDB.HttpRequest`.

The method must return a `SpacetimeDB.HttpResponse`.

``` codeBlockStandalone_LlrK
using System.Collections.Generic;
using SpacetimeDB;

#pragma warning disable STDB_UNSTABLE
public static partial class Module
{
    [SpacetimeDB.HttpHandler]
    public static HttpResponse SayHello(HandlerContext ctx, HttpRequest request)
    {
        return new HttpResponse(
            200,
            HttpVersion.Http11,
            new List<HttpHeader>(),
            HttpBody.FromString("Hello!")
        );
    }
}
```

## Registering Handlers to Routes

Once you've [defined an HTTP handler](#defining-http-handlers), you must
register it to a route in order to make it reachable for requests.

- TypeScript
- Rust
- C++
- C#

All routes exposed by your module are declared in a `Router`. Register
the `Router` for your database by passing it to
`spacetimedb.httpRouter`.

``` codeBlockStandalone_LlrK
import { Router } from "spacetimedb/server";

export const router = spacetimedb.httpRouter(
    new Router()
        .get("/say-hello", say_hello)
);
```

Add routes within a router with the `get`, `head`, `options`, `put`,
`delete`, `post`, `patch` and `any` methods, which register an HTTP
handler for that HTTP method at a given path.

Nest routers with `router.nest(prefix, subRouter)`, which causes
`subRouter` to handle routing for all paths that start with `prefix`.

Combine routers with `router.merge(otherRouter)`, which combines both
routers.

All routes exposed by your module are declared in a
`spacetimedb::http::Router`. Register the `Router` for your database by
returning it from a function annotated with
`#[spacetimedb::http::router]`.

``` codeBlockStandalone_LlrK
use spacetimedb::http::{router, Router};

#[router]
fn router() -> Router {
    Router::new()
        .get("/say-hello", say_hello)
}
```

Add routes within a router with the `get`, `head`, `options`, `put`,
`delete`, `post`, `patch` and `any` methods, which register an HTTP
handler for that HTTP method at a given path.

Nest routers with `router.nest(prefix, sub_router)`, which causes
`sub_router` to handle routing for all paths that start with `prefix`.

Combine routers with `router.merge(other_router)`, which combines both
routers.

All routes exposed by your module are declared in a
`SpacetimeDB::Router`. Register the `Router` for your database by
returning it from a function defined with `SPACETIMEDB_HTTP_ROUTER`.

``` codeBlockStandalone_LlrK
SPACETIMEDB_HTTP_ROUTER(router) {
    return Router()
        .get("/say-hello", say_hello);
}
```

Add routes within a router with the `get`, `head`, `options`, `put`,
`delete_`, `post`, `patch` and `any` methods, which register an HTTP
handler for that HTTP method at a given path.

Nest routers with `router.nest(prefix, sub_router)`, which causes
`sub_router` to handle routing for all paths that start with `prefix`.

Combine routers with `router.merge(other_router)`, which combines both
routers.

All routes exposed by your module are declared in a
`SpacetimeDB.Router`. Register the `Router` for your database by
returning it from a method annotated with `[SpacetimeDB.HttpRouter]`.

``` codeBlockStandalone_LlrK
public static partial class Module
{
    [SpacetimeDB.HttpRouter]
    public static Router Router() =>
        SpacetimeDB.Router.New()
            .Get("/say-hello", Handlers.SayHello);
}
```

Add routes within a router with the `Get`, `Head`, `Options`, `Put`,
`Delete`, `Post`, `Patch` and `Any` methods, which register an HTTP
handler for that HTTP method at a given path.

Nest routers with `router.Nest(prefix, subRouter)`, which causes
`subRouter` to handle routing for all paths that start with `prefix`.

Combine routers with `router.Merge(otherRouter)`, which combines both
routers.

### Strict Routing

SpacetimeDB uses strict routing, meaning that a request must match a
path exactly in order to be routed to that handler. Trailing slashes are
significant.

## Sending Requests

Routes defined by a SpacetimeDB database are exposed under the prefix
`/v1/database/:name/route`. To access the `say-hello` route above, send
a request to `$SPACETIMEDB_URI/v1/database/$DATABASE/route/say-hello`,
where `$SPACETIMEDB_URI` is the SpacetimeDB host (usually
`https://maincloud.spacetimedb.com`), and `$DATABASE` is the name of the
database.

- [Defining HTTP Handlers](#defining-http-handlers)
- [Registering Handlers to Routes](#registering-handlers-to-routes)
  - [Strict Routing](#strict-routing)
- [Sending Requests](#sending-requests)

