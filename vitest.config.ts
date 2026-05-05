import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: [
      {
        extends: true,
        test: {
          name: "default",
          environment: "node",
          exclude: ["**/node_modules/**", "integration/fixtures/for/**"],
        },
      },
      {
        extends: true,
        test: {
          name: "fixtures-for",
          environment: "jsdom",
          include: ["integration/fixtures/for/**/*.test.ts"],
        },
      },
    ],
  },
});
