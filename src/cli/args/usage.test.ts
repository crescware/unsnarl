import { describe, expect, test } from "vitest";

import { DEFAULT_GENERATIONS } from "./cli-args.js";
import { usage } from "./usage.js";

describe("usage", () => {
  test("starts with the Usage: header", () => {
    expect(usage()).toMatch(/^Usage:/);
  });

  test("documents every CLI option once", () => {
    const text = usage();
    for (const flag of [
      "-f, --format",
      "--stdin",
      "--lang",
      "--pretty / --no-pretty",
      "--mermaid-renderer",
      "-r, --roots",
      "-A, --descendants",
      "-B, --ancestors",
      "-C, --context",
      "-o, --out-dir",
      "--list-formats",
      "-h, --help",
      "-v, --version",
    ]) {
      expect(text).toContain(flag);
    }
  });

  test("interpolates DEFAULT_GENERATIONS into the radius defaults", () => {
    expect(usage()).toContain(`Default: ${DEFAULT_GENERATIONS}`);
  });
});
