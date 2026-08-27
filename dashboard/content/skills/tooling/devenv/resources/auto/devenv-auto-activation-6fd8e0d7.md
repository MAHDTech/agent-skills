+++
title = "devenv-auto-activation-6fd8e0d7"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Auto Activation

devenv includes a built in shell hook that automatically activates your
developer environment when you `cd` into a project directory. No
external tools required.

## Setup

- [Bash](#tab-panel-68)
- [Zsh](#tab-panel-69)
- [Fish](#tab-panel-70)
- [Nushell](#tab-panel-71)

Add one line to your shell configuration file:

```
eval "$(devenv hook bash)"
```

~/.bashrc

Add one line to your shell configuration file:

```
eval "$(devenv hook zsh)"
```

~/.zshrc

Usually nothing to do — devenv installed via Nix ships a snippet that
fish loads automatically. If it doesn’t load for you, add this instead:

```
devenv hook fish | source
```

~/.config/fish/config.fish

Usually nothing to do — devenv installed via Nix ships a snippet that nu
loads automatically. If it doesn’t load for you, add this instead:

Run once, in Nu:

```
mkdir ($nu.default-config-dir | path join autoload)devenv hook nu | save --force ($nu.default-config-dir | path join autoload/devenv-hook.nu)
```

Terminal window

## Trusting a project

Before a project can auto activate, you need to explicitly trust it.
This is a security measure that prevents untrusted projects from
modifying your shell.

Navigate to the project directory and run:

```
$ cd ~/myproject$ devenv allowdevenv: allowed /home/user/myproject
```

Terminal window

To activate one or more profiles whenever the project is entered, pass
them when allowing it:

```
$ devenv --profile backend --profile observability allowdevenv: allowed /home/user/myproject with profile backend, observability
```

The selected profiles also apply to subsequent devenv commands in the
project. An explicit `--profile` takes priority. Run plain
`devenv allow` again to clear the saved profile selection without
revoking trust.

When you `cd` into the directory next time, devenv will automatically
start a shell:

```
$ cd ~/myproject(devenv) $
```

Terminal window

## Revoking trust

To stop a project from auto activating:

```
$ cd ~/myproject$ devenv revokedevenv: revoked /home/user/myproject
```

Terminal window

## How it works

The hook runs on every directory change and:

2.  Checks the trust database to verify the project was allowed.
3.  If trusted, runs `devenv shell` in a subshell for that project.

If a project has not been trusted yet, you will see a message asking you
to run `devenv allow`:

```
devenv: /home/user/myproject is not allowed. Run 'devenv allow' to trust this directory.
```

## Automatic deactivation

When you `cd` out of the project directory (or any of its
subdirectories), the devenv shell exits automatically and you return to
your normal shell:

```
(devenv) $ cd ..$
```

Terminal window

## Re-entry protection

The hook will not nest environments. While inside a `devenv shell`,
navigating into a subdirectory of the same project keeps the current
shell. Only navigating outside the project triggers deactivation.

## Comparison with direnv

| Feature | `devenv hook` | [direnv](https://devenv.sh/integrations/direnv/) |
|----|----|----|
| External dependencies | None | Requires direnv |
| Setup | One line in shell config | direnv install + `.envrc` per project |
| Trust granularity | Per project directory | Per `.envrc` file |
| Environment application | Spawns a subshell | Modifies current shell in place |
| Unloading on exit | Subshell exits automatically | direnv unloads variables |

Use `devenv hook` for a simple, dependency free setup. Use
[direnv](https://devenv.sh/integrations/direnv/) if you prefer in place environment
modification without a subshell.

