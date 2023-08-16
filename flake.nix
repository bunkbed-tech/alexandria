{
  description = "Media tracker in Tauri + SolidJS";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    devshell.url = "github:numtide/devshell";
    devshell.inputs.nixpkgs.follows = "nixpkgs";
    devshell.inputs.flake-utils.follows = "flake-utils";
    flake-utils.url = "github:numtide/flake-utils";
    fenix-flake.url = "github:nix-community/fenix";
    fenix-flake.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { self, nixpkgs, devshell, flake-utils, fenix-flake }: flake-utils.lib.eachDefaultSystem (system:
    let
      project = "alexandria";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ devshell.overlays.default fenix-flake.overlays.default ];
      };
    in
    rec {
      devShell = pkgs.devshell.mkShell {
        name = "${project}-shell";
        packages = with pkgs; [
          nixpkgs-fmt
          (fenix.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ])
          nodePackages.pnpm
          rust-analyzer-nightly
        ];
      };
    }
  );
}
