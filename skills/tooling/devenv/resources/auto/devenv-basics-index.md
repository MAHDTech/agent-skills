Given a hello world example, click on the end of each line to get an explanation:

devenv.nix

```
{ pkgs, ... }: # (1)!

{ # (2)!
  env.GREET = "hello"; # (3)!

  packages = [ pkgs.jq ];

  enterShell = ''
    echo $GREET
    jq --version
  ''; # (4)!
}
```

1. `devenv.nix` is a function with inputs. `pkgs` is an [input](https://devenv.sh/inputs/index.md) passed as a special argument to the function. We use a special input `...` at the end as a catch-all to avoid enumerating all of the inputs.
1. Our function is returning an attribute set, similar to an object in JSON.
1. Attributes can be nested and have similar values as in JSON.
1. Values can refer to the inputs. See [Inputs](https://devenv.sh/inputs/index.md) for how to define inputs.

`enterShell` allows you to execute bash code once the shell activates, while `env` allows you to set environment variables.

Consider using tasks instead

For more complex setup operations, consider using [tasks](https://devenv.sh/tasks/#entershell-entertest) instead of `enterShell`. Tasks provide better control over execution order, dependencies, and can run in parallel:

```
$ devenv shell
Building shell ...
Entering shell ...

hello
jq-1.6

(devenv) $ echo $GREET
hello
```

See [Nix language tutorial](https://nix.dev/tutorials/first-steps/nix-language) for a 1-2 hour deep dive that will allow you to read any Nix file.

## Environment Summary

If you'd like to print the summary of the current environment:

```
$ devenv info
...

# env
- DEVENV_DOTFILE: .../myproject/.devenv
- DEVENV_ROOT: .../myproject
- DEVENV_STATE: .../myproject/.devenv/state
- GREET: hello

# packages
- jq-1.6

# scripts

# processes
```