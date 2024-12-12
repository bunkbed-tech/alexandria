{
  perSystem = {
    inputs',
    lib,
    pkgs,
    self',
    ...
  }: let
    inherit (lib) const flatten optional optionals;
    inherit (pkgs) cargo-tauri makeRustPlatform fetchFromGitHub stdenv darwin trunk wasm-bindgen-cli;
    # TODO split toolchains up for dev and deployment
    # Stable rust toolchain with wasm support (for Tauri)
    toolchain = with inputs'.fenix.packages;
      combine (flatten [
        (with stable; [cargo rustc rust-src])
        targets.wasm32-unknown-unknown.stable.rust-std
        rust-analyzer
      ]);
    # Tauri v2
    cargo-tauri-v2 =
      (cargo-tauri.override {
        rustPlatform = makeRustPlatform {
          cargo = self'.packages.rust-toolchain;
          rustc = self'.packages.toolchain;
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
      });
  in {
    canivete.devShell.packages = lib.flatten [
      toolchain
      cargo-tauri-v2
      trunk
      wasm-bindgen-cli
      (optionals stdenv.isDarwin [
        darwin.libiconv
        (with darwin.apple_sdk.frameworks; [Carbon WebKit])
      ])
    ];
  };
}
