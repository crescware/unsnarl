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
    expect(out).toContain('"a<br/>L1"');
    expect(out).toContain('"b<br/>L2"');
  });

  test("decorates labels per Definition kind", () => {
    const out = emit(
      [
        "import imp from 'x';",
        "function foo() { return 1; }",
        "class Bar {}",
        "function take(p: number) { try { p; } catch (e) { e; } }",
        "const used = imp;",
        "const a = take;",
        "const b = foo;",
        "const c = Bar;",
        "const d = used;",
        "void a; void b; void c; void d;",
      ].join("\n"),
    );
    expect(out).toContain('"imp<br/>');
    expect(out).not.toContain('"import imp<br/>');
    expect(out).toContain('"foo()<br/>');
    expect(out).toContain('"class Bar<br/>');
    expect(out).toContain('"take()<br/>');
    expect(out).toContain('"p<br/>');
    expect(out).not.toContain('"param p<br/>');
    expect(out).toContain('"catch e<br/>');
    expect(out).toContain('"used<br/>');
    expect(out).not.toMatch(/" : Variable/);
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
    expect(out).toMatch(/subgraph n_scope_0_f_9\["f\(\)/);
    expect(out).toContain("direction RL");
    expect(out).toContain("return_scope_0_f_9((return))");
    expect(out).toMatch(/n_scope_1_x_\d+ -->\|read\| return_scope_0_f_9/);
    expect(out).toContain("end");
  });

  test("subgraphs arrow functions assigned to a const", () => {
    const out = emit("const fn = (p: number) => p + 1;\n");
    expect(out).toMatch(/subgraph n_scope_0_fn_6\["fn\(\)/);
    expect(out).toContain("return_scope_0_fn_6((return))");
    expect(out).toMatch(/n_scope_1_p_\d+ -->\|read\| return_scope_0_fn_6/);
  });

  test("subgraphs function expressions assigned to a const", () => {
    const out = emit("const fn = function inner(p: number) { return p; };\n");
    expect(out).toMatch(/subgraph n_scope_0_fn_6\["fn\(\)/);
    expect(out).toContain("return_scope_0_fn_6((return))");
  });

  test("highlights unused variables with a colorless dashed stroke", () => {
    const out = emit("const a = 1;\nconst unused = 2;\nconst b = a;\n");
    expect(out).toContain("classDef unused stroke-dasharray: 5 5;");
    expect(out).not.toMatch(/fill:#|stroke:#/);
    expect(out).toMatch(/class n_scope_0_unused_/);
  });

  test("renders ImplicitGlobalVariable as a 'global' node when used directly", () => {
    const out = emit("function f() { return globalThing; }\n");
    expect(out).toContain('"global globalThing<br/>');
  });

  test("hides ImplicitGlobalVariable that only appears as a member receiver", () => {
    const out = emit("const xs = Object.keys(arg);\n");
    expect(out).not.toContain('"global Object');
    // arg もグローバルだが、引数として直接読まれるので残る
    expect(out).toContain('"global arg<br/>');
  });

  test("falls back to a (module) sink only for module-level owner-less references", () => {
    const out = emit("function f() {}\nf();\n");
    expect(out).toContain("module_root((module))");
  });

  test("subgraphs try / catch / finally blocks with line numbers", () => {
    const code = [
      "let v = 0;",
      "try {",
      "  v = 1;",
      "} catch (err) {",
      "  v = 2;",
      "} finally {",
      "  v = 3;",
      "}",
    ].join("\n");
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["try L2"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["catch L4"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["finally L6"\]/);
  });

  test("subgraphs if / else blocks", () => {
    const code = "let x = 0;\nif (true) {\n  x = 1;\n} else {\n  x = 2;\n}\n";
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["if L2"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["else L4"\]/);
  });

  test("subgraphs switch statements", () => {
    const code =
      "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'a': l = 'A'; break;\n  default: l = '?';\n}\n";
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["switch L3"\]/);
  });

  test("encodes double quotes in labels with HTML entity, never with backslash", () => {
    const code =
      'let l = "";\nconst k = "a";\nswitch (k) {\n  case "x": l = "x"; break;\n}\n';
    const out = emit(code);
    expect(out).not.toContain('\\"');
    expect(out).toMatch(/case &quot;x&quot;/);
  });

  test("encodes & < > in case labels with HTML entities", () => {
    const code =
      "let l = 0;\nconst a = 1;\nconst b = 2;\nswitch (a) {\n  case (a & b): l = 1; break;\n  case (a < b ? 1 : 0): l = 2; break;\n}\n";
    const out = emit(code);
    expect(out).not.toMatch(/case [^"]*&[^a-z]/);
    expect(out).toContain("&amp;");
    expect(out).toContain("&lt;");
  });

  test("expands import declarations into module/intermediate nodes", () => {
    const out = emit(
      [
        "import def from 'some-default';",
        "import { named, other as renamed } from 'some-named';",
        "import * as ns from 'some-namespace';",
        "const a = def;",
        "const b = named;",
        "const c = renamed;",
        "const d = ns;",
      ].join("\n"),
    );
    expect(out).toContain('mod_some_default["module some-default<br/>L1"]');
    expect(out).toContain('mod_some_named["module some-named<br/>L2"]');
    expect(out).toContain('mod_some_namespace["module some-namespace<br/>L3"]');
    expect(out).toMatch(/import_some_named__other\["import other<br\/>L2"\]/);
    expect(out).toMatch(/mod_some_default -->\|read\| n_scope_0_def_/);
    expect(out).toMatch(/mod_some_named -->\|read\| n_scope_0_named_/);
    expect(out).toMatch(/mod_some_named -->\|read\| import_some_named__other/);
    expect(out).toMatch(
      /import_some_named__other -->\|read\| n_scope_0_renamed_/,
    );
    expect(out).toMatch(/mod_some_namespace -->\|read\| n_scope_0_ns_/);
    expect(out).toContain('"import ns<br/>');
    expect(out).toContain('"def<br/>');
    expect(out).toContain('"named<br/>');
    expect(out).toContain('"renamed<br/>');
    expect(out).not.toContain('"import def<br/>');
    expect(out).not.toContain('"import named<br/>');
    expect(out).not.toContain('"import renamed<br/>');
  });
});
