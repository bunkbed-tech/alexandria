{
  perSystem = {pkgs, ...}: {
    canivete.devShells.shells.default.packages = [pkgs.uv];
    canivete.pre-commit.languages.python.enable = true;
    # TODO build package
  };
}
