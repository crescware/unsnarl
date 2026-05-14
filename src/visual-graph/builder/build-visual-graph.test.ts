import { describe, expect, test } from "vitest";

import { LANGUAGE, type Language } from "../../language.js";
import { OxcParser } from "../../parser/oxc-parser.js";
import { runAnalysis } from "../../pipeline/analyze/run-analysis.js";
import { defaultSourceTypeFor } from "../../pipeline/parse/default-source-type-for.js";
import { FlatSerializer } from "../../serializer/flat/flat-serializer.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { freshName } from "../../testing/fresh-name.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import type { VisualEdge } from "../visual-edge.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualGraph } from "../visual-graph.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { buildVisualGraph } from "./build-visual-graph.js";

const parser = new OxcParser();
const serializer = new FlatSerializer();

function build(code: string, language: Language = LANGUAGE.Ts): VisualGraph {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
    sourceType: defaultSourceTypeFor(language),
  });
  const analyzed = runAnalysis(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    annotations: analyzed.annotations,
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
    (v): v is Extract<VisualSubgraph, { kind: K }> => v.kind === kind,
  );
}

function findNodes<K extends VisualNode["kind"]>(
  graph: VisualGraph,
  kind: K,
): readonly Extract<VisualNode, { kind: K }>[] {
  return flattenNodes(graph.elements).filter(
    (v): v is Extract<VisualNode, { kind: K }> => v.kind === kind,
  );
}

function nodeByName(graph: VisualGraph, name: string): VisualNode | null {
  return flattenNodes(graph.elements).find((v) => v.name === name) ?? null;
}

function variableByName(
  graph: VisualGraph,
  name: string,
): Extract<VisualNode, { kind: typeof NODE_KIND.LegacyVariable }> | null {
  return (
    findNodes(graph, NODE_KIND.LegacyVariable).find((v) => v.name === name) ??
    null
  );
}

function importBindingByName(
  graph: VisualGraph,
  name: string,
): Extract<VisualNode, { kind: typeof NODE_KIND.LegacyImportBinding }> | null {
  return (
    findNodes(graph, NODE_KIND.LegacyImportBinding).find(
      (v) => v.name === name,
    ) ?? null
  );
}

function edgesFrom(graph: VisualGraph, fromId: string): readonly VisualEdge[] {
  return graph.edges.filter((v) => v.from === fromId);
}

function edgesTo(graph: VisualGraph, toId: string): readonly VisualEdge[] {
  return graph.edges.filter((v) => v.to === toId);
}

function childSubgraphsOf(sg: VisualSubgraph): readonly VisualSubgraph[] {
  return sg.elements.filter(
    (v): v is VisualSubgraph => v.type === VISUAL_ELEMENT_TYPE.Subgraph,
  );
}

