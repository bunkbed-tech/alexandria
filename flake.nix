{
  description = "Media tracker in Tauri + SolidJS";
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/release-23.11;
    flake-parts.url = github:hercules-ci/flake-parts;
    fenix.url = github:nix-community/fenix;
    systems.url = github:nix-systems/default;
  };
  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} {
    systems = import inputs.systems;
    perSystem = {config, lib, pkgs, system, ...}: {
      devShells.default = pkgs.mkShell {
        packages = lib.lists.flatten [
          inputs.fenix.packages.${system}.minimal.toolchain
          (with pkgs; [
            nodePackages.pnpm
            (lib.optionals stdenv.isDarwin (with darwin; [
              libiconv
              (with apple_sdk.frameworks; [
                Carbon
                WebKit
              ])
            ]))
          ])
        ];
      };
    };
  };
}
