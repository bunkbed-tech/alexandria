{
  description = "Media tracker in Tauri + SolidJS";
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs;
    flake-parts.url = github:hercules-ci/flake-parts;
    fenix.url = github:nix-community/fenix;
    systems.url = github:nix-systems/default;
  };
  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} {
    systems = import inputs.systems;
    perSystem = {config, lib, pkgs, system, ...}: {
      packages.alexandria = let
        rust-minimal = inputs.fenix.packages.${system}.minimal.toolchain;
        platform = pkgs.makeRustPlatform {
          cargo = rust-minimal;
          rustc = rust-minimal;
        };
      in platform.buildRustPackage {
        pname = "alexandria";
        version = "0.0.1";
        src = ./alexandria/src-tauri;
        cargoLock.lockFile = ./alexandria/src-tauri/Cargo.lock;
        # TODO override the carbon headers because they seem to not show up at all
        propagatedBuildInputs = lib.optionals pkgs.stdenv.isDarwin [pkgs.darwin.CarbonHeaders];
      };
      devShells.default = pkgs.mkShell {
        packages = lib.lists.flatten [
          (with config.packages; [alexandria])
          (with inputs.fenix.packages.${system}; [
            complete.toolchain
            rust-analyzer
            rust-analyzer-vscode-extension
          ])
          (with pkgs; [nodePackages.pnpm])
        ];
      };
    };
  };
}
