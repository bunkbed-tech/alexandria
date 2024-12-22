{config, ...}: let
  inherit (config.alexandria) postgres;
in {
  perSystem = {
    inputs',
    lib,
    pkgs,
    self',
    ...
  }: let
    inherit (lib) flatten getExe optionals;
    inherit (pkgs) cargo-tauri stdenv darwin wasm-bindgen-cli;
    # TODO split toolchains up for dev and deployment
    # Stable rust toolchain with wasm support (for Tauri)
    toolchain = with inputs'.fenix.packages;
      combine (flatten [
        (with stable; [cargo rustc rust-src])
        targets.wasm32-unknown-unknown.stable.rust-std
        rust-analyzer
      ]);
  in {
    # TODO fix hyper-tls vendoring issue
    canivete.dream2nix.packages.desktop.module = {
      config,
      dream2nix,
      ...
    }: let
      inherit (config.deps) iconv stdenv wrapFlags SystemConfiguration;
      inherit (self'.packages) web;
      buildInputs = optionals stdenv.isDarwin [iconv SystemConfiguration];
    in {
      imports = with dream2nix.modules.dream2nix; [rust-cargo-lock buildRustPackage];
      paths.package = ../.;
      deps = {nixpkgs, ...}: {
        inherit (nixpkgs) cargo-tauri iconv stdenv wrapFlags;
        inherit (nixpkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
      };
      name = "alexandria-desktop";
      version = "0.0.1";
      mkDerivation = {
        src = ../.;
        # TODO is this correct?
        patchPhase = "cp ${web} frontend/web/dist";
        nativeBuildInputs = [cargo-tauri];
        inherit buildInputs;
        buildPhase = "DATABASE_URL=${postgres.local.url} cargo tauri build --bundles app";
        installPhase = ''
          mkdir -p $out/bin
          cp -r target/release/bundle/macos $out/Applications
          mv $out/Applications/Alexandria.app/Contents/MacOS/Alexandria $out/bin/alexandria
          ln -s $out/bin/alexandria $out/Applications/Alexandria.app/Contents/MacOS/Alexandria
        '';
      };
      public.local = wrapFlags self'.packages.alexandria-tauri "--set DATABASE_URL=${postgres.local.url}";
    };

    canivete.devShells.shells.default.packages = flatten [
      toolchain
      cargo-tauri
      wasm-bindgen-cli
      (optionals stdenv.isDarwin [
        darwin.libiconv
        (with darwin.apple_sdk.frameworks; [Carbon WebKit])
      ])
    ];

    canivete.process-compose.services.settings.processes.desktop = {
      command = getExe self'.packages.desktop.local;
      depends_on.migrate.condition = "process_completed_successfully";
      readiness_probe.exec.command = "exec 3<>/dev/tcp/localhost/5173";
    };
  };
}
