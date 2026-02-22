import { defineConfig } from "vite";

export default defineConfig({
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
