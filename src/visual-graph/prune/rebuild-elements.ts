import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualElement, VisualSubgraph } from "../model.js";

export function rebuildElements(
  elements: readonly VisualElement[],
  keep: ReadonlySet<string>,
): /* mutable */ VisualElement[] {
  const result: /* mutable */ VisualElement[] = [];
  for (const item of elements) {
    if (item.type === VISUAL_ELEMENT_TYPE.Node) {
      if (keep.has(item.id)) {
        result.push({ ...item });
      }
    } else {
      const children = rebuildElements(item.elements, keep);
      // Subgraphs only survive when at least one descendant survived.
      // Keeping an empty subgraph -- even if it appeared as an edge
      // endpoint during BFS -- crashes Mermaid's elk layout because the
      // cluster has no labels[0] for the renderer to size against. The
      // edges that pointed at this subgraph are filtered out below by
      // the `survivors` check, so dropping the cluster is consistent.
      if (children.length > 0) {
        const cloned = {
          ...item,
          elements: children,
        } as const satisfies VisualSubgraph;
        result.push(cloned);
      }
    }
  }
  return result;
}
