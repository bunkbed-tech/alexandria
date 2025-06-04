{
  perSystem = {
    inputs',
    lib,
    pkgs,
    ...
  }: {
    canivete = {
      devShells.shells.default.packages = [inputs'.fenix.packages.stable.toolchain];
      pre-commit.languages.rust.enable = true;
      pre-commit.settings = {config, ...}: {
        hooks = {
          clippy.entry = "sh -c 'cd frontend/cli && ${lib.getExe' config.hooks.clippy.package "cargo-clippy"} clippy --offline -- \"$@\"'";
          rustfmt.entry = "sh -c 'cd frontend/cli && ${lib.getExe' config.hooks.rustfmt.package "cargo-fmt"} fmt -- \"$@\"'";
          taplo.entry = "sh -c 'cd frontend/cli && ${lib.getExe config.hooks.taplo.package} fmt \"$@\"'";
        };
      };
      dream2nix.packages.alexandria-cli.module = {
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
    };
  };
}
