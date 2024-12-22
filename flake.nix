{
  description = "https://gitlab.com/bunkbed/alexandria";
  outputs = inputs:
    inputs.canivete.lib.mkFlake {inherit inputs;} [./backend ./devops ./frontend ./modules] {
      # Allow arbitrary line length in markdown (paragraph wrapping preferred)
      perSystem.canivete.pre-commit.settings.hooks.markdownlint.settings.configuration.MD013.line_length = -1;
    };
  inputs = {
    canivete.url = github:schradert/canivete;
    nix2container.url = github:nlewo/nix2container;
    nix2container.inputs.nixpkgs.follows = "canivete/nixpkgs";
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "canivete/nixpkgs";
  };
}