describe("buildVisualGraph: top-level structure", () => {
  test("top-level metadata mirrors the IR source path/language and direction is RL", () => {
    const g = build("const a = 1;\n");
    expect(g.version).toEqual(SERIALIZED_IR_VERSION);
    expect(g.source.path).toEqual("input.ts");
    expect(g.source.language).toEqual("ts");
    expect(g.direction).toEqual(DIRECTION.RL);
  });

  test("an empty source produces an empty graph", () => {
    const g = build("");
    expect(g.elements).toEqual([]);
    expect(g.edges).toEqual([]);
  });

  test("a single const declaration emits exactly one Variable node and no edges", () => {
    const g = build("const a = 1;\n");
    const nodes = findNodes(g, NODE_KIND.LegacyVariable);
    expect(nodes).toHaveLength(1);
    expect(nodes[0]?.name).toEqual("a");
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
    expect(
      findNodes(g, NODE_KIND.LegacyImportBinding).map((v) => v.name),
    ).toContain("imp");
    expect(
      findNodes(g, NODE_KIND.LegacyFunctionName).map((v) => v.name),
    ).toContain("foo");
    expect(
      findNodes(g, NODE_KIND.LegacyParameter).map((v) => v.name),
    ).toContain("p");
    expect(
      findNodes(g, NODE_KIND.LegacyCatchClause).map((v) => v.name),
    ).toContain("e");
    expect(
      findNodes(g, NODE_KIND.ClassDeclaration).map((v) => v.name),
    ).toContain("Bar");
  });

  test("unused declarations carry the unused flag; declarations with readers do not", () => {
    const g = build("const a = 1;\nconst b = a;\n");
    expect(nodeByName(g, "a")?.unused).toEqual(false);
    expect(nodeByName(g, "b")?.unused).toEqual(true);
  });

  test("declarationKind is preserved on Variable nodes for let / const", () => {
    const g = build("let a = 1;\nconst b = 2;\n");
    expect(variableByName(g, "a")?.declarationKind).toEqual("let");
    expect(variableByName(g, "b")?.declarationKind).toEqual("const");
  });

  test("a const initialised by a function expression is marked initIsFunction", () => {
    const g = build("const fn = function () {};\n");
    expect(variableByName(g, "fn")?.initIsFunction).toEqual(true);
  });

  test("ImplicitGlobalVariable is kept as a node regardless of whether refs are receiver-only", () => {
    const directRead = build("function f() { return globalThing; }\n");
    expect(
      nodeByName(directRead, "globalThing")?.kind ===
        NODE_KIND.SyntheticImplicitGlobal,
    ).toEqual(true);

    const onlyReceiver = build("const x = Object.keys({});\n");
    expect(nodeByName(onlyReceiver, "Object")?.kind).toEqual(
      NODE_KIND.SyntheticImplicitGlobal,
    );
  });

  test("named imports renamed at the import site keep the local name on the node", () => {
    const g = build("import { other as renamed } from 'm';\nvoid renamed;\n");
    const node = importBindingByName(g, "renamed");
    expect(node?.kind).toEqual(NODE_KIND.LegacyImportBinding);
    expect(node?.importKind).toEqual(IMPORT_KIND.Named);
    if (node?.importKind === IMPORT_KIND.Named) {
      expect(node.importedName).toEqual("other");
    }
  });
});

