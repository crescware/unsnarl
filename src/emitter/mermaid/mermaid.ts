import type { SerializedIR } from "../../ir/model.js";
import type { EmitOptions, Emitter } from "../../pipeline/types.js";
import { buildVisualGraph } from "../../visual-graph/builder.js";
import type { VisualGraph, VisualNode } from "../../visual-graph/model.js";
import { collectEdgeEndpointIds } from "./collect-edge-endpoint-ids.js";
import { collectImportSources } from "./collect-import-sources.js";
import { collectNodesInto } from "./collect-nodes-into.js";
import { collectWrappedOwnerIds } from "./collect-wrapped-owner-ids.js";
import { pushEdgeLines } from "./push-edge-lines.js";
import { renderBoundaryEdges } from "./render-boundary-edges.js";
import { renderClassDefs } from "./render-class-defs.js";
import { renderPruningComment } from "./render-pruning-comment.js";
import type { RenderState } from "./render-state.js";
import { renderSyntheticNodeBlock } from "./render-synthetic-node-block.js";
import { renderTopLevelNodes } from "./render-top-level-nodes.js";
import { renderTopLevelSubgraphs } from "./render-top-level-subgraphs.js";
import { splitEdges } from "./split-edges.js";
import type { MermaidStrategy } from "./strategy/strategy.js";

export type { CliMermaidRenderer as MermaidRenderer } from "../../cli-mermaid-renderer.js";

export type MermaidEmitterOptions = Readonly<{
  /**
   * Renderer-specific strategy. The strategy carries the preamble (e.g. the
   * `%%{init: ...}%%` directive that elk needs), the empty-subgraph patch
   * that papers over a layout-elk crash, and any trailer lines (classDef /
   * class declarations) that reference placeholders the strategy injected.
   * Required so callers must make this choice explicitly — the default
   * lives at the CLI / pipeline-default boundary, not inside the emitter.
   * See `dagreStrategy` / `elkStrategy` in `./mermaid-strategy.js`.
   */
  strategy: MermaidStrategy;
}>;

export class MermaidEmitter implements Emitter {
  readonly format = "mermaid";
  readonly contentType = "text/vnd.mermaid";
  readonly extension = "mmd";

  private readonly strategy: MermaidStrategy;

  constructor(options: MermaidEmitterOptions) {
    this.strategy = options.strategy;
  }

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const graph = opts.prunedGraph ?? buildVisualGraph(ir);
    return renderMermaid(graph, this.strategy);
  }
}

function renderMermaid(graph: VisualGraph, strategy: MermaidStrategy): string {
  // The strategy decides which renderer-specific lines (e.g. the elk init
  // directive) and which empty-subgraph patches are needed. dagre struggles
  // with nested subgraphs that share edges across boundaries (the function
  // wrapper containing the FunctionName node and the body subgraph, with
  // edges reaching from outside into the body) and produces colliding
  // routes and inconsistent node-vs-body ordering, which is why elk is the
  // default at the CLI / pipeline boundary. dagre is still selectable for
  // environments that cannot register the elk loader (e.g. GitHub's
  // markdown preview).
  const lines: /* mutable */ string[] = [];
  for (const l of strategy.preambleLines) {
    lines.push(l);
  }
  lines.push(`flowchart ${graph.direction}`);
  renderPruningComment(graph, lines);

  const nodeMap = new Map<string, VisualNode>();
  collectNodesInto(graph.elements, nodeMap);

  // FunctionName nodes that own a function subgraph are absorbed into a
  // wrapper subgraph alongside the body, so they must NOT also be emitted
  // as a sibling node at their declaring scope.
  const wrappedOwnerIds = new Set<string>();
  collectWrappedOwnerIds(graph.elements, wrappedOwnerIds);

  // Pre-compute the set of ids that appear as edge endpoints. The
  // emptySubgraphPlaceholder hook uses this to decide whether a workaround
  // is needed (the layout-elk crash only triggers during edge processing,
  // so subgraphs that are not edge endpoints don't need the patch even
  // under elk).
  const edgeEndpointIds = collectEdgeEndpointIds(graph.edges);

  const state = {
    lines,
    nodeMap,
    wrappedOwnerIds,
    edgeEndpointIds,
    placeholderIds: [],
    wrapperIds: [],
    strategy,
  } as const satisfies RenderState;

  // Emit top-level "tree" nodes (anything that isn't a synthetic top-level
  // import/module/sink), then top-level subgraphs, then synthetic top-level
  // nodes -- preserves the historical Mermaid output ordering and keeps the
  // module/intermediate cluster grouped near the import edges.
  renderTopLevelNodes(state, graph);
  renderTopLevelSubgraphs(state, graph);

  // Edges originating from a synthetic node (ModuleSource / ImportIntermediate)
  // are import edges and rendered after the synthetic node block. Edges that
  // merely point INTO a synthetic node (e.g. `n_x -->|read| module_root`) stay
  // with the body edges to preserve the historical ordering.
  const importSources = collectImportSources(nodeMap);
  const { body: bodyEdges, imports: importEdges } = splitEdges(
    graph.edges,
    importSources,
  );
  pushEdgeLines(bodyEdges, lines);

  renderSyntheticNodeBlock(state, graph);
  pushEdgeLines(importEdges, lines);

  const stubIds: /* mutable */ string[] = [];
  renderBoundaryEdges(graph, lines, stubIds);

  renderClassDefs(state.wrapperIds, stubIds, lines);

  for (const l of strategy.trailerLines(state.placeholderIds)) {
    lines.push(l);
  }

  return `${lines.join("\n")}\n`;
}
