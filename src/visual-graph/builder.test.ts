import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../analyzer/definition-type.js";
import { EslintCompatAnalyzer } from "../analyzer/eslint-compat/eslint-compat.js";
import { LANGUAGE, type Language } from "../cli/language.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { FlatSerializer } from "../serializer/flat/flat-serializer.js";
import { IMPORT_KIND } from "../serializer/import-kind.js";
import { SERIALIZED_IR_VERSION } from "../serializer/serialized-ir-version.js";
import { buildVisualGraph } from "./builder.js";
import { DIRECTION } from "./direction.js";
import { NODE_KIND } from "./node-kind.js";
import { SUBGRAPH_KIND } from "./subgraph-kind.js";
import type { VisualEdge } from "./visual-edge.js";
import { VISUAL_ELEMENT_TYPE } from "./visual-element-type.js";
import type { VisualElement } from "./visual-element.js";
import type { VisualGraph } from "./visual-graph.js";
import type { VisualNode } from "./visual-node.js";
import type { VisualSubgraph } from "./visual-subgraph.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();

function build(code: string, language: Language = LANGUAGE.Ts): VisualGraph {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  const analyzed = analyzer.analyze(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: `input.${language}`, language },
  });
  return buildVisualGraph(ir);
}

function flattenNodes(elements: VisualElement[]): readonly VisualNode[] {
  const out: /* mutable */ VisualNode[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      out.push(e);
    } else {
      out.push(...flattenNodes(e.elements));
    }
  }
  return out;
}

function flattenSubgraphs(
  elements: VisualElement[],
): readonly VisualSubgraph[] {
  const out: /* mutable */ VisualSubgraph[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      out.push(e);
      out.push(...flattenSubgraphs(e.elements));
    }
  }
  return out;
}

function findSubgraphs<K extends VisualSubgraph["kind"]>(
  graph: VisualGraph,
  kind: K,
): readonly Extract<VisualSubgraph, { kind: K }>[] {
  return flattenSubgraphs(graph.elements).filter(
    (s): s is Extract<VisualSubgraph, { kind: K }> => s.kind === kind,
  );
}

function findNodes<K extends VisualNode["kind"]>(
  graph: VisualGraph,
  kind: K,
): readonly Extract<VisualNode, { kind: K }>[] {
  return flattenNodes(graph.elements).filter(
    (n): n is Extract<VisualNode, { kind: K }> => n.kind === kind,
  );
}

function nodeByName(graph: VisualGraph, name: string): VisualNode | null {
  return flattenNodes(graph.elements).find((n) => n.name === name) ?? null;
}

function variableByName(
  graph: VisualGraph,
  name: string,
): Extract<VisualNode, { kind: typeof NODE_KIND.Variable }> | null {
  return (
    findNodes(graph, NODE_KIND.Variable).find((n) => n.name === name) ?? null
  );
}

function importBindingByName(
  graph: VisualGraph,
  name: string,
): Extract<VisualNode, { kind: typeof NODE_KIND.ImportBinding }> | null {
  return (
    findNodes(graph, NODE_KIND.ImportBinding).find((n) => n.name === name) ??
    null
  );
}

function edgesFrom(graph: VisualGraph, fromId: string): readonly VisualEdge[] {
  return graph.edges.filter((e) => e.from === fromId);
}

function edgesTo(graph: VisualGraph, toId: string): readonly VisualEdge[] {
  return graph.edges.filter((e) => e.to === toId);
}

function childSubgraphsOf(sg: VisualSubgraph): readonly VisualSubgraph[] {
  return sg.elements.filter(
    (e): e is VisualSubgraph => e.type === VISUAL_ELEMENT_TYPE.Subgraph,
  );
}