describe("buildVisualGraph: function subgraphs", () => {
  test("a FunctionDeclaration becomes a function subgraph with ownerNodeId pointing to the FunctionName", () => {
    const g = build("function add(a, b) { return a + b; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn !== null && fn !== undefined).toEqual(true);
    expect(fn?.ownerNodeId !== null && fn?.ownerNodeId !== undefined).toEqual(
      true,
    );
    const ownerNode = flattenNodes(g.elements).find(
      (v) => v.id === fn?.ownerNodeId,
    );
    expect(ownerNode?.name).toEqual("add");
    expect(ownerNode?.kind).toEqual(NODE_KIND.LegacyFunctionName);
  });

  test("function subgraph mirrors the owner's name as ownerName so labels survive when pruning drops the owner node", () => {
    const g = build("function add(a, b) { return a + b; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn?.ownerName).toEqual("add");
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
    expect(fn?.line).toEqual(1);
    expect(fn?.endLine).toEqual(3);
  });

  test("a single-line function reports endLine equal to line (renderers collapse equal ranges)", () => {
    const g = build("function f() { return 1; }\n");
    const fn = findSubgraphs(g, "function")[0];
    expect(fn?.line).toEqual(1);
    expect(fn?.endLine).toEqual(1);
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
    expect(ifS?.line).toEqual(3);
    const anchor = findNodes(g, NODE_KIND.SyntheticIfStatementTest)[0];
    expect(anchor?.line).toEqual(3);
    const flag = nodeByName(g, "flag");
    expect(flag !== null && flag !== undefined).toEqual(true);
    expect(
      g.edges.some(
        (v) => v.from === flag?.id && v.to === anchor?.id && v.label === "read",
      ),
    ).toEqual(true);
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
    expect(container?.line).toEqual(3);
    expect(container?.endLine).toEqual(7);
    expect(container?.hasElse).toEqual(true);
    const childKinds = childSubgraphsOf(container!).map((v) => v.kind);
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
    expect(sw?.line).toEqual(3);
    expect(sw?.endLine).toEqual(9);
    const cases = findSubgraphs(g, "case");
    expect(cases).toHaveLength(2);
    const tests = cases.map((v) => v.caseTest);
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
    const writeOps = findNodes(g, NODE_KIND.LegacyWriteOp);
    expect(writeOps.map((v) => v.name)).toEqual(["v", "v"]);
    const setEdges = g.edges.filter((v) => v.label === "set");
    // n_v -> wr0 and wr0 -> wr1
    expect(setEdges).toHaveLength(2);
  });

  test("WriteOp nodes carry the declarationKind of the variable being mutated", () => {
    const g = build("let v = 0;\nv = 1;\n");
    const wr = findNodes(g, NODE_KIND.LegacyWriteOp)[0];
    expect(wr?.declarationKind).toEqual("let");
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
    expect(g.edges.some((v) => v.label === "fallthrough")).toEqual(true);
  });
});

describe("buildVisualGraph: read origin edges", () => {
  test("a const initialised from another variable produces one read edge from the source", () => {
    const g = build("const a = 1;\nconst b = a;\n");
    const a = nodeByName(g, "a");
    const b = nodeByName(g, "b");
    expect(
      g.edges.some(
        (v) => v.from === a?.id && v.to === b?.id && v.label === "read",
      ),
    ).toEqual(true);
  });

  test("call expressions produce read,call edges to the variable that consumes the result", () => {
    const g = build("function f() {}\nconst x = f();\n");
    const f = nodeByName(g, "f");
    const x = nodeByName(g, "x");
    expect(
      g.edges.some(
        (v) => v.from === f?.id && v.to === x?.id && v.label === "read,call",
      ),
    ).toEqual(true);
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
    const reads = edgesTo(g, result!.id).filter((v) => v.label === "read");
    // One read edge per branch's last write.
    expect(reads).toHaveLength(2);
    const writeOps = findNodes(g, NODE_KIND.LegacyWriteOp);
    const writeOpIds = new Set(writeOps.map((v) => v.id));
    expect(reads.every((v) => writeOpIds.has(v.from))).toEqual(true);
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
    const reads = edgesTo(g, result!.id).filter((v) => v.label === "read");
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
    const reads = edgesTo(g, result!.id).filter((v) => v.label === "read");
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
    const postSwitchReturnUse = findNodes(g, NODE_KIND.LegacyReturnUse).find(
      (v) => v.line === 13,
    );
    expect(
      postSwitchReturnUse !== null && postSwitchReturnUse !== undefined,
    ).toEqual(true);
    const incoming = edgesTo(g, postSwitchReturnUse!.id);
    // case "a" must not contribute (it exits the function).
    const writeOps = findNodes(g, NODE_KIND.LegacyWriteOp);
    const aWrite = writeOps.find((v) => v.line === 5);
    expect(aWrite !== null && aWrite !== undefined).toEqual(true);
    expect(incoming.some((v) => v.from === aWrite?.id)).toEqual(false);
    // case "b" (break) and default (fallthrough end) must contribute.
    const bWrite = writeOps.find((v) => v.line === 8);
    const dWrite = writeOps.find((v) => v.line === 11);
    expect(incoming.some((v) => v.from === bWrite?.id)).toEqual(true);
    expect(incoming.some((v) => v.from === dWrite?.id)).toEqual(true);
  });
});

describe("buildVisualGraph: return subgraphs", () => {
  test("a ReturnStatement yields a return subgraph with one ReturnUse per ownerless ref", () => {
    const g = build("function f(a, b) { return a + b; }\n");
    const ret = findSubgraphs(g, "return")[0];
    expect(ret !== null && ret !== undefined).toEqual(true);
    const uses = ret!.elements.filter(
      (v) =>
        v.type === VISUAL_ELEMENT_TYPE.Node &&
        v.kind === NODE_KIND.LegacyReturnUse,
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
    const lines = returns.map((v) => v.line).sort((a, b) => a - b);
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
      (v) => v.kind === SUBGRAPH_KIND.Return,
    );
    const ifDirectReturns = childSubgraphsOf(ifS!).filter(
      (v) => v.kind === SUBGRAPH_KIND.Return,
    );
    expect(fnDirectReturns).toHaveLength(1);
    expect(ifDirectReturns).toHaveLength(1);
    expect(fnDirectReturns[0]?.line).toEqual(6);
    expect(ifDirectReturns[0]?.line).toEqual(4);
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
    const caseWithReturnUse = cases.find((caseSg) =>
      childSubgraphsOf(caseSg).some((v) => v.kind === SUBGRAPH_KIND.Return),
    );
    expect(
      caseWithReturnUse !== null && caseWithReturnUse !== undefined,
    ).toEqual(true);
    expect(caseWithReturnUse?.caseTest).toEqual(null);
  });

  test("an arrow with an expression body uses the body span as the implicit return container", () => {
    const g = build(["const fn = (x) => (", "  x + 1", ");"].join("\n"));
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toEqual(1);
    expect(ret?.endLine).toEqual(3);
  });

  test("an arrow with a block body falls through to the inner ReturnStatement's span", () => {
    const g = build(["const fn = (x) => {", "  return x;", "};"].join("\n"));
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toEqual(2);
    expect(ret?.endLine).toEqual(null);
  });

  test("a function with no ownerless refs in its body produces no return subgraph", () => {
    const g = build("function f() { return 1; }\n"); // literal only
    expect(findSubgraphs(g, "return")).toHaveLength(0);
  });

  test("a single-line return subgraph leaves endLine null (rendered as plain L<n>)", () => {
    const g = build("function f(x) { return x; }\n");
    const ret = findSubgraphs(g, "return")[0];
    expect(ret?.line).toEqual(1);
    expect(ret?.endLine).toEqual(null);
  });

  test("ownerless refs flow into ReturnUse nodes via |read| edges", () => {
    const g = build("function f(a) { return a; }\n");
    const aParam = nodeByName(g, "a");
    const ret = findSubgraphs(g, "return")[0];
    const retUse = ret?.elements.find(
      (v) =>
        v.type === VISUAL_ELEMENT_TYPE.Node &&
        v.kind === NODE_KIND.LegacyReturnUse,
    );
    expect(
      g.edges.some(
        (v) =>
          v.from === aParam?.id && v.to === retUse?.id && v.label === "read",
      ),
    ).toEqual(true);
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
    const retUses = findNodes(g, NODE_KIND.LegacyReturnUse);
    const a = retUses.find((v) => v.name === "A");
    expect(a?.isJsxElement).toEqual(true);
    expect(a?.line).toEqual(3);
    expect(a?.endLine).toEqual(5);
  });

  test("a single-line JSX element still flags isJsxElement, and endLine stays null", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => <A>hi</A>;",
    ].join("\n");
    const g = build(code, "tsx");
    const a = findNodes(g, NODE_KIND.LegacyReturnUse).find(
      (v) => v.name === "A",
    );
    expect(a?.isJsxElement).toEqual(true);
    expect(a?.endLine).toEqual(null);
  });

  test("non-JSX ReturnUse stays isJsxElement=false with endLine null", () => {
    const g = build("function f(a) { return a; }\n");
    const ret = findSubgraphs(g, "return")[0];
    const retUse =
      ret?.elements.find(
        (v): v is VisualNode =>
          v.type === VISUAL_ELEMENT_TYPE.Node &&
          v.kind === NODE_KIND.LegacyReturnUse,
      ) ?? null;
    expect(retUse?.isJsxElement).toEqual(false);
    expect(retUse?.endLine).toEqual(null);
  });
});

