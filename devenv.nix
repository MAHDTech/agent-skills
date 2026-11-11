{
  pkgs,
  config,
  lib,
  ...
}:
let
  isNative = !config.container.isBuilding;
  rustInputs = "\\.rs$|(^|/)Cargo\\.(toml|lock)$|^rust-toolchain\\.toml$|^devenv\\.(nix|yaml|lock)$";

  # Packages are installed native and in containers.
  packages = with pkgs; [
    gh
  ];

  # Dev Packages are only installed in native environments.
  devPackages = with pkgs; [
    cargo-audit
    cargo-deny
    cargo-edit
    cargo-llvm-cov
    cargo-nextest
    cargo-watch
    clang
    codeql
    convco
    figlet
    git
    hello
    jq
    mold
    pagefind
    pandoc
    ripgrep
    tailwindcss_4
    trivy
    zola
  ];

  hookBinPath = lib.makeBinPath (
    [
      config.languages.rust.toolchainPackage
      pkgs.clang
      pkgs.stdenv.cc
      pkgs.pkg-config
      pkgs.cmake
      pkgs.git
      pkgs.zola
      pkgs.tailwindcss_4
      pkgs.pagefind
      pkgs.coreutils
    ]
    ++ lib.optionals pkgs.stdenv.isLinux [ pkgs.mold ]
  );

  rustHookEnv = lib.concatStringsSep "\n" (
    lib.mapAttrsToList (name: value: ''export ${name}="''${${name}:-${toString value}}"'') (
      lib.filterAttrs (
        name: value:
        value != null
        && (
          builtins.elem name [
            "CC"
            "CXX"
            "CC_FOR_BUILD"
            "CXX_FOR_BUILD"
          ]
          || lib.hasPrefix "CARGO_TARGET_" name
        )
      ) config.env
    )
  );

  cargoHook = pkgs.writeShellScript "cargo-git-hook" ''
    export PATH="${hookBinPath}:''${PATH:-}"
    ${rustHookEnv}
    exec cargo "$@"
  '';
