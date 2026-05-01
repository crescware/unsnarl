import type { VisualElement, VisualNode } from "../../visual-graph/model.js";

export function collectNodes(elements: readonly VisualElement[]): VisualNode[] {
  const out: VisualNode[] = [];
  for (const e of elements) {
    if (e.type === "node") {
      out.push(e);
    } else {
      out.push(...collectNodes(e.elements));
    }
  }
  return out;
}
