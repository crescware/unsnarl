import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { collectImportSources } from "./collect-import-sources.js";
import { makeNode } from "./testing/make-node.js";

function asMap(...nodes: VisualNode[]): Map<string, VisualNode> {
  return new Map(nodes.map((n) => [n.id, n]));
}

describe("collectImportSources", () => {
  test("collects ids of ModuleSource and ImportIntermediate nodes", () => {
    const map = asMap(
      makeNode({ id: "mod_a", kind: "ModuleSource" }),
      makeNode({ id: "import_b", kind: "ImportIntermediate" }),
      makeNode({ id: "n_x", kind: "Variable" }),
    );
    expect([...collectImportSources(map)].sort()).toEqual([
      "import_b",
      "mod_a",
    ]);
  });

  test("excludes other synthetic kinds (e.g. ModuleSink)", () => {
    const map = asMap(makeNode({ id: "module_root", kind: "ModuleSink" }));
    expect(collectImportSources(map).size).toBe(0);
  });

  test("excludes non-synthetic kinds", () => {
    const map = asMap(
      makeNode({ id: "n_x", kind: "Variable" }),
      makeNode({ id: "n_f", kind: "FunctionName" }),
    );
    expect(collectImportSources(map).size).toBe(0);
  });

  test("empty map -> empty set", () => {
    expect(collectImportSources(asMap()).size).toBe(0);
  });
});
