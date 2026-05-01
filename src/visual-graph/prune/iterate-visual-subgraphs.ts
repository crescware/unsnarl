import type { VisualElement, VisualSubgraph } from "../model.js";

export function* iterateVisualSubgraphs(
  elements: readonly VisualElement[],
): Generator<VisualSubgraph> {
  for (const e of elements) {
    if (e.type === "subgraph") {
      yield e;
      yield* iterateVisualSubgraphs(e.elements);
    }
  }
}
