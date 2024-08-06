{config, ...}: {
  perSystem = {
    nix,
    self',
    ...
  }: {
    canivete = {
      arion.modules.tauri = {self'', ...}: {
        services.migrations = {
          image.contents = [self''.packages.alexandria-migrations];
          image.command = ["migrate"];
          service.depends_on.postgres.condition = "service_healthy";
          service.environment.DATABASE_URL = config.alexandria.postgres.dev.url;
        };
      };
      process-compose.services.settings.processes = {
        migrations.command = nix.getExe self'.packages.alexandria-migrations.local;
        migrations.depends_on.postgres.condition = "process_healthy";
        tauri.command = nix.getExe self'.packages.alexandria-tauri.local;
        tauri.depends_on.migrations.condition = "process_completed_successfully";
        tauri.readiness_probe.exec.command = "exec 3<>/dev/tcp/localhost/5173";
      };
    };
  };
}
