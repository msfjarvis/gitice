{
  description = "gitice";

  inputs = {
    nixpkgs = {url = "github:NixOS/nixpkgs/nixpkgs-unstable";};

    flake-utils = {url = "github:numtide/flake-utils";};

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-compat.follows = "flake-compat";
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustStable =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustStable;
      src = craneLib.cleanCargoSource ./.;
      nativeBuildInputs = [];
      buildInputs = [];
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src buildInputs nativeBuildInputs;
      };

      commonArgs = {
        inherit src cargoArtifacts buildInputs nativeBuildInputs;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      };

      gitice = craneLib.buildPackage (commonArgs // {doCheck = false;});
      gitice-clippy = craneLib.cargoClippy (commonArgs // {});
      gitice-fmt = craneLib.cargoFmt (commonArgs // {});
      gitice-audit =
        craneLib.cargoAudit (commonArgs // {inherit advisory-db;});
      gitice-nextest = craneLib.cargoNextest (commonArgs
        // {
          partitions = 1;
          partitionType = "count";
        });
    in {
      checks = {
        inherit gitice gitice-audit gitice-clippy gitice-fmt gitice-nextest;
      };

      packages.default = gitice;

      apps.default = flake-utils.lib.mkApp {drv = gitice;};

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        nativeBuildInputs = with pkgs; [cargo-dist cargo-nextest cargo-release rustStable];

        CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse";
      };
    });
}
