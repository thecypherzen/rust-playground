import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import tsconfigPaths from "vite-tsconfig-paths";
//import wasm from "vite-plugin-wasm";
//import topLevelAwait from "vite-plugin-top-level-await";

// https://vite.dev/config/
export default defineConfig({
  //plugins: [react(), tailwindcss(), tsconfigPaths(), wasm(), topLevelAwait()],
  plugins: [react(), tailwindcss(), tsconfigPaths()],
  server: {
    host: "localhost",
    port: 5080,
  },
  resolve: {
    alias: {
      "@": "/src",
    },
  },
});
