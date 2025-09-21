import { createDefaultEsmPreset, type JestConfigWithTsJest } from "ts-jest";

export default {
  ...createDefaultEsmPreset({
    tsconfig: "./tsconfig.test.json",
  }),
  roots: ["<rootDir>"],
  cache: true,
  cacheDirectory: "<rootDir>/.jestcache",
  collectCoverage: true,
  randomize: true,
  maxWorkers: 1,
  coverageThreshold: {
    global: {
      branches: 100,
      functions: 100,
      lines: 100,
      statements: 100,
    },
  },
} satisfies JestConfigWithTsJest;
