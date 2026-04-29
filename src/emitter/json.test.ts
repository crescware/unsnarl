import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../analyzer/eslint-compat.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat.js";
import { JsonEmitter } from "./json.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();
const emitter = new JsonEmitter();

interface FlatElement {
  type: "node" | "subgraph";
  id: string;
  kind: string;
  name?: string;
  ownerNodeId?: string;
  caseTest?: string | null;
  elements?: FlatElement[];
  declarationKind?: string;
  importKind?: string;
  importedName?: string | null;
  importSource?: string;
  label?: unknown;
}

function flattenNodes(elements: FlatElement[]): FlatElement[] {
  const out: FlatElement[] = [];
  for (const e of elements) {
    if (e.type === "node") {
      out.push(e);
    } else if (e.elements) {
      for (const inner of flattenNodes(e.elements)) {
        out.push(inner);
      }
    }
  }
  return out;
}

function flattenSubgraphs(elements: FlatElement[]): FlatElement[] {
  const out: FlatElement[] = [];
  for (const e of elements) {
    if (e.type === "subgraph" && e.elements) {
      out.push(e);
      for (const inner of flattenSubgraphs(e.elements)) {
        out.push(inner);
      }
    }
  }
  return out;
}

function emit(code: string, pretty = true): string {
  const parsed = parser.parse(code, {
    language: "ts",
    sourcePath: "input.ts",
  });
  const analyzed = analyzer.analyze(parsed);
  const ir = serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: "input.ts", language: "ts" },
  });
  return emitter.emit(ir, pretty ? {} : { pretty: false });
}

describe("JsonEmitter", () => {
  test("identifies as 'json' with the application/json content type", () => {
    expect(emitter.format).toBe("json");
    expect(emitter.contentType).toBe("application/json");
  });

  test("emits a versioned VisualGraph with elements/edges arrays", () => {
    const graph = JSON.parse(emit("const a = 1;\nconst b = a;\n"));
    expect(graph.version).toBe(1);
    expect(graph.source).toEqual({ path: "input.ts", language: "ts" });
    expect(graph.direction).toBe("RL");
    expect(Array.isArray(graph.elements)).toBe(true);
    expect(Array.isArray(graph.edges)).toBe(true);
  });

  test("nodes carry semantic kind and raw attributes, never a precomputed label", () => {
    const graph = JSON.parse(emit("const a = 1;\nconst b = a;\n"));
    const nodes = flattenNodes(graph.elements);
    const a = nodes.find((n) => n.name === "a");
    expect(a?.kind).toBe("Variable");
    expect(a?.declarationKind).toBe("const");
    expect(a?.label).toBeUndefined();
  });

  test("import nodes record kind / source / imported name", () => {
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
    const def = nodes.find((n) => n.name === "def");
    expect(def?.kind).toBe("ImportBinding");
    expect(def?.importKind).toBe("default");
    expect(def?.importSource).toBe("some-default");
    expect(def?.importedName).toBeNull();

    const renamed = nodes.find((n) => n.name === "renamed");
    expect(renamed?.importKind).toBe("named");
    expect(renamed?.importedName).toBe("other");

    const moduleNode = nodes.find(
      (n) => n.kind === "ModuleSource" && n.name === "some-default",
    );
    expect(moduleNode).toBeDefined();
  });

  test("write ops appear as WriteOp nodes carrying the underlying declaration kind", () => {
    const graph = JSON.parse(
      emit("function f() { let v = 0; v = 1; v = 2; return v; }\n"),
    );
    const writeOps = flattenNodes(graph.elements).filter(
      (n) => n.kind === "WriteOp",
    );
    expect(writeOps).toHaveLength(2);
    for (const op of writeOps) {
      expect(op.declarationKind).toBe("let");
      expect(op.name).toBe("v");
    }
  });

  test("function bodies become subgraphs of kind 'function' carrying ownerNodeId of the FunctionName", () => {
    const graph = JSON.parse(emit("function add(a, b) { return a + b; }\n"));
    const fnSubgraph = flattenSubgraphs(graph.elements).find(
      (s) => s.kind === "function",
    );
    expect(fnSubgraph).toBeDefined();
    const ownerNode = flattenNodes(graph.elements).find(
      (n) => n.id === fnSubgraph?.ownerNodeId,
    );
    expect(ownerNode).toBeDefined();
    expect(ownerNode?.kind).toBe("FunctionName");
    expect(ownerNode?.name).toBe("add");
    const returnSink = (fnSubgraph?.elements ?? []).find(
      (e) => e.type === "node" && e.kind === "ReturnSink",
    );
    expect(returnSink).toBeDefined();
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
      (s) => s.kind === "case",
    );
    const caseTests = cases.map((c) => c.caseTest);
    expect(caseTests).toContain('"a"');
    expect(caseTests).toContain(null);
  });

  test("emits compact JSON when pretty is false", () => {
    const out = emit("const a = 1;\n", false);
    expect(out).not.toContain("\n  ");
  });
});
