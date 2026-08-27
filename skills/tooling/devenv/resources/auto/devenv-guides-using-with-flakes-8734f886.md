# Using devenv with Nix Flakes

[Nix Flakes](https://wiki.nixos.org/wiki/Flakes) provide a standardized
way to manage Nix projects. They allow you to:

- Specify dependencies as inputs
- Pin those dependencies in a lock file
- Define structured outputs for your project

You can integrate the devenv module system (languages, processes,
services, etc.) into a Nix Flake as a `devShell` output. This allows
devenv to work within your existing Flake-based projects.

While Flakes are more widely supported by existing tooling, be aware
that using devenv through Flakes has some performance limitations and
reduced features compared to the dedicated devenv CLI, which we’ll
explain in the comparison below.

## Choosing between devenv and Nix Flakes

For most projects, we recommend using devenv.nix with the dedicated
devenv CLI for the best developer experience. This approach offers
several advantages:

- **Simplicity**: A more straightforward interface with less boilerplate
- **Performance**: Faster evaluation and more efficient caching of
  environments
- **Developer-focused**: Purpose-built for development environments with
  integrated tooling

[Getting started with devenv.](https://devenv.sh/getting-started/)

Consider using the Flake integration when:

- You maintain an existing flake-based project ecosystem
- Your developer environment needs to be consumed by downstream flakes
- You’re an experienced Nix user
- You understand and can work around the technical limitations of Flakes
  (evaluation model, impurity constraints, etc.)

### Comparison of features

| Feature                             | devenv CLI | Nix Flakes              |
|-------------------------------------|------------|-------------------------|
| External flake inputs               | ✓          | ✓                       |
| Shared remote configs               | ✓          | ✓                       |
| Designed for developer environments | ✓          | ✗                       |
| Built-in container support          | ✓          | ✗                       |
| Protection from garbage-collection  | ✓          | ✗                       |
| Faster evaluation (lazy trees)      | ✓          | ✗                       |
| Evaluation caching                  | ✓          | ✗                       |
| Pure evaluation                     | ✓          | ✗ (`impure` by default) |
| Cross-project references            | ✓          | ✓                       |
| secretspec.dev                      | ✓          | ✗                       |
| Running processes when testing      | ✓          | ✗                       |

## Getting started

Set up a new project with Nix flakes using our template:

```
nix flake init --template github:cachix/devenv
```

Terminal window

This template will create:

- A `flake.nix` file containing a basic devenv configuration.
- A `.envrc` file to optionally set up automatic shell activation with
  direnv.

## Working with flake shells

### The `flake.nix` file

Here’s a minimal `flake.nix` to start you off that includes a `devShell`
created with `devenv.lib.mkShell`. See [the reference
documentation](https://devenv.sh/reference/options/) for the possible options to use
here.

```
{  inputs = {    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";    devenv.url = "github:cachix/devenv";  };
  nixConfig = {    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";    extra-substituters = "https://devenv.cachix.org";  };
  outputs = { self, nixpkgs, devenv, ... } @ inputs:    let      system = "x86_64-linux";      pkgs = nixpkgs.legacyPackages.${system};    in    {      devShells.${system}.default = devenv.lib.mkShell {        inherit inputs pkgs;        modules = [          ({ pkgs, config, ... }: {            # This is your devenv configuration            packages = [ pkgs.hello ];
            enterShell = ''              hello            '';
            processes.run.exec = "hello";          })        ];      };    };}
```

### Entering the shell

Create and enter the `devenv` shell with:

```
nix develop --no-pure-eval
```

Terminal window

This will evaluate the inputs to your flake, create a `flake.lock` lock
file, and open a new shell using the `devenv` configuration from your
`flake.nix`.

### Launching processes, services, and tests

Once in the shell, you can launch [processes and services with
`devenv up`](https://devenv.sh/processes/).

```
$ devenv up17:34:37 system | run.1 started (pid=1046939)17:34:37 run.1  | Hello, world!17:34:37 system | run.1 stopped (rc=0)
```

Terminal window

And run [tests with `devenv test`](https://devenv.sh/tests/).

```
$ devenv testRunning tasks     devenv:enterShellSucceeded         devenv:git-hooks:install 10msSucceeded         devenv:enterShell         4ms2 Succeeded                                 14.75ms• Testing ...Running tasks     devenv:enterTestSucceeded         devenv:git-hooks:run     474msNo command        devenv:enterTest1 Skipped, 1 Succeeded                      474.62ms
```

Terminal window

### Automated shell switching with direnv

Activate your shell automatically when you enter the project directory.

```
```consolecurl -o .envrc https://raw.githubusercontent.com/cachix/devenv/main/templates/flake/.envrc```

```consoledirenv allow```
```

#### Caching `devenv up` with direnv

By default, `devenv up` re-evaluates the flake before starting processes
to pick up any changes. With direnv, the shell is reloaded automatically
whenever the flake changes, keeping the environment up to date.

Under direnv, `devenv up`, `devenv test`, and `devenv tasks` skip
re-evaluation and use the cached environment directly, starting
significantly faster.

## Multiple shells

Some projects lend themselves to defining multiple development shells.
For instance, you may want to define multiple development shells for
different subprojects in a monorepo. You can do this by defining the
various development shells in a central `flake.nix` file in the root of
the repository.

The `flake.nix` file contains multiple `devShells`. For example:

```
{  inputs = {    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";    devenv.url = "github:cachix/devenv";  };
  outputs = { self, nixpkgs, devenv, ... } @ inputs:    let      system = "x86_64-linux";      pkgs = nixpkgs.legacyPackages.${system};    in    {      devShells.${system} = {        projectA = devenv.lib.mkShell {          inherit inputs pkgs;          modules = [            {              enterShell = ''                echo this is project A              '';            }          ];        };
        projectB = devenv.lib.mkShell {          inherit inputs pkgs;          modules = [            {              enterShell = ''                echo this is project B              '';            }          ];        };      };    };}
```

Here we’ve define two shells, each with a separate `devenv`
configuration.

To enter the shell of `project A`:

```
$ nix develop --no-pure-eval .#projectAthis is project A(devenv) $
```

Terminal window

To enter the shell of `project B`:

```
$ nix develop --no-pure-eval .#projectBthis is project B(devenv) $
```

Terminal window

## External flakes

If you cannot, or don’t want to, add a `flake.nix` file to your
project’s repository, you can use external flakes instead.

Create a separate repository with a `flake.nix` file, as in the example
above. Then refer to this flake in your project:

```
$ nix develop --no-pure-eval file:/path/to/central/flake#projectAthis is project A(devenv) $
```

Terminal window

You can also add this to the `direnv` configuration of the project. Make
sure the following line is in `.envrc`:

```
nix flake --no-pure-eval file:/path/to/central/flake#projectA
```

External flakes aren’t limited to local paths using `file:`. You can
refer to flakes on `github:` and generic `git:` repositories. See [Nix
flake
references](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html#flake-references)
for more options.

When using this method to refer to external flakes, it’s important to
remember that there is no lock file, so there is no certainty about
which version of the flake is used. A local project flake file will give
you more control over which version of the flake is used.