describe("buildVisualGraph: imports", () => {
  test("default imports get a single ModuleSource and one read edge into the local binding", () => {
    const g = build("import def from 'lib';\nvoid def;\n");
    const moduleSource = findNodes(g, NODE_KIND.SyntheticModuleSource)[0];
    expect(moduleSource?.name).toEqual("lib");
    const def = nodeByName(g, "def");
    expect(
      g.edges.some((v) => v.from === moduleSource?.id && v.to === def?.id),
    ).toEqual(true);
  });

  test("renamed named imports introduce an ImportIntermediate carrying the original name", () => {
    const g = build(
      ["import { other as renamed } from 'lib';", "void renamed;"].join("\n"),
    );
    const intermediates = findNodes(g, NODE_KIND.SyntheticImportIntermediate);
    expect(intermediates).toHaveLength(1);
    expect(intermediates[0]?.name).toEqual("other");
    // ModuleSource -> Intermediate -> local binding
    const moduleSource = findNodes(g, NODE_KIND.SyntheticModuleSource)[0];
    const renamed = nodeByName(g, "renamed");
    expect(
      g.edges.some(
        (v) => v.from === moduleSource?.id && v.to === intermediates[0]?.id,
      ),
    ).toEqual(true);
    expect(
      g.edges.some(
        (v) => v.from === intermediates[0]?.id && v.to === renamed?.id,
      ),
    ).toEqual(true);
  });

  test("namespace imports point directly from the module to the local binding (no intermediate)", () => {
    const g = build("import * as ns from 'lib';\nvoid ns;\n");
    expect(findNodes(g, NODE_KIND.SyntheticImportIntermediate)).toHaveLength(0);
    const moduleSource = findNodes(g, NODE_KIND.SyntheticModuleSource)[0];
    const ns = nodeByName(g, "ns");
    expect(
      g.edges.some((v) => v.from === moduleSource?.id && v.to === ns?.id),
    ).toEqual(true);
  });
});

