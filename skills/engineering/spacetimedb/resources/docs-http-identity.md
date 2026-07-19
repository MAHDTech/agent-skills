Version: 2.0.0

On this page

The HTTP endpoints in `/v1/identity` allow clients to generate and
manage Spacetime public identities and private tokens.

## At a glance

| Route | Description |
|----|----|
| [`POST /v1/identity`](#post-v1identity) | Generate a new identity and token. |
| [`POST /v1/identity/websocket-token`](#post-v1identitywebsocket-token) | Generate a short-lived access token for use in untrusted contexts. |
| [`GET /v1/identity/public-key`](#get-v1identitypublic-key) | Get the public key used for verifying tokens. |
| [`GET /v1/identity/:identity/databases`](#get-v1identityidentitydatabases) | List databases owned by an identity. |
| [`GET /v1/identity/:identity/verify`](#get-v1identityidentityverify) | Verify an identity and token. |

## `POST /v1/identity`

Create a new identity.

#### Returns

Returns JSON in the form:

``` codeBlockStandalone_LlrK
{
    "identity": string,
    "token": string
}
```

## `POST /v1/identity/websocket-token`

Generate a short-lived access token which can be used in untrusted
contexts, e.g. embedded in URLs.

#### Required Headers

| Name | Value |
|----|----|
| `Authorization` | A Spacetime token [encoded as Bearer authorization](https://spacetimedb.com/docs/http/authorization). |

#### Returns

Returns JSON in the form:

``` codeBlockStandalone_LlrK
{
    "token": string
}
```

The `token` value is a short-lived [JSON Web
Token](https://datatracker.ietf.org/doc/html/rfc7519).

## `GET /v1/identity/public-key`

Fetches the public key used by the database to verify tokens.

#### Returns

Returns a response of content-type `application/pem-certificate-chain`.

## `GET /v1/identity/:identity/databases`

List all databases owned by an identity.

#### Parameters

| Name        | Value                 |
|-------------|-----------------------|
| `:identity` | A Spacetime identity. |

#### Returns

Returns JSON in the form:

``` codeBlockStandalone_LlrK
{
    "identities": array<string>
}
```

The `identities` value is an array of zero or more strings, each of
which is the identity of a database owned by the identity passed as a
parameter.

## `GET /v1/identity/:identity/verify`

Verify the validity of an identity/token pair.

#### Parameters

| Name        | Value                   |
|-------------|-------------------------|
| `:identity` | The identity to verify. |

#### Required Headers

| Name | Value |
|----|----|
| `Authorization` | A Spacetime token [encoded as Bearer authorization](https://spacetimedb.com/docs/http/authorization). |

#### Returns

Returns no data.

If the token is valid and matches the identity, returns
`204 No Content`.

If the token is valid but does not match the identity, returns
`400 Bad Request`.

If the token is invalid, or no `Authorization` header is included in the
request, returns `401 Unauthorized`.

- [At a glance](#at-a-glance)
- [`POST /v1/identity`](#post-v1identity)
- [`POST /v1/identity/websocket-token`](#post-v1identitywebsocket-token)
- [`GET /v1/identity/public-key`](#get-v1identitypublic-key)
- [`GET /v1/identity/:identity/databases`](#get-v1identityidentitydatabases)
- [`GET /v1/identity/:identity/verify`](#get-v1identityidentityverify)
