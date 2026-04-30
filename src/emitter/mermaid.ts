import type { SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";
import { buildVisualGraph } from "../visual-graph/builder.js";
import type {
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "../visual-graph/model.js";

export type MermaidRenderer = "dagre" | "elk";

export interface MermaidEmitterOptions {
  /**
   * Layout engine for the Mermaid flowchart. `"elk"` gives cleaner
   * nested-subgraph layouts but requires the `@mermaid-js/layout-elk`
   * loader to be registered in the consuming environment. `"dagre"` is
   * Mermaid's built-in default and is what unconfigured renderers (e.g.
   * GitHub markdown preview) fall back to. Required so callers must make
   * this choice explicitly — the default lives at the CLI / pipeline-default
   * boundary, not inside the emitter.
   */
  renderer: MermaidRenderer;
}

export class MermaidEmitter implements Emitter {
  readonly format = "mermaid";
  readonly contentType = "text/vnd.mermaid";

  private readonly renderer: MermaidRenderer;

  constructor(options: MermaidEmitterOptions) {
    this.renderer = options.renderer;
  }

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const graph = opts.prunedGraph ?? buildVisualGraph(ir);
    return renderMermaid(graph, this.renderer);
  }
}

function renderMermaid(graph: VisualGraph, renderer: MermaidRenderer): string {
  // dagre is Mermaid's built-in default; elk has to be opted into with an
  // init directive (and `mermaid.registerLayoutLoaders([elkLayouts])` on
  // the consuming side). dagre struggles with nested subgraphs that share
  // edges across boundaries (the function wrapper containing the
  // FunctionName node and the body subgraph, with edges reaching from
  // outside into the body) and produces colliding routes and inconsistent
  // node-vs-body ordering, which is why elk is the default here. dagre is
  // still selectable for environments that cannot register the elk loader
  // (e.g. GitHub's markdown preview).
  const lines: string[] = [];
  if (renderer === "elk") {
    lines.push('%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%');
  }
  lines.push(`flowchart ${graph.direction}`);
  if (graph.pruning !== undefined) {
    const summary = graph.pruning.roots
      .map((r) => `${r.query}(${r.matched})`)
      .join(", ");
    lines.push(
      `  %% pruning: roots=[${summary}] ancestors=${graph.pruning.ancestors} descendants=${graph.pruning.descendants}`,
    );
    for (const r of graph.pruning.roots) {
      if (r.matched === 0) {
        lines.push(`  %% pruning: warning: query '${r.query}' matched 0 roots`);
      }
    }
  }

  const nodeMap = new Map<string, VisualNode>();
  collectNodesInto(graph.elements, nodeMap);

  // FunctionName nodes that own a function subgraph are absorbed into a
  // wrapper subgraph alongside the body, so they must NOT also be emitted as
  // a sibling node at their declaring scope.
  const wrappedOwnerIds = new Set<string>();
  collectWrappedOwnerIds(graph.elements, wrappedOwnerIds);

  const wrapperIds: string[] = [];

  function emitNode(n: VisualNode, indent: string): void {
    lines.push(`${indent}${n.id}${nodeSyntax(n)}`);
  }

  function emitPlainSubgraph(sg: VisualSubgraph, indent: string): void {
    lines.push(`${indent}subgraph ${sg.id}["${subgraphLabel(sg, nodeMap)}"]`);
    const childIndent = `${indent}  `;
    lines.push(`${childIndent}direction ${sg.direction}`);
    for (const e of sg.elements) {
      if (e.type === "node" && !wrappedOwnerIds.has(e.id)) {
        emitNode(e, childIndent);
      }
    }
    for (const e of sg.elements) {
      if (e.type === "subgraph") {
        emitSubgraph(e, childIndent);
      }
    }
    lines.push(`${indent}end`);
  }

  function emitSubgraph(sg: VisualSubgraph, indent: string): void {
    if (sg.kind === "function" && sg.ownerNodeId !== undefined) {
      const ownerNode = nodeMap.get(sg.ownerNodeId);
      if (ownerNode !== undefined) {
        // Wrap the FunctionName node and the function body subgraph as
        // SIBLINGS inside a single wrapper subgraph. The FunctionName node
        // belongs to the parent scope (it names the function from the
        // outside), so it must NOT live inside the body subgraph — that
        // would imply "f references itself from within its own body".
        // The wrapper exists purely to keep these two siblings adjacent in
        // the rendered diagram.
        const wrapId = `wrap_${sg.id}`;
        wrapperIds.push(wrapId);
        lines.push(`${indent}subgraph ${wrapId}[" "]`);
        const wrapIndent = `${indent}  `;
        lines.push(`${wrapIndent}direction TB`);
        emitNode(ownerNode, wrapIndent);
        emitPlainSubgraph(sg, wrapIndent);
        lines.push(`${indent}end`);
        return;
      }
    }
    emitPlainSubgraph(sg, indent);
  }

  // Emit top-level "tree" nodes (anything that isn't a synthetic top-level
  // import/module/sink), then top-level subgraphs, then synthetic top-level
  // nodes — preserves the historical Mermaid output ordering and keeps the
  // module/intermediate cluster grouped near the import edges.
  const synthetic = (n: VisualNode): boolean =>
    n.kind === "ModuleSink" ||
    n.kind === "ModuleSource" ||
    n.kind === "ImportIntermediate";
  for (const e of graph.elements) {
    if (e.type === "node" && !synthetic(e) && !wrappedOwnerIds.has(e.id)) {
      emitNode(e, "  ");
    }
  }
  for (const e of graph.elements) {
    if (e.type === "subgraph") {
      emitSubgraph(e, "  ");
    }
  }

  // Edges originating from a synthetic node (ModuleSource / ImportIntermediate)
  // are import edges and rendered after the synthetic node block. Edges that
  // merely point INTO a synthetic node (e.g. `n_x -->|read| module_root`) stay
  // with the body edges to preserve the historical ordering.
  const importSources = new Set<string>();
  for (const n of nodeMap.values()) {
    if (n.kind === "ModuleSource" || n.kind === "ImportIntermediate") {
      importSources.add(n.id);
    }
  }
  const bodyEdges: VisualEdge[] = [];
  const importEdges: VisualEdge[] = [];
  for (const e of graph.edges) {
    if (importSources.has(e.from)) {
      importEdges.push(e);
    } else {
      bodyEdges.push(e);
    }
  }
  for (const e of bodyEdges) {
    lines.push(`  ${e.from} -->|${e.label}| ${e.to}`);
  }

  for (const e of graph.elements) {
    if (e.type === "node" && synthetic(e)) {
      emitNode(e, "  ");
    }
  }
  for (const e of importEdges) {
    lines.push(`  ${e.from} -->|${e.label}| ${e.to}`);
  }

  // Boundary edges: pruning detected one or more neighbors past the
  // requested radius. Mermaid cannot draw a truly dangling edge, so each
  // boundary edge gets a faint stub node "(...)" attached via a dashed
  // arrow. The stub stands in for "more graph keeps going beyond here".
  // The label question follows the edge semantics `from -label-> to`,
  // where the label describes the action `to` performs on `from`:
  //
  // - "out" (`inside -> stub`): the actor is the stub, which is unknown,
  //   so we cannot honestly attach a label.
  // - "in"  (`stub -> inside`): the actor is the kept inside node, so we
  //   keep the original label.
  const stubIds: string[] = [];
  if (graph.boundaryEdges !== undefined && graph.boundaryEdges.length > 0) {
    let stubCounter = 0;
    for (const be of graph.boundaryEdges) {
      stubCounter += 1;
      const stubId = `boundary_stub_${stubCounter}`;
      stubIds.push(stubId);
      lines.push(`  ${stubId}((…))`);
      if (be.direction === "out") {
        lines.push(`  ${be.inside} -.-> ${stubId}`);
      } else {
        lines.push(`  ${stubId} -.->|${be.label}| ${be.inside}`);
      }
    }
  }

  if (wrapperIds.length > 0) {
    // Distinct background so the function wrapper is visually separable
    // from the inner function body subgraph (otherwise both inherit the
    // same Mermaid cluster fill and the nesting becomes invisible).
    lines.push("  classDef fnWrap fill:#1a2030,stroke:#5a7d99;");
    for (const id of wrapperIds) {
      lines.push(`  class ${id} fnWrap;`);
    }
  }

  if (stubIds.length > 0) {
    lines.push(
      "  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;",
    );
    for (const id of stubIds) {
      lines.push(`  class ${id} boundaryStub;`);
    }
  }

  return `${lines.join("\n")}\n`;
}

function collectNodesInto(
  elements: VisualElement[],
  out: Map<string, VisualNode>,
): void {
  for (const e of elements) {
    if (e.type === "node") {
      out.set(e.id, e);
    } else {
      collectNodesInto(e.elements, out);
    }
  }
}

function collectWrappedOwnerIds(
  elements: VisualElement[],
  out: Set<string>,
): void {
  for (const e of elements) {
    if (e.type !== "subgraph") {
      continue;
    }
    if (e.kind === "function" && e.ownerNodeId !== undefined) {
      out.add(e.ownerNodeId);
    }
    collectWrappedOwnerIds(e.elements, out);
  }
}

function nodeSyntax(n: VisualNode): string {
  const label = nodeLabel(n);
  switch (n.kind) {
    case "WriteOp":
      return `(["${label}"])`;
    case "ReturnSink":
    case "ModuleSink":
      return `((${label}))`;
    default:
      return `["${label}"]`;
  }
}

function nodeLabel(n: VisualNode): string {
  const head = nodeHead(n);
  if (n.kind === "ReturnSink") {
    return "return";
  }
  if (n.kind === "ModuleSink") {
    return "module";
  }
  // Unused declarations are surfaced via a textual prefix instead of a
  // dashed border. This keeps the visual cue legible even when the node
  // already has another classDef applied (boundary stub, fnWrap, ...).
  const prefixed = n.unused === true ? `unused ${head}` : head;
  return `${prefixed}<br/>L${n.line}`;
}

function nodeHead(n: VisualNode): string {
  const name = escape(n.name);
  switch (n.kind) {
    case "FunctionName":
      return `${name}()`;
    case "ClassName":
      return `class ${name}`;
    case "ImportBinding": {
      const isRenamedNamed =
        n.importKind === "named" &&
        n.importedName !== null &&
        n.importedName !== undefined &&
        n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case "CatchClause":
      return `catch ${name}`;
    case "ImplicitGlobalVariable":
      return `global ${name}`;
    case "WriteOp":
      return n.declarationKind === "let" ? `let ${name}` : name;
    case "ModuleSource":
      return `module ${name}`;
    case "ImportIntermediate":
      return `import ${name}`;
    default:
      if (n.initIsFunction) {
        return `${name}()`;
      }
      if (n.declarationKind === "let") {
        return `let ${name}`;
      }
      return name;
  }
}

function subgraphLabel(
  sg: VisualSubgraph,
  nodeMap: Map<string, VisualNode>,
): string {
  switch (sg.kind) {
    case "function": {
      const ownerNode = sg.ownerNodeId
        ? nodeMap.get(sg.ownerNodeId)
        : undefined;
      return `${escape(ownerNode?.name ?? "")}()<br/>L${sg.line}`;
    }
    case "switch":
      return `switch L${sg.line}`;
    case "case":
      if (sg.caseTest === null || sg.caseTest === undefined) {
        return `default L${sg.line}`;
      }
      return `case ${escape(sg.caseTest)} L${sg.line}`;
    case "if":
      return `if L${sg.line}`;
    case "else":
      return `else L${sg.line}`;
    case "if-else-container":
      return `${sg.hasElse ? "if-else" : "if"} L${sg.line}`;
    case "try":
      return `try L${sg.line}`;
    case "catch":
      return `catch L${sg.line}`;
    case "finally":
      return `finally L${sg.line}`;
    case "for":
      return `for L${sg.line}`;
  }
}

function escape(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}
