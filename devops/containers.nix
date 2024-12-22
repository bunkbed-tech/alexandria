{
  lib,
  flake-parts-lib,
  ...
}:
flake-parts-lib.mkTransposedPerSystemModule {
  name = "containers";
  option = lib.mkOption {
    type = with lib.types; lazyAttrsOf package;
    default = {};
    description = "Container image packages exposed at the flake top-level";
  };
  file = ./containers.nix;
}
