import type { SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";
import { buildVisualGraph } from "../visual-graph/builder.js";
import type {
  VisualEdge,
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
  const lines: string[] = [`flowchart ${graph.direction}`];

  const nodesByParent = new Map<string | null, VisualNode[]>();
  for (const n of graph.nodes) {
    const arr = nodesByParent.get(n.parent) ?? [];
    arr.push(n);
    nodesByParent.set(n.parent, arr);
  }
  const subgraphsByParent = new Map<string | null, VisualSubgraph[]>();
  for (const s of graph.subgraphs) {
    const arr = subgraphsByParent.get(s.parent) ?? [];
    arr.push(s);
    subgraphsByParent.set(s.parent, arr);
  }

  function emitNode(n: VisualNode, indent: string): void {
    lines.push(`${indent}${n.id}${nodeSyntax(n)}`);
  }

  function emitSubgraph(sg: VisualSubgraph, indent: string): void {
    lines.push(`${indent}subgraph ${sg.id}["${subgraphLabel(sg)}"]`);
    const childIndent = `${indent}  `;
    lines.push(`${childIndent}direction ${sg.direction}`);
    const childNodes = nodesByParent.get(sg.id) ?? [];
    for (const cn of childNodes) {
      emitNode(cn, childIndent);
    }
    const childSubgraphs = subgraphsByParent.get(sg.id) ?? [];
    for (const cs of childSubgraphs) {
      emitSubgraph(cs, childIndent);
    }
    lines.push(`${indent}end`);
  }

  const topNodes = nodesByParent.get(null) ?? [];
  const topSubgraphs = subgraphsByParent.get(null) ?? [];

  // Emit top-level "tree" nodes (anything that isn't a synthetic top-level
  // import/module/sink), then top-level subgraphs, then synthetic top-level
  // nodes — preserves the historical Mermaid output ordering and keeps the
  // module/intermediate cluster grouped near the import edges.
  const synthetic = (n: VisualNode): boolean =>
    n.kind === "ModuleSink" ||
    n.kind === "ModuleSource" ||
    n.kind === "ImportIntermediate";
  for (const n of topNodes) {
    if (!synthetic(n)) {
      emitNode(n, "  ");
    }
  }
  for (const sg of topSubgraphs) {
    emitSubgraph(sg, "  ");
  }

  // Edges originating from a synthetic node (ModuleSource / ImportIntermediate)
  // are import edges and rendered after the synthetic node block. Edges that
  // merely point INTO a synthetic node (e.g. `n_x -->|read| module_root`) stay
  // with the body edges to preserve the historical ordering.
  const importSources = new Set<string>();
  for (const n of graph.nodes) {
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

  for (const n of topNodes) {
    if (synthetic(n)) {
      emitNode(n, "  ");
    }
  }
  for (const e of importEdges) {
    lines.push(`  ${e.from} -->|${e.label}| ${e.to}`);
  }

  const unusedIds: string[] = [];
  for (const n of graph.nodes) {
    if (n.unused) {
      unusedIds.push(n.id);
    }
  }
  for (const sg of graph.subgraphs) {
    if (sg.unused) {
      unusedIds.push(sg.id);
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

function subgraphLabel(sg: VisualSubgraph): string {
  switch (sg.kind) {
    case "function":
      return `${escape(sg.ownerName ?? "")}()<br/>L${sg.line}`;
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
