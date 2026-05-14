import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { collectImportSources } from "./collect-import-sources.js";
import { baseNode, baseSimpleNode } from "./testing/make-node.js";

function asMap(...nodes: readonly VisualNode[]): Map<string, VisualNode> {
  return new Map(nodes.map((v) => [v.id, v]));
}

describe("collectImportSources", () => {
  test("collects ids of ModuleSource and ImportIntermediate nodes", () => {
    const map = asMap(
      { ...baseSimpleNode(NODE_KIND.SyntheticModuleSource), id: "mod_a" },
      { ...baseSimpleNode(NODE_KIND.LegacyImportIntermediate), id: "import_b" },
      { ...baseNode(), id: "n_x" },
    );
    expect([...collectImportSources(map)].sort()).toEqual([
      "import_b",
      "mod_a",
    ]);
  });

  test("excludes other synthetic kinds (e.g. ModuleSink)", () => {
    const map = asMap({
      ...baseSimpleNode(NODE_KIND.SyntheticModuleSink),
      id: "module_root",
    });
    expect(collectImportSources(map).size).toEqual(0);
  });

  test("excludes non-synthetic kinds", () => {
    const map = asMap(
      { ...baseNode(), id: "n_x" },
      { ...baseSimpleNode(NODE_KIND.LegacyFunctionName), id: "n_f" },
    );
    expect(collectImportSources(map).size).toEqual(0);
  });

  test("empty map -> empty set", () => {
    expect(collectImportSources(asMap()).size).toEqual(0);
  });
});
