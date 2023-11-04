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
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.parts.follows = "flake-parts";
      inputs.treefmt.follows = "treefmt-nix";
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
      systems =  [ 
        "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ]; 
      imports = [
        inputs.nci.flakeModule
        inputs.treefmt-nix.flakeModule
        ./crates.nix
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs."seekr";
      in {
        treefmt = import ./treefmt.nix;
        # export the crate devshell as the default devshell
        devShells.default = crateOutputs.devShell;
        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;
      };
    };
}
