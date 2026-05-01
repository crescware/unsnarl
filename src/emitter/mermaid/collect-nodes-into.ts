import type { VisualElement, VisualNode } from "../../visual-graph/model.js";

export function collectNodesInto(
  elements: VisualElement[],
  out: Map<string, VisualNode>,
): void {
  for (const e of elements) {
    if (e.type === "node") {
      out.set(e.id, e);
    } else {
      collectNodesInto(e.elements, out);
    }
  }
}
