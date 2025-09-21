import { defineConfig } from "@rslib/core";

export default defineConfig({
  lib: [
    {
      dts: true,
      format: "esm",
      bundle: true,
    },
  ],
  source: {
    tsconfigPath: "./tsconfig.lib.json",
  },
  output: {
    minify: false,
    sourceMap: false,
    target: "node",
  },
  tools: {
    rspack: {
      output: {
        library: {
          type: "module",
        },
      },
      experiments: {
        outputModule: true,
      },
    },
  },
});
