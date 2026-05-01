import type {
  ImportKind,
  Language,
  VariableDeclarationKind,
} from "../ir/model.js";

export type Direction = "RL" | "LR" | "TB" | "BT";

export type NodeKind =
  | "Variable"
  | "FunctionName"
  | "ClassName"
  | "Parameter"
  | "CatchClause"
  | "ImportBinding"
  | "ImplicitGlobalVariable"
  | "WriteOp"
  | "ReturnUse"
  | "ModuleSink"
  | "ModuleSource"
  | "ImportIntermediate";

export type SubgraphKind =
  | "function"
  | "switch"
  | "case"
  | "if"
  | "else"
  | "if-else-container"
  | "try"
  | "catch"
  | "finally"
  | "for"
  | "return";

export type VisualNode = {
  type: "node";
  id: string;
  kind: NodeKind;
  name: string;
  line: number;
  // Set when the reference logically extends past its identifier line --
  // currently the JSX element case where <A>...</A> spans line..endLine.
  // Renderers display L{line}-{endLine} and prune treats line queries as
  // matching anywhere within the closed range.
  endLine?: number;
  // Marks a reference whose identifier names a JSX element opening tag, so
  // renderers can wrap the label as `<Name>` regardless of whether the
  // element happens to be single-line (no endLine).
  isJsxElement: boolean;
  unused?: boolean;
  declarationKind?: VariableDeclarationKind;
  initIsFunction?: boolean;
  importKind?: ImportKind;
  importedName?: string | null;
  importSource?: string;
};

export type VisualSubgraph = {
  type: "subgraph";
  id: string;
  kind: SubgraphKind;
  line: number;
  endLine?: number;
  direction: Direction;
  caseTest?: string | null;
  hasElse?: boolean;
  ownerNodeId?: string;
  // Mirrors the owner node's display name so the subgraph label survives
  // pruning even when the owner node itself gets cut out of the graph.
  ownerName?: string;
  elements: VisualElement[];
};

export type VisualElement = VisualNode | VisualSubgraph;

export type VisualEdge = {
  from: string;
  to: string;
  label: string;
};

export type VisualGraphPruning = {
  roots: readonly { query: string; matched: number }[];
  descendants: number;
  ancestors: number;
};

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
  | { inside: string; direction: "out" }
  | { inside: string; direction: "in"; label: string };

export type VisualGraph = {
  version: 1;
  source: { path: string; language: Language };
  direction: Direction;
  elements: VisualElement[];
  edges: VisualEdge[];
  boundaryEdges?: readonly VisualBoundaryEdge[];
  pruning?: VisualGraphPruning;
};
