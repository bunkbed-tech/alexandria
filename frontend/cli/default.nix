{
  perSystem = {pkgs, ...}: {
    packages.alexandria-cli = pkgs.buildGoModule rec {
      pname = "alexandria-cli";
      version = "0.0.1";
      src = ./.;
      vendorHash = "";
      meta.mainProgram = "alexandria-cli";
    };
    canivete.devShell.packages = [pkgs.go];
    canivete.pre-commit.settings.hooks.golangci-lint = {
      enable = true;
      raw.args = [
        "--config"
        (pkgs.writers.writeYAML "golangci-lint.yaml" {linters.enable = ["revive"];})
      ];
    };
    canivete.process-compose.services.settings.processes.cli.command = "${pkgs.go}/bin/go run";
  };
}
