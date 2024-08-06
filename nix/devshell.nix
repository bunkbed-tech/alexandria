{
  inputs,
  nix,
  ...
}: {
  perSystem = {
    pkgs,
    self',
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
  in {
    # Tauri v2
    packages.cargo-tauri =
      (pkgs.cargo-tauri.override {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      })
      .overrideAttrs (old: rec {
        inherit (old) pname;
        version = "2.0.0-rc.1";
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
          outputHash = "sha256-lc8wQoax8dhYp9QCNC8DODOaca2P9AOMC8qn0pNDlic=";
        });
        buildInputs = old.buildInputs ++ nix.optional pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration;
      });
    canivete.devShell = {
      packages = nix.flatten [
        toolchain
        self'.packages.cargo-tauri
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