describe("buildVisualGraph: top-level structure", () => {
  test("top-level metadata mirrors the IR source path/language and direction is RL", () => {
    const g = build("const a = 1;\n");
    expect(g.version).toBe(SERIALIZED_IR_VERSION);
    expect(g.source.path).toBe("input.ts");
    expect(g.source.language).toBe("ts");
    expect(g.direction).toBe(DIRECTION.RL);
  });

  test("an empty source produces an empty graph", () => {
    const g = build("");
    expect(g.elements).toEqual([]);
    expect(g.edges).toEqual([]);
  });

  test("a single const declaration emits exactly one Variable node and no edges", () => {
    const g = build("const a = 1;\n");
    const nodes = findNodes(g, NODE_KIND.Variable);
    expect(nodes).toHaveLength(1);
    expect(nodes[0]?.name).toBe("a");
    expect(g.edges).toEqual([]);
  });
});

describe("buildVisualGraph: variable nodes", () => {
  test("classifies functions, classes, parameters, catch params, and imports by NodeKind", () => {
    const g = build(
      [
        "import imp from 'x';",
        "function foo(p) { try { p; } catch (e) { e; } }",
        "class Bar {}",
      ].join("\n"),
    );
    expect(findNodes(g, NODE_KIND.ImportBinding).map((n) => n.name)).toContain(
      "imp",
    );
    expect(findNodes(g, NODE_KIND.FunctionName).map((n) => n.name)).toContain(
      "foo",
    );
    expect(findNodes(g, NODE_KIND.Parameter).map((n) => n.name)).toContain("p");
    expect(findNodes(g, NODE_KIND.CatchClause).map((n) => n.name)).toContain(
      "e",
    );
    expect(findNodes(g, NODE_KIND.ClassName).map((n) => n.name)).toContain(
      "Bar",
    );
  });

  test("unused declarations carry the unused flag; declarations with readers do not", () => {
    const g = build("const a = 1;\nconst b = a;\n");
    expect(nodeByName(g, "a")?.unused).toBe(false);
    expect(nodeByName(g, "b")?.unused).toBe(true);
  });

  test("declarationKind is preserved on Variable nodes for let / const (var is intentionally skipped)", () => {
    const g = build("let a = 1;\nconst b = 2;\n");
    expect(variableByName(g, "a")?.declarationKind).toBe("let");
    expect(variableByName(g, "b")?.declarationKind).toBe("const");
  });

  test("a const initialised by a function expression is marked initIsFunction", () => {
    const g = build("const fn = function () {};\n");
    expect(variableByName(g, "fn")?.initIsFunction).toBe(true);
  });

  test("ImplicitGlobalVariable is kept as a node regardless of whether refs are receiver-only", () => {
    const directRead = build("function f() { return globalThing; }\n");
    expect(
      nodeByName(directRead, "globalThing")?.kind ===
        NODE_KIND.ImplicitGlobalVariable,
    ).toBe(true);

    const onlyReceiver = build("const x = Object.keys({});\n");
    expect(nodeByName(onlyReceiver, "Object")?.kind).toBe(
      NODE_KIND.ImplicitGlobalVariable,
    );
  });

  test("named imports renamed at the import site keep the local name on the node", () => {
    const g = build("import { other as renamed } from 'm';\nvoid renamed;\n");
    const node = importBindingByName(g, "renamed");
    expect(node?.kind).toBe(DEFINITION_TYPE.ImportBinding);
    expect(node?.importKind).toBe(IMPORT_KIND.Named);
    if (node?.importKind === IMPORT_KIND.Named) {
      expect(node.importedName).toBe("other");
    }
  });
});

