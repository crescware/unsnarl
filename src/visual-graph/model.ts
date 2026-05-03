import type { Language, VariableDeclarationKind } from "../ir/model.js";
import type { IMPORT_KIND } from "../serializer/import-kind.js";
import type { Direction } from "./direction.js";
import type { NODE_KIND, NodeKind } from "./node-kind.js";
import type { BOUNDARY_EDGE_DIRECTION } from "./prune/boundary-edge-direction.js";
import type { SUBGRAPH_KIND, SubgraphKind } from "./subgraph-kind.js";
import type { VISUAL_ELEMENT_TYPE } from "./visual-element-type.js";

export type { Direction, NodeKind, SubgraphKind };

// Common shape across every kind. Mutable: builder.ts and the various
// builder/* helpers may patch endLine / unused after the node is first
// inserted into its container. Wrapping in Readonly would force a
// refactor of every post-construction patch site.
type CommonNodeFields = {
  type: typeof VISUAL_ELEMENT_TYPE.Node;
  id: string;
  name: string;
  line: number;
  // Set when the reference logically extends past its identifier line --
  // currently the JSX element case where <A>...</A> spans line..endLine.
  // Renderers display L{line}-{endLine} and prune treats line queries as
  // matching anywhere within the closed range. null when single-line.
  endLine: number | null;
  // Marks a reference whose identifier names a JSX element opening tag, so
  // renderers can wrap the label as `<Name>` regardless of whether the
  // element happens to be single-line (endLine is null).
  isJsxElement: boolean;
  unused: boolean;
};

export type VisualNode =
  | (CommonNodeFields & { kind: typeof NODE_KIND.FunctionName })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ClassName })
  | (CommonNodeFields & { kind: typeof NODE_KIND.Parameter })
  | (CommonNodeFields & { kind: typeof NODE_KIND.CatchClause })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ImplicitGlobalVariable })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ReturnUse })
  | (CommonNodeFields & { kind: typeof NODE_KIND.IfTest })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ModuleSink })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ModuleSource })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ImportIntermediate })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ExpressionStatement })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.Variable;
      declarationKind: VariableDeclarationKind | null;
      initIsFunction: boolean;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.WriteOp;
      declarationKind: VariableDeclarationKind | null;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Named;
      importedName: string;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Default;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ImportBinding;
      importKind: typeof IMPORT_KIND.Namespace;
    });

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

export type VisualElement = VisualNode | VisualSubgraph;

export type VisualEdge = Readonly<{
  from: string;
  to: string;
  label: string;
}>;

export type VisualGraphPruning = Readonly<{
  roots: readonly Readonly<{ query: string; matched: number }>[];
  descendants: number;
  ancestors: number;
}>;

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
