import type { VisualElement } from "../model.js";

export function collectNodeIds(elements: readonly VisualElement[]): string[] {
  const out: string[] = [];
  walk(elements);
  return out;

  function walk(items: readonly VisualElement[]): void {
    for (const item of items) {
      if (item.type === "node") {
        out.push(item.id);
      } else {
        walk(item.elements);
      }
    }
  }
}
