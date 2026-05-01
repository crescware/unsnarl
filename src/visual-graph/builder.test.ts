import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../analyzer/eslint-compat/eslint-compat.js";
import { SUBGRAPH_KIND, VISUAL_ELEMENT_TYPE } from "../constants.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat/flat-serializer.js";
import { buildVisualGraph } from "./builder.js";
import type {
  VisualEdge,
  VisualElement,
  VisualGraph,
  VisualNode,
  VisualSubgraph,
} from "./model.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();

function build(
  code: string,
  language: "ts" | "tsx" | "js" | "jsx" = "ts",
): VisualGraph {
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

function findSubgraphs(
  graph: VisualGraph,
  kind: VisualSubgraph["kind"],
): readonly VisualSubgraph[] {
  return flattenSubgraphs(graph.elements).filter((s) => s.kind === kind);
}

function findNodes(
  graph: VisualGraph,
  kind: VisualNode["kind"],
): readonly VisualNode[] {
  return flattenNodes(graph.elements).filter((n) => n.kind === kind);
}

function nodeByName(graph: VisualGraph, name: string): VisualNode | undefined {
  return flattenNodes(graph.elements).find((n) => n.name === name);
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
    expect(g.version).toBe(1);
    expect(g.source.path).toBe("input.ts");
    expect(g.source.language).toBe("ts");
    expect(g.direction).toBe("RL");
  });

  test("an empty source produces an empty graph", () => {
    const g = build("");
    expect(g.elements).toEqual([]);
    expect(g.edges).toEqual([]);
  });

  test("a single const declaration emits exactly one Variable node and no edges", () => {
    const g = build("const a = 1;\n");
    const nodes = findNodes(g, "Variable");
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
    expect(findNodes(g, "ImportBinding").map((n) => n.name)).toContain("imp");
    expect(findNodes(g, "FunctionName").map((n) => n.name)).toContain("foo");
    expect(findNodes(g, "Parameter").map((n) => n.name)).toContain("p");
    expect(findNodes(g, "CatchClause").map((n) => n.name)).toContain("e");
    expect(findNodes(g, "ClassName").map((n) => n.name)).toContain("Bar");
  });

  test("unused declarations carry the unused flag; declarations with readers do not", () => {
    const g = build("const a = 1;\nconst b = a;\n");
    expect(nodeByName(g, "a")?.unused).toBeUndefined();
    expect(nodeByName(g, "b")?.unused).toBe(true);
  });

  test("declarationKind is preserved on Variable nodes for let / const (var is intentionally skipped)", () => {
    const g = build("let a = 1;\nconst b = 2;\n");
    expect(nodeByName(g, "a")?.declarationKind).toBe("let");
    expect(nodeByName(g, "b")?.declarationKind).toBe("const");
  });

  test("a const initialised by a function expression is marked initIsFunction", () => {
    const g = build("const fn = function () {};\n");
    expect(nodeByName(g, "fn")?.initIsFunction).toBe(true);
  });

  test("ImplicitGlobalVariable that is only a member receiver is hidden, but a direct read is kept", () => {
    const directRead = build("function f() { return globalThing; }\n");
    expect(
      nodeByName(directRead, "globalThing")?.kind === "ImplicitGlobalVariable",
    ).toBe(true);

    const onlyReceiver = build("const x = Object.keys({});\n");
    // Object is only used as a member receiver (Object.keys), so it gets hidden.
    expect(nodeByName(onlyReceiver, "Object")).toBeUndefined();
  });

  test("named imports renamed at the import site keep the local name on the node", () => {
    const g = build("import { other as renamed } from 'm';\nvoid renamed;\n");
    const node = nodeByName(g, "renamed");
    expect(node?.kind).toBe("ImportBinding");
    expect(node?.importKind).toBe("named");
    expect(node?.importedName).toBe("other");
    expect(node?.importSource).toBe("m");
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
    expect(ownerNode?.kind).toBe("FunctionName");
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

  test("an if without else has no if-else container and the predicate flows to the bare if", () => {
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
    const flag = nodeByName(g, "flag");
    expect(flag).toBeDefined();
    expect(
      g.edges.some(
        (e) => e.from === flag?.id && e.to === ifS?.id && e.label === "read",
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
    const writeOps = findNodes(g, "WriteOp");
    expect(writeOps.map((w) => w.name)).toEqual(["v", "v"]);
    const setEdges = g.edges.filter((e) => e.label === "set");
    // n_v -> wr0 and wr0 -> wr1
    expect(setEdges).toHaveLength(2);
  });

  test("WriteOp nodes carry the declarationKind of the variable being mutated", () => {
    const g = build("let v = 0;\nv = 1;\n");
    const wr = findNodes(g, "WriteOp")[0];
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
    const writeOps = findNodes(g, "WriteOp");
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
    const postSwitchReturnUse = findNodes(g, "ReturnUse").find(
      (n) => n.line === 13,
    );
    expect(postSwitchReturnUse).toBeDefined();
    const incoming = edgesTo(g, postSwitchReturnUse!.id);
    // case "a" must not contribute (it exits the function).
    const writeOps = findNodes(g, "WriteOp");
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
      (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === "ReturnUse",
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
    expect(ret?.endLine).toBeUndefined();
  });

  test("a function with no ownerless refs in its body produces no return subgraph", () => {
    const g = build("function f() { return 1; }\n"); // literal only
    expect(findSubgraphs(g, "return")).toHaveLength(0);
  });

  test("a single-line return subgraph omits endLine (rendered as plain L<n>)", () => {
    const g = build("function f(x) { return x; }\n");
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toBe(1);
    expect(ret?.endLine).toBeUndefined();
  });

  test("ownerless refs flow into ReturnUse nodes via |read| edges", () => {
    const g = build("function f(a) { return a; }\n");
    const aParam = nodeByName(g, "a");
    const ret = findSubgraphs(g, "return")[0];
    const retUse = ret?.elements.find(
      (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === "ReturnUse",
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
    const retUses = findNodes(g, "ReturnUse");
    const a = retUses.find((n) => n.name === "A");
    expect(a?.isJsxElement).toBe(true);
    expect(a?.line).toBe(3);
    expect(a?.endLine).toBe(5);
  });

  test("a single-line JSX element still flags isJsxElement, and endLine is omitted", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => <A>hi</A>;",
    ].join("\n");
    const g = build(code, "tsx");
    const a = findNodes(g, "ReturnUse").find((n) => n.name === "A");
    expect(a?.isJsxElement).toBe(true);
    expect(a?.endLine).toBeUndefined();
  });

  test("non-JSX ReturnUse stays isJsxElement=false with no endLine", () => {
    const g = build("function f(a) { return a; }\n");
    const ret = findSubgraphs(g, "return")[0];
    const retUse = ret?.elements.find(
      (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.kind === "ReturnUse",
    ) as VisualNode | undefined;
    expect(retUse?.isJsxElement).toBe(false);
    expect(retUse?.endLine).toBeUndefined();
  });
});

describe("buildVisualGraph: imports", () => {
  test("default imports get a single ModuleSource and one read edge into the local binding", () => {
    const g = build("import def from 'lib';\nvoid def;\n");
    const moduleSource = findNodes(g, "ModuleSource")[0];
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
    const intermediates = findNodes(g, "ImportIntermediate");
    expect(intermediates).toHaveLength(1);
    expect(intermediates[0]?.name).toBe("other");
    // ModuleSource -> Intermediate -> local binding
    const moduleSource = findNodes(g, "ModuleSource")[0];
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
    expect(findNodes(g, "ImportIntermediate")).toHaveLength(0);
    const moduleSource = findNodes(g, "ModuleSource")[0];
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

  test("an if/else predicate identifier feeds the if-else container subgraph", () => {
    const g = build(
      [
        "let v = 0;",
        "const flag = true;",
        "if (flag) { v = 1; } else { v = 2; }",
      ].join("\n"),
    );
    const flag = nodeByName(g, "flag");
    const container = findSubgraphs(g, "if-else-container")[0];
    expect(
      g.edges.some(
        (e) =>
          e.from === flag?.id && e.to === container?.id && e.label === "read",
      ),
    ).toBe(true);
  });

  test("a bare if predicate identifier feeds the bare if subgraph (no container)", () => {
    const g = build(
      ["let v = 0;", "const flag = true;", "if (flag) { v = 1; }"].join("\n"),
    );
    const flag = nodeByName(g, "flag");
    const ifS = findSubgraphs(g, "if")[0];
    expect(
      g.edges.some(
        (e) => e.from === flag?.id && e.to === ifS?.id && e.label === "read",
      ),
    ).toBe(true);
  });
});

describe("buildVisualGraph: ownerless refs at module scope", () => {
  test("a top-level expression that consumes a variable creates a ModuleSink and routes the read there", () => {
    const g = build("const a = 1;\nconsole.log(a);\n");
    const moduleSink = findNodes(g, "ModuleSink")[0];
    expect(moduleSink).toBeDefined();
    const a = nodeByName(g, "a");
    // Either directly or via ImplicitGlobalVariable("console") routing — but
    // an edge into the module sink must exist.
    expect(edgesFrom(g, a!.id).some((e) => e.to === moduleSink?.id)).toBe(true);
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
        e.kind === "ReturnUse" &&
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
