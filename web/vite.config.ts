import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
  build: {
    lib: {
      name: "dropspot",
      entry: ["src/index.ts"],
      // fileName: (format, entryName) => `${entryName}.${format}.js`,
      fileName: "index",
      cssFileName: "style",
    },
  },
});