describe("buildVisualGraph: predicate references", () => {
  test("a switch discriminant identifier feeds the SwitchDiscriminant anchor inside the switch subgraph", () => {
    const g = build(
      [
        "let l = 0;",
        "const k = 1;",
        "switch (k) { case 1: l = 1; break; default: l = 2; }",
      ].join("\n"),
    );
    const k = nodeByName(g, "k");
    const anchor = findNodes(
      g,
      NODE_KIND.SyntheticSwitchStatementDiscriminant,
    )[0];
    expect(anchor !== null && anchor !== undefined).toEqual(true);
    const sw = findSubgraphs(g, "switch")[0];
    expect(sw?.elements[0]).toEqual(anchor);
    expect(
      g.edges.some(
        (v) => v.from === k?.id && v.to === anchor?.id && v.label === "read",
      ),
    ).toEqual(true);
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
    const anchor = findNodes(g, NODE_KIND.SyntheticIfStatementTest)[0];
    expect(anchor !== null && anchor !== undefined).toEqual(true);
    const ifBranch = container?.elements.find(
      (v): v is VisualSubgraph => v.type === "subgraph" && v.kind === "if",
    );
    expect(ifBranch?.elements).toContainEqual(
      expect.objectContaining({ id: anchor?.id }),
    );
    const elseBranch = container?.elements.find(
      (v): v is VisualSubgraph => v.type === "subgraph" && v.kind === "else",
    );
    expect(
      elseBranch?.elements.every(
        (v) =>
          !(v.type === "node" && v.kind === NODE_KIND.SyntheticIfStatementTest),
      ),
    ).toEqual(true);
    expect(
      g.edges.some(
        (v) => v.from === flag?.id && v.to === anchor?.id && v.label === "read",
      ),
    ).toEqual(true);
  });

  test("a bare if predicate identifier feeds the if-test anchor (no container)", () => {
    const g = build(
      ["let v = 0;", "const flag = true;", "if (flag) { v = 1; }"].join("\n"),
    );
    const flag = nodeByName(g, "flag");
    const anchor = findNodes(g, NODE_KIND.SyntheticIfStatementTest)[0];
    expect(anchor !== null && anchor !== undefined).toEqual(true);
    expect(
      g.edges.some(
        (v) => v.from === flag?.id && v.to === anchor?.id && v.label === "read",
      ),
    ).toEqual(true);
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
    expect(container !== null && container !== undefined).toEqual(true);
    const anchors = findNodes(g, NODE_KIND.SyntheticIfStatementTest);
    expect(anchors).toHaveLength(2);
    const lines = anchors.map((v) => v.line).sort((a, b) => a - b);
    expect(lines).toEqual([3, 5]);
    const ifBranches = (container?.elements ?? []).filter(
      (v): v is VisualSubgraph => v.type === "subgraph" && v.kind === "if",
    );
    for (const anchor of anchors) {
      const hosts = ifBranches.filter((branch) =>
        branch.elements.some((v) => v.type === "node" && v.id === anchor.id),
      );
      expect(hosts).toHaveLength(1);
    }
    const elseBranch = container?.elements.find(
      (v): v is VisualSubgraph => v.type === "subgraph" && v.kind === "else",
    );
    expect(
      elseBranch?.elements.every(
        (v) =>
          !(v.type === "node" && v.kind === NODE_KIND.SyntheticIfStatementTest),
      ),
    ).toEqual(true);
    const n = nodeByName(g, "n");
    const targets = g.edges
      .filter((v) => v.from === n?.id && v.label === "read")
      .map((v) => v.to);
    expect(new Set(targets)).toEqual(new Set(anchors.map((v) => v.id)));
  });
});

