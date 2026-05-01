import { describe, expect, test } from "vitest";

import type { VisualNode } from "../../visual-graph/model.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { collectImportSources } from "./collect-import-sources.js";
import { baseNode, baseSimpleNode } from "./testing/make-node.js";

function asMap(...nodes: readonly VisualNode[]): Map<string, VisualNode> {
  return new Map(nodes.map((n) => [n.id, n]));
}

describe("collectImportSources", () => {
  test("collects ids of ModuleSource and ImportIntermediate nodes", () => {
    const map = asMap(
      { ...baseSimpleNode(NODE_KIND.ModuleSource), id: "mod_a" },
      { ...baseSimpleNode(NODE_KIND.ImportIntermediate), id: "import_b" },
      { ...baseNode(), id: "n_x" },
    );
    expect([...collectImportSources(map)].sort()).toEqual([
      "import_b",
      "mod_a",
    ]);
  });

  test("excludes other synthetic kinds (e.g. ModuleSink)", () => {
    const map = asMap({
      ...baseSimpleNode(NODE_KIND.ModuleSink),
      id: "module_root",
    });
    expect(collectImportSources(map).size).toBe(0);
  });

  test("excludes non-synthetic kinds", () => {
    const map = asMap(
      { ...baseNode(), id: "n_x" },
      { ...baseSimpleNode(NODE_KIND.FunctionName), id: "n_f" },
    );
    expect(collectImportSources(map).size).toBe(0);
  });

  test("empty map -> empty set", () => {
    expect(collectImportSources(asMap()).size).toBe(0);
  });
});
