import type { Language } from "../language.js";
import type { Direction } from "./direction.js";
import type { VisualBoundaryEdge } from "./visual-boundary-edge.js";
import type { VisualEdge } from "./visual-edge.js";
import type { VisualElement } from "./visual-element.js";
import type { VisualGraphPruning } from "./visual-graph-pruning.js";

// `elements` and `edges` stay mutable arrays: the builder appends to them
// during graph construction. Readonly only locks the property bindings,
// not the array contents.
export type VisualGraph = Readonly<{
  version: 1;
  source: Readonly<{ path: string; language: Language }>;
  direction: Direction;
  elements: /* mutable */ VisualElement[];
  edges: /* mutable */ VisualEdge[];
  boundaryEdges: readonly VisualBoundaryEdge[];
  pruning: VisualGraphPruning | null;
}>;
