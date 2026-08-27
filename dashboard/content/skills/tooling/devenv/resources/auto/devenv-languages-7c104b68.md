+++
title = "devenv-languages-7c104b68"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# Overview

# Languages

What if you could have the tooling for any programming language by
flipping a toggle?

```
{ pkgs, ... }:
{  languages.python.enable = true;  languages.python.version = "3.11.3";
  languages.rust.enable = true;  # https://devenv.sh/reference/options/#languagesrustchannel  languages.rust.channel = "stable";}
```

devenv.nix

`devenv` will provide executables for both languages:

```
$ devenv shellBuilding shell ...Entering shell ...
(devenv) $ python --versionPython 3.11.3
```

Terminal window

