+++
title = "docs-how-to-logging"
[extra]
skill = false
category = "engineering"
mermaid = false
skill_name = "spacetimedb"
+++

{% raw %}
Version: 2.0.0

On this page

SpacetimeDB provides logging capabilities for debugging and monitoring
your modules. Log messages are private to the database owner and are not
visible to clients.

## Writing Logs

- TypeScript
- C#
- Rust
- C++

Use the standard `console` API to write logs from your reducers:

``` codeBlockStandalone_LlrK
import { schema, t } from 'spacetimedb/server';

const spacetimedb = schema({ /* tables */ });
export default spacetimedb;

export const process_data = spacetimedb.reducer({ value: t.u32() }, (ctx, { value }) => {
  console.log(`Processing data with value: ${value}`);
  
  if (value > 100) {
    console.warn(`Value ${value} exceeds threshold`);
  }
  
  if (value === 0) {
    console.error('Invalid value: 0');
    throw new Error('Value cannot be zero');
  }
  
  console.debug(`Debug information: ctx.sender = ${ctx.sender}`);
});
```

Available console methods:

- `console.error()` - Error messages
- `console.warn()` - Warning messages
- `console.log()` - Informational messages
- `console.debug()` - Debug messages

SpacetimeDB automatically routes these standard console calls through
its internal logging system.

Use the `SpacetimeDB.Log` class to write logs from your reducers:

``` codeBlockStandalone_LlrK
using SpacetimeDB;

public static partial class Module
{
    [SpacetimeDB.Reducer]
    public static void ProcessData(ReducerContext ctx, uint value)
    {
        Log.Info($"Processing data with value: {value}");
        
        if (value > 100)
        {
            Log.Warn($"Value {value} exceeds threshold");
        }
        
        if (value == 0)
        {
            Log.Error("Invalid value: 0");
            throw new ArgumentException("Value cannot be zero");
        }
        
        Log.Debug($"Debug information: ctx.Sender = {ctx.Sender}");
    }
}
```

Available log methods:

- `Log.Error()` - Error messages
- `Log.Warn()` - Warning messages
- `Log.Info()` - Informational messages
- `Log.Debug()` - Debug messages
- `Log.Trace()` - Trace messages

Use the `log` crate to write logs from your reducers:

``` codeBlockStandalone_LlrK
use spacetimedb::{reducer, ReducerContext};

#[reducer]
pub fn process_data(ctx: &ReducerContext, value: u32) -> Result<(), String> {
    log::info!("Processing data with value: {}", value);
    
    if value > 100 {
        log::warn!("Value {} exceeds threshold", value);
    }
    
    if value == 0 {
        log::error!("Invalid value: 0");
        return Err("Value cannot be zero".to_string());
    }
    
    log::debug!("Debug information: ctx.sender = {:?}", ctx.sender());
    
    Ok(())
}
```

Available log levels:

- `log::error!()` - Error messages
- `log::warn!()` - Warning messages
- `log::info!()` - Informational messages
- `log::debug!()` - Debug messages
- `log::trace!()` - Trace messages

Use the `LOG_*` macros to write logs from your reducers:

``` codeBlockStandalone_LlrK
using namespace SpacetimeDB;

SPACETIMEDB_REDUCER(process_data, ReducerContext ctx, uint32_t value) {
    LOG_INFO("Processing data with value: " + std::to_string(value));
    
    if (value > 100) {
        LOG_WARN("Value " + std::to_string(value) + " exceeds threshold");
    }
    
    if (value == 0) {
        LOG_ERROR("Invalid value: 0");
        return Err("Value cannot be zero");
    }
    
    LOG_DEBUG("Debug information: ctx.sender = " + ctx.sender().to_string());
    
    return Ok();
}
```

Available log macros:

