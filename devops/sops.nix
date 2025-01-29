{
  perSystem = {
    inputs',
    lib,
    pkgs,
    ...
  }: {
    canivete.pre-commit.settings.hooks.typos.settings.exclude = "devops/secrets.yaml";
    # FIXME fix the damn bug in canivete to avoid forcing a devshell override
    # canivete.devShells.shells.default.packages = [pkgs.sops];
    devShells.default = lib.mkForce (pkgs.mkShell {
      packages = [inputs'.fenix.packages.stable.toolchain] ++ (with pkgs; [sops age ssh-to-age]);
    });
  };
}
