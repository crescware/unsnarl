import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import type { VisualElement } from "../../visual-graph/visual-element.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

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
