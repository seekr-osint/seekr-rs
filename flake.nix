{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        parts.follows = "flake-parts";
        treefmt.follows = "treefmt-nix";
      };
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:semnix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-compat.follows = "flake-compat";
    };
  };

  outputs = inputs @ {
    flake-parts,
    nci,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      imports = [
        inputs.nci.flakeModule
        inputs.treefmt-nix.flakeModule
        inputs.pre-commit-hooks.flakeModule
      ];
      perSystem = {
        config,
        pkgs,
        self',
        system,
        ...
      }: let
        fromTreefmtFile = {
          toFilter ? [],
          path ? ./treefmt.nix,
          extraHooks ? {},
        }: let
          treefmt' = import path;

          # treefmt
          pre-commitFormatters = builtins.attrNames inputs.pre-commit-hooks.packages.${system};
          programs =
            treefmt'.programs
            // (
              builtins.mapAttrs
              # add the package from pre-commit-hooks to the formatter
              (n: v: {
                inherit (v) enable;
                package = inputs.pre-commit-hooks.packages.${system}.${n};
              })
              (pkgs.lib.filterAttrs (n: _v: (builtins.elem n pre-commitFormatters)) treefmt'.programs) # attrSet of the formatters which pre-commit-hooks has a package to use
            );
          treefmtCfg = treefmt' // {inherit programs;};

          hooksFromTreefmtFormatters =
            builtins.mapAttrs
            (_n: v: {inherit (v) enable;}) (pkgs.lib.filterAttrs (n: _v: (!builtins.elem n toFilter)) treefmt'.programs);
        in {
          treefmt =
            treefmtCfg;
          pre-commit = {
            settings = {
              src = ./.;
              hooks =
                hooksFromTreefmtFormatters // extraHooks;
            };
          };
        };
      in {
        inherit
          (fromTreefmtFile {
            toFilter = ["yamlfmt"];
            path = ./treefmt.nix;
          })
          treefmt
          pre-commit
          ;

        nci = {
          projects = {
            seekr = {
              path = ./.;
              export = true;
              drvConfig = {
                env = {
                  DATABASE_URL = "sqlite:db/dev.db";
                };
              };
            };
          };
          # crates = {
          #   seekr = {
          #     export = true;
          #     drvConfig = {
          #       env = {
          #       };
          #     };
          #   };
          # };
          toolchainConfig = ./rust-toolchain.toml;
        };
        packages = {
          default = self'.packages.seekr-release;
          toolchain = config.nci.toolchains.shell;
        };
        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
          nativeBuildInputs = with pkgs; [
            config.nci.toolchains.shell
            sqlite
            sqlx-cli
          ];
        };
      };
    };
}
