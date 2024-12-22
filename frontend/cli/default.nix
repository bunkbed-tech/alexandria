{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    packages.alexandria-cli = pkgs.buildGoModule rec {
      pname = "alexandria-cli";
      version = "0.0.1";
      src = ./.;
      vendorHash = "";
      meta.mainProgram = "alexandria-cli";
    };
    canivete = {
      devShells.shells.default.packages = [pkgs.go];
      pre-commit.languages.golang.enable = true;
      pre-commit.settings = {config, ...}: {
        hooks.golangci-lint.entry = let
          script = pkgs.writeShellScriptBin "precommit-golangci-lint" ''
            set -e
            for dir in $(echo "$@" | xargs -n1 dirname | sort -u); do
              ${config.hooks.golangci-lint.package}/bin/golangci-lint run ./"$dir"
            done
          '';
        in "sh -c 'cd frontend/cli && ${lib.getExe script} \"$@\"'";
      };
      process-compose.services.settings.processes.cli.command = "${pkgs.go}/bin/go run";
    };
  };
}
