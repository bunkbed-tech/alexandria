{
  description = "https://gitlab.com/bunkbed/alexandria";
  outputs = inputs: inputs.canivete.lib.mkFlake {inherit inputs;} {imports = [./backend ./devops ./frontend];};
  inputs = {
    canivete.url = github:schradert/canivete;
    nix2container.url = github:nlewo/nix2container;
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };
}
