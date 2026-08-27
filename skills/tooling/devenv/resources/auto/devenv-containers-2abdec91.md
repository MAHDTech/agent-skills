# Containers

Added in `0.6`

To use ‘containers’ command, add the following dependencies:

```
$ devenv inputs add nix2container github:nlewo/nix2container --follows nixpkgs$ devenv inputs add mk-shell-bin github:rrbutani/nix-mk-shell-bin
```

Terminal window

Use `devenv container build <name>` to generate an [OCI
container](https://opencontainers.org/) from your development
environment.

By default, `shell` and `processes` containers are predefined. You can
also [craft your own](#running-artifacts)!

Examples of what `devenv container` can do:

- `devenv container build shell`: Generate a container and [start the
  environment](#entering-the-development-environment), equivalent of
  using `devenv shell`.
- `devenv container build processes`: Generate a container and [start
  processes](#running-processes), equivalent of using `devenv up`.
- `devenv container --registry docker://ghcr.io/ copy <name>`: [Copy the
  container](#copying-a-container-to-a-registry) `<name>` into the
  **GitHub package registry**.
- `devenv container run <name>`: Run the container `<name>` using
  **Docker**.

See the [list of all container
options](https://devenv.sh/reference/options/#containers).

## Entering the development environment

Given a simple environment, using Python:

```
{  name = "simple-python-app";
  languages.python.enable = true;}
```

devenv.nix

Generate a container specification that enters the environment:

```
$ devenv container build shell/nix/store/...-image-devenv.json
```

Terminal window

Let’s test it locally using Docker:

```
$ devenv container run shell...(devenv) bash-5.2# pythonPython 3.10.9 (main, Dec  6 2022, 18:44:57) [GCC 12.2.0] on linuxType "help", "copyright", "credits" or "license" for more information.>>>
```

Terminal window

## Running processes

A common deployment strategy is to run each [process](https://devenv.sh/processes/) as an
entrypoint to the container.

```
{  name = "myapp";
  packages = [ pkgs.procps ];
  processes = {    hello-docker.exec = "while true; do echo 'Hello Docker!' && sleep 1; done";    hello-nix.exec = "while true; do echo 'Hello Nix!' && sleep 1; done";  };
  # Exclude the source repo to make the container smaller.  # containers.processes.copyToRoot = null;}
```

devenv.nix

You can now copy the newly created image and start the container:

```
$ devenv container run processes...06:30:06 system         | hello-docker.1 started (pid=15)06:30:06 hello-docker.1 | Hello Docker!06:30:06 system         | hello-nix.1 started (pid=16)06:30:06 hello-nix.1    | Hello Nix!06:30:07 hello-nix.1    | Hello Nix!06:30:07 hello-docker.1 | Hello Docker!06:30:08 hello-nix.1    | Hello Nix!06:30:08 hello-docker.1 | Hello Docker!
```

Terminal window

## Running a single process

You can specify the command to run when the container starts (instead of
entering the default development environment):

```
{  processes.serve.exec = "python -m http.server";
  containers."serve" = {    name = "myapp";    startupCommand = config.processes.serve.exec;  };}
```

devenv.nix

```
$ devenv container run serve
```

Terminal window

## Running artifacts

If you’re building binaries as part of the development environment, you
can choose to only include those in the final image:

```
{  # watch local changes and build the project to ./dist  processes.build.exec = "${pkgs.watchexec}/bin/watchexec my-build-tool";
  containers."prod" = {    copyToRoot = ./dist;    startupCommand = "/mybinary serve";  };}
```

devenv.nix

```
$ devenv container run prod...
```

Terminal window

## Copying a container to a registry

To copy a container into a registry use `copy` subcommand:

```
$ devenv container --registry docker:// copy processes
```

Terminal window

Another common example is deploying to [fly.io](https://fly.io). Any
arguments passed to `--copy-args` are forwarded to [skopeo
copy](https://github.com/containers/skopeo/blob/main/docs/skopeo-copy.1.md#options):

```
$ devenv container --registry docker://registry.fly.io/ --copy-args="--dest-creds x:$(flyctl auth token)" copy processes
```

Terminal window

You can also specify these options declaratively:

```
{  containers."processes" = {    registry = "docker://registry.fly.io/";    defaultCopyArgs = [      "--dest-creds"      "x:\"$(${pkgs.flyctl}/bin/flyctl auth token)\""    ];  };}
```

devenv.nix

See this [fly.io
example](https://github.com/cachix/devenv/tree/main/examples/fly.io) for
how to get started.

## Changing the environment based on the build type

If you want to provide the `openssl` package to native and container
environments, but `git` only for native environments:

```
{ pkgs, config, lib, ... }:
{  packages = [ pkgs.openssl ]    ++ lib.optionals (!config.container.isBuilding) [ pkgs.git ];}
```

devenv.nix

You can also conditionalize based on the particular container that is
being built, for example, `config.containers."processes".isBuilding`.