describe("buildVisualGraph: ownerless refs at module scope", () => {
  test("a top-level ExpressionStatement gets its own node carrying the call head and line, and consuming reads route into it", () => {
    const g = build("const a = 1;\nconsole.log(a);\n");
    const exprNode = findNodes(g, NODE_KIND.SyntheticExpressionStatement)[0];
    expect(exprNode !== null && exprNode !== undefined).toEqual(true);
    expect(exprNode?.name).toEqual("console.log()");
    expect(exprNode?.line).toEqual(2);
    const a = nodeByName(g, "a");
    expect(edgesFrom(g, a!.id).some((v) => v.to === exprNode?.id)).toEqual(
      true,
    );
    const consoleNode = nodeByName(g, "console");
    expect(
      edgesFrom(g, consoleNode!.id).some((v) => v.to === exprNode?.id),
    ).toEqual(true);
  });

  test("a non-call top-level ExpressionStatement uses the bare expression as the head", () => {
    const g = build("const a = 1;\na;\n");
    const exprNode = findNodes(g, NODE_KIND.SyntheticExpressionStatement)[0];
    expect(exprNode?.name).toEqual("a");
    expect(exprNode?.line).toEqual(2);
  });

  test("a receiver-only ImplicitGlobalVariable in an if-predicate flows into the if-test anchor (issue #20 regression)", () => {
    const g = build(
      "function f() { if (Math.random() < 0.5) { return 1; } return 0; }\n",
    );
    const math = nodeByName(g, "Math");
    expect(math?.kind).toEqual(NODE_KIND.SyntheticImplicitGlobal);
    const anchor = findNodes(g, NODE_KIND.SyntheticIfStatementTest)[0];
    expect(anchor !== null && anchor !== undefined).toEqual(true);
    expect(
      g.edges.some(
        (v) => v.from === math?.id && v.to === anchor?.id && v.label === "read",
      ),
    ).toEqual(true);
  });

  test("a receiver-only ImplicitGlobalVariable consumed by a Variable emits an outgoing read edge", () => {
    const g = build("const xs = Object.keys({});\n");
    const obj = nodeByName(g, "Object");
    const xs = nodeByName(g, "xs");
    expect(obj?.kind).toEqual(NODE_KIND.SyntheticImplicitGlobal);
    expect(
      g.edges.some(
        (v) => v.from === obj?.id && v.to === xs?.id && v.label === "read",
      ),
    ).toEqual(true);
  });
});

describe("buildVisualGraph: var declarations", () => {
  test("var-declared variables emit a node but no edges to/from references", () => {
    const varName = freshName();
    const graph = build(`var ${varName} = 0;\nconsole.log(${varName});\n`);
    const varNode = nodeByName(graph, varName);
    expect(varNode).not.toEqual(null);
    expect(varNode?.kind).toEqual(NODE_KIND.LegacyVariable);
    // No edges incident on the var node.
    expect(edgesFrom(graph, varNode!.id)).toHaveLength(0);
    expect(edgesTo(graph, varNode!.id)).toHaveLength(0);
    // No WriteOp nodes for the var (the init `= 0` does not produce one).
    const writeOps = flattenNodes(graph.elements).filter(
      (node) => node.kind === NODE_KIND.LegacyWriteOp && node.name === varName,
    );
    expect(writeOps).toHaveLength(0);
  });

  test("var-declared name does not get classified as ImplicitGlobalVariable", () => {
    const varName = freshName();
    const graph = build(`var ${varName} = 0;\nconsole.log(${varName});\n`);
    const varNode = nodeByName(graph, varName);
    expect(varNode?.kind).toEqual(NODE_KIND.LegacyVariable);
    // Implicit global classification would have produced this kind.
    const implicitGlobals = flattenNodes(graph.elements).filter(
      (node) =>
        node.kind === NODE_KIND.SyntheticImplicitGlobal &&
        node.name === varName,
    );
    expect(implicitGlobals).toHaveLength(0);
  });

  test("an unused var node is not flagged as unused in the visual graph", () => {
    // The IR still records the variable as unused; the visual graph
    // intentionally drops the flag because the var node has no edges that
    // would otherwise back up the usage signal.
    const varName = freshName();
    const graph = build(`var ${varName} = 0;\n`);
    const varNode = nodeByName(graph, varName);
    expect(varNode?.unused).toEqual(false);
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
      (v) =>
        v.type === VISUAL_ELEMENT_TYPE.Node &&
        v.kind === NODE_KIND.LegacyReturnUse &&
        v.name === "a",
    );
    expect(uses.length >= 1).toEqual(true);
    // Aggregate: only as many edges as there are ReturnUse nodes for `a`.
    const edges = edgesFrom(g, a!.id).filter((edge) =>
      uses.some((v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === edge.to),
    );
    expect(edges).toHaveLength(uses.length);
  });
});
