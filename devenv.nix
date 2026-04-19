{
  pkgs,
  config,
  lib,
  ...
}:
let
  isNative = !config.container.isBuilding;

  # Packages are installed native and in containers.
  packages = [ ];

  # Dev Packages are only installed in native environments.
  devPackages = with pkgs; [
    codeql
    figlet
    git
    hello
    jq
    pagefind
    ripgrep
    tailwindcss_4
    trivy
    zola
  ];
in
{
  name = "agent-skills";

  env = {
    AGENT_SKILLS_HOME = config.devenv.root;
    PROJECT = config.name;
  };

  cachix = lib.mkIf isNative {
    enable = true;
    pull = [
      "MAHDTech"
      "devenv"
      "nix-community"
      "pre-commit-hooks"
    ];
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
    if [[ "${"CI:-false"}" == "true" ]];
    then
      echo "devenv running in CI"
    else
      # showfigfonts 2>/dev/null | less
      figlet -f slant -w 180 "$(echo "$PROJECT" | tr '[:lower:]-' '[:upper:] ')"

      hello --greeting="Hello ''${USER:-user}, welcome to the $PROJECT project!"

      echo -e "\nAGENT_SKILLS_HOME is set to ''${AGENT_SKILLS_HOME:-(not set!)}"

      echo ""
      echo "#########################"
      echo "#### Helper scripts #####"
      echo "#########################"
      echo "🦾"
      ${lib.concatStrings (
        lib.mapAttrsToList (
          name: value: "printf '🦾 %-20s  %s\\n' '${name}' '${value.description or ""}'\n"
        ) config.scripts
      )}
      echo "🦾"
      echo "#########################"
    fi
  '';

  languages = {
    nix.enable = isNative;
    javascript = {
      enable = isNative;
      bun = {
        enable = true;
        install = {
          enable = true;
        };
      };
      npm = {
        enable = false;
      };
      lsp = {
        enable = true;
      };
    };
    typescript = {
      enable = isNative;
      lsp = {
        enable = true;
      };
    };
    shell.enable = isNative;
  };

  git-hooks = lib.mkIf isNative {
    excludes = [
      ".devenv/"
      ".git/"
      "^.vscode/"
      ".agents/"
    ];
    hooks = {
      action-validator.enable = true;
      actionlint.enable = true;
      check-json.enable = true;
      check-merge-conflicts.enable = true;
      check-shebang-scripts-are-executable = {
        enable = true;
        excludes = [
          "\\.rs$"
        ];
      };
      check-symlinks.enable = true;
      check-yaml.enable = true;
      commitizen.enable = true;
      cspell = {
        enable = true;
        args = [
          "lint"
          "--no-must-find-files"
        ];
      };
      deadnix.enable = true;
      editorconfig-checker.enable = true;
      eslint = {
        enable = true;
        settings = {
          extensions = "\\.js$|\\.ts$";
        };
      };
      markdownlint = {
        enable = true;
        excludes = [
          "dashboard/content/skills/.*\\.md$"
          "dashboard/content/_index\\.md$"
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
              ];
            };
          };
        };
      };
      mixed-line-endings.enable = true;
      nixfmt.enable = true;
      prettier = {
        enable = true;
        excludes = [
          "\\.html$"
          "dashboard/content/skills/.*\\.md$"
          "dashboard/content/_index\\.md$"
          "README\\.md$"
        ];
      };
      ripsecrets.enable = true;
      shellcheck.enable = true;
      shfmt.enable = true;
      skills-lint = {
        enable = true;
        name = "Skills Linter";
        entry = "skills --action lint";
        files = "(SKILL|COMMAND)\\.md$";
        pass_filenames = false;
      };
      skills-sync = {
        enable = true;
        name = "Skills Sync";
        entry = "skills --action sync";
        files = "(SKILL|COMMAND)\\.md$";
        pass_filenames = false;
      };
      trim-trailing-whitespace = {
        enable = true;
        excludes = [
        ];
      };
      trufflehog = {
        enable = true;
        excludes = [
        ];
      };
      tsc = {
        enable = true;
        name = "TypeScript Type Check";
        entry = "tsc --noEmit --project tsconfig.json";
        files = "\\.ts$";
        pass_filenames = false;
      };
      yamllint = {
        enable = true;
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
    setup = {
      description = "Set up the agent skills environment";
      exec = "bun run setup";
    };
    skills = {
      description = "Manage agent skills (usage: skills --action <lint|sync|install>)";
      exec = "bun run skills \"$@\"";
    };
    build-css = {
      description = "Build CSS";
      exec = "bun run build:css";
    };
    build-dashboard = {
      description = "Build dashboard";
      exec = "bun run build:dashboard";
    };
    serve-dashboard = {
      description = "Serve dashboard locally";
      exec = "bun run serve:dashboard";
    };
    codeql-run = {
      package = pkgs.bash;
      description = "Run CodeQL static analysis locally.";
      exec = ''
        ./scripts/codeql-run.sh "$@"
      '';
    };
  };

  enterTest = ''
    echo "Running devenv tests..."
  '';
}
