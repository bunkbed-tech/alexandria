{config, ...}: let
  inherit (config.alexandria) postgres;
in {
  perSystem = {inputs', pkgs, self', ...}: {
    packages.alexandria-migrations = let
      inherit (inputs'.fenix.packages.stable) toolchain;
      sqlx-cli = pkgs.sqlx-cli.override {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      };
      migrations = pkgs.linkFarm "migrations" {migrations = ../src-tauri/migrations;};
      srcs = [toolchain sqlx-cli migrations];
    in
      pkgs.wrapProgram srcs "migrate" "cargo" "--add-flags \"sqlx migrate run\"" {
        passthru.local = pkgs.wrapFlags self'.packages.alexandria-migrations "--set DATABASE_URL=${postgres.local.url}";
      };
    canivete.dream2nix.packages.alexandria-tauri-frontend-modules.module = {config, lib, dream2nix, ...}: let
      inherit (config.deps) bun;
    in {
      imports = [dream2nix.modules.dream2nix.mkDerivation];
      paths.package = ../.;
      deps = {nixpkgs, ...}: {
        inherit (nixpkgs) bun;
      };
      name = "alexandria-tauri-frontend-node_modules";
      version = "0.0.1";
      mkDerivation = {
        src = ../.;
        nativeBuildInputs = [bun];
        buildPhase = "bun install --no-progress --frozen-lockfile";
        installPhase = ''
          mkdir -p $out/node_modules
          cp -R ./node_modules $out
        '';
        outputHash = "VnmqR4SkIBATXVfz4wana9pvzP8ObQ+yldTobklE2lw=";
        outputHashAlgo = "sha256";
        outputHashMode = "recursive";
      };
    };
    # TODO fix hyper-tls vendoring issue
    canivete.dream2nix.packages.alexandria-tauri.module = {
      config,
      dream2nix,
      lib,
      ...
    }: let
      inherit (config.deps) bun iconv stdenv wrapFlags SystemConfiguration;
      buildInputs = lib.optionals stdenv.isDarwin [iconv SystemConfiguration];
      node_modules = self'.packages.alexandria-tauri-frontend-modules;
    in {
      imports = with dream2nix.modules.dream2nix; [rust-cargo-lock buildRustPackage];
      paths.package = ../.;
      deps = {nixpkgs, ...}: {
        inherit (nixpkgs) bun iconv stdenv wrapFlags;
        inherit (nixpkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
      };
      name = "alexandria-tauri";
      version = "0.0.1";
      mkDerivation = {
        src = ../.;
        nativeBuildInputs = [bun self'.packages.cargo-tauri];
        inherit buildInputs;
        configurePhase = "cp -R ${node_modules}/node_modules .";
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
  };
}
