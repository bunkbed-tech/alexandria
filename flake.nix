{
  inputs = {
    canivete.url = github:schradert/canivete;
    # Compose rust toolchains
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "canivete/nixpkgs";
  };
  outputs = inputs:
    inputs.canivete.lib.mkFlake {
      inherit inputs;
      everything = [./nix];
    } {};
}
