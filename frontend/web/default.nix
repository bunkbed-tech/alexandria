{
  perSystem = {
    lib,
    pkgs,
    ...
  }: {
    canivete.devShell.packages = [pkgs.bun];
    canivete.dream2nix.alexandria-web.module = {
      config,
      dream2nix,
      ...
    }: let
      inherit (config) bun http-server makeBinaryWrapper nodejs-slim_latest stdenv;
      inherit (lib.importJSON ../package.json) name version;
      node_modules = stdenv.mkDerivation {
        pname = "${name}-node_modules";
        version = "0.0.1";
        src = ./.;
        nativeBuildInputs = [bun];
        buildPhase = "bun install --no-progress --frozen-lockfile";
        installPhase = ''
          mkdir -p $out/node_modules
          cp -R ./node_modules $out
        '';
        dontPatchShebangs = true;
        outputHash =
          {
            x86_64-linux = "";
            x86_64-darwin = "";
            aarch64-linux = "";
            aarch64-darwin = "";
          }
          .${stdenv.hostPlatform.system};
        outputHashAlgo = "sha256";
        outputHashMode = "recursive";
      };
    in {
      imports = [dream2nix.modules.dream2nix.mkDerivation];
      deps = {nixpkgs, ...}: {inherit (nixpkgs) bun http-server makeBinaryWrapper nodejs-slim_latest stdenv;};
      inherit name version;
      paths.package = ./.;
      mkDerivation = {
        src = ./.;
        nativeBuildInputs = [bun makeBinaryWrapper];
        buildInputs = [http-server];
        configurePhase = ''
          cp -R ${node_modules}/node_modules .
          substituteInPlace node_modules/.bin/vite --replace "/usr/bin/env node" "${nodejs-slim_latest}/bin/node"
        '';
        buildPhase = "bun run build";
        installPhase = ''
          mkdir -p $out/bin
          cp -R ./build/* $out
          makeBinaryWrapper ${lib.getExe http-server} $out/bin/${name} --add-flags "$out"
        '';
        meta.mainProgram = name;
      };
    };
    canivete.pre-commit = {
      languages.javascript.enable = true;
      # Also run biome on .svelte files
      settings.hooks.biome.types_or = ["svelte"];
      settings.hooks.lychee.settings.flags = lib.concatStringsSep " " [
        # Exclude hardcoded localhost links
        "--exclude localhost"
        # Remap sveltekit assets to correct folder for static link checking
        "--remap 'src/%25sveltekit.assets%25 static'"
      ];
    };
  };
}
