{
  config,
  nix,
  ...
}: let
  cfg = config.alexandria.postgres;
in
  with nix; {
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
    config.perSystem.canivete = {
      # TODO convert this to nixos postgres
      arion.modules.postgres.services.postgres.service = with cfg.dev; {
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
      process-compose.services.services.postgres.postgres = with cfg.local; {
        enable = true;
        inherit port;
        initialScript.before = "CREATE USER ${username} WITH PASSWORD '${password}';";
        initialDatabases = [{name = database;}];
      };
    };
  }
