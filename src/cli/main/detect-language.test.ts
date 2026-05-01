import { describe, expect, test } from "vitest";

import { detectLanguage } from "./detect-language.js";

describe("detectLanguage", () => {
  test("null path → fallback verbatim", () => {
    expect(detectLanguage(null, "ts")).toBe("ts");
    expect(detectLanguage(null, "tsx")).toBe("tsx");
    expect(detectLanguage(null, "js")).toBe("js");
    expect(detectLanguage(null, "jsx")).toBe("jsx");
  });

  test(".tsx → tsx", () => {
    expect(detectLanguage("Component.tsx", "ts")).toBe("tsx");
  });

  test(".jsx → jsx", () => {
    expect(detectLanguage("Component.jsx", "ts")).toBe("jsx");
  });

  test(".js → js", () => {
    expect(detectLanguage("foo.js", "ts")).toBe("js");
  });

  test(".ts → ts (covered by default branch)", () => {
    expect(detectLanguage("foo.ts", "tsx")).toBe("ts");
  });

  test("unknown extension → ts (default branch ignores fallback)", () => {
    expect(detectLanguage("foo.cjs", "jsx")).toBe("ts");
    expect(detectLanguage("Makefile", "tsx")).toBe("ts");
  });

  test("nested paths still inspect the trailing suffix", () => {
    expect(detectLanguage("src/deep/Component.tsx", "ts")).toBe("tsx");
  });
});
