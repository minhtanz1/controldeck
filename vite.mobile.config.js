import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { viteSingleFile } from "vite-plugin-singlefile";
import path from "node:path";

export default defineConfig({
  root: path.resolve(import.meta.dirname, "src/mobile"),
  plugins: [svelte(), viteSingleFile()],
  build: {
    target: "esnext",
    outDir: path.resolve(import.meta.dirname, "src-tauri/src/server/dist-mobile"),
    emptyOutDir: true,
  },
});
