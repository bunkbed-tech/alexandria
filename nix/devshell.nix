{inputs, nix, ...}: {
  perSystem = {
    nix,
    pkgs,
    system,
    ...
  }: let
    fenix = inputs.fenix.packages.${system};
    # Stable rust toolchain with wasm support (for Tauri)
    toolchain = fenix.combine (nix.flatten [
      (with fenix.stable; [cargo rustc rust-src])
      fenix.targets.wasm32-unknown-unknown.stable.rust-std
      fenix.rust-analyzer
    ]);
    # Tauri v2
    cargo-tauri =
      (pkgs.cargo-tauri.override {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      })
      .overrideAttrs (old: rec {
        inherit (old) pname;
        version = "2.0.0-beta.11";
        src = pkgs.fetchFromGitHub {
          owner = "tauri-apps";
          repo = "tauri";
          rev = "tauri-v${version}";
          hash = "sha256-Few8BuF2PX5BCXKeTrh6iCxVCuLoYCMpHAKnwesynNQ=";
        };
        sourceRoot = "${src.name}/tooling/cli";
        cargoDeps = old.cargoDeps.overrideAttrs (nix.const {
          name = "${pname}-${version}-vendor.tar.gz";
          inherit src;
          outputHash = "sha256-iCrLNuaOUCR6wcHsblE1It0F81c973rYqlkOdw53cDA=";
        });
        buildInputs = old.buildInputs ++ nix.optional pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration;
      });
  in {
    canivete.devShell = {
      packages = nix.flatten [
        toolchain
        cargo-tauri
        (with pkgs; [
          bun
          sqlx-cli
          wasm-bindgen-cli
          (nix.optionals stdenv.isDarwin [
            darwin.libiconv
            (with darwin.apple_sdk.frameworks; [
              Carbon
              WebKit
            ])
          ])
        ])
      ];
    };
  };
}
