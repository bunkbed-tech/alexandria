{
  inputs = {
    canivete.url = github:schradert/canivete;
    # Compose rust toolchains
    fenix.url = github:nix-community/fenix;
    fenix.inputs.nixpkgs.follows = "canivete/nixpkgs";
  };
  outputs = inputs:
    inputs.canivete.lib.mkFlake {inherit inputs;} {
      perSystem = {
        nix,
        pkgs,
        system,
        ...
      }: let
        fenix = inputs.fenix.packages.${system};
        # Stable rust toolchain with wasm support (for Tauri)
        toolchain = fenix.combine (nix.flatten [
          (with fenix.stable; [cargo rustc rust-src])
          fenix.targets.wasm32-unknown-unknown.stable.rust-std
          fenix.rust-analyzer
        ]);
        # Tauri v2
        cargo-tauri =
          (cargo-tauri.override {
            rustPlatform = pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            };
          })
          .overrideAttrs (old: rec {
            inherit (old) pname;
            version = "2.0.0-beta.11";
            src = pkgs.fetchFromGitHub {
              owner = "tauri-apps";
              repo = "tauri";
              rev = "tauri-v${version}";
              hash = "sha256-Few8BuF2PX5BCXKeTrh6iCxVCuLoYCMpHAKnwesynNQ=";
            };
            sourceRoot = "${src.name}/tooling/cli";
            cargoDeps = old.cargoDeps.overrideAttrs (nix.const {
              name = "${pname}-${version}-vendor.tar.gz";
              inherit src;
              outputHash = "sha256-iCrLNuaOUCR6wcHsblE1It0F81c973rYqlkOdw53cDA=";
            });
            buildInputs = old.buildInputs ++ nix.optional pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.SystemConfiguration;
          });
      in {
        canivete.devShell.packages = nix.flatten [
          toolchain
          cargo-tauri
          (with pkgs; [
            bun
            trunk
            wasm-bindgen-cli
            (nix.optionals stdenv.isDarwin [
              darwin.libiconv
              (with darwin.apple_sdk.frameworks; [
                Carbon
                WebKit
              ])
            ])
          ])
        ];
        canivete.pre-commit = {
          languages.javascript.enable = true;
          # Also run biome on .svelte files
          settings.hooks.biome.types_or = ["svelte"];
          # Allow arbitrary line length in markdown (paragraph wrapping preferred)
          settings.hooks.markdownlint.settings.configuration.MD013.line_length = -1;
          settings.hooks.lychee.settings.flags = nix.concatStringsSep " " [
            # Exclude hardcoded localhost links
            "--exclude localhost"
            # Remap sveltekit assets to correct folder for static link checking
            "--remap 'src/%25sveltekit.assets%25 static'"
          ];
        };
      };
    };
}
