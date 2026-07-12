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
    shell = {
      enable = isNative;
    };
  };

  git-hooks = lib.mkIf isNative {
    excludes = [
      ".agents/"
      ".devenv/"
      ".git/"
      "^.vscode/"
      "^node_modules/"
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
      convco = {
        enable = true;
        settings = {
          configPath = ".versionrc";
        };
      };
      cspell = {
        enable = true;
        excludes = [
          "skills/tooling/opencode-acp/.*\\.md$"
          "dashboard/content/.*\\.md$"
          "^README\\.md$"
          "^agents/AGENTS\\.md$"
          "\\.versionrc$"
        ];
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
      lychee = {
        enable = true;
        excludes = [
        ];
      };
      markdownlint = {
        enable = true;
        excludes = [
          "dashboard/content/.*\\.md$"
          "skills/tooling/opencode-acp/.*\\.md$"
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
      mixed-line-endings.enable = true;
      nixfmt = {
        enable = true;
        package = pkgs.nixfmt-rfc-style;
      };
      prettier = {
        enable = true;
        excludes = [
          "\\.html$"
          "dashboard/content/.*\\.md$"
          "README\\.md$"
          "^agents/AGENTS\\.md$"
          "^skills\\.sh\\.json$"
          "\\.devcontainer\\.json$"
          "\\.devcontainer/devcontainer\\.json$"
        ];
      };
      ripsecrets.enable = true;
      shellcheck.enable = true;
      shfmt.enable = true;
      skills-lint = {
        enable = true;
        name = "Skills Linter";
        entry = "skills --action lint";
        files = "SKILL\\.md$";
        pass_filenames = false;
      };
      skills-sync = {
        enable = true;
        name = "Skills Sync";
        entry = "env SKILLS_REPO_ONLY=1 skills --action sync";
        files = "SKILL\\.md$";
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
    skills = {
      description = "Manage agent skills (usage: skills --action <lint|sync|install|uninstall>)";
      exec = "bun run skills \"$@\"";
    };
    dashboard = {
      description = "Manage the dashboard (usage: dashboard --action <build|serve|css>)";
      exec = "bun run dashboard \"$@\"";
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
