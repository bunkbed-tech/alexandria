import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig(async () => ({
  plugins: [solidPlugin()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
}));
