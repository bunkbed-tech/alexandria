{
  config,
  lib,
  ...
}: let
  inherit (lib) getExe mergeAttrs mkOption types;
  inherit (types) str int attrsOf submodule;
  inherit (config.alexandria) postgres;
in {
  options.alexandria.postgres = mkOption {
    type = attrsOf (submodule ({config, ...}: {
      options = {
        protocol = mkOption {
          type = str;
          default = "postgresql";
          description = "Protocol for communication with database from Python runtime";
        };
        username = mkOption {
          type = str;
          default = "postgres";
          description = "Postgres username";
        };
        password = mkOption {
          type = str;
          description = "Postgres password";
        };
        hostname = mkOption {
          type = str;
          description = "Postgres server hostname";
        };
        port = mkOption {
          type = int;
          default = 5432;
          description = "Postgres port";
        };
        database = mkOption {
          type = str;
          default = "postgres";
          description = "Postgres database";
        };
        url = mkOption {
          type = str;
          default = with config; "${protocol}://${username}:${password}@${hostname}:${toString port}/${database}";
          description = "Full connection string to database";
        };
      };
    }));
  };
  config.alexandria.postgres = let
    common.username = "tester";
    common.database = "testing";
    common.password = "password";
  in {
    local = mergeAttrs common {hostname = "127.0.0.1";};
    dev = mergeAttrs common {hostname = "postgres";};
  };
  config.perSystem = {
    inputs',
    pkgs,
    self',
    ...
  }: {
    canivete = {
      # TODO convert this to nixos postgres
      arion.projects.arion.modules.database = {self'', ...}: {
        services.migrate = {
          image.contents = [self''.packages.migrate];
          image.command = ["migrate"];
          service.depends_on.postgres.condition = "service_healthy";
          service.environment.DATABASE_URL = config.alexandria.postgres.dev.url;
        };
        services.postgres.service = with postgres.dev; {
          image = "postgres:16.3";
          environment.POSTGRES_PASSWORD = password;
          environment.POSTGRES_USER = username;
          environment.POSTGRES_DB = database;
          healthcheck = {
            test = ["CMD" "pg_isready" "-U" username "-d" database];
            interval = "5s";
            timeout = "5s";
            start_period = "5s";
            retries = 5;
          };
          ports = ["${toString port}:${toString port}"];
        };
      };
      process-compose.services = {
        services.postgres.postgres = with postgres.local; {
          enable = true;
          inherit port;
          initialScript.before = "CREATE USER ${username} WITH PASSWORD '${password}';";
          initialDatabases = [{name = database;}];
        };
        settings.processes.migrate = {
          command = getExe self'.packages.migrate.local;
          depends_on.postgres.condition = "process_healthy";
        };
      };
    };
    packages.migrate = let
      inherit (inputs'.fenix.packages.stable) toolchain;
      sqlx-cli = pkgs.sqlx-cli.override {
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      };
      migrations = pkgs.linkFarm "migrations" {migrations = ./migrations;};
      srcs = [toolchain sqlx-cli migrations];
    in
      pkgs.wrapProgram srcs "migrate" "cargo" "--add-flags \"sqlx migrate run\"" {
        passthru.local = pkgs.wrapFlags self'.packages.migrate "--set DATABASE_URL=${postgres.local.url}";
      };
  };
}
