import type { VisualElement } from "../../visual-graph/model.js";

export function collectWrappedOwnerIds(
  elements: readonly VisualElement[],
  out: Set<string>,
): void {
  for (const e of elements) {
    if (e.type !== "subgraph") {
      continue;
    }
    if (e.kind === "function" && e.ownerNodeId !== undefined) {
      out.add(e.ownerNodeId);
    }
    collectWrappedOwnerIds(e.elements, out);
  }
}
