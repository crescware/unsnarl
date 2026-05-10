import { describe, expect, test } from "vitest";

import { codeFenceLang } from "./code-fence-lang.js";

describe("codeFenceLang", () => {
  test("tsx → tsx", () => {
    expect(codeFenceLang("tsx")).toEqual("tsx");
  });

  test("jsx → jsx", () => {
    expect(codeFenceLang("jsx")).toEqual("jsx");
  });

  test("js → js", () => {
    expect(codeFenceLang("js")).toEqual("js");
  });

  test("ts → ts (default branch)", () => {
    expect(codeFenceLang("ts")).toEqual("ts");
  });
});
