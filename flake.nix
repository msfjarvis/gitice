{
  description = "gitice";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  inputs.systems.url = "github:msfjarvis/flake-systems";

  inputs.advisory-db.url = "github:rustsec/advisory-db";
  inputs.advisory-db.flake = false;

  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";

  inputs.devshell.url = "github:numtide/devshell";
  inputs.devshell.inputs.nixpkgs.follows = "nixpkgs";
  inputs.devshell.inputs.flake-utils.follows = "flake-utils";

  inputs.fenix.url = "github:nix-community/fenix";
  inputs.fenix.inputs.nixpkgs.follows = "nixpkgs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.systems.follows = "systems";

  inputs.flake-compat.url = "github:nix-community/flake-compat";
  inputs.flake-compat.flake = false;

  outputs = {
    self,
    nixpkgs,
    advisory-db,
    crane,
    devshell,
    fenix,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [devshell.overlays.default];
      };

      rustNightly = (import fenix {inherit pkgs;}).fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-3+hzpuIkf2KFGyop0DgMPI5nCY6IQX8I6XdIUwn4nAI=";
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustNightly;
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
        inherit gitice gitice-clippy gitice-fmt gitice-nextest;
        # TODO: Re-enable once https://github.com/NixOS/nixpkgs/issues/288064 is fixed
        # inherit gitice-audit;
      };

      packages.default = gitice;

      apps.default = flake-utils.lib.mkApp {drv = gitice;};

      devShells.default = pkgs.devshell.mkShell {
        bash = {interactive = "";};

        env = [
          {
            name = "DEVSHELL_NO_MOTD";
            value = 1;
          }
        ];

        packages = with pkgs; [
          cargo-dist
          cargo-nextest
          cargo-release
          git-cliff
          rustNightly
          stdenv.cc
        ];
      };
    });
}
