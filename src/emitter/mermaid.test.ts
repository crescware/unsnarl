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

  test("emits flowchart RL with one node per declared variable", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/^flowchart RL\n/);
    expect(out).toContain("a : Variable");
    expect(out).toContain("b : Variable");
  });

  test("draws a data-flow edge from the source variable to the initialized variable", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/n_scope_0_a_6 -->\|read\| n_scope_0_b_19/);
  });

  test("attaches read,call edge from callee to the variable receiving the call result", () => {
    const out = emit("function f() {}\nconst x = f();\n");
    expect(out).toMatch(/n_scope_0_f_9 -->\|read,call\| n_scope_0_x_/);
  });

  test("renders a function as a subgraph and routes return through a return node", () => {
    const out = emit("function f() {\n  const x = 1;\n  return x;\n}\n");
    expect(out).toMatch(/subgraph n_scope_0_f_9\["f : FunctionName/);
    expect(out).toContain("direction RL");
    expect(out).toContain("return_scope_0_f_9((return))");
    expect(out).toMatch(/n_scope_1_x_\d+ -->\|read\| return_scope_0_f_9/);
    expect(out).toContain("end");
  });

  test("highlights unused variables with a colorless dashed stroke", () => {
    const out = emit("const a = 1;\nconst unused = 2;\nconst b = a;\n");
    expect(out).toContain("classDef unused stroke-dasharray: 5 5;");
    expect(out).not.toMatch(/fill:#|stroke:#/);
    expect(out).toMatch(/class n_scope_0_unused_/);
  });

  test("renders ImplicitGlobalVariable as an unresolved node", () => {
    const out = emit('console.log("hi");\n');
    expect(out).toContain("(unresolved:console)");
  });

  test("falls back to a (module) sink only for module-level owner-less references", () => {
    const out = emit('console.log("hi");\n');
    expect(out).toContain("module_root((module))");
  });
});
