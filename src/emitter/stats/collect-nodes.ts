import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import type { VisualElement } from "../../visual-graph/visual-element.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function collectNodes(
  elements: readonly VisualElement[],
): readonly VisualNode[] {
  const out: /* mutable */ VisualNode[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      out.push(e);
    } else {
      out.push(...collectNodes(e.elements));
    }
  }
  return out;
}
