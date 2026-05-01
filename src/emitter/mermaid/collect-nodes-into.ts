import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualElement, VisualNode } from "../../visual-graph/model.js";

export function collectNodesInto(
  elements: readonly VisualElement[],
  out: Map<string, VisualNode>,
): void {
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      out.set(e.id, e);
    } else {
      collectNodesInto(e.elements, out);
    }
  }
}
