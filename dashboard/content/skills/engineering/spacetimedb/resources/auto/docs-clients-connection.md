+++
title = "docs-clients-connection"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

{% raw %}
Version: 2.0.0

On this page

After [generating client bindings](https://spacetimedb.com/docs/clients/codegen) for your
module, you can establish a connection to your SpacetimeDB
[database](https://spacetimedb.com/docs/databases) from your client application. The
`DbConnection` type provides a persistent WebSocket connection that
enables real-time communication with the server.

## Prerequisites

Before connecting, ensure you have:

1.  [Generated client bindings](https://spacetimedb.com/docs/clients/codegen) for your module
2.  A published database running on SpacetimeDB (local or on
    [MainCloud](https://spacetimedb.com/docs/how-to/deploy/maincloud))
3.  The database's URI and name or identity

## Basic Connection

Create a connection using the `DbConnection` builder pattern:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
import { DbConnection } from './module_bindings';

const conn = new DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withDatabaseName("my_database");
```

``` codeBlockStandalone_LlrK
using SpacetimeDB;

var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my_database")
    .Build();
```

``` codeBlockStandalone_LlrK
use module_bindings::DbConnection;

let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my_database")
    .build();
```

``` codeBlockStandalone_LlrK
#include "ModuleBindings/DbConnection.h"

UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
    ->WithDatabaseName(TEXT("my_database"))
    ->Build();
```

Replace `"https://maincloud.spacetimedb.com"` with your SpacetimeDB host
URI, and `"my_database"` with your database's name or identity.

### Connecting to MainCloud

To connect to a database hosted on MainCloud:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
const conn = new DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withDatabaseName("my_database");
```

``` codeBlockStandalone_LlrK
var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my_database")
    .Build();
```

``` codeBlockStandalone_LlrK
let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my_database")
    .build();
```

``` codeBlockStandalone_LlrK
UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
    ->WithDatabaseName(TEXT("my_database"))
    ->Build();
```

## Authentication with Tokens

To authenticate with a token (for example, from
[SpacetimeAuth](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/)),
provide it when building the connection:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
const conn = new DbConnection.builder()
    .withUri("https://maincloud.spacetimedb.com")
    .withDatabaseName("my_database")
    .withToken("your_auth_token_here");
```

``` codeBlockStandalone_LlrK
var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my_database")
    .WithToken("your_auth_token_here")
    .Build();
```

``` codeBlockStandalone_LlrK
let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my_database")
    .with_token("your_auth_token_here")
    .build();
```

``` codeBlockStandalone_LlrK
UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
    ->WithDatabaseName(TEXT("my_database"))
    ->WithToken(TEXT("your_auth_token_here"))
    ->Build();
```

The token is sent to the server during connection and validates your
identity. See the [SpacetimeAuth
documentation](https://spacetimedb.com/docs/core-concepts/authentication/spacetimeauth/) for
details on obtaining and managing tokens.

## Advancing the Connection

Critical: C#, Unity, and Unreal Users

In C# (including Unity), you **must** manually advance the connection to
process incoming messages. In Unreal Engine, you must either manually
advance the connection or enable automatic ticking. If the connection is
not advanced, it will not process messages.

Call `FrameTick()` in your game loop or update method:

- C#
- Unreal

``` codeBlockStandalone_LlrK
// In Unity, call this in your Update() method
void Update()
{
    conn.FrameTick();
}

// Or in a console application, call this in your main loop
while (running)
{
    conn.FrameTick();
    // Your application logic...
}
```

``` codeBlockStandalone_LlrK
// Option 1: call FrameTick() from your Actor's Tick() method
void AMyActor::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (Conn)
    {
        Conn->FrameTick();
    }
}

// Option 2: enable automatic ticking once after building the connection
Conn = Builder->Build();
Conn->SetAutoTicking(true);
```

Failure to advance the connection means your client will not receive any
updates from the server, including subscription data, reducer callbacks,
or connection events.

In Rust and TypeScript, the connection processes messages automatically
via the browser's event loop or Node.js's event loop. No manual polling
is required.

## Connection Lifecycle

### Connection Callbacks

Register callbacks to observe connection state changes:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
const HOST = "https://maincloud.spacetimedb.com";
const DB_NAME = "my_database";
const TOKEN_KEY = `${HOST}/${DB_NAME}/auth_token`;

const conn = DbConnection.builder()
    .withUri(HOST)
    .withDatabaseName(DB_NAME)
    .onConnect((conn, identity, token) => {
        console.log(`Connected! Identity: ${identity.toHexString()}`);
        // Save token for reconnection — keyed per server/database
        localStorage.setItem(TOKEN_KEY, token);
    })
    .onConnectError((_ctx, error) => {
        console.error(`Connection failed:`, error);
    })
    .onDisconnect(() => {
        console.log('Disconnected from SpacetimeDB');
    });
```

``` codeBlockStandalone_LlrK
var conn = DbConnection.Builder()
    .WithUri("https://maincloud.spacetimedb.com")
    .WithDatabaseName("my_database")
    .OnConnect((conn, identity, token) =>
    {
        Console.WriteLine($"Connected! Identity: {identity}");
        // Save token for reconnection
    })
    .OnConnectError((error) =>
    {
        Console.WriteLine($"Connection failed: {error}");
    })
    .OnDisconnect((conn, error) =>
    {
        if (error != null)
        {
            Console.WriteLine($"Disconnected with error: {error}");
        }
        else
        {
            Console.WriteLine("Disconnected normally");
        }
    })
    .Build();
```

``` codeBlockStandalone_LlrK
let conn = DbConnection::builder()
    .with_uri("https://maincloud.spacetimedb.com")
    .with_database_name("my_database")
    .on_connect(|_ctx, _identity, token| {
        println!("Connected! Saving token...");
        // Save token for reconnection
    })
    .on_connect_error(|_ctx, error| {
        eprintln!("Connection failed: {}", error);
    })
    .on_disconnect(|_ctx, error| {
        if let Some(err) = error {
            eprintln!("Disconnected with error: {}", err);
        } else {
            println!("Disconnected normally");
        }
    })
    .build()
    .expect("Failed to connect");
```

``` codeBlockStandalone_LlrK
// Create delegates
FOnConnectDelegate ConnectDelegate;
ConnectDelegate.BindDynamic(this, &AMyActor::OnConnected);

FOnConnectErrorDelegate ErrorDelegate;
ErrorDelegate.BindDynamic(this, &AMyActor::OnConnectError);

FOnDisconnectDelegate DisconnectDelegate;
DisconnectDelegate.BindDynamic(this, &AMyActor::OnDisconnected);

// Build connection with callbacks
UDbConnection* Conn = UDbConnection::Builder()
    ->WithUri(TEXT("https://maincloud.spacetimedb.com"))
    ->WithDatabaseName(TEXT("my_database"))
    ->OnConnect(ConnectDelegate)
    ->OnConnectError(ErrorDelegate)
    ->OnDisconnect(DisconnectDelegate)
    ->Build();

// Callback functions (must be UFUNCTION)
UFUNCTION()
void OnConnected(UDbConnection* Connection, FSpacetimeDBIdentity Identity, const FString& Token)
{
    UE_LOG(LogTemp, Log, TEXT("Connected! Identity: %s"), *Identity.ToHexString());
    // Save token for reconnection
}

UFUNCTION()
void OnConnectError(const FString& Error)
{
    UE_LOG(LogTemp, Error, TEXT("Connection failed: %s"), *Error);
}

UFUNCTION()
void OnDisconnected()
{
    UE_LOG(LogTemp, Warning, TEXT("Disconnected from SpacetimeDB"));
}
```

### Disconnecting

Explicitly close the connection when you're done:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
conn.disconnect();
```

``` codeBlockStandalone_LlrK
conn.Disconnect();
```

``` codeBlockStandalone_LlrK
conn.disconnect();
```

``` codeBlockStandalone_LlrK
Conn->Disconnect();
```

### Reconnection Behavior

Current Limitation

Automatic reconnection behavior is inconsistently implemented across
SDKs. If your connection is interrupted, you may need to create a new
`DbConnection` to re-establish connectivity.

We recommend implementing reconnection logic in your application if
reliable connectivity is critical.

## Connection Identity

Every connection receives a unique
[identity](https://spacetimedb.com/docs/intro/key-architecture#identity) from the server.
Access it through the `on_connect` callback:

- TypeScript
- C#
- Rust
- Unreal

``` codeBlockStandalone_LlrK
.onConnect((conn, identity, token) => {
    console.log(`Identity: ${identity.toHexString()}, ConnectionId: ${conn.connectionId}`);
})
```

``` codeBlockStandalone_LlrK
.OnConnect((conn, identity, token) =>
{
    var connectionId = conn.ConnectionId;
    Console.WriteLine($"Identity: {identity}, ConnectionId: {connectionId}");
})
```

``` codeBlockStandalone_LlrK
.on_connect(|ctx, identity, token| {
    let connection_id = ctx.connection_id();
    println!("Identity: {:?}, ConnectionId: {:?}", identity, connection_id);
})
```

``` codeBlockStandalone_LlrK
UFUNCTION()
void OnConnected(UDbConnection* Connection, FSpacetimeDBIdentity Identity, const FString& Token)
{
    FSpacetimeDBConnectionId ConnectionId = Connection->GetConnectionId();
    UE_LOG(LogTemp, Log, TEXT("Identity: %s, ConnectionId: %s"),
        *Identity.ToHexString(), *ConnectionId.ToHexString());
}
```

The [identity](https://spacetimedb.com/docs/intro/key-architecture#identity) persists across
connections and represents the user, while the [connection
ID](https://spacetimedb.com/docs/intro/key-architecture#connectionid) is unique to each
connection session.

## Next Steps

Now that you have a connection established, you can:

- [Use the SDK API](https://spacetimedb.com/docs/clients/api) to interact with tables, invoke
  reducers, and subscribe to data
- Register callbacks for observing database changes
- Call reducers and procedures on the server

For language-specific details, see:

- [Rust SDK Reference](https://spacetimedb.com/docs/clients/rust)
- [C# SDK Reference](https://spacetimedb.com/docs/clients/c-sharp)
- [TypeScript SDK Reference](https://spacetimedb.com/docs/clients/typescript)
- [Unreal SDK Reference](https://spacetimedb.com/docs/clients/unreal)

{% endraw %}
