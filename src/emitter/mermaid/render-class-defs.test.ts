import { describe, expect, test } from "vitest";

import { renderClassDefs } from "./render-class-defs.js";

describe("renderClassDefs", () => {
  test("emits nothing when all id lists are empty", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs([], [], [], lines);
    expect(lines).toEqual([]);
  });

  test("emits the fnWrap classDef and per-id 'class' lines for wrapper ids", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs(["wrap_a", "wrap_b"], [], [], lines);
    expect(lines).toEqual([
      "  classDef fnWrap fill:#1a2030,stroke:#5a7d99;",
      "  class wrap_a fnWrap;",
      "  class wrap_b fnWrap;",
    ]);
  });

  test("emits the boundaryStub classDef and per-id 'class' lines for stub ids", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs([], ["stub_1", "stub_2"], [], lines);
    expect(lines[0]?.startsWith("  classDef boundaryStub ")).toBe(true);
    expect(lines).toContain("  class stub_1 boundaryStub;");
    expect(lines).toContain("  class stub_2 boundaryStub;");
  });

  test("emits the varNode classDef and per-id 'class' lines for var ids", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs([], [], ["v_one", "v_two"], lines);
    expect(lines).toEqual([
      "  classDef varNode stroke-dasharray:5 5;",
      "  class v_one varNode;",
      "  class v_two varNode;",
    ]);
  });

  test("emits all classDefs when every list is non-empty", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs(["wrap_a"], ["stub_1"], ["v_one"], lines);
    expect(lines).toEqual([
      "  classDef fnWrap fill:#1a2030,stroke:#5a7d99;",
      "  class wrap_a fnWrap;",
      "  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;",
      "  class stub_1 boundaryStub;",
      "  classDef varNode stroke-dasharray:5 5;",
      "  class v_one varNode;",
    ]);
  });
});