describe("buildVisualGraph: function subgraphs", () => {
  test("a FunctionDeclaration becomes a function subgraph with ownerNodeId pointing to the FunctionName", () => {
    const g = build("function add(a, b) { return a + b; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn).toBeDefined();
    expect(fn?.ownerNodeId).toBeDefined();
    const ownerNode = flattenNodes(g.elements).find(
      (n) => n.id === fn?.ownerNodeId,
    );
    expect(ownerNode?.name).toBe("add");
    expect(ownerNode?.kind).toBe(DEFINITION_TYPE.FunctionName);
  });

  test("function subgraph mirrors the owner's name as ownerName so labels survive when pruning drops the owner node", () => {
    const g = build("function add(a, b) { return a + b; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn?.ownerName).toBe("add");
  });

  test("an arrow function const, function expression const, and FunctionDeclaration all subgraph alike", () => {
    for (const code of [
      "const fn = (p) => p;",
      "const fn = function (p) { return p; };",
      "function fn(p) { return p; }",
    ]) {
      const g = build(code + "\n");
      expect(findSubgraphs(g, "function")).toHaveLength(1);
    }
  });

  test("function subgraph line range covers the whole function block", () => {
    const g = build("function f() {\n  return 1;\n}\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn?.line).toBe(1);
    expect(fn?.endLine).toBe(3);
  });

  test("a single-line function reports endLine equal to line (renderers collapse equal ranges)", () => {
    const g = build("function f() { return 1; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn?.line).toBe(1);
    expect(fn?.endLine).toBe(1);
  });
});

describe("buildVisualGraph: control subgraphs", () => {
  test("a try/catch/finally produces three sibling subgraphs with ascending line ranges", () => {
    const g = build(
      [
        "let v = 0;",
        "try {",
        "  v = 1;",
        "} catch (err) {",
        "  v = 2;",
        "} finally {",
        "  v = 3;",
        "}",
      ].join("\n"),
    );
    const tryS = findSubgraphs(g, "try")[0];
    const catchS = findSubgraphs(g, "catch")[0];
    const finallyS = findSubgraphs(g, "finally")[0];
    expect(tryS).toEqual(
      expect.objectContaining({ line: 2, endLine: 4, kind: SUBGRAPH_KIND.Try }),
    );
    expect(catchS).toEqual(
      expect.objectContaining({
        line: 4,
        endLine: 6,
        kind: SUBGRAPH_KIND.Catch,
      }),
    );
    expect(finallyS).toEqual(
      expect.objectContaining({
        line: 6,
        endLine: 8,
        kind: SUBGRAPH_KIND.Finally,
      }),
    );
  });

  test("an if without else has no if-else container and the predicate flows to the if-test anchor", () => {
    const g = build(
      [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "}",
      ].join("\n"),
    );
    expect(findSubgraphs(g, "if-else-container")).toHaveLength(0);
    const ifS = findSubgraphs(g, "if")[0];
    expect(ifS?.line).toBe(3);
    const anchor = findNodes(g, NODE_KIND.IfTest)[0];
    expect(anchor?.line).toBe(3);
    const flag = nodeByName(g, "flag");
    expect(flag).toBeDefined();
    expect(
      g.edges.some(
        (e) => e.from === flag?.id && e.to === anchor?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("an if/else pair lives inside an if-else container whose range spans both arms", () => {
    const g = build(
      [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "} else {",
        "  counter = 2;",
        "}",
      ].join("\n"),
    );
    const container = findSubgraphs(g, "if-else-container")[0];
    expect(container?.line).toBe(3);
    expect(container?.endLine).toBe(7);
    expect(container?.hasElse).toBe(true);
    const childKinds = childSubgraphsOf(container!).map((s) => s.kind);
    expect(childKinds).toEqual(["if", "else"]);
  });

  test("switch / case / default subgraphs preserve caseTest as raw source text", () => {
    const g = build(
      [
        "let l = 0;",
        "const k = 1;",
        "switch (k) {",
        "  case 0:",
        "    l = 1;",
        "    break;",
        "  default:",
        "    l = 2;",
        "}",
      ].join("\n"),
    );
    const sw = findSubgraphs(g, "switch")[0];
    expect(sw?.line).toBe(3);
    expect(sw?.endLine).toBe(9);
    const cases = findSubgraphs(g, "case");
    expect(cases).toHaveLength(2);
    const tests = cases.map((c) => c.caseTest);
    // The default clause is also a "case" subgraph but with caseTest = null;
    // the numeric "0" case keeps its raw text.
    expect(tests).toContain("0");
    expect(tests).toContain(null);
  });

  test("for loop bodies become for subgraphs", () => {
    const g = build("for (let i = 0; i < 1; i++) { i; }\n");
    expect(findSubgraphs(g, "for")).toHaveLength(1);
  });
});

describe("buildVisualGraph: write operations and let-chain edges", () => {
  test("each let assignment becomes its own WriteOp node and they form a set chain", () => {
    const g = build("let v = 0;\nv = 1;\nv = 2;\n");
    const writeOps = findNodes(g, NODE_KIND.WriteOp);
    expect(writeOps.map((w) => w.name)).toEqual(["v", "v"]);
    const setEdges = g.edges.filter((e) => e.label === "set");
    // n_v -> wr0 and wr0 -> wr1
    expect(setEdges).toHaveLength(2);
  });

  test("WriteOp nodes carry the declarationKind of the variable being mutated", () => {
    const g = build("let v = 0;\nv = 1;\n");
    const wr = findNodes(g, NODE_KIND.WriteOp)[0];
    expect(wr?.declarationKind).toBe("let");
  });

  test("a switch case that falls through emits a |fallthrough| edge to the next case's WriteOp", () => {
    const g = build(
      [
        'let l = "";',
        'const k = "a";',
        "switch (k) {",
        '  case "a":',
        '    l = "first";',
        '  case "b":',
        '    l = "second";',
        "    break;",
        "}",
      ].join("\n"),
    );
    expect(g.edges.some((e) => e.label === "fallthrough")).toBe(true);
  });
});

describe("buildVisualGraph: read origin edges", () => {
  test("a const initialised from another variable produces one read edge from the source", () => {
    const g = build("const a = 1;\nconst b = a;\n");
    const a = nodeByName(g, "a");
    const b = nodeByName(g, "b");
    expect(
      g.edges.some(
        (e) => e.from === a?.id && e.to === b?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("call expressions produce read,call edges to the variable that consumes the result", () => {
    const g = build("function f() {}\nconst x = f();\n");
    const f = nodeByName(g, "f");
    const x = nodeByName(g, "x");
    expect(
      g.edges.some(
        (e) => e.from === f?.id && e.to === x?.id && e.label === "read,call",
      ),
    ).toBe(true);
  });

  test("after if/else both branches' last writes feed a post-merge read of the same variable", () => {
    const g = build(
      [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "} else {",
        "  counter = 2;",
        "}",
        "const result = counter;",
      ].join("\n"),
    );
    const result = nodeByName(g, "result");
    const reads = edgesTo(g, result!.id).filter((e) => e.label === "read");
    // One read edge per branch's last write.
    expect(reads).toHaveLength(2);
    const writeOps = findNodes(g, NODE_KIND.WriteOp);
    const writeOpIds = new Set(writeOps.map((n) => n.id));
    expect(reads.every((e) => writeOpIds.has(e.from))).toBe(true);
  });

  test("an if without else also keeps an edge from the pre-if state of the variable", () => {
    const g = build(
      [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "}",
        "const result = counter;",
      ].join("\n"),
    );
    const result = nodeByName(g, "result");
    const reads = edgesTo(g, result!.id).filter((e) => e.label === "read");
    // One from the if-branch write, one from the original declaration.
    expect(reads).toHaveLength(2);
  });

  test("switch cases that fall through skip their own contribution; the next case takes over the write", () => {
    const g = build(
      [
        'let l = "";',
        'const k = "a";',
        "switch (k) {",
        '  case "a":',
        '    l = "x";',
        '  case "b":',
        '    l = "y";',
        "    break;",
        "  default:",
        '    l = "z";',
        "}",
        "const result = l;",
      ].join("\n"),
    );
    const result = nodeByName(g, "result");
    const reads = edgesTo(g, result!.id).filter((e) => e.label === "read");
    // case "a" falls through into case "b"; its write is taken over by "b".
    // So result reads from case "b" and default only (2 edges).
    expect(reads).toHaveLength(2);
  });

  test("a switch case ending in return cannot reach a read that follows the switch", () => {
    const g = build(
      [
        "function classify(k) {",
        '  let label = "";',
        "  switch (k) {",
        '    case "a":',
        '      label = "alpha";',
        "      return label;", // case "a" exits the function
        '    case "b":',
        '      label = "beta";',
        "      break;",
        "    default:",
        '      label = "other";',
        "  }",
        "  return label;", // post-switch read
        "}",
      ].join("\n"),
    );
    // Find the post-switch return-use node for label.
    const postSwitchReturnUse = findNodes(g, NODE_KIND.ReturnUse).find(
      (n) => n.line === 13,
    );
    expect(postSwitchReturnUse).toBeDefined();
    const incoming = edgesTo(g, postSwitchReturnUse!.id);
    // case "a" must not contribute (it exits the function).
    const writeOps = findNodes(g, NODE_KIND.WriteOp);
    const aWrite = writeOps.find((w) => w.line === 5);
    expect(aWrite).toBeDefined();
    expect(incoming.some((e) => e.from === aWrite?.id)).toBe(false);
    // case "b" (break) and default (fallthrough end) must contribute.
    const bWrite = writeOps.find((w) => w.line === 8);
    const dWrite = writeOps.find((w) => w.line === 11);
    expect(incoming.some((e) => e.from === bWrite?.id)).toBe(true);
    expect(incoming.some((e) => e.from === dWrite?.id)).toBe(true);
  });
});

describe("buildVisualGraph: return subgraphs", () => {
  test("a ReturnStatement yields a return subgraph with one ReturnUse per ownerless ref", () => {
    const g = build("function f(a, b) { return a + b; }\n");
    const ret = findSubgraphs(g, "return")[0];
    expect(ret).toBeDefined();
    const uses = ret!.elements.filter(
      (e) =>
        e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === NODE_KIND.ReturnUse,
    );
    expect(uses).toHaveLength(2);
  });

  test("each ReturnStatement gets its own subgraph (sibling returns are not merged)", () => {
    const g = build(
      [
        "function pick(k) {",
        "  const a = 1;",
        "  const b = 2;",
        "  if (k) { return a; }",
        "  return b;",
        "}",
      ].join("\n"),
    );
    const returns = findSubgraphs(g, "return");
    expect(returns).toHaveLength(2);
    const lines = returns.map((r) => r.line).sort((x, y) => x - y);
    expect(lines).toEqual([4, 5]);
  });

  test("a return inside an if branch nests in the if subgraph, not in the function subgraph", () => {
    const g = build(
      [
        "function pick(k) {",
        "  const a = 1;",
        "  if (k) {",
        "    return a;",
        "  }",
        "  return a;",
        "}",
      ].join("\n"),
    );
    const fn = findSubgraphs(g, "function")[0];
    const ifS = findSubgraphs(g, "if")[0];
    const fnDirectReturns = childSubgraphsOf(fn!).filter(
      (s) => s.kind === SUBGRAPH_KIND.Return,
    );
    const ifDirectReturns = childSubgraphsOf(ifS!).filter(
      (s) => s.kind === SUBGRAPH_KIND.Return,
    );
    expect(fnDirectReturns).toHaveLength(1);
    expect(ifDirectReturns).toHaveLength(1);
    expect(fnDirectReturns[0]?.line).toBe(6);
    expect(ifDirectReturns[0]?.line).toBe(4);
  });

  test("a return inside a switch case nests inside the case subgraph", () => {
    const g = build(
      [
        "function classify(k) {",
        "  switch (k) {",
        '    case "a":',
        "      return 1;", // literal — no ret_use, but still fixes nesting
        "    default:",
        "      const x = 2;",
        "      return x;",
        "  }",
        "}",
      ].join("\n"),
    );
    const cases = findSubgraphs(g, "case");
    // The default case body has a ReturnUse for x; check it is nested there.
    const caseWithReturnUse = cases.find((c) =>
      childSubgraphsOf(c).some((s) => s.kind === SUBGRAPH_KIND.Return),
    );
    expect(caseWithReturnUse).toBeDefined();
    expect(caseWithReturnUse?.caseTest).toBe(null);
  });

  test("an arrow with an expression body uses the body span as the implicit return container", () => {
    const g = build(["const fn = (x) => (", "  x + 1", ");"].join("\n"));
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toBe(1);
    expect(ret?.endLine).toBe(3);
  });

  test("an arrow with a block body falls through to the inner ReturnStatement's span", () => {
    const g = build(["const fn = (x) => {", "  return x;", "};"].join("\n"));
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toBe(2);
    expect(ret?.endLine).toBeNull();
  });

  test("a function with no ownerless refs in its body produces no return subgraph", () => {
    const g = build("function f() { return 1; }\n"); // literal only
    expect(findSubgraphs(g, "return")).toHaveLength(0);
  });

  test("a single-line return subgraph leaves endLine null (rendered as plain L<n>)", () => {
    const g = build("function f(x) { return x; }\n");
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toBe(1);
    expect(ret?.endLine).toBeNull();
  });

  test("ownerless refs flow into ReturnUse nodes via |read| edges", () => {
    const g = build("function f(a) { return a; }\n");
    const aParam = nodeByName(g, "a");
    const ret = findSubgraphs(g, "return")[0];
    const retUse = ret?.elements.find(
      (e) =>
        e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === NODE_KIND.ReturnUse,
    );
    expect(
      g.edges.some(
        (e) =>
          e.from === aParam?.id && e.to === retUse?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("ReturnUse for a multi-line JSX opening tag carries isJsxElement and the closing-tag endLine", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => (",
      "  <A>",
      "    hello",
      "  </A>",
      ");",
    ].join("\n");
    const g = build(code, "tsx");
    const retUses = findNodes(g, NODE_KIND.ReturnUse);
    const a = retUses.find((n) => n.name === "A");
    expect(a?.isJsxElement).toBe(true);
    expect(a?.line).toBe(3);
    expect(a?.endLine).toBe(5);
  });

  test("a single-line JSX element still flags isJsxElement, and endLine stays null", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => <A>hi</A>;",
    ].join("\n");
    const g = build(code, "tsx");
    const a = findNodes(g, NODE_KIND.ReturnUse).find((n) => n.name === "A");
    expect(a?.isJsxElement).toBe(true);
    expect(a?.endLine).toBeNull();
  });

  test("non-JSX ReturnUse stays isJsxElement=false with endLine null", () => {
    const g = build("function f(a) { return a; }\n");
    const ret = findSubgraphs(g, "return")[0];
    const retUse =
      ret?.elements.find(
        (e): e is VisualNode =>
          e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === NODE_KIND.ReturnUse,
      ) ?? null;
    expect(retUse?.isJsxElement).toBe(false);
    expect(retUse?.endLine).toBeNull();
  });
});

describe("buildVisualGraph: imports", () => {
  test("default imports get a single ModuleSource and one read edge into the local binding", () => {
    const g = build("import def from 'lib';\nvoid def;\n");
    const moduleSource = findNodes(g, NODE_KIND.ModuleSource)[0];
    expect(moduleSource?.name).toBe("lib");
    const def = nodeByName(g, "def");
    expect(
      g.edges.some((e) => e.from === moduleSource?.id && e.to === def?.id),
    ).toBe(true);
  });

  test("renamed named imports introduce an ImportIntermediate carrying the original name", () => {
    const g = build(
      ["import { other as renamed } from 'lib';", "void renamed;"].join("\n"),
    );
    const intermediates = findNodes(g, NODE_KIND.ImportIntermediate);
    expect(intermediates).toHaveLength(1);
    expect(intermediates[0]?.name).toBe("other");
    // ModuleSource -> Intermediate -> local binding
    const moduleSource = findNodes(g, NODE_KIND.ModuleSource)[0];
    const renamed = nodeByName(g, "renamed");
    expect(
      g.edges.some(
        (e) => e.from === moduleSource?.id && e.to === intermediates[0]?.id,
      ),
    ).toBe(true);
    expect(
      g.edges.some(
        (e) => e.from === intermediates[0]?.id && e.to === renamed?.id,
      ),
    ).toBe(true);
  });

  test("namespace imports point directly from the module to the local binding (no intermediate)", () => {
    const g = build("import * as ns from 'lib';\nvoid ns;\n");
    expect(findNodes(g, NODE_KIND.ImportIntermediate)).toHaveLength(0);
    const moduleSource = findNodes(g, NODE_KIND.ModuleSource)[0];
    const ns = nodeByName(g, "ns");
    expect(
      g.edges.some((e) => e.from === moduleSource?.id && e.to === ns?.id),
    ).toBe(true);
  });
});

describe("buildVisualGraph: predicate references", () => {
  test("a switch discriminant identifier feeds the switch subgraph itself", () => {
    const g = build(
      [
        "let l = 0;",
        "const k = 1;",
        "switch (k) { case 1: l = 1; break; default: l = 2; }",
      ].join("\n"),
    );
    const k = nodeByName(g, "k");
    const sw = findSubgraphs(g, "switch")[0];
    expect(
      g.edges.some(
        (e) => e.from === k?.id && e.to === sw?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("an if/else predicate identifier feeds the if-test anchor inside the consequent (`if`) branch", () => {
    const g = build(
      [
        "let v = 0;",
        "const flag = true;",
        "if (flag) { v = 1; } else { v = 2; }",
      ].join("\n"),
    );
    const flag = nodeByName(g, "flag");
    const container = findSubgraphs(g, "if-else-container")[0];
    const anchor = findNodes(g, NODE_KIND.IfTest)[0];
    expect(anchor).toBeDefined();
    const ifBranch = container?.elements.find(
      (e): e is VisualSubgraph => e.type === "subgraph" && e.kind === "if",
    );
    expect(ifBranch?.elements).toContainEqual(
      expect.objectContaining({ id: anchor?.id }),
    );
    const elseBranch = container?.elements.find(
      (e): e is VisualSubgraph => e.type === "subgraph" && e.kind === "else",
    );
    expect(
      elseBranch?.elements.every(
        (e) => !(e.type === "node" && e.kind === NODE_KIND.IfTest),
      ),
    ).toBe(true);
    expect(
      g.edges.some(
        (e) => e.from === flag?.id && e.to === anchor?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("a bare if predicate identifier feeds the if-test anchor (no container)", () => {
    const g = build(
      ["let v = 0;", "const flag = true;", "if (flag) { v = 1; }"].join("\n"),
    );
    const flag = nodeByName(g, "flag");
    const anchor = findNodes(g, NODE_KIND.IfTest)[0];
    expect(anchor).toBeDefined();
    expect(
      g.edges.some(
        (e) => e.from === flag?.id && e.to === anchor?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("an else-if chain has one if-test anchor per IfStatement, each inside its corresponding `if` (consequent) branch", () => {
    const g = build(
      [
        "function classify(n) {",
        "  let label;",
        "  if (n > 0) {",
        "    label = 'positive';",
        "  } else if (n < 0) {",
        "    label = 'negative';",
        "  } else {",
        "    label = 'zero';",
        "  }",
        "  return label;",
        "}",
      ].join("\n"),
    );
    const container = findSubgraphs(g, "if-else-container")[0];
    expect(container).toBeDefined();
    const anchors = findNodes(g, NODE_KIND.IfTest);
    expect(anchors).toHaveLength(2);
    const lines = anchors.map((a) => a.line).sort((a, b) => a - b);
    expect(lines).toEqual([3, 5]);
    const ifBranches = (container?.elements ?? []).filter(
      (e): e is VisualSubgraph => e.type === "subgraph" && e.kind === "if",
    );
    for (const anchor of anchors) {
      const hosts = ifBranches.filter((b) =>
        b.elements.some((e) => e.type === "node" && e.id === anchor.id),
      );
      expect(hosts).toHaveLength(1);
    }
    const elseBranch = container?.elements.find(
      (e): e is VisualSubgraph => e.type === "subgraph" && e.kind === "else",
    );
    expect(
      elseBranch?.elements.every(
        (e) => !(e.type === "node" && e.kind === NODE_KIND.IfTest),
      ),
    ).toBe(true);
    const n = nodeByName(g, "n");
    const targets = g.edges
      .filter((e) => e.from === n?.id && e.label === "read")
      .map((e) => e.to);
    expect(new Set(targets)).toEqual(new Set(anchors.map((a) => a.id)));
  });
});

describe("buildVisualGraph: ownerless refs at module scope", () => {
  test("a top-level ExpressionStatement gets its own node carrying the call head and line, and consuming reads route into it", () => {
    const g = build("const a = 1;\nconsole.log(a);\n");
    const exprNode = findNodes(g, NODE_KIND.ExpressionStatement)[0];
    expect(exprNode).toBeDefined();
    expect(exprNode?.name).toBe("console.log()");
    expect(exprNode?.line).toBe(2);
    const a = nodeByName(g, "a");
    expect(edgesFrom(g, a!.id).some((e) => e.to === exprNode?.id)).toBe(true);
    const consoleNode = nodeByName(g, "console");
    expect(
      edgesFrom(g, consoleNode!.id).some((e) => e.to === exprNode?.id),
    ).toBe(true);
  });

  test("a non-call top-level ExpressionStatement uses the bare expression as the head", () => {
    const g = build("const a = 1;\na;\n");
    const exprNode = findNodes(g, NODE_KIND.ExpressionStatement)[0];
    expect(exprNode?.name).toBe("a");
    expect(exprNode?.line).toBe(2);
  });

  test("a receiver-only ImplicitGlobalVariable in an if-predicate flows into the if-test anchor (issue #20 regression)", () => {
    const g = build(
      "function f() { if (Math.random() < 0.5) { return 1; } return 0; }\n",
    );
    const math = nodeByName(g, "Math");
    expect(math?.kind).toBe(NODE_KIND.ImplicitGlobalVariable);
    const anchor = findNodes(g, NODE_KIND.IfTest)[0];
    expect(anchor).toBeDefined();
    expect(
      g.edges.some(
        (e) => e.from === math?.id && e.to === anchor?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("a receiver-only ImplicitGlobalVariable consumed by a Variable emits an outgoing read edge", () => {
    const g = build("const xs = Object.keys({});\n");
    const obj = nodeByName(g, "Object");
    const xs = nodeByName(g, "xs");
    expect(obj?.kind).toBe(NODE_KIND.ImplicitGlobalVariable);
    expect(
      g.edges.some(
        (e) => e.from === obj?.id && e.to === xs?.id && e.label === "read",
      ),
    ).toBe(true);
  });
});

describe("buildVisualGraph: edge deduplication", () => {
  test("the same logical read is emitted only once even if the analyzer surfaces it twice", () => {
    // `a + a` has two reads of `a`; before dedup, two |read| edges into the
    // ReturnUse would be emitted. After dedup, only one survives.
    const g = build("function f(a) { return a + a; }\n");
    const a = nodeByName(g, "a");
    const ret = findSubgraphs(g, "return")[0];
    // Only one ReturnUse for the variable `a` survives because both reads
    // share the same destination.
    const uses = (ret?.elements ?? []).filter(
      (e) =>
        e.type === VISUAL_ELEMENT_TYPE.Node &&
        e.kind === NODE_KIND.ReturnUse &&
        e.name === "a",
    );
    expect(uses.length).toBeGreaterThanOrEqual(1);
    // Aggregate: only as many edges as there are ReturnUse nodes for `a`.
    const edges = edgesFrom(g, a!.id).filter((e) =>
      uses.some((u) => u.type === VISUAL_ELEMENT_TYPE.Node && u.id === e.to),
    );
    expect(edges).toHaveLength(uses.length);
  });
});
