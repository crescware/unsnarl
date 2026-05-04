import type { Direction } from "./direction.js";
import type { SUBGRAPH_KIND } from "./subgraph-kind.js";
import type { VISUAL_ELEMENT_TYPE } from "./visual-element-type.js";
import type { VisualElement } from "./visual-element.js";

// Common shape across every subgraph kind. Mutable: builder patches
// endLine after construction and pushes into elements as it walks
// scopes. rebuild-elements also rewires children through
// `{ ...item, elements: children }`, so we cannot lock the property
// bindings either.
type CommonSubgraphFields = {
  type: typeof VISUAL_ELEMENT_TYPE.Subgraph;
  id: string;
  line: number;
  endLine: number | null;
  direction: Direction;
  elements: /* mutable */ VisualElement[];
};

export type VisualSubgraph =
  | (CommonSubgraphFields & {
      kind: typeof SUBGRAPH_KIND.Function;
      ownerNodeId: string;
      // Mirrors the owner node's display name so the subgraph label
      // survives pruning even when the owner node itself gets cut out.
      ownerName: string;
    })
  | (CommonSubgraphFields & {
      kind: typeof SUBGRAPH_KIND.Case;
      // null when this is the `default:` clause; otherwise the source
      // text of the case test expression.
      caseTest: string | null;
    })
  | (CommonSubgraphFields & {
      kind: typeof SUBGRAPH_KIND.IfElseContainer;
      hasElse: boolean;
    })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Switch })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.If })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Else })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Try })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Catch })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Finally })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.For })
  | (CommonSubgraphFields & { kind: typeof SUBGRAPH_KIND.Return });
