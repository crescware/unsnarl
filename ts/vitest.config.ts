import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    projects: [
      {
        extends: true,
        test: {
          name: "default",
          environment: "node",
          exclude: [
            "**/node_modules/**",
            "integration/fixtures/**",
            "parity/**",
          ],
        },
      },
      {
        extends: true,
        test: {
          name: "parity",
          environment: "node",
          include: ["parity/**/*.test.ts"],
        },
      },
    ],
  },
});
