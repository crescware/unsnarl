import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: [
      {
        extends: true,
        test: {
          name: "default",
          environment: "node",
          exclude: ["**/node_modules/**", "integration/fixtures/**"],
        },
      },
      {
        extends: true,
        test: {
          name: "fixtures",
          environment: "jsdom",
          include: ["integration/fixtures/**/*.test.ts"],
        },
      },
    ],
  },
});
