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
  in {
    canivete.devShells.shells.default.packages = lib.flatten [
      toolchain
      cargo-tauri
      trunk
      wasm-bindgen-cli
      (optionals stdenv.isDarwin [
        darwin.libiconv
        (with darwin.apple_sdk.frameworks; [Carbon WebKit])
      ])
    ];
  };
}
