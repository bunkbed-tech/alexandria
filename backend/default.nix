{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    canivete = {
      pre-commit.languages.rust.enable = true;
      pre-commit.settings = {config, ...}: {
        hooks = {
          clippy.entry = "sh -c 'cd backend && ${lib.getExe' config.hooks.clippy.package "cargo-clippy"} clippy --offline --'";
          clippy.pass_filenames = false;
        };
      };
      dream2nix.packages.alexandria-backend.module = {
        config,
        dream2nix,
        ...
      }: {
        # TODO how can I integrate fenix toolchain here as well?
        mkDerivation.src = ./.;
        paths.package = ./.;
        imports = [
          dream2nix.modules.dream2nix.rust-cargo-lock
          dream2nix.modules.dream2nix.rust-cargo-vendor
          dream2nix.modules.dream2nix.rust-crane
        ];
      };
      process-compose.services.settings.processes.backend.command = "";
    };
  };
}
