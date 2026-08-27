# Basics

Given a hello world example, click on the end of each line to get an
explanation:

```
{ pkgs, ... }: # ``devenv.nix`` is a function with inputs. `pkgs` is an [input](https://devenv.sh/inputs/) passed as a special argument to the function.
{ # Our function is returning an attribute set, similar to an object in JSON.  env.GREET = "hello"; # Attributes can be nested and have similar values as in JSON.
  packages = [ pkgs.jq ];
  enterShell = ''    echo $GREET    jq --version  ''; # Values can refer to the inputs. See [Inputs](https://devenv.sh/inputs/) for how to define inputs.}
```

devenv.nix

We use a special input `...` at the end as a catch-all to avoid
enumerating all of the inputs. 2. Our function is returning an attribute
set, similar to an object in JSON. 3. Attributes can be nested and have
similar values as in JSON. 4. Values can refer to the inputs. See
[Inputs](https://devenv.sh/inputs/) for how to define inputs.

`enterShell` allows you to execute bash code once the shell activates,
while `env` allows you to set environment variables.

```
$ devenv shellBuilding shell ...Entering shell ...
hellojq-1.6
(devenv) $ echo $GREEThello
```

Terminal window

See [Nix language
tutorial](https://nix.dev/tutorials/first-steps/nix-language) for a 1-2
hour deep dive that will allow you to read any Nix file.

## Environment Summary

If you’d like to print the summary of the current environment:

```
$ devenv info...
# env- DEVENV_DOTFILE: .../myproject/.devenv- DEVENV_ROOT: .../myproject- DEVENV_STATE: .../myproject/.devenv/state- GREET: hello
# packages- jq-1.6
# scripts
# processes
```

Terminal window
