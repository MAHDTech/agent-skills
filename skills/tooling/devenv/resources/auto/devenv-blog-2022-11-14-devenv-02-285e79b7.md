# devenv 0.2

Nov 14, 2022

[![Domen
Kožar](https://github.com/domenkozar.png)](https://github.com/domenkozar)

Domen Kožar

After an intense weekend and lots of incoming contributions, `v0.2` is
out!

# Highlights

- All the `devenv.nix` options you can define now come as an input
  (instead of being packaged with each devenv release). To update the
  options you can run `devenv update` and it will match [devenv.nix
  reference](https://devenv.sh/reference/options/).

- New `devenv search` command:

```
$ devenv search ncduname         version  descriptionpkgs.ncdu    2.1.2    Disk usage analyzer with an ncurses interfacepkgs.ncdu_1  1.17     Disk usage analyzer with an ncurses interfacepkgs.ncdu_2  2.1.2    Disk usage analyzer with an ncurses interface
Found 3 results.
```

Terminal window

- [shyim](https://github.com/shyim) contributed Redis support and is
  working on MySQL.

- Languages: [raymens](https://github.com/raymens) contributed dotnet,
  [ankhers](https://github.com/ankhers) contributed Elixir and Erlang
  support.

- If `devenv.local.nix` exists it’s now also loaded, allowing you to
  override git committed `devenv.nix` with local changes. Hurrah
  composability!

# Bug fixes

- Variables like `env.DEVENV_ROOT`, `env.DEVENV_STATE` and
  `env.DEVENV_DOTFILE` are now absolute paths paths
- [shyim](https://github.com/shyim) fixed `/dev/stderr` that is in some
  environments not available.
- [domen](https://github.com/domenkozar) fixed shell exiting on non-zero
  exit status code.

Domen
