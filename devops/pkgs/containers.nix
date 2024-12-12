{
  lib,
  flake-parts-lib,
  ...
}: let
  inherit (lib) flip getExe mapAttrsToList mkMerge mkOption types;
  inherit (types) lazyAttrsOf listOf package submodule;
in
  mkMerge [
    (flake-parts-lib.mkTransposedPerSystemModule {
      name = "containers";
      option = mkOption {
        type = lazyAttrsOf package;
        default = {};
        description = "Container image packages exposed at the flake top-level";
      };
      file = ./containers.nix;
    })
    {
      perSystem = {
        config,
        inputs',
        self',
        ...
      }: let
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
        config = mkMerge (flip mapAttrsToList self'.packages (name: package: {
          canivete.arion.modules.${name} = {self'', ...}: {services.${name}.image.command = [(getExe self''.packages.${name})];};
          containers.${name} = buildImage {
            inherit name;
            tag = package.version;
            config.entrypoint = [(getExe package)];
            layers = map buildLayer config.alexandria.nix2container.${name}.layers;
          };
        }));
      };
    }
  ]
