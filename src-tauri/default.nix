{config, ...}: let
  inherit (config.alexandria) postgres;
in {
  perSystem = {inputs', nix, pkgs, self', ...}: {
    packages.alexandria-migrations = let
      inherit (inputs'.fenix.packages.stable) toolchain;
      sqlx-cli = pkgs.sqlx-cli.override {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      };
      migrations = pkgs.linkFarm "migrations" {migrations = ./migrations;};
      srcs = [toolchain sqlx-cli migrations];
    in pkgs.wrapProgram srcs "migrate" "cargo" "--add-flags \"sqlx migrate run\"" {
      passthru.local = pkgs.wrapFlags self'.packages.alexandria-migrations "--set DATABASE_URL=${postgres.local.url}";
    };
    canivete = {
      arion.modules.tauri = {self'', ...}: {
        services.migrations = {
          image.contents = [self''.packages.alexandria-migrations];
          image.command = ["migrate"];
          service.depends_on.postgres.condition = "service_healthy";
          service.environment.DATABASE_URL = postgres.dev.url;
        };
      };
      process-compose.services.settings.processes = {
        migrations.command = nix.getExe self'.packages.alexandria-migrations.local;
        migrations.depends_on.postgres.condition = "process_healthy";
        tauri.command = nix.getExe self'.packages.alexandria-tauri.local;
        tauri.depends_on.migrations.condition = "process_completed_successfully";
        tauri.readiness_probe.exec.command = "exec 3<>/dev/tcp/localhost/5173";
      };
      dream2nix.packages.alexandria-tauri.module = {config, dream2nix, ...}: let
        buildInputs = with config.deps; nix.optionals stdenv.isDarwin [iconv SystemConfiguration];
      in {
        imports = with dream2nix.modules.dream2nix; [rust-cargo-lock rust-crane];
        paths.package = ./.;
        deps = {nixpkgs, ...}: {
          inherit (nixpkgs) iconv;
          inherit (nixpkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
        };
        name = "alexandria-tauri";
        version = "0.0.1";
        mkDerivation.src = ./.;
        mkDerivation.buildInputs = buildInputs;
        rust-crane.depsDrv.mkDerivation.buildInputs = buildInputs;
        public.local = pkgs.wrapFlags self'.packages.alexandria-tauri "--set DATABASE_URL=${postgres.local.url}";
      };
    };
  };
}
