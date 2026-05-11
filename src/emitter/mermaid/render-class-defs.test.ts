import { describe, expect, test } from "vitest";

import { renderClassDefs } from "./render-class-defs.js";
import { darkTheme } from "./theme/dark-theme.js";
import { lightTheme } from "./theme/light-theme.js";

const emptyNestMap: ReadonlyMap<number, readonly string[]> = new Map();

describe("renderClassDefs", () => {
  test("emits nothing when all id lists and the nest map are empty", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs([], [], emptyNestMap, darkTheme, lines);
    expect(lines).toEqual([]);
  });

  test("emits the boundaryStub classDef without a fill so stubs share the regular node background", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs(["stub_1", "stub_2"], [], emptyNestMap, darkTheme, lines);
    expect(lines).toEqual([
      "  classDef boundaryStub stroke:#888,stroke-dasharray:3 3,color:#888;",
      "  class stub_1 boundaryStub;",
      "  class stub_2 boundaryStub;",
    ]);
  });

  test("emits the varNode classDef from the dark theme with the original dash pattern (contract)", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs([], ["v_one", "v_two"], emptyNestMap, darkTheme, lines);
    expect(lines).toEqual([
      "  classDef varNode stroke-dasharray:5 5;",
      "  class v_one varNode;",
      "  class v_two varNode;",
    ]);
  });

  test("emits boundaryStub and varNode together when both lists are non-empty (dark theme)", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs(["stub_1"], ["v_one"], emptyNestMap, darkTheme, lines);
    expect(lines).toEqual([
      "  classDef boundaryStub stroke:#888,stroke-dasharray:3 3,color:#888;",
      "  class stub_1 boundaryStub;",
      "  classDef varNode stroke-dasharray:5 5;",
      "  class v_one varNode;",
    ]);
  });

  test("routes through the supplied theme so a light theme produces its own literals", () => {
    const lines: /* mutable */ string[] = [];
    renderClassDefs(["stub_1"], [], emptyNestMap, lightTheme, lines);
    expect(lines).toContain(
      `  classDef boundaryStub stroke:${lightTheme.boundaryStub.stroke},stroke-dasharray:${lightTheme.boundaryStub.strokeDasharray},color:${lightTheme.boundaryStub.color};`,
    );
  });

  test("emits per-level nest classDefs in palette-slot order with 1-based names", () => {
    const lines: /* mutable */ string[] = [];
    const nestMap = new Map<number, readonly string[]>([
      [0, ["s_outer"]],
      [1, ["s_mid"]],
      [2, ["s_inner"]],
    ]);
    renderClassDefs([], [], nestMap, darkTheme, lines);
    const expected = [
      `  classDef nestL1 fill:${darkTheme.nestPalette[0]?.fill},stroke:${darkTheme.nestPalette[0]?.stroke};`,
      "  class s_outer nestL1;",
      `  classDef nestL2 fill:${darkTheme.nestPalette[1]?.fill},stroke:${darkTheme.nestPalette[1]?.stroke};`,
      "  class s_mid nestL2;",
      `  classDef nestL3 fill:${darkTheme.nestPalette[2]?.fill},stroke:${darkTheme.nestPalette[2]?.stroke};`,
      "  class s_inner nestL3;",
    ];
    expect(lines).toEqual(expected);
  });

  test("emits slots in ascending palette order regardless of insertion order", () => {
    const lines: /* mutable */ string[] = [];
    const nestMap = new Map<number, readonly string[]>([
      [2, ["s_inner"]],
      [0, ["s_outer"]],
      [1, ["s_mid"]],
    ]);
    renderClassDefs([], [], nestMap, darkTheme, lines);
    const headers = lines.filter((v) => v.includes("classDef nestL"));
    expect(headers).toEqual([
      `  classDef nestL1 fill:${darkTheme.nestPalette[0]?.fill},stroke:${darkTheme.nestPalette[0]?.stroke};`,
      `  classDef nestL2 fill:${darkTheme.nestPalette[1]?.fill},stroke:${darkTheme.nestPalette[1]?.stroke};`,
      `  classDef nestL3 fill:${darkTheme.nestPalette[2]?.fill},stroke:${darkTheme.nestPalette[2]?.stroke};`,
    ]);
  });

  test("skips slots that have no subgraph ids", () => {
    const lines: /* mutable */ string[] = [];
    const nestMap = new Map<number, readonly string[]>([
      [0, ["s_outer"]],
      [2, ["s_far"]],
    ]);
    renderClassDefs([], [], nestMap, darkTheme, lines);
    expect(lines.some((v) => v.includes("nestL2"))).toEqual(false);
    expect(lines.some((v) => v.includes("nestL1"))).toEqual(true);
    expect(lines.some((v) => v.includes("nestL3"))).toEqual(true);
  });

  test("places a function wrapper id alongside other subgraphs in the same palette slot", () => {
    const lines: /* mutable */ string[] = [];
    const nestMap = new Map<number, readonly string[]>([
      [0, ["wrap_s_fn", "s_fn"]],
    ]);
    renderClassDefs([], [], nestMap, darkTheme, lines);
    expect(lines).toContain("  class wrap_s_fn nestL1;");
    expect(lines).toContain("  class s_fn nestL1;");
  });
});
