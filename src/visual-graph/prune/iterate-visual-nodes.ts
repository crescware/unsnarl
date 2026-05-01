import type { VisualElement, VisualNode } from "../model.js";
import { ROOT_CANDIDATE_KINDS } from "./root-candidate-kinds.js";

export function* iterateVisualNodes(
  elements: readonly VisualElement[],
): Generator<VisualNode> {
  for (const e of elements) {
    if (e.type === "node") {
      if (ROOT_CANDIDATE_KINDS.has(e.kind)) {
        yield e;
      }
    } else {
      yield* iterateVisualNodes(e.elements);
    }
  }
}
