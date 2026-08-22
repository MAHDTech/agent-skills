+++
title = "reference-acl-scope-5f973076"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "tauri"
+++

# Scope

An argument for fine grained behavior control of Tauri commands.

It can be of any serde serializable type and is used to allow or prevent
certain actions inside a Tauri command. The configured scope is passed
to the command and will be enforced by the command implementation.

### Example

```
{  "allow": [{ "path": "$HOME/**" }],  "deny": [{ "path": "$HOME/secret.txt" }]}
```

**Object Properties**:

- allow
- deny

### allow

[`Value`](#value)\[\] \| `null`

Data that defines what is allowed by the scope.

### deny

[`Value`](#value)\[\] \| `null`

Data that defines what is denied by the scope. This should be
prioritized by validation logic.

## Definitions

### Number

**Any of the following**:

- `integer` formatted as `int64` Represents an \[`i64`\].
- `number` formatted as `double` Represents a \[`f64`\].

A valid ACL number.

### Value

**Any of the following**:

- `null` Represents a null JSON value.
- `boolean` Represents a \[`bool`\].
- [`Number`](#number) Represents a valid ACL \[`Number`\].
- `string` Represents a \[`String`\].
- [`Value`](#value)\[\] Represents a list of other \[`Value`\]s.
- Represents a map of \[`String`\] keys to \[`Value`\]s. **Allows
  additional properties**: [`Value`](#value)

All supported ACL values.

------------------------------------------------------------------------

[Support on Open Collective](https://opencollective.com/tauri)[Sponsor
on GitHub](https://github.com/sponsors/tauri-apps)

© 2026 Tauri Contributors. CC-BY / MIT

