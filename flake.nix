{
  description = "Media tracker in Tauri + SolidJS";
  inputs = {
    # Base package registries
    nixpkgs.url = github:nixos/nixpkgs/release-23.11;
    nixpkgs-unstable.url = github:nixos/nixpkgs/nixos-unstable;

    # Modularize repo at flake level
    flake-parts.url = github:hercules-ci/flake-parts;
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs-unstable";

    # Upstream system types
    systems.url = github:nix-systems/default;

    # Compose rust toolchains
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "nixpkgs-unstable";

    # Configure git hooks
    pre-commit-hooks-nix.url = github:cachix/pre-commit-hooks.nix;
    pre-commit-hooks-nix.inputs.nixpkgs-stable.follows = "nixpkgs";
    pre-commit-hooks-nix.inputs.nixpkgs.follows = "nixpkgs-unstable";
  };
  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [inputs.pre-commit-hooks-nix.flakeModule];
      systems = import inputs.systems;
      perSystem = {
        config,
        lib,
        pkgs,
        system,
        ...
      }: {
        pre-commit.settings.default_stages = ["push" "manual"];
        pre-commit.settings.hooks = {
          alejandra.enable = true;
          typos.enable = true;
          commitizen.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
        };
        devShells.default = pkgs.mkShell {
          inputsFrom = [config.pre-commit.devShell];
          packages = let
            toolchain = with inputs.fenix.packages.${system};
              combine (lib.lists.flatten [
                (with stable; [cargo rustc rust-src])
                targets.wasm32-unknown-unknown.stable.rust-std
                rust-analyzer
              ]);
          in
            lib.lists.flatten [
              toolchain
              (with pkgs; [
                trunk
                wasm-bindgen-cli
                ((cargo-tauri.override {
                    rustPlatform = makeRustPlatform {
                      cargo = toolchain;
                      rustc = toolchain;
                    };
                  })
                  .overrideAttrs (old: rec {
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
