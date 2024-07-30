{nix, ...}: {
  perSystem.canivete.pre-commit = {
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
}
