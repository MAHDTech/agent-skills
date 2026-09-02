[Latest from the blogJul 28, 2026devenv 2.2: attach to running processes
and persistent out-of-tree environmentsRead post
→](https://devenv.sh/blog/2026/07/28/devenv-22-attach-to-running-processes-and-persistent-out-of-tree-environments/)

# Your whole development environment, declared.

Define packages, languages, services, tasks, secrets, and tooling once.
Give every developer—and CI—the same fast, reproducible environment.

Generate with AI →

Rust + Postgres

Python ML

Node + Redis

Go + MySQL

Rails + React

Ready to compose

**Ingredient streams**Drag anything into the file. Send configured items
back to remove them.

![](https://devenv.sh/_astro/devenv-mark.BVQ3twVL.webp)![](https://devenv.sh/_astro/devenv-mark-dark.oPfRewUL.webp)devenv.nix

Copy

``` landing-terminal-body
{ pkgs, ... }: {  languages = {    # devenv.sh/languages/rust/    rust = {      enable = true;      channel = "stable";    };  };   services = {    # devenv.sh/services/postgres/    postgres = {      enable = true;      initialDatabases = [{ name = "app"; }];    };     # devenv.sh/services/redis/    redis.enable = true;  };   # devenv.sh/tasks/  tasks."app:test".exec = "cargo test";   # devenv.sh/integrations/secretspec/  processes.api.exec =    "secretspec run -- cargo run";   packages = [    # devenv.sh/packages/    pkgs.docker  ];}
```

Ydevenv.yaml

Copy

``` landing-terminal-body
```

![](https://devenv.sh/_astro/devenv-mark.BVQ3twVL.webp)![](https://devenv.sh/_astro/devenv-mark-dark.oPfRewUL.webp)devenv.nix✦

Composing with AI

## Teaching your stack to speak Nix.

Reading your stack

Languages · Services · Tools

![](https://devenv.sh/technology-icons/ansible-73a7f1.svg)ANAnsible

![](https://devenv.sh/technology-icons/c-9a8ee8.svg)CC

![](https://devenv.sh/technology-icons/clojure-64b99a.svg)CLClojure

![](https://devenv.sh/technology-icons/cplusplus-df9a62.svg)C+C++

![](https://devenv.sh/technology-icons/crystal-d97583.svg)CRCrystal

CUCUE

![](https://devenv.sh/technology-icons/dart-73a7f1.svg)DADart

![](https://devenv.sh/technology-icons/deno-9a8ee8.svg)DEDeno

![](https://devenv.sh/technology-icons/dotnet-64b99a.svg)NE.NET

![](https://devenv.sh/technology-icons/elixir-df9a62.svg)ELElixir

![](https://devenv.sh/technology-icons/elm-d97583.svg)ELElm

![](https://devenv.sh/technology-icons/erlang-66b7c5.svg)ERErlang

![](https://devenv.sh/technology-icons/fortran-73a7f1.svg)FOFortran

![](https://devenv.sh/technology-icons/gnu-9a8ee8.svg)GNGNU Awk

![](https://devenv.sh/technology-icons/gleam-64b99a.svg)GLGleam

![](https://devenv.sh/technology-icons/go-df9a62.svg)GOGo

HAHare

![](https://devenv.sh/technology-icons/haskell-66b7c5.svg)HAHaskell

![](https://devenv.sh/technology-icons/helm-73a7f1.svg)HEHelm

IDIdris

![](https://devenv.sh/technology-icons/openjdk-64b99a.svg)JAJava

![](https://devenv.sh/technology-icons/javascript-f7df1e.svg)JAJavaScript

JSJsonnet

![](https://devenv.sh/technology-icons/julia-66b7c5.svg)JUJulia

![](https://devenv.sh/technology-icons/kotlin-73a7f1.svg)KOKotlin

LELean 4

LOLobster

![](https://devenv.sh/technology-icons/lua-df9a62.svg)LULua

![](https://devenv.sh/technology-icons/nim-d97583.svg)NINim

![](https://devenv.sh/technology-icons/nixos-66b7c5.svg)NINix

![](https://devenv.sh/technology-icons/ocaml-73a7f1.svg)OCOCaml

ODOdin

![](https://devenv.sh/technology-icons/opentofu-64b99a.svg)OPOpenTofu

PAPascal

![](https://devenv.sh/technology-icons/perl-d97583.svg)PEPerl

![](https://devenv.sh/technology-icons/php-66b7c5.svg)PHPHP

PKPkl

![](https://devenv.sh/technology-icons/purescript-9a8ee8.svg)PUPureScript

![](https://devenv.sh/technology-icons/python-64b99a.svg)PYPython

![](https://devenv.sh/technology-icons/r-df9a62.svg)RR

![](https://devenv.sh/technology-icons/racket-d97583.svg)RARacket

RARaku

![](https://devenv.sh/technology-icons/robotframework-73a7f1.svg)RORobot Framework

![](https://devenv.sh/technology-icons/ruby-9a8ee8.svg)RURuby

![](https://devenv.sh/technology-icons/rust-ce422b.svg)![](https://devenv.sh/technology-icons/rust-e8664a.svg)RURust

![](https://devenv.sh/technology-icons/scala-df9a62.svg)SCScala

![](https://devenv.sh/technology-icons/gnubash-d97583.svg)SHShell

![](https://devenv.sh/technology-icons/solidity-66b7c5.svg)SOSolidity

STStandard ML

![](https://devenv.sh/technology-icons/swift-9a8ee8.svg)SWSwift

![](https://devenv.sh/technology-icons/terraform-64b99a.svg)TETerraform

![](https://devenv.sh/technology-icons/latex-df9a62.svg)TETeX Live

![](https://devenv.sh/technology-icons/typescript-d97583.svg)TYTypeScript

![](https://devenv.sh/technology-icons/typst-66b7c5.svg)TYTypst

UNUnison

VV

VAVala

![](https://devenv.sh/technology-icons/zig-df9a62.svg)ZIZig

![](https://devenv.sh/technology-icons/ansible-73a7f1.svg)ANAnsible

![](https://devenv.sh/technology-icons/c-9a8ee8.svg)CC

![](https://devenv.sh/technology-icons/clojure-64b99a.svg)CLClojure

![](https://devenv.sh/technology-icons/cplusplus-df9a62.svg)C+C++

![](https://devenv.sh/technology-icons/crystal-d97583.svg)CRCrystal

CUCUE

![](https://devenv.sh/technology-icons/dart-73a7f1.svg)DADart

![](https://devenv.sh/technology-icons/deno-9a8ee8.svg)DEDeno

![](https://devenv.sh/technology-icons/dotnet-64b99a.svg)NE.NET

![](https://devenv.sh/technology-icons/elixir-df9a62.svg)ELElixir

![](https://devenv.sh/technology-icons/elm-d97583.svg)ELElm

![](https://devenv.sh/technology-icons/erlang-66b7c5.svg)ERErlang

![](https://devenv.sh/technology-icons/fortran-73a7f1.svg)FOFortran

![](https://devenv.sh/technology-icons/gnu-9a8ee8.svg)GNGNU Awk

![](https://devenv.sh/technology-icons/gleam-64b99a.svg)GLGleam

![](https://devenv.sh/technology-icons/go-df9a62.svg)GOGo

HAHare

![](https://devenv.sh/technology-icons/haskell-66b7c5.svg)HAHaskell

![](https://devenv.sh/technology-icons/helm-73a7f1.svg)HEHelm

IDIdris

![](https://devenv.sh/technology-icons/openjdk-64b99a.svg)JAJava

![](https://devenv.sh/technology-icons/javascript-f7df1e.svg)JAJavaScript

JSJsonnet

![](https://devenv.sh/technology-icons/julia-66b7c5.svg)JUJulia

![](https://devenv.sh/technology-icons/kotlin-73a7f1.svg)KOKotlin

LELean 4

LOLobster

![](https://devenv.sh/technology-icons/lua-df9a62.svg)LULua

![](https://devenv.sh/technology-icons/nim-d97583.svg)NINim

![](https://devenv.sh/technology-icons/nixos-66b7c5.svg)NINix

![](https://devenv.sh/technology-icons/ocaml-73a7f1.svg)OCOCaml

ODOdin

![](https://devenv.sh/technology-icons/opentofu-64b99a.svg)OPOpenTofu

PAPascal

![](https://devenv.sh/technology-icons/perl-d97583.svg)PEPerl

![](https://devenv.sh/technology-icons/php-66b7c5.svg)PHPHP

PKPkl

![](https://devenv.sh/technology-icons/purescript-9a8ee8.svg)PUPureScript

![](https://devenv.sh/technology-icons/python-64b99a.svg)PYPython

![](https://devenv.sh/technology-icons/r-df9a62.svg)RR

![](https://devenv.sh/technology-icons/racket-d97583.svg)RARacket

RARaku

![](https://devenv.sh/technology-icons/robotframework-73a7f1.svg)RORobot Framework

![](https://devenv.sh/technology-icons/ruby-9a8ee8.svg)RURuby

![](https://devenv.sh/technology-icons/rust-ce422b.svg)![](https://devenv.sh/technology-icons/rust-e8664a.svg)RURust

![](https://devenv.sh/technology-icons/scala-df9a62.svg)SCScala

![](https://devenv.sh/technology-icons/gnubash-d97583.svg)SHShell

![](https://devenv.sh/technology-icons/solidity-66b7c5.svg)SOSolidity

STStandard ML

![](https://devenv.sh/technology-icons/swift-9a8ee8.svg)SWSwift

![](https://devenv.sh/technology-icons/terraform-64b99a.svg)TETerraform

![](https://devenv.sh/technology-icons/latex-df9a62.svg)TETeX Live

![](https://devenv.sh/technology-icons/typescript-d97583.svg)TYTypeScript

![](https://devenv.sh/technology-icons/typst-66b7c5.svg)TYTypst

UNUnison

VV

VAVala

![](https://devenv.sh/technology-icons/zig-df9a62.svg)ZIZig

![](https://devenv.sh/technology-icons/adminer-34567c.svg)![](https://devenv.sh/technology-icons/adminer-7fa6c9.svg)ADAdminer

BLBlackfire

![](https://devenv.sh/technology-icons/caddy-1f88c0.svg)![](https://devenv.sh/technology-icons/caddy-48a9db.svg)CACaddy

![](https://devenv.sh/technology-icons/apachecassandra-1287b1.svg)![](https://devenv.sh/technology-icons/apachecassandra-44b5d9.svg)CACassandra

![](https://devenv.sh/technology-icons/clickhouse-ffcc01.svg)CLClickHouse

![](https://devenv.sh/technology-icons/cockroachlabs-6933ff.svg)![](https://devenv.sh/technology-icons/cockroachlabs-9674ff.svg)COCockroachDB

![](https://devenv.sh/technology-icons/apachecouchdb-e42528.svg)![](https://devenv.sh/technology-icons/apachecouchdb-f05a5c.svg)COCouchDB

DYDynamoDB Local

ELElasticMQ

![](https://devenv.sh/technology-icons/elasticsearch-005571.svg)![](https://devenv.sh/technology-icons/elasticsearch-36b8c5.svg)ELElasticsearch

GAGarage

HThttpbin

![](https://devenv.sh/technology-icons/influxdb-22adf6.svg)INInfluxDB

![](https://devenv.sh/technology-icons/apachekafka-231f20.svg)![](https://devenv.sh/technology-icons/apachekafka-f4f1f2.svg)KAKafka

![](https://devenv.sh/technology-icons/keycloak-4d4d4d.svg)![](https://devenv.sh/technology-icons/keycloak-b8b8b8.svg)KEKeycloak

MAMailHog

MAMailpit

![](https://devenv.sh/technology-icons/meilisearch-ff5caa.svg)![](https://devenv.sh/technology-icons/meilisearch-ff7dba.svg)MEMeilisearch

MEMemcached

![](https://devenv.sh/technology-icons/minio-c72e49.svg)![](https://devenv.sh/technology-icons/minio-e76078.svg)MIMinIO

![](https://devenv.sh/technology-icons/mongodb-47a248.svg)![](https://devenv.sh/technology-icons/mongodb-65c466.svg)MOMongoDB

![](https://devenv.sh/technology-icons/eclipsemosquitto-3c5280.svg)![](https://devenv.sh/technology-icons/eclipsemosquitto-7f95c5.svg)MOMosquitto

![](https://devenv.sh/technology-icons/mysql-4479a1.svg)![](https://devenv.sh/technology-icons/mysql-71a8d1.svg)MYMySQL

![](https://devenv.sh/technology-icons/natsdotio-27aae1.svg)![](https://devenv.sh/technology-icons/natsdotio-45bded.svg)NANATS

![](https://devenv.sh/technology-icons/nginx-009639.svg)![](https://devenv.sh/technology-icons/nginx-33c66a.svg)NGnginx

![](https://devenv.sh/technology-icons/nixos-5277c3.svg)![](https://devenv.sh/technology-icons/nixos-7fa0e2.svg)NINixseparatedebuginfod

![](https://devenv.sh/technology-icons/opensearch-005eb8.svg)![](https://devenv.sh/technology-icons/opensearch-4a9be8.svg)OPOpenSearch

![](https://devenv.sh/technology-icons/opentelemetry-000000.svg)![](https://devenv.sh/technology-icons/opentelemetry-f5f5f5.svg)OPOpenTelemetry
Collector

![](https://devenv.sh/technology-icons/postgresql-4169e1.svg)![](https://devenv.sh/technology-icons/postgresql-7895f0.svg)POPostgreSQL

![](https://devenv.sh/technology-icons/prometheus-e6522c.svg)![](https://devenv.sh/technology-icons/prometheus-f07859.svg)PRPrometheus

![](https://devenv.sh/technology-icons/rabbitmq-ff6600.svg)![](https://devenv.sh/technology-icons/rabbitmq-ff8533.svg)RARabbitMQ

![](https://devenv.sh/technology-icons/redis-ff4438.svg)![](https://devenv.sh/technology-icons/redis-ff6c63.svg)RERedis

![](https://devenv.sh/technology-icons/rustfs-0196d0.svg)![](https://devenv.sh/technology-icons/rustfs-32bceb.svg)RURustFS

![](https://devenv.sh/technology-icons/turso-4ff8d2.svg)SQsqld

![](https://devenv.sh/technology-icons/tailscale-242424.svg)![](https://devenv.sh/technology-icons/tailscale-f2f2f2.svg)TATailscale
Funnel

![](https://devenv.sh/technology-icons/temporal-000000.svg)![](https://devenv.sh/technology-icons/temporal-ffffff.svg)TETemporal

TITideways

![](https://devenv.sh/technology-icons/apache-d22128.svg)![](https://devenv.sh/technology-icons/apache-ee5c61.svg)TRTraffic
Server

TYTypesense

VAVarnish

![](https://devenv.sh/technology-icons/vault-ffec6e.svg)VAVault

WIWireMock

![](https://devenv.sh/technology-icons/adminer-34567c.svg)![](https://devenv.sh/technology-icons/adminer-7fa6c9.svg)ADAdminer

BLBlackfire

![](https://devenv.sh/technology-icons/caddy-1f88c0.svg)![](https://devenv.sh/technology-icons/caddy-48a9db.svg)CACaddy

![](https://devenv.sh/technology-icons/apachecassandra-1287b1.svg)![](https://devenv.sh/technology-icons/apachecassandra-44b5d9.svg)CACassandra

![](https://devenv.sh/technology-icons/clickhouse-ffcc01.svg)CLClickHouse

![](https://devenv.sh/technology-icons/cockroachlabs-6933ff.svg)![](https://devenv.sh/technology-icons/cockroachlabs-9674ff.svg)COCockroachDB

![](https://devenv.sh/technology-icons/apachecouchdb-e42528.svg)![](https://devenv.sh/technology-icons/apachecouchdb-f05a5c.svg)COCouchDB

DYDynamoDB Local

ELElasticMQ

![](https://devenv.sh/technology-icons/elasticsearch-005571.svg)![](https://devenv.sh/technology-icons/elasticsearch-36b8c5.svg)ELElasticsearch

GAGarage

HThttpbin

![](https://devenv.sh/technology-icons/influxdb-22adf6.svg)INInfluxDB

![](https://devenv.sh/technology-icons/apachekafka-231f20.svg)![](https://devenv.sh/technology-icons/apachekafka-f4f1f2.svg)KAKafka

![](https://devenv.sh/technology-icons/keycloak-4d4d4d.svg)![](https://devenv.sh/technology-icons/keycloak-b8b8b8.svg)KEKeycloak

MAMailHog

MAMailpit

![](https://devenv.sh/technology-icons/meilisearch-ff5caa.svg)![](https://devenv.sh/technology-icons/meilisearch-ff7dba.svg)MEMeilisearch

MEMemcached

![](https://devenv.sh/technology-icons/minio-c72e49.svg)![](https://devenv.sh/technology-icons/minio-e76078.svg)MIMinIO

![](https://devenv.sh/technology-icons/mongodb-47a248.svg)![](https://devenv.sh/technology-icons/mongodb-65c466.svg)MOMongoDB

![](https://devenv.sh/technology-icons/eclipsemosquitto-3c5280.svg)![](https://devenv.sh/technology-icons/eclipsemosquitto-7f95c5.svg)MOMosquitto

![](https://devenv.sh/technology-icons/mysql-4479a1.svg)![](https://devenv.sh/technology-icons/mysql-71a8d1.svg)MYMySQL

![](https://devenv.sh/technology-icons/natsdotio-27aae1.svg)![](https://devenv.sh/technology-icons/natsdotio-45bded.svg)NANATS

![](https://devenv.sh/technology-icons/nginx-009639.svg)![](https://devenv.sh/technology-icons/nginx-33c66a.svg)NGnginx

![](https://devenv.sh/technology-icons/nixos-5277c3.svg)![](https://devenv.sh/technology-icons/nixos-7fa0e2.svg)NINixseparatedebuginfod

![](https://devenv.sh/technology-icons/opensearch-005eb8.svg)![](https://devenv.sh/technology-icons/opensearch-4a9be8.svg)OPOpenSearch

![](https://devenv.sh/technology-icons/opentelemetry-000000.svg)![](https://devenv.sh/technology-icons/opentelemetry-f5f5f5.svg)OPOpenTelemetry
Collector

![](https://devenv.sh/technology-icons/postgresql-4169e1.svg)![](https://devenv.sh/technology-icons/postgresql-7895f0.svg)POPostgreSQL

![](https://devenv.sh/technology-icons/prometheus-e6522c.svg)![](https://devenv.sh/technology-icons/prometheus-f07859.svg)PRPrometheus

![](https://devenv.sh/technology-icons/rabbitmq-ff6600.svg)![](https://devenv.sh/technology-icons/rabbitmq-ff8533.svg)RARabbitMQ

![](https://devenv.sh/technology-icons/redis-ff4438.svg)![](https://devenv.sh/technology-icons/redis-ff6c63.svg)RERedis

![](https://devenv.sh/technology-icons/rustfs-0196d0.svg)![](https://devenv.sh/technology-icons/rustfs-32bceb.svg)RURustFS

![](https://devenv.sh/technology-icons/turso-4ff8d2.svg)SQsqld

![](https://devenv.sh/technology-icons/tailscale-242424.svg)![](https://devenv.sh/technology-icons/tailscale-f2f2f2.svg)TATailscale
Funnel

![](https://devenv.sh/technology-icons/temporal-000000.svg)![](https://devenv.sh/technology-icons/temporal-ffffff.svg)TETemporal

TITideways

![](https://devenv.sh/technology-icons/apache-d22128.svg)![](https://devenv.sh/technology-icons/apache-ee5c61.svg)TRTraffic
Server

TYTypesense

VAVarnish

![](https://devenv.sh/technology-icons/vault-ffec6e.svg)VAVault

WIWireMock

![](https://devenv.sh/technology-icons/docker-54b9f5.svg)+Docker

![](https://devenv.sh/technology-icons/git-e47758.svg)+Git

![](https://devenv.sh/technology-icons/curl-72a7e8.svg)+curl

{}jq

rgripgrep

![](https://devenv.sh/technology-icons/rust-d2a85d.svg)+cargo-watch

AWSAWS CLI

![](https://devenv.sh/technology-icons/terraform-9b82e5.svg)+Terraform CLI

![](https://devenv.sh/technology-icons/nodedotjs-70b678.svg)+Node.js 22

\$✓ShellCheck

![](https://devenv.sh/technology-icons/just-c79a6b.svg)\$\_just

✓Test task

›\_App process

![](https://devenv.sh/technology-icons/1password-a98bff.svg)◇SecretSpec

![](https://devenv.sh/technology-icons/git-e47758.svg)✓Pre-commit

![](https://devenv.sh/technology-icons/docker-54b9f5.svg)+Docker

![](https://devenv.sh/technology-icons/git-e47758.svg)+Git

![](https://devenv.sh/technology-icons/curl-72a7e8.svg)+curl

{}jq

rgripgrep

![](https://devenv.sh/technology-icons/rust-d2a85d.svg)+cargo-watch

AWSAWS CLI

![](https://devenv.sh/technology-icons/terraform-9b82e5.svg)+Terraform CLI

![](https://devenv.sh/technology-icons/nodedotjs-70b678.svg)+Node.js 22

\$✓ShellCheck

![](https://devenv.sh/technology-icons/just-c79a6b.svg)\$\_just

✓Test task

›\_App process

![](https://devenv.sh/technology-icons/1password-a98bff.svg)◇SecretSpec

![](https://devenv.sh/technology-icons/git-e47758.svg)✓Pre-commit

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

Languages, services, processes, tasks, and secrets. Everything is
declarative.

[](https://devenv.sh/languages/)

58

Languages

Python, Rust, Go, Node, Ruby, PHP, Java, Elixir, and more, with version
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

Ad-hoc shells, zero config

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

🐚`$ secretspec run -- npm start`, exposed only to the app

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

Set up your first environment in minutes, or generate one from the hero
above.

[Get Started](https://devenv.sh/getting-started/)[View on
GitHub](https://github.com/cachix/devenv)
