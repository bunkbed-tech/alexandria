{
  description = "Media tracker in Tauri + SolidJS";
  inputs = {
    canivete.url = github:schradert/canivete;
    # Compose rust toolchains
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "canivete/nixpkgs";
  };
  outputs = inputs:
    inputs.canivete.lib.mkFlake {inherit inputs;} {
      perSystem = {
        nix,
        pkgs,
        system,
        ...
      }:
        with nix; {
          canivete.pre-commit.rust.enable = true;
          devShells.alexandria = pkgs.mkShell {
            packages = let
              toolchain = with inputs.fenix.packages.${system};
                combine (flatten [
                  (with stable; [cargo rustc rust-src])
                  targets.wasm32-unknown-unknown.stable.rust-std
                  rust-analyzer
                ]);
            in
              flatten [
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
                      cargoDeps = old.cargoDeps.overrideAttrs (const {
                        name = "${pname}-${version}-vendor.tar.gz";
                        inherit src;
                        outputHash = "sha256-iCrLNuaOUCR6wcHsblE1It0F81c973rYqlkOdw53cDA=";
                      });
                      buildInputs = old.buildInputs ++ optional stdenv.isDarwin darwin.apple_sdk.frameworks.SystemConfiguration;
                    }))
                  (optionals stdenv.isDarwin (with darwin; [
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
