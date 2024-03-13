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
        packages = let
          toolchain = with inputs.fenix.packages.${system}; combine (lib.lists.flatten [
            (with stable; [cargo rustc rust-src])
            targets.wasm32-unknown-unknown.stable.rust-std
            rust-analyzer
          ]);
        in lib.lists.flatten [
          toolchain
          (with pkgs; [
            trunk
            wasm-bindgen-cli
            ((cargo-tauri.override {
              rustPlatform = makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              };
            }).overrideAttrs (old: rec {
              inherit (old) pname;
              version = "2.0.0-beta.11";
              src = fetchFromGitHub {
                owner = "tauri-apps";
                repo = "tauri";
                rev = "tauri-v${version}";
                hash = "sha256-Few8BuF2PX5BCXKeTrh6iCxVCuLoYCMpHAKnwesynNQ=";
              };
              sourceRoot = "${src.name}/tooling/cli";
              cargoDeps = old.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-${version}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-iCrLNuaOUCR6wcHsblE1It0F81c973rYqlkOdw53cDA=";
              });
              buildInputs = old.buildInputs ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.SystemConfiguration;
            }))
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
