{
  perSystem = {pkgs, ...}: {
    canivete.pre-commit.languages.rust.enable = true;
    canivete.dream2nix.packages.alexandria-backend.module = {
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
    canivete.process-compose.services.settings.processes.backend.command = "";
  };
}
