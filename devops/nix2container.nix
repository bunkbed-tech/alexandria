{
  perSystem = {
    config,
    inputs',
    lib,
    self',
    ...
  }: let
    inherit (lib) flip getExe mapAttrs mkMerge mkOption types;
    inherit (types) lazyAttrsOf listOf package submodule;
    inherit (inputs'.nix2container.packages.nix2container) buildImage buildLayer;
  in {
    options.alexandria.nix2container = mkOption {
      default = {};
      description = "Nix2Container image details";
      type = lazyAttrsOf (submodule {
        options.layers = mkOption {
          default = [];
          type = listOf (submodule {
            options.deps = mkOption {
              type = listOf package;
            };
          });
        };
      });
    };
    # config.canivete.just.recipes."arion *ARGS" = "${getExe config.canivete.arion.projects.arion.finalPackage} {{ ARGS }}";
    config.canivete.arion.projects.arion.modules.default = {self'', ...}: {
      services = flip mapAttrs self'.packages (_: package: {image.command = [(getExe package)];});
    };
    config.containers = flip mapAttrs self'.packages (name: package:
      buildImage {
        inherit name;
        tag = package.version;
        config.entrypoint = [(getExe package)];
        layers = map buildLayer config.alexandria.nix2container.${name}.layers;
      });
  };
}
