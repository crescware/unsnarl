import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { LANGUAGE } from "../../language.js";
import { OxcParser } from "../../parser/oxc-parser.js";
import { runAnalysis } from "../../pipeline/analyze/run-analysis.js";
import { defaultSourceTypeFor } from "../../pipeline/parse/default-source-type-for.js";
import { FlatSerializer } from "../../serializer/flat/flat-serializer.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { DIRECTION } from "../../visual-graph/direction.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import { JsonEmitter } from "./json.js";

const parser = new OxcParser();
const serializer = new FlatSerializer();
const emitter = new JsonEmitter();

type FlatElement = {
  type: typeof VISUAL_ELEMENT_TYPE.Node | typeof VISUAL_ELEMENT_TYPE.Subgraph;
  id: string;
  kind: string;
  name?: string;
  ownerNodeId?: string;
  caseTest?: string | null;
  elements?: readonly FlatElement[];
  declarationKind?: string;
  importKind?: string;
  importedName?: string | null;
  importSource?: string;
  label?: unknown;
};

function flattenNodes(
  elements: readonly FlatElement[],
): readonly FlatElement[] {
  const out: /* mutable */ FlatElement[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node) {
      out.push(e);
    } else if (e.elements) {
      for (const inner of flattenNodes(e.elements)) {
        out.push(inner);
      }
    }
  }
  return out;
}

function flattenSubgraphs(
  elements: readonly FlatElement[],
): readonly FlatElement[] {
  const out: /* mutable */ FlatElement[] = [];
  for (const e of elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph && e.elements) {
      out.push(e);
      for (const inner of flattenSubgraphs(e.elements)) {
        out.push(inner);
      }
    }
  }
  return out;
}

function emit(code: string, prettyJson = true): string {
  const parsed = parser.parse(code, {
    language: LANGUAGE.Ts,
    sourcePath: "input.ts",
    sourceType: defaultSourceTypeFor(LANGUAGE.Ts),
  });
  const analyzed = runAnalysis(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    annotations: analyzed.annotations,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: "input.ts", language: LANGUAGE.Ts },
  });
  return emitter.emit(ir, {
    prettyJson,
    prunedGraph: null,
    resolutions: null,
    highlightIds: null,
    highlight: null,
    debug: false,
  });
}

describe("JsonEmitter", () => {
  test("identifies as 'json' with the application/json content type", () => {
    expect(emitter.format).toEqual("json");
    expect(emitter.contentType).toEqual("application/json");
  });

  test("emits a versioned VisualGraph with elements/edges arrays", () => {
    const graph = JSON.parse(emit("const a = 1;\nconst b = a;\n"));
    expect(graph.version).toEqual(SERIALIZED_IR_VERSION);
    expect(graph.source).toEqual({ path: "input.ts", language: LANGUAGE.Ts });
    expect(graph.direction).toEqual(DIRECTION.RL);
    expect(Array.isArray(graph.elements)).toEqual(true);
    expect(Array.isArray(graph.edges)).toEqual(true);
  });

  test("nodes carry semantic kind and raw attributes, never a precomputed label", () => {
    const graph = JSON.parse(emit("const a = 1;\nconst b = a;\n"));
    const nodes = flattenNodes(graph.elements);
    const a = nodes.find((v) => v.name === "a");
    expect(a?.kind).toEqual(DEFINITION_TYPE.Variable);
    expect(a?.declarationKind).toEqual("const");
    expect(a?.label).toEqual(undefined);
  });

  test("import nodes record kind / imported name; module source surfaces as a ModuleSource node", () => {
    const graph = JSON.parse(
      emit(
        [
          "import def from 'some-default';",
          "import { other as renamed } from 'some-named';",
          "void def; void renamed;",
        ].join("\n"),
      ),
    );
    const nodes = flattenNodes(graph.elements);
    const def = nodes.find((v) => v.name === "def");
    expect(def?.kind).toEqual(DEFINITION_TYPE.ImportBinding);
    expect(def?.importKind).toEqual(IMPORT_KIND.Default);

    const renamed = nodes.find((v) => v.name === "renamed");
    expect(renamed?.importKind).toEqual(IMPORT_KIND.Named);
    expect(renamed?.importedName).toEqual("other");

    const moduleNode = nodes.find(
      (v) => v.kind === NODE_KIND.ModuleSource && v.name === "some-default",
    );
    expect(moduleNode !== null && moduleNode !== undefined).toEqual(true);
  });

  test("write ops appear as WriteOp nodes carrying the underlying declaration kind", () => {
    const graph = JSON.parse(
      emit("function f() { let v = 0; v = 1; v = 2; return v; }\n"),
    );
    const writeOps = flattenNodes(graph.elements).filter(
      (v) => v.kind === NODE_KIND.WriteOp,
    );
    expect(writeOps).toHaveLength(2);
    for (const op of writeOps) {
      expect(op.declarationKind).toEqual("let");
      expect(op.name).toEqual("v");
    }
  });

  test("function bodies become subgraphs of kind 'function' carrying ownerNodeId of the FunctionName", () => {
    const graph = JSON.parse(emit("function add(a, b) { return a + b; }\n"));
    const fnSubgraph = flattenSubgraphs(graph.elements).find(
      (v) => v.kind === SUBGRAPH_KIND.Function,
    );
    expect(fnSubgraph !== null && fnSubgraph !== undefined).toEqual(true);
    const ownerNode = flattenNodes(graph.elements).find(
      (v) => v.id === fnSubgraph?.ownerNodeId,
    );
    expect(ownerNode !== null && ownerNode !== undefined).toEqual(true);
    expect(ownerNode?.kind).toEqual(DEFINITION_TYPE.FunctionName);
    expect(ownerNode?.name).toEqual("add");
    const returnSubgraph = (fnSubgraph?.elements ?? []).find(
      (v) =>
        v.type === VISUAL_ELEMENT_TYPE.Subgraph &&
        v.kind === SUBGRAPH_KIND.Return,
    );
    expect(returnSubgraph !== null && returnSubgraph !== undefined).toEqual(
      true,
    );
    const returnUseNodes = (returnSubgraph?.elements ?? []).filter(
      (v) =>
        v.type === VISUAL_ELEMENT_TYPE.Node && v.kind === NODE_KIND.ReturnUse,
    );
    expect(returnUseNodes.length > 0).toEqual(true);
  });

  test("switch cases become subgraphs of kind 'case' with caseTest preserved as raw text", () => {
    const graph = JSON.parse(
      emit(
        [
          'let l = "";',
          'const k = "a";',
          "switch (k) {",
          '  case "a": l = "alpha"; break;',
          '  default: l = "other"; break;',
          "}",
        ].join("\n"),
      ),
    );
    const cases = flattenSubgraphs(graph.elements).filter(
      (v) => v.kind === SUBGRAPH_KIND.Case,
    );
    const caseTests = cases.map((v) => v.caseTest);
    expect(caseTests).toContain('"a"');
    expect(caseTests).toContain(null);
  });

  test("emits compact JSON when prettyJson is false", () => {
    const out = emit("const a = 1;\n", false);
    expect(out).not.toContain("\n  ");
  });
});
