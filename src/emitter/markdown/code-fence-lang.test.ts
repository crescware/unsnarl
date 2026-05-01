import { describe, expect, test } from "vitest";

import { codeFenceLang } from "./code-fence-lang.js";

describe("codeFenceLang", () => {
  test("tsx → tsx", () => {
    expect(codeFenceLang("tsx")).toBe("tsx");
  });

  test("jsx → jsx", () => {
    expect(codeFenceLang("jsx")).toBe("jsx");
  });

  test("js → js", () => {
    expect(codeFenceLang("js")).toBe("js");
  });

  test("ts → ts (default branch)", () => {
    expect(codeFenceLang("ts")).toBe("ts");
  });
});
