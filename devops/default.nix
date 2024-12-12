{
  inputs,
  self,
  ...
}: {
  imports = [
    ./modules/darwin.nix
    ./modules/home.nix
    ./modules/nixos.nix
    ./modules/droid.nix
    ./pkgs/containers.nix
    ./pkgs/devshell.nix
    ./pkgs/overlay.nix
    ./pkgs/packages.nix
    ./pre-commit.nix
  ];
  # Allow arbitrary line length in markdown (paragraph wrapping preferred)
  perSystem.canivete.pre-commit.settings.hooks.markdownlint.settings.configuration.MD013.line_length = -1;
}
