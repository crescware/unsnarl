import type { VisualElement } from "../model.js";

export function collectIds(elements: readonly VisualElement[]): Set<string> {
  const ids = new Set<string>();
  walk(elements);
  return ids;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      ids.add(item.id);
      if (item.type === "subgraph") {
        walk(item.elements);
      }
    }
  }
}
