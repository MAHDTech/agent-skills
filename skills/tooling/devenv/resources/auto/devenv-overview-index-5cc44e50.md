[Latest from the blogJul 28, 2026devenv 2.2: attach to running processes
and persistent out-of-tree environmentsRead post
→](https://devenv.sh/blog/2026/07/28/devenv-22-attach-to-running-processes-and-persistent-out-of-tree-environments/)

Developer Environments using Nix

# Your whole development environment, declared.

Define packages, languages, services, tasks, secrets, and tooling once.
Give every developer—and CI—the same fast, reproducible environment.

Describe your stack

Rust + Postgres

Python ML

Node + Redis

Go + MySQL

Rails + React

[View examples](https://devenv.sh/examples/)

Generate config →

Generates a complete `devenv.nix` via [devenv.new](https://devenv.new/)

devenv.nix · preview

Copy

``` landing-terminal-body
{ pkgs, ... }: {
  languages.rust.enable = true;

  services.postgres = {
    enable = true;
    initialDatabases = [{ name = "app"; }];
  };

  packages = [ pkgs.cargo-watch ];
}
```

devenv.yaml

Copy

``` landing-terminal-body
```

\<100ms

cached shell startup

100k+

prebuilt packages

58

languages

Linux · macOS · WSL

x64 · ARM64

Languages · Services · Tools

![](https://cdn.simpleicons.org/rust/888888)Rust

![](https://cdn.simpleicons.org/python/888888)Python

![](https://cdn.simpleicons.org/nodedotjs/888888)Node.js

![](https://cdn.simpleicons.org/go/888888)Go

![](https://cdn.simpleicons.org/ruby/888888)Ruby

![](https://cdn.simpleicons.org/php/888888)PHP

![](https://cdn.simpleicons.org/typescript/888888)TypeScript

![](https://cdn.simpleicons.org/elixir/888888)Elixir

![](https://cdn.simpleicons.org/kotlin/888888)Kotlin

![](https://cdn.simpleicons.org/swift/888888)Swift

![](https://cdn.simpleicons.org/haskell/888888)Haskell

![](https://cdn.simpleicons.org/scala/888888)Scala

![](https://cdn.simpleicons.org/dotnet/888888).NET

![](https://cdn.simpleicons.org/lua/888888)Lua

![](https://cdn.simpleicons.org/erlang/888888)Erlang

![](https://cdn.simpleicons.org/ocaml/888888)OCaml

![](https://cdn.simpleicons.org/perl/888888)Perl

![](https://cdn.simpleicons.org/dart/888888)Dart

![](https://cdn.simpleicons.org/cplusplus/888888)C++

![](https://cdn.simpleicons.org/nim/888888)Nim

![](https://cdn.simpleicons.org/crystal/888888)Crystal

![](https://cdn.simpleicons.org/zig/888888)Zig

![](https://cdn.simpleicons.org/r/888888)R

![](https://cdn.simpleicons.org/julia/888888)Julia

![](https://cdn.simpleicons.org/rust/888888)Rust

![](https://cdn.simpleicons.org/python/888888)Python

![](https://cdn.simpleicons.org/nodedotjs/888888)Node.js

![](https://cdn.simpleicons.org/go/888888)Go

![](https://cdn.simpleicons.org/ruby/888888)Ruby

![](https://cdn.simpleicons.org/php/888888)PHP

![](https://cdn.simpleicons.org/typescript/888888)TypeScript

![](https://cdn.simpleicons.org/elixir/888888)Elixir

![](https://cdn.simpleicons.org/kotlin/888888)Kotlin

![](https://cdn.simpleicons.org/swift/888888)Swift

![](https://cdn.simpleicons.org/haskell/888888)Haskell

![](https://cdn.simpleicons.org/scala/888888)Scala

![](https://cdn.simpleicons.org/dotnet/888888).NET

![](https://cdn.simpleicons.org/lua/888888)Lua

![](https://cdn.simpleicons.org/erlang/888888)Erlang

![](https://cdn.simpleicons.org/ocaml/888888)OCaml

![](https://cdn.simpleicons.org/perl/888888)Perl

![](https://cdn.simpleicons.org/dart/888888)Dart

![](https://cdn.simpleicons.org/cplusplus/888888)C++

![](https://cdn.simpleicons.org/nim/888888)Nim

![](https://cdn.simpleicons.org/crystal/888888)Crystal

![](https://cdn.simpleicons.org/zig/888888)Zig

![](https://cdn.simpleicons.org/r/888888)R

![](https://cdn.simpleicons.org/julia/888888)Julia

![](https://cdn.simpleicons.org/postgresql/888888)PostgreSQL

![](https://cdn.simpleicons.org/redis/888888)Redis

![](https://cdn.simpleicons.org/mysql/888888)MySQL

![](https://cdn.simpleicons.org/mongodb/888888)MongoDB

![](https://cdn.simpleicons.org/mariadb/888888)MariaDB

![](https://cdn.simpleicons.org/elasticsearch/888888)Elasticsearch

![](https://cdn.simpleicons.org/rabbitmq/888888)RabbitMQ

![](https://cdn.simpleicons.org/minio/888888)MinIO

![](https://cdn.simpleicons.org/caddy/888888)Caddy

![](https://cdn.simpleicons.org/nginx/888888)nginx

![](https://cdn.simpleicons.org/clickhouse/888888)ClickHouse

![](https://cdn.simpleicons.org/natsdotio/888888)NATS

![](https://cdn.simpleicons.org/meilisearch/888888)Meilisearch

![](https://cdn.simpleicons.org/apachecouchdb/888888)CouchDB

![](https://cdn.simpleicons.org/influxdb/888888)InfluxDB

![](https://cdn.simpleicons.org/opensearch/888888)OpenSearch

![](https://cdn.simpleicons.org/keycloak/888888)Keycloak

![](https://cdn.simpleicons.org/vault/888888)Vault

![](https://cdn.simpleicons.org/apachekafka/888888)Kafka

![](https://cdn.simpleicons.org/postgresql/888888)PostgreSQL

![](https://cdn.simpleicons.org/redis/888888)Redis

![](https://cdn.simpleicons.org/mysql/888888)MySQL

![](https://cdn.simpleicons.org/mongodb/888888)MongoDB

![](https://cdn.simpleicons.org/mariadb/888888)MariaDB

![](https://cdn.simpleicons.org/elasticsearch/888888)Elasticsearch

![](https://cdn.simpleicons.org/rabbitmq/888888)RabbitMQ

![](https://cdn.simpleicons.org/minio/888888)MinIO

![](https://cdn.simpleicons.org/caddy/888888)Caddy

![](https://cdn.simpleicons.org/nginx/888888)nginx

![](https://cdn.simpleicons.org/clickhouse/888888)ClickHouse

![](https://cdn.simpleicons.org/natsdotio/888888)NATS

![](https://cdn.simpleicons.org/meilisearch/888888)Meilisearch

![](https://cdn.simpleicons.org/apachecouchdb/888888)CouchDB

![](https://cdn.simpleicons.org/influxdb/888888)InfluxDB

![](https://cdn.simpleicons.org/opensearch/888888)OpenSearch

![](https://cdn.simpleicons.org/keycloak/888888)Keycloak

![](https://cdn.simpleicons.org/vault/888888)Vault

![](https://cdn.simpleicons.org/apachekafka/888888)Kafka

![](https://cdn.simpleicons.org/docker/888888)Docker

![](https://cdn.simpleicons.org/nixos/888888)Nix

![](https://cdn.simpleicons.org/githubactions/888888)GitHub Actions

![](https://cdn.simpleicons.org/vscodium/888888)VS Code

![](https://cdn.simpleicons.org/intellijidea/888888)IntelliJ

![](https://cdn.simpleicons.org/zedindustries/888888)Zed

![](https://cdn.simpleicons.org/android/888888)Android

![](https://cdn.simpleicons.org/wordpress/888888)WordPress

![](https://cdn.simpleicons.org/terraform/888888)Terraform

![](https://cdn.simpleicons.org/opentofu/888888)OpenTofu

![](https://cdn.simpleicons.org/claude/888888)Claude Code

![](https://cdn.simpleicons.org/linux/888888)Linux

![](https://cdn.simpleicons.org/apple/888888)macOS

![](https://cdn.simpleicons.org/docker/888888)Docker

![](https://cdn.simpleicons.org/nixos/888888)Nix

![](https://cdn.simpleicons.org/githubactions/888888)GitHub Actions

![](https://cdn.simpleicons.org/vscodium/888888)VS Code

![](https://cdn.simpleicons.org/intellijidea/888888)IntelliJ

![](https://cdn.simpleicons.org/zedindustries/888888)Zed

![](https://cdn.simpleicons.org/android/888888)Android

![](https://cdn.simpleicons.org/wordpress/888888)WordPress

![](https://cdn.simpleicons.org/terraform/888888)Terraform

![](https://cdn.simpleicons.org/opentofu/888888)OpenTofu

![](https://cdn.simpleicons.org/claude/888888)Claude Code

![](https://cdn.simpleicons.org/linux/888888)Linux

![](https://cdn.simpleicons.org/apple/888888)macOS

## Three steps. Done.

From zero to a reproducible environment your whole team can use.

1

Install

Once Nix is installed. Works on Linux, macOS, and WSL.

``` landing-code-hl
$ nix profile install nixpkgs#devenv
```

2

Initialize & describe

Scaffold the project, then edit `devenv.nix` or generate it from a
prompt above.

``` landing-code-hl
$ devenv init
$EDITOR devenv.nix
```

3

Activate

Everything available, in under 100ms after the first build.

``` landing-code-hl
$ devenv shell
(devenv) $ cargo run
```

## Your whole stack, declared together

Languages, services, processes, tasks, secrets — all declarative.

[](https://devenv.sh/languages/)

58

Languages

Python, Rust, Go, Node, Ruby, PHP, Java, Elixir, and more — with version
pinning and LSP servers.

[](https://devenv.sh/services/)

Services

42 preconfigured services, including PostgreSQL, Redis, MySQL, RabbitMQ,
MinIO, Caddy, and Elasticsearch.

``` landing-code-hl
services.postgres.enable = true;
services.redis.enable = true;
```

[](https://devenv.sh/processes/)

Processes

Declarative process management with logs, restarts, and dependencies.
Just `devenv up`.

[](https://devenv.sh/tasks/)

Tasks & git hooks

Define dependencies, run in parallel, hook into your shell or commits.
Linters, formatters, codegen.

``` landing-code-hl
tasks."app:build" = {
  exec = "yarn build";
  before = [ "devenv:enterShell" ];
};
```

[](https://devenv.sh/integrations/secretspec/)

SecretSpec

Declarative secrets from Keychain, 1Password, LastPass, or dotenv. Keep
values out of config and committed `.env` files.

``` landing-code-hl
processes.api.exec =
  "secretspec run -- npm start";
```

[](https://devenv.sh/containers/)

Containers

Build OCI containers from your dev environment. Same packages, same
versions, same behavior.

[](https://devenv.sh/tests/)

Tests

Run integration tests with all processes active. `devenv test` and done.

[](https://devenv.sh/basics/)

Basics

Packages, variables, files, and scripts in native **bash**, **zsh**,
**fish**, or **nushell**. Auto-activate on `cd` and apply updates at the
next prompt.

[](https://devenv.sh/ad-hoc-developer-environments/)

Ad-hoc shells — zero config

Spin up a temporary environment without writing a `devenv.nix`. Great
for experiments, scripts, and CI matrices.

``` landing-code-hl
$ devenv -O languages.python.enable:bool true \
       -O languages.python.version:string "3.12" \
       shell
```

[](https://devenv.sh/packages/)

Search packages & options

Search 100,000+ packages and every devenv option from the CLI, using the
exact Nixpkgs version pinned by your project.

``` landing-code-hl
$ devenv search postgres
pkgs.postgresql_17
services.postgres.enable
```

Evaluation caching

## Warm shells in milliseconds.

Auto-invalidated evaluation caching skips unchanged work. No daemons. No
manual cache management. [How it
works.](https://devenv.sh/blog/2024/10/03/devenv-13-instant-developer-environments-with-nix-caching/)

Cached shell startup

\<100ms

Cold

4832 ms

Warm

47 ms

Same config. **~100×** faster on every subsequent shell.

Composability

## Many environments. One shell.

Import config across folders and repositories. [First-class monorepo
support.](https://devenv.sh/composing-using-imports/)

frontend/devenv.nix

languages.javascript.enable = true;

backend/devenv.nix

languages.rust.enable = true;  
services.postgres.enable = true;

github:myorg/shared

services.redis.enable = true;

→

Unified environment

\$ devenv shell

noderustpostgresredis

Profiles

## One project. Every workflow.

Switch between frontend, backend, testing, and full-stack environments
without duplicating configuration. [Layer profiles manually or activate
them by hostname and user.](https://devenv.sh/profiles/)

\$ devenv --profile backend shell

postgresredisAPI tools

\$ devenv --profile fullstack up

backendfrontenddev server

hostname.ci-runner + user.alice

automaticdeterministic priority

devenv.nix

``` landing-code-hl
profiles = {
  backend.module = {
    services.postgres.enable = true;
    services.redis.enable = true;
  };

  frontend.module = {
    languages.javascript.enable = true;
    processes.dev.exec = "npm run dev";
  };

  fullstack.extends = [ "backend" "frontend" ];
};
```

Native process manager

## One `devenv up`. Whole stack alive.

Built-in process supervision: [dependencies, ready probes, restart
policies, socket activation, automatic port allocation](https://devenv.sh/processes/).
[Or swap in process-compose, overmind,
mprocs.](https://devenv.sh/supported-process-managers/)

postgresrunning

port :5432 · uptime 3m 12s

ready probe: `pg_isready`

redisrunning

port :6379 · uptime 3m 12s

restart: `on-failure`

migratecompleted

one-shot · exit 0

after: `postgres`

apirunning

port :8080 · uptime 3m 09s

after: `postgres`, `redis`, `migrate`

socket activation · zero-downtime restarts

Declared once in `devenv.nix`. Started, ordered, restarted, and stopped
by the native supervisor.

SecretSpec

## Declare a secret. Swap the source.

The same declaration reads from Keychain, 1Password, dotenv, or env.
[Load secrets at runtime without committing their
values.](https://devenv.sh/integrations/secretspec/)

Keychain

1Password

LastPass

.env

env

secretspec.toml

``` landing-code-hl
[project]
name = "my-app"

[profiles.default]
DATABASE_URL = { required = true }
STRIPE_SECRET_KEY = { required = true }
```

🔐Source: **macOS Keychain**

↓

📄**secretspec.toml** declares `DATABASE_URL`

↓

🐚`$ secretspec run -- npm start` — exposed only to the app

Packaging

## From dev shell to deployable artifact.

Same languages, same versions, same packages. [Define
outputs](https://devenv.sh/outputs/) and ship a Nix derivation.

1 · Declare

devenv.nix

``` landing-code-hl
languages.rust.enable = true;
outputs.app =
  config.languages.rust.import ./. {};
```

→

2 · Build

devenv build

``` landing-code-hl
$ devenv build outputs.app
• Built /nix/store/...-app
in 12.4s
```

→

3 · Ship

Nix store · binary cache · CI

``` landing-code-hl
$ nix copy --to ssh://deploy \
  /nix/store/...-app
```

## Built in the open

Active community, frequent releases, and a growing ecosystem of services
and languages.

[GitHubStar & contributeRead the source, file an issue, send a PR. MIT
licensed.](https://github.com/cachix/devenv)[DiscordChat with the
teamGet help, share setups, follow what's coming
next.](https://discord.gg/naMgvexb6q)[BlogWhat's newRelease notes,
design deep-dives, and case studies.](https://devenv.sh/blog/)[ExamplesReady-made
setupsCopy a working `devenv.nix` for your
stack.](https://github.com/cachix/devenv/tree/main/examples)

## Start building.

Set up your first environment in minutes — or generate one from the hero
above.

[Get Started](https://devenv.sh/getting-started/)[View on
GitHub](https://github.com/cachix/devenv)
