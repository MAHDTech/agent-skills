# Direnv

You can configure `devenv` to **seamlessly switch development
environments** when navigating between project directories.

This feature relies on a separate tool called
[direnv](https://direnv.net) (not to be confused with devenv).

## Installing `direnv`

2.  [Add the direnv hook to your
    shell](https://direnv.net/docs/hook.html)

## Configure shell activation

Create an `.envrc` file in your project directory with the following
content:

- [v1.4+](#tab-panel-90)
- [v1.3 and older](#tab-panel-91)

```
#!/usr/bin/env bash
eval "$(devenv direnvrc)"
# You can pass flags to the devenv command# For example: use devenv --impure --option services.postgres.enable:bool trueuse devenv
```

.envrc

```
#!/usr/bin/env bash
source_url "https://raw.githubusercontent.com/cachix/devenv/82c0147677e510b247d8b9165c54f73d32dfd899/direnvrc" "sha256-7u4iDd1nZpxL4tCzmPG0dQgC5V+/44Ba+tHkPob1v2k="
use devenv
```

.envrc

This file configures direnv to use devenv for shell activation.

## Approving and loading the shell

Once the `.envrc` file is in place, you’ll see a warning in your shell:

```
direnv: error ~/myproject/.envrc is blocked. Run `direnv allow` to approve its content
```

Run `direnv allow` to approve the `.envrc` file. This step is a security
measure to ensure you’ve reviewed the content before allowing it to
modify your shell environment.

After approval, direnv will automatically load and unload the devenv
environment whenever you enter and exit the project directory:

```
$ cd /home/user/myproject/direnv: loading ~/myproject/.envrcBuilding shell ...Entering shell ...
(devenv) $
```

Terminal window

## Passing flags to devenv

You can pass command-line options directly to devenv by adding them
after the `use devenv` command in your `.envrc` file:

```
# Example: override configuration optionsuse devenv --option services.postgres.enable:bool true
```

Terminal window

## Customizing PS1

If you’d like to use direnv and have your prompt be aware of it, we
recommend [installing Starship](https://starship.rs/guide/).

## Ignoring the `.direnv` directory

The `.direnv` directory will be added to your `.gitignore` file by
default when you run `devenv init`.

To add it manually, run:

```
echo ".direnv" >> .gitignore
```

Terminal window

## Manually managing updates to direnvrc

We occasionally make updates to our direnv integration script, also
known as the `direnvrc`.

From v1.4 and onwards, devenv will use the latest compatible version if
set up using the latest method described above in [Configure Shell
Activation](#configure-shell-activation). For older versions, the pinned
script has to be updated manually.

Pinning the `direnvrc` to a specific version from the source repository
allows you audit the `direnvrc` script and have full control over when
it is updated. The downside is that you will have to manually update the
URL and content hash of the script for every single project
individually.

The `direnvrc` can be found at:

```
https://raw.githubusercontent.com/cachix/devenv/VERSION/devenv/direnvrc
```

Replace `VERSION` with a valid git tag or branch name.

For instance, for version 1.9.2, use:

```
https://raw.githubusercontent.com/cachix/devenv/v1.9.2/devenv/direnvrc
```

To use it in your `.envrc`, first compute its sha256 hash:

```
direnv fetchurl "https://raw.githubusercontent.com/cachix/devenv/VERSION/devenv/direnvrc"
```

Terminal window

```
Found hash: <HASH>
```

Terminal window

Then modify your `.envrc`, updating the URL and inserting the computed
hash from the previous step:

```
source_url "https://raw.githubusercontent.com/cachix/devenv/VERSION/devenv/direnvrc" "<HASH>"
use devenv
```

Terminal window
