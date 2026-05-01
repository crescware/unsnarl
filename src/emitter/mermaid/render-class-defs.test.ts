import { describe, expect, test } from "vitest";

import { renderClassDefs } from "./render-class-defs.js";

describe("renderClassDefs", () => {
  test("emits nothing when both id lists are empty", () => {
    const lines: string[] = [];
    renderClassDefs([], [], lines);
    expect(lines).toEqual([]);
  });

  test("emits the fnWrap classDef and per-id 'class' lines for wrapper ids", () => {
    const lines: string[] = [];
    renderClassDefs(["wrap_a", "wrap_b"], [], lines);
    expect(lines).toEqual([
      "  classDef fnWrap fill:#1a2030,stroke:#5a7d99;",
      "  class wrap_a fnWrap;",
      "  class wrap_b fnWrap;",
    ]);
  });

  test("emits the boundaryStub classDef and per-id 'class' lines for stub ids", () => {
    const lines: string[] = [];
    renderClassDefs([], ["stub_1", "stub_2"], lines);
    expect(lines[0]?.startsWith("  classDef boundaryStub ")).toBe(true);
    expect(lines).toContain("  class stub_1 boundaryStub;");
    expect(lines).toContain("  class stub_2 boundaryStub;");
  });

  test("emits both classDefs when both lists are non-empty", () => {
    const lines: string[] = [];
    renderClassDefs(["wrap_a"], ["stub_1"], lines);
    expect(lines).toEqual([
      "  classDef fnWrap fill:#1a2030,stroke:#5a7d99;",
      "  class wrap_a fnWrap;",
      "  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;",
      "  class stub_1 boundaryStub;",
    ]);
  });
});
