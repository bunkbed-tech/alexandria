{
  perSystem = {pkgs, ...}: {
    packages.alexandria-cli = pkgs.buildGoModule rec {
      pname = "alexandria-cli";
      version = "0.0.1";
      src = ./.;
      vendorHash = "";
      meta.mainProgram = "alexandria-cli";
    };
    canivete.devShells.shells.default.packages = [pkgs.go];
    canivete.pre-commit.languages.golang.enable = true;
    canivete.process-compose.services.settings.processes.cli.command = "${pkgs.go}/bin/go run";
  };
}