- `LOG_ERROR()` - Error messages
- `LOG_WARN()` - Warning messages
- `LOG_INFO()` - Informational messages
- `LOG_DEBUG()` - Debug messages
- `LOG_PANIC()` + `LOG_FATAL()` - Fatal errors (terminates the reducer)

## Viewing Logs

To view logs from your database, use the `spacetime logs` command:

``` codeBlockStandalone_LlrK
spacetime logs <DATABASE_NAME>
```

### Following Logs in Real-Time

To stream logs as they're generated (similar to `tail -f`):

``` codeBlockStandalone_LlrK
spacetime logs --follow <DATABASE_NAME>
```

### Filtering Logs

You can filter logs by various criteria:

``` codeBlockStandalone_LlrK
# Show only errors
spacetime logs --level error <DATABASE_NAME>

# Show warnings and errors
spacetime logs --level warn <DATABASE_NAME>

# Show logs from a specific time range
spacetime logs --since "2023-01-01 00:00:00" <DATABASE_NAME>
```

For all log viewing options, see the [`spacetime logs` CLI
reference](https://spacetimedb.com/docs/cli-reference#spacetime-logs).

## Best Practices

### Log Levels

Use appropriate log levels for different types of messages:

- **Error**: Use for actual errors that prevent operations from
  completing
- **Warn**: Use for potentially problematic situations that don't
  prevent execution
- **Info**: Use for important application events (user actions, state
  changes)
- **Debug**: Use for detailed diagnostic information useful during
  development
- **Trace**: Use for very detailed diagnostic information (typically
  disabled in production)

### Performance Considerations

- Logging has minimal overhead, but excessive logging can impact
  performance
- Avoid logging in tight loops or high-frequency operations
- Consider using debug/trace logs for verbose output that can be
  filtered in production

### Privacy and Security

- Logs are only visible to the database owner, not to clients
- Avoid logging sensitive information like passwords or authentication
  tokens
- Be mindful of personally identifiable information (PII) in logs

### Structured Logging

- TypeScript
- C#
- Rust
- C++

Include relevant context in your log messages:

``` codeBlockStandalone_LlrK
export const transfer_credits = spacetimedb.reducer(
  { to_user: t.u64(), amount: t.u32() },
  (ctx, { to_user, amount }) => {
    console.log(`Credit transfer: from=${ctx.sender}, to=${to_user}, amount=${amount}`);
    
    // ... transfer logic
  }
);
```

Include relevant context in your log messages:

``` codeBlockStandalone_LlrK
[SpacetimeDB.Reducer]
public static void TransferCredits(ReducerContext ctx, ulong toUser, uint amount)
{
    Log.Info($"Credit transfer: from={ctx.Sender}, to={toUser}, amount={amount}");
    
    // ... transfer logic
}
```

Use structured logging with key-value pairs for better log analysis:

``` codeBlockStandalone_LlrK
use spacetimedb::log;

#[reducer]
pub fn transfer_credits(ctx: &ReducerContext, to_user: u64, amount: u32) -> Result<(), String> {
    log::info!(
        "Credit transfer: from={:?}, to={}, amount={}", 
        ctx.sender(), 
        to_user, 
        amount
    );
    
    // ... transfer logic
    
    Ok(())
}
```

Include relevant context in your log messages:

``` codeBlockStandalone_LlrK
using namespace SpacetimeDB;

SPACETIMEDB_REDUCER(transfer_credits, ReducerContext ctx, uint64_t to_user, uint32_t amount) {
    LOG_INFO("Credit transfer: from=" + ctx.sender().to_string() + 
             ", to=" + std::to_string(to_user) + 
             ", amount=" + std::to_string(amount));
    
    // ... transfer logic
    
    return Ok();
}
```

## Next Steps

- Learn about [Error Handling](https://spacetimedb.com/docs/functions/reducers/error-handling)
  in reducers
- Explore the [CLI Reference](https://spacetimedb.com/docs/cli-reference) for more logging
  options
- Set up monitoring and alerting for your production databases

{% endraw %}
