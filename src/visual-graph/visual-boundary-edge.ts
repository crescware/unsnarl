import type { BOUNDARY_EDGE_DIRECTION } from "./prune/boundary-edge-direction.js";

/**
 * An edge whose `inside` end is kept by pruning but whose other end was
 * cut by the requested radius. Pruning emits one entry per such edge so
 * renderers can hint at "more context exists in this direction" without
 * dragging the next generation of nodes back into the graph.
 *
 * In an edge `from -label-> to`, the label describes the action `to`
 * performs on `from` (e.g. "read", "set"). We only know the label when
 * the action's actor (= `to`) is the kept side:
 *
 * - "out" (`inside -> beyond`): the actor is the unseen `beyond` node,
 *   so the label is unknowable and is intentionally absent here.
 * - "in"  (`beyond -> inside`): the actor is `inside` itself, which is
 *   visible, so we can keep the original edge label.
 */
export type VisualBoundaryEdge =
  | Readonly<{ inside: string; direction: typeof BOUNDARY_EDGE_DIRECTION.Out }>
  | Readonly<{
      inside: string;
      direction: typeof BOUNDARY_EDGE_DIRECTION.In;
      label: string;
    }>;
