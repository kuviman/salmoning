import { defineConfig } from "vite";
import externalize from "vite-plugin-externalize-dependencies";

export default defineConfig({
  plugins: [
    externalize({
      externals: ["salmoning"],
    }),
  ],
  build: {
    rollupOptions: {
      external: ["salmoning"],
    },
  },
});
