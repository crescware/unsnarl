import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../analyzer/eslint-compat.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat.js";
import { MermaidEmitter } from "./mermaid.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();
const emitter = new MermaidEmitter();

function emit(code: string, language: "ts" | "tsx" | "js" = "ts"): string {
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
  return emitter.emit(ir, {});
}

describe("MermaidEmitter", () => {
  test("identifies as 'mermaid'", () => {
    expect(emitter.format).toBe("mermaid");
    expect(emitter.contentType).toBe("text/vnd.mermaid");
  });

  test("emits flowchart LR with one node per declared variable", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/^flowchart LR\n/);
    expect(out).toContain("a : Variable");
    expect(out).toContain("b : Variable");
  });

  test("draws an edge labelled with the reference flags", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/-->\|read\| n_scope_0_a_6/);
  });

  test("attaches read,call edge for function callsites", () => {
    const out = emit("function f() {}\nf();\n");
    expect(out).toMatch(/-->\|read,call\|/);
  });

  test("uses an enclosing function variable as the edge source when nested", () => {
    const code = `
      const target = 1;
      function caller() {
        return target;
      }
    `;
    const out = emit(code);
    // caller scope is "function"; its FunctionName "caller" should be the edge source
    expect(out).toMatch(
      /n_scope_0_caller_\d+ -->\|read\| n_scope_0_target_\d+/,
    );
  });

  test("falls back to module_root when there is no owner or enclosing function", () => {
    const out = emit('console.log("hi");\n');
    expect(out).toContain("module_root");
    expect(out).toContain('module_root["(module)"]');
  });

  test("uses the initialized variable as the edge source for VariableDeclarator inits", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/n_scope_0_b_19 -->\|read\| n_scope_0_a_6/);
    expect(out).not.toContain("module_root");
  });

  test("highlights unused variables with a classDef", () => {
    const out = emit("const a = 1;\nconst unused = 2;\nconst b = a;\n");
    expect(out).toContain("classDef unused fill:#fdd,stroke:#c00;");
    expect(out).toMatch(/class n_scope_0_unused_/);
  });

  test("renders ImplicitGlobalVariable as an unresolved node", () => {
    const out = emit('console.log("hi");\n');
    expect(out).toContain("(unresolved:console)");
  });
});
