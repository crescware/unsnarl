import { DIRECTION } from "../../../visual-graph/direction.js";
import { SUBGRAPH_KIND } from "../../../visual-graph/subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";
import type { VisualSubgraph } from "../../../visual-graph/visual-subgraph.js";

const COMMON = {
  type: VISUAL_ELEMENT_TYPE.Subgraph,
  id: "s_x",
  line: 1,
  endLine: null,
  direction: DIRECTION.RL,
  elements: [],
} as const;

export function baseSubgraph(): Extract<
  VisualSubgraph,
  { kind: typeof SUBGRAPH_KIND.Function }
> {
  return {
    ...COMMON,
    kind: SUBGRAPH_KIND.Function,
    ownerNodeId: "n_owner",
    ownerName: "owner",
    elements: [],
  };
}

export function baseCaseSubgraph(): Extract<
  VisualSubgraph,
  { kind: typeof SUBGRAPH_KIND.Case }
> {
  return { ...COMMON, kind: SUBGRAPH_KIND.Case, caseTest: null, elements: [] };
}

export function baseIfElseContainerSubgraph(): Extract<
  VisualSubgraph,
  { kind: typeof SUBGRAPH_KIND.IfElseContainer }
> {
  return {
    ...COMMON,
    kind: SUBGRAPH_KIND.IfElseContainer,
    hasElse: false,
    elements: [],
  };
}

type PlainSubgraphKind =
  | typeof SUBGRAPH_KIND.Switch
  | typeof SUBGRAPH_KIND.If
  | typeof SUBGRAPH_KIND.Else
  | typeof SUBGRAPH_KIND.Try
  | typeof SUBGRAPH_KIND.Catch
  | typeof SUBGRAPH_KIND.Finally
  | typeof SUBGRAPH_KIND.For
  | typeof SUBGRAPH_KIND.Return;

export function basePlainSubgraph(
  kind: PlainSubgraphKind,
): Extract<VisualSubgraph, { kind: PlainSubgraphKind }> {
  return { ...COMMON, kind, elements: [] };
}
