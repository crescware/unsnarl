import type { VisualElement } from "../model.js";

export function buildParentMap(
  elements: readonly VisualElement[],
): Map<string, string> {
  const parent = new Map<string, string>();
  walk(elements, undefined);
  return parent;

  function walk(
    items: readonly VisualElement[],
    parentId: string | undefined,
  ): void {
    for (const item of items) {
      if (parentId !== undefined) {
        parent.set(item.id, parentId);
      }
      if (item.type === "subgraph") {
        walk(item.elements, item.id);
      }
    }
  }
}
