+++
title = "devenv-languages-java-62320f17"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "devenv"
+++

# java

## Options

### languages.java.enable

Whether to enable tools for Java development.

*Type:* boolean

*Default:*

```
false
```

*Example:*

```
true
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.gradle.enable

Whether to enable gradle.

*Type:* boolean

*Default:*

```
false
```

*Example:*

```
true
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.gradle.package

The Gradle package to use. The Gradle package by default inherits the
JDK from `languages.java.jdk.package`.

*Type:* package

*Default:*

```
pkgs.gradle.override { java = cfg.jdk.package; }
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.jdk.package

The JDK package to use. This will also become available as `JAVA_HOME`.

*Type:* package

*Default:*

```
pkgs.jdk
```

*Example:*

```
pkgs.jdk8
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.lsp.enable

Whether to enable Java Language Server.

*Type:* boolean

*Default:*

```
true
```

*Example:*

```
true
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.lsp.package

The Java language server package to use. The Java language server
package by default inherits the JDK from `languages.java.jdk.package`.

*Type:* package

*Default:*

```
pkgs.jdt-language-server.override { jdk = cfg.jdk.package; }
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.maven.enable

Whether to enable maven.

*Type:* boolean

*Default:*

```
false
```

*Example:*

```
true
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

### languages.java.maven.package

The Maven package to use. The Maven package by default inherits the JDK
from `languages.java.jdk.package`.

*Type:* package

*Default:*

```
pkgs.maven.override { jdk_headless = cfg.jdk.package; }
```

*Declared by:*

- <https://github.com/cachix/devenv/blob/main/src/modules/languages/java.nix>

