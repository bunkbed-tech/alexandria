{
  perSystem = {pkgs, ...}: {
    canivete.devShells.shells.default.packages = with pkgs; [sops age ssh-to-age];
    canivete.pre-commit.settings.hooks.typos.settings.exclude = "devops/secrets.yaml";
  };
}
