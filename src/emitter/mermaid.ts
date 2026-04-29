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

export class MermaidEmitter implements Emitter {
  readonly format = "mermaid";
  readonly contentType = "text/vnd.mermaid";

  emit(ir: SerializedIR, _opts: EmitOptions): string {
    const graph = buildVisualGraph(ir);
    return renderMermaid(graph);
  }
}

function renderMermaid(graph: VisualGraph): string {
  // Use the elk layout engine instead of the default dagre. dagre struggles
  // with nested subgraphs that share edges across boundaries (function
  // wrapper containing the FunctionName node and the body subgraph, with
  // edges from outside reaching into the body) — it produces colliding
  // routes and inconsistent node-vs-body ordering. elk handles these cases
  // far better and is registered as a layout loader at module init time
  // (see src/index.ts).
  const lines: string[] = [
    '%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%',
    `flowchart ${graph.direction}`,
  ];

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

  if (wrapperIds.length > 0) {
    // Distinct background so the function wrapper is visually separable
    // from the inner function body subgraph (otherwise both inherit the
    // same Mermaid cluster fill and the nesting becomes invisible).
    lines.push("  classDef fnWrap fill:#1a2030,stroke:#5a7d99;");
    for (const id of wrapperIds) {
      lines.push(`  class ${id} fnWrap;`);
    }
  }

  const unusedIds: string[] = [];
  for (const n of nodeMap.values()) {
    if (n.unused) {
      unusedIds.push(n.id);
    }
  }
  if (unusedIds.length > 0) {
    lines.push("  classDef unused stroke-dasharray: 5 5;");
    for (const id of unusedIds) {
      lines.push(`  class ${id} unused;`);
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
  return `${head}<br/>L${n.line}`;
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