in
{
  name = "agent-skills";

  env = {
    AGENT_SKILLS_HOME = config.devenv.root;
    PROJECT = config.name;
    RUST_BACKTRACE = "1";
    RUST_LOG = "info";
    NEXTEST_PAGER = "cat";
  };

  cachix = lib.mkIf isNative {
    enable = true;
    pull = [
      "mahdtech"
    ];
    push = "mahdtech";
  };

  devenv = lib.mkIf isNative {
    warnOnNewVersion = true;
  };

  dotenv = lib.mkIf isNative {
    enable = true;
    disableHint = true;
  };

  packages = packages ++ lib.optionals isNative devPackages;

  enterShell = ''
    if [[ "''${CI:-false}" == "true" ]]; then
      echo "devenv running in CI"
    else
      figlet -f slant -w 180 "$(echo "$PROJECT" | tr '[:lower:]-' '[:upper:] ')"

      hello --greeting="Hello ''${USER:-user}, welcome to the $PROJECT project."

      ${lib.optionalString (config.scripts != { }) ''
        echo ""
        echo "#########################"
        echo "#### Helper scripts #####"
        echo "#########################"
        echo "🦾"
        ${lib.concatStrings (
          lib.mapAttrsToList (
            name: value:
            "printf '🦾 %-20s  %s\\n' '${name}' '${
              if value.description != null then value.description else ""
            }'\n"
          ) config.scripts
        )}
        echo "🦾"
        echo "#########################"
      ''}
    fi
    echo -e "\nAGENT_SKILLS_HOME is set to ''${AGENT_SKILLS_HOME:-(not set!)}"
  '';

  languages = {
    nix.enable = isNative;
    javascript.enable = false;
    typescript.enable = false;
    shell = {
      enable = isNative;
    };
    rust = {
      enable = isNative;
      toolchainFile = ./rust-toolchain.toml;
      mold.enable = pkgs.stdenv.isLinux;
    };
  };

  git-hooks = lib.mkIf isNative {
    excludes = [
      ".agents/"
      ".devenv/"
      "^\\.github/badges/"
      "^\\.github/crap/"
      "^.vscode/"
      "^node_modules/"
      "^dashboard/content/"
      "^dashboard/public/"
      "^dashboard/static/build/"
      "^dashboard/static/pagefind/"
      "/resources/"
      "^\\.cache/"
      "CHANGELOG.md"
    ];
    hooks = {
      action-validator = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      actionlint = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      cargo-check = {
        enable = true;
        entry = lib.mkForce "${cargoHook} check";
        files = rustInputs;
        args = [
          "--workspace"
          "--all-targets"
          "--locked"
        ];
      };
      check-json = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      check-merge-conflicts = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      check-shebang-scripts-are-executable = {
        enable = true;
        excludes = [
          "^skills/vendor/"
          "\\.rs$"
        ];
      };
      check-symlinks.enable = true;
      check-yaml = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      clippy = {
        enable = true;
        entry = lib.mkForce "${cargoHook} clippy --workspace --all-targets --locked";
        files = rustInputs;
      };
      commitizen.enable = true;
      convco = {
        enable = true;
        settings = {
          configPath = ".versionrc";
        };
      };
      cspell = {
        enable = true;
        excludes = [
          "^skills/vendor/"
          "skills(-archive)?/tooling/opencode-acp/.*\\.md$"
          "^README\\.md$"
          "^agents/AGENTS\\.md$"
          "^skills\\.sh\\.json$"
          "^dashboard/static/\\.well-known/"
          "\\.versionrc$"
        ];
        args = [
          "lint"
          "--no-must-find-files"
        ];
      };

      dashboard-lint = {
        enable = true;
        name = "Dashboard Lint";
        entry = "${cargoHook} run -p ask-cli -- dashboard lint";
        files = "^(skills(-archive)?/.*\\.md$|dashboard/(content|themes|css)/|dashboard/config\\.toml$)";
        excludes = [ ];
        always_run = true;
        pass_filenames = false;
        require_serial = true;
      };
      deadnix.enable = true;
      editorconfig-checker = {
        enable = true;
        excludes = [
          "\\.lock$"
          "^skills/vendor/"
        ];
      };
      lychee = {
        enable = true;
        excludes = [
          "\\.lock$"
          "^\\.out-of-scope/"
          "^dashboard/"
          "^skills/vendor/"
          "^skills(-archive)?/tooling/opencode-acp/"
        ];
      };
      markdownlint = {
        enable = true;
        excludes = [
          "^skills/vendor/"
          "^README\\.md$"
          "^agents/AGENTS\\.md$"
        ];
        settings = {
          configuration = {
            MD013 = false;
            MD025 = false;
            MD036 = false;
            MD041 = false;
            MD051 = false;
            MD033 = {
              allowed_elements = [
                "a"
                "b"
                "br"
                "h3"
                "nobr"
                "pre"
                "sup"
                "summary"
                "details"
                "ParamField"
                "Expandable"
                "Warning"
                "ResponseField"
                "span"
                "Card"
                "Note"
                "Info"
                "Steps"
                "Step"
                "Icon"
                "img"
                "mandate"
                "constraints"
                "instructions"
                "exit_criteria"
              ];
            };
          };
        };
      };
      mixed-line-endings = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      nixfmt = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      prettier = {
        enable = true;
        excludes = [
          "^skills/vendor/"
          "\\.html$"
          "README\\.md$"
          "^agents/AGENTS\\.md$"
          "^skills\\.sh\\.json$"
          "^dashboard/static/\\.well-known/"
          "\\.devcontainer\\.json$"
          "\\.devcontainer/devcontainer\\.json$"
        ];
      };
      ripsecrets.enable = true;
      rustfmt = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      shellcheck = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      shfmt = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
      };
      skills-test = {
        enable = true;
        name = "Skills Lint";
        entry = "${cargoHook} run -p ask-cli -- skills lint";
        files = "^skills/.*\\.md$";
        pass_filenames = false;
      };
      skills-sync = {
        enable = true;
        name = "Skills Sync";
        entry = "env SKILLS_REPO_ONLY=1 SKILLS_SKIP_DASHBOARD=1 ${cargoHook} run -p ask-cli -- skills sync";
        files = "SKILL\\.md$";
        pass_filenames = false;
        require_serial = true;
      };
      trim-trailing-whitespace = {
        enable = true;
        excludes = [
          "^skills/vendor/"
        ];
      };
      trufflehog = {
        enable = true;
        excludes = [
        ];
      };
      yamllint = {
        enable = true;
        excludes = [ "^skills/vendor/" ];
        settings = {
          configuration = ''
            extends: relaxed
            rules:
              line-length: disable
              indentation: enable
          '';
        };
      };
    };
  };

  starship = {
    enable = isNative;
    config.enable = false;
  };

  devcontainer = {
    enable = isNative;
    settings = {
      customizations = {
        vscode = {
          extensions = [
          ];
        };
      };
    };
  };

  scripts = {
    ask = {
      description = "Unified ask CLI management tool";
      exec = ''
        if [ -f Cargo.toml ]; then
          cargo run -p ask-cli -- "$@"
        else
          echo "ask-cli not yet available; Cargo.toml not found"
          exit 1
        fi
      '';
    };
    skills = {
      description = "Manage agent skills (usage: skills --action <lint|sync|install|uninstall|download-resources|clean-resources>)";
      exec = ''
        cargo run -p ask-cli -- skills "$@"
      '';
    };
    dashboard = {
      description = "Manage the dashboard (usage: dashboard --action <build|serve|css|lint>)";
      exec = ''
        cargo run -p ask-cli -- dashboard "$@"
      '';
    };
    codeql-run = {
      package = pkgs.bash;
      description = "Run CodeQL static analysis locally.";
      exec = ''
        ./scripts/codeql-run.sh "$@"
      '';
    };
    crap = {
      description = "Run cargo-crap analysis";
      exec = ''
        cargo llvm-cov --workspace --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info "$@"
      '';
    };
  };

  enterTest = ''
    ./scripts/run-tests.sh
  '';
}
