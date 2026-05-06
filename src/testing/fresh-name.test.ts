import { describe, expect, test } from "vitest";

import { freshName } from "./fresh-name.js";

describe("freshName", () => {
  test("is a valid JavaScript identifier", () => {
    const name = freshName();
    expect(name).toMatch(/^[A-Za-z_$][A-Za-z0-9_$]*$/);
  });

  test("returns a different name on each call", () => {
    const names = new Set(Array.from({ length: 8 }, () => freshName()));
    expect(names.size).toBe(8);
  });

  test("starts with the fixed `v` prefix followed by hex characters", () => {
    expect(freshName()).toMatch(/^v[0-9a-f]+$/);
  });
});
