import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../../analyzer/eslint-compat.js";
import { OxcParser } from "../../parser/oxc.js";
import { FlatSerializer } from "../../serializer/flat.js";
import { MermaidEmitter } from "./mermaid.js";
import { dagreStrategy } from "./strategy/dagre-strategy.js";
import { elkStrategy } from "./strategy/elk-strategy.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();
const emitter = new MermaidEmitter({ strategy: elkStrategy });

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

  test("renderer defaults to elk and prepends an init directive", () => {
    const out = emit("const a = 1;\n");
    expect(
      out.startsWith('%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%\n'),
    ).toBe(true);
  });

  test("renderer 'dagre' omits the init directive entirely (Mermaid's default)", () => {
    const dagre = new MermaidEmitter({ strategy: dagreStrategy });
    const parsed = parser.parse("const a = 1;\n", {
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
    const out = dagre.emit(ir, {});
    expect(out).not.toContain("%%{init");
    expect(out).toMatch(/^flowchart RL\n/);
  });

  test("emits flowchart RL with one node per declared variable", () => {
    const out = emit("const a = 1;\nconst b = a;\n");
    expect(out).toMatch(/^%%\{init:.*"elk".*\}%%\nflowchart RL\n/);
    expect(out).toContain('"a<br/>L1"');
    // b is the terminal of the chain so it ends up unused; the label
    // gains the "unused " prefix introduced for the textual cue.
    expect(out).toContain('"unused b<br/>L2"');
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
    expect(out).toContain('"import imp<br/>');
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

  test("renders a function as a subgraph and routes return through a return subgraph with per-use nodes", () => {
    const out = emit("function f() {\n  const x = 1;\n  return x;\n}\n");
    expect(out).toMatch(/subgraph s_scope_\d+\["f\(\)/);
    // f is never called, so its declaration node carries the "unused " prefix.
    expect(out).toMatch(/n_scope_0_f_9\["unused f\(\)<br\/>L1"\]/);
    expect(out).toContain("direction RL");
    expect(out).toMatch(/subgraph s_return_scope_0_f_9_\w+\["return L3"\]/);
    expect(out).toMatch(/ret_use_\w+\["x<br\/>L3"\]/);
    expect(out).toMatch(/n_scope_1_x_\d+ -->\|read\| ret_use_\w+/);
    expect(out).toContain("end");
  });

  test("subgraphs arrow functions assigned to a const", () => {
    const out = emit("const fn = (p: number) => p + 1;\n");
    expect(out).toMatch(/subgraph s_scope_\d+\["fn\(\)/);
    expect(out).toMatch(/n_scope_0_fn_6\["unused fn\(\)<br\/>L1"\]/);
    expect(out).toMatch(/subgraph s_return_scope_0_fn_6_\w+\["return L1"\]/);
    expect(out).toMatch(/n_scope_1_p_\d+ -->\|read\| ret_use_\w+/);
  });

  test("subgraphs function expressions assigned to a const", () => {
    const out = emit("const fn = function inner(p: number) { return p; };\n");
    expect(out).toMatch(/subgraph s_scope_\d+\["fn\(\)/);
    expect(out).toMatch(/n_scope_0_fn_6\["unused fn\(\)<br\/>L1"\]/);
    expect(out).toMatch(/subgraph s_return_scope_0_fn_6_\w+\["return L1"\]/);
  });

  test("multi-line JSX opening tags render as &lt;Name&gt; with an L{open}-{close} range", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => (",
      "  <A>",
      "    hello",
      "  </A>",
      ");",
    ].join("\n");
    const out = emit(code, "tsx");
    expect(out).toMatch(/ret_use_\w+\["&lt;A&gt;<br\/>L3-5"\]/);
  });

  test("single-line JSX elements still render as &lt;Name&gt; but collapse to a single line label", () => {
    const code = [
      "import { A } from 'm';",
      "const App = () => <A>hi</A>;",
    ].join("\n");
    const out = emit(code, "tsx");
    expect(out).toMatch(/ret_use_\w+\["&lt;A&gt;<br\/>L2"\]/);
    expect(out).not.toMatch(/L2-/);
  });

  test("a non-JSX ReturnUse keeps the bare name without angle-bracket wrapping", () => {
    const out = emit("function f(a) { return a; }\n");
    expect(out).toMatch(/ret_use_\w+\["a<br\/>L1"\]/);
    expect(out).not.toContain("&lt;a&gt;");
  });

  test("a multi-line return statement yields a return subgraph spanning the whole statement", () => {
    const code = [
      "function build() {",
      "  const a = 1;",
      "  const b = 2;",
      "  return {",
      "    a,",
      "    b,",
      "  };",
      "}",
    ].join("\n");
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["build\(\)<br\/>L1-8"\]/);
    expect(out).toMatch(/subgraph s_return_scope_\w+\["return L4-7"\]/);
    expect(out).toMatch(/ret_use_\w+\["a<br\/>L5"\]/);
    expect(out).toMatch(/ret_use_\w+\["b<br\/>L6"\]/);
  });

  test("a block-body arrow with an explicit ReturnStatement uses the return statement's span", () => {
    const code = ["const fn = (x) => {", "  return x;", "};"].join("\n");
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["fn\(\)<br\/>L1-3"\]/);
    expect(out).toMatch(/subgraph s_return_scope_\w+\["return L2"\]/);
    expect(out).toMatch(/ret_use_\w+\["x<br\/>L2"\]/);
  });

  test("a multi-line arrow with an expression body uses the body's span as the implicit return", () => {
    const code = ["const fn = (x) => (", "  x + 1", ");"].join("\n");
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["fn\(\)<br\/>L1-3"\]/);
    expect(out).toMatch(/subgraph s_return_scope_\w+\["return L1-3"\]/);
    expect(out).toMatch(/ret_use_\w+\["x<br\/>L2"\]/);
  });

  test("each ReturnStatement renders its own subgraph; they are not merged", () => {
    const code = [
      "function pick(k) {",
      "  const a = 1;",
      "  const b = 2;",
      "  if (k) {",
      "    return a;",
      "  }",
      "  return b;",
      "}",
    ].join("\n");
    const out = emit(code);
    // Two distinct return subgraphs, one per ReturnStatement, with line
    // labels matching their own statement (not a merged span).
    expect(
      countMatches(out, /^\s*subgraph s_return_scope_\w+\["return L5"\]/),
    ).toBe(1);
    expect(
      countMatches(out, /^\s*subgraph s_return_scope_\w+\["return L7"\]/),
    ).toBe(1);
    // The merged "L5-7" header that the previous single-subgraph design
    // would have produced must not appear.
    expect(out).not.toMatch(/return L5-7/);
  });

  test("marks unused declarations with an 'unused' prefix in the label", () => {
    const out = emit("const a = 1;\nconst unused = 2;\nconst b = a;\n");
    // The dashed-stroke classDef is gone; the cue is the textual prefix.
    expect(out).not.toContain("classDef unused");
    expect(out).not.toMatch(/^\s*class n_scope_/m);
    // `unused` (the variable name) plus the "unused " prefix yield this
    // intentional double-word label.
    expect(out).toMatch(/n_scope_0_unused_\d+\["unused unused<br\/>L2"\]/);
    // `b` is also unused (terminal of the chain).
    expect(out).toMatch(/n_scope_0_b_\d+\["unused b<br\/>L3"\]/);
    // `a` is read by b so it stays without the prefix.
    expect(out).toMatch(/n_scope_0_a_\d+\["a<br\/>L1"\]/);
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
    expect(out).toMatch(/subgraph s_scope_\d+\["try L2-4"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["catch L4-6"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["finally L6-8"\]/);
  });

  test("subgraphs if / else blocks", () => {
    const code = "let x = 0;\nif (true) {\n  x = 1;\n} else {\n  x = 2;\n}\n";
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["if L2-4"\]/);
    expect(out).toMatch(/subgraph s_scope_\d+\["else L4-6"\]/);
  });

  test("subgraphs switch statements", () => {
    const code =
      "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'a': l = 'A'; break;\n  default: l = '?';\n}\n";
    const out = emit(code);
    expect(out).toMatch(/subgraph s_scope_\d+\["switch L3-6"\]/);
  });

  test("encodes double quotes in labels with HTML entity, never with backslash", () => {
    const code =
      'let l = "";\nconst k = "a";\nswitch (k) {\n  case "x": l = "x"; break;\n}\n';
    const out = emit(code);
    expect(out).not.toContain('\\"');
    expect(out).toMatch(/case &quot;x&quot;/);
  });

  test("preserves single quotes in case labels verbatim, without HTML entities", () => {
    const code =
      "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'x': l = 'x'; break;\n}\n";
    const out = emit(code);
    expect(out).toMatch(/case 'x' L\d+/);
    expect(out).not.toContain("&quot;");
    expect(out).not.toContain("&apos;");
  });

  test("case label quote style mirrors the source verbatim except for HTML escaping", () => {
    const single = emit(
      "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'x': l = 'x'; break;\n}\n",
    );
    const double = emit(
      'let l = "";\nconst k = "a";\nswitch (k) {\n  case "x": l = "x"; break;\n}\n',
    );
    expect(single).toMatch(/case 'x' L\d+/);
    expect(double).toMatch(/case &quot;x&quot; L\d+/);
    expect(single).not.toMatch(/case &quot;/);
    expect(double).not.toMatch(/case '/);
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
    expect(out).toContain('"import def<br/>');
    expect(out).toContain('"import named<br/>');
    expect(out).toContain('"renamed<br/>');
    expect(out).not.toContain('"import renamed<br/>');
  });
});

function lines(out: string): string[] {
  return out.split("\n");
}

function edgesFor(out: string): string[] {
  return lines(out).filter((l) => l.includes(" -->|"));
}

function countMatches(out: string, re: RegExp): number {
  let count = 0;
  for (const line of lines(out)) {
    if (re.test(line)) {
      count += 1;
    }
  }
  return count;
}

function nodeIdOf(out: string, name: string): string {
  const re = new RegExp(`(n_scope_0_${name}_\\d+)\\["[^"]*${name}[^"]*"\\]`);
  const m = out.match(re);
  if (!m) {
    throw new Error(`node for "${name}" not found in:\n${out}`);
  }
  return m[1] as string;
}

describe("MermaidEmitter rendering: switch with break", () => {
  const code = [
    'let label = "";',
    'const kind = "a";',
    "switch (kind) {",
    '  case "a":',
    '    label = "alpha";',
    "    break;",
    '  case "b":',
    '    label = "beta";',
    "    break;",
    "  default:",
    '    label = "other";',
    "    break;",
    "}",
    "const result = label;",
  ].join("\n");
  const out = emit(code);

  test("each case becomes its own labelled subgraph and no fallthrough edges are drawn", () => {
    expect(
      countMatches(out, /^\s*subgraph s_scope_\d+\["case .* L\d+(?:-\d+)?"\]/),
    ).toBe(2);
    expect(
      countMatches(out, /^\s*subgraph s_scope_\d+\["default L\d+(?:-\d+)?"\]/),
    ).toBe(1);
    expect(out).not.toContain("|fallthrough|");
  });

  test("the declaration fans out to every case via one |set| edge each", () => {
    const decl = "n_scope_0_label_4";
    const setEdges = edgesFor(out).filter(
      (l) => l.startsWith(`  ${decl} -->`) && l.includes("|set|"),
    );
    expect(setEdges).toHaveLength(3);
  });

  test("each case fans into result via one |read| edge", () => {
    const result = nodeIdOf(out, "result");
    const reads = edgesFor(out).filter(
      (l) => l.includes("|read|") && l.endsWith(result),
    );
    expect(reads).toHaveLength(3);
  });

  test("the discriminant routes into the switch subgraph, not the module sink", () => {
    expect(out).toMatch(/n_scope_0_kind_\d+ -->\|read\| s_scope_1\b/);
    expect(out).not.toMatch(/n_scope_0_kind_\d+ -->\|read\| module_root/);
  });
});

describe("MermaidEmitter rendering: switch with fallthrough", () => {
  const code = [
    'let label = "";',
    'const kind = "a";',
    "switch (kind) {",
    '  case "a":',
    '    label = "alpha";',
    '  case "b":',
    '    label = "beta";',
    "  default:",
    '    label = "other";',
    "}",
    "const result = label;",
  ].join("\n");
  const out = emit(code);

  test("the declaration only emits one |set| edge, into the head case", () => {
    const setEdges = edgesFor(out).filter(
      (l) => l.startsWith(`  n_scope_0_label_4 -->`) && l.includes("|set|"),
    );
    expect(setEdges).toHaveLength(1);
  });

  test("non-terminal cases are stitched together with |fallthrough| edges", () => {
    const ft = edgesFor(out).filter((l) => l.includes("|fallthrough|"));
    expect(ft).toHaveLength(2);
  });

  test("only the terminal case feeds result", () => {
    const result = nodeIdOf(out, "result");
    const reads = edgesFor(out).filter(
      (l) => l.includes("|read|") && l.endsWith(result),
    );
    expect(reads).toHaveLength(1);
  });
});

describe("MermaidEmitter rendering: if/else", () => {
  const code = [
    "let counter = 0;",
    "const flag = true;",
    "if (flag) {",
    "  counter = 1;",
    "} else {",
    "  counter = 2;",
    "}",
    "const result = counter;",
  ].join("\n");
  const out = emit(code);

  test("an outer if-else container subgraph wraps both arms", () => {
    expect(out).toMatch(/^\s*subgraph cont_if_scope_0_\d+\["if-else L3-7"\]/m);
    expect(out).toMatch(/^\s*subgraph s_scope_\d+\["if L3-5"\]/m);
    expect(out).toMatch(/^\s*subgraph s_scope_\d+\["else L5-7"\]/m);
  });

  test("the predicate identifier feeds the if-else container", () => {
    expect(out).toMatch(/n_scope_0_flag_\d+ -->\|read\| cont_if_scope_0_\d+/);
  });

  test("both branches independently feed result; the declaration does NOT bypass", () => {
    const result = nodeIdOf(out, "result");
    const reads = edgesFor(out).filter(
      (l) => l.includes("|read|") && l.endsWith(result),
    );
    expect(reads).toHaveLength(2);
    expect(out).not.toMatch(
      new RegExp(`n_scope_0_counter_4 -->\\|read\\| ${result}\\b`),
    );
  });
});

describe("MermaidEmitter rendering: if without else", () => {
  const code = [
    "let counter = 0;",
    "const flag = true;",
    "if (flag) {",
    "  counter = 1;",
    "}",
    "const result = counter;",
  ].join("\n");
  const out = emit(code);

  test("there is no if-else container, just a bare if subgraph", () => {
    expect(out).not.toContain("if-else L");
    expect(out).toMatch(/^\s*subgraph s_scope_\d+\["if L3-5"\]/m);
  });

  test("the predicate flows directly into the bare if subgraph", () => {
    expect(out).toMatch(/n_scope_0_flag_\d+ -->\|read\| s_scope_1\b/);
  });

  test("result has two origins: the if-write AND the original declaration", () => {
    const result = nodeIdOf(out, "result");
    const reads = edgesFor(out).filter(
      (l) => l.includes("|read|") && l.endsWith(result),
    );
    expect(reads).toHaveLength(2);
    expect(out).toMatch(
      new RegExp(`n_scope_0_counter_4 -->\\|read\\| ${result}\\b`),
    );
  });
});

describe("MermaidEmitter rendering: catch parameter placement", () => {
  test("the catch parameter node lives inside the catch subgraph", () => {
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
    const ls = lines(out);
    const catchStart = ls.findIndex((l) => l.includes('"catch L4-6"'));
    expect(catchStart).toBeGreaterThan(-1);
    const catchEnd = ls.slice(catchStart).findIndex((l) => l.trim() === "end");
    expect(catchEnd).toBeGreaterThan(0);
    const inside = ls.slice(catchStart, catchStart + catchEnd);
    // err isn't read inside the catch body, so it carries the unused prefix.
    expect(inside.some((l) => l.includes('"unused catch err<br/>'))).toBe(true);
  });
});

describe("MermaidEmitter rendering: let writes form a state chain", () => {
  const code = [
    "function f() {",
    "  let v = 0;",
    "  v = 1;",
    "  v = 2;",
    "  return v;",
    "}",
  ].join("\n");
  const out = emit(code);

  test("the chain passes through one wr_ref node per assignment, in source order", () => {
    expect(out).toMatch(/n_scope_1_v_\d+ -->\|set\| wr_ref_0/);
    expect(out).toMatch(/wr_ref_0 -->\|set\| wr_ref_1/);
    expect(out).toMatch(/wr_ref_1 -->\|read\| ret_use_\w+/);
  });

  test("there is no v -> v self loop", () => {
    expect(out).not.toMatch(/n_scope_1_v_\d+ -->\|.*\| n_scope_1_v_\d+/);
  });

  test("the declaration node uses a rectangle and write ops use stadium", () => {
    expect(out).toMatch(/n_scope_1_v_\d+\["let v<br\/>L2"\]/);
    expect(out).toMatch(/wr_ref_0\(\["let v<br\/>L3"\]\)/);
    expect(out).toMatch(/wr_ref_1\(\["let v<br\/>L4"\]\)/);
  });
});

describe("MermaidEmitter rendering: case labels", () => {
  test("numeric and identifier case tests are rendered verbatim", () => {
    const out = emit(
      "const X = 1; let l = 0; const k = 1; switch (k) { case 0: l = 1; break; case X: l = 2; break; }\n",
    );
    expect(out).toMatch(/case 0 L\d+/);
    expect(out).toMatch(/case X L\d+/);
  });

  test("the default clause label is just 'default L<n>', not 'case default'", () => {
    const out = emit(
      'let l = ""; switch (1) { case 1: l = "a"; break; default: l = "b"; }\n',
    );
    expect(out).toMatch(/default L\d+/);
    expect(out).not.toContain("case default");
  });
});

describe("MermaidEmitter rendering: destructuring fan-out", () => {
  const code = [
    "const source = { a: 1, b: 2, nested: { d: 4 } };",
    "const list = [10, 20, 30];",
    "const { a, b: renamed } = source;",
    "const { nested: { d } } = source;",
    "const [first, , third] = list;",
    "const sum = a + renamed + d + first + third;",
  ].join("\n");
  const out = emit(code);

  test("the object source fans out to every named/renamed/deep binding individually", () => {
    expect(out).toMatch(/n_scope_0_source_\d+ -->\|read\| n_scope_0_a_\d+/);
    expect(out).toMatch(
      /n_scope_0_source_\d+ -->\|read\| n_scope_0_renamed_\d+/,
    );
    expect(out).toMatch(/n_scope_0_source_\d+ -->\|read\| n_scope_0_d_\d+/);
  });

  test("the array source fans out to its positional bindings, never to object bindings", () => {
    expect(out).toMatch(/n_scope_0_list_\d+ -->\|read\| n_scope_0_first_/);
    expect(out).toMatch(/n_scope_0_list_\d+ -->\|read\| n_scope_0_third_/);
    expect(out).not.toMatch(
      /n_scope_0_list_\d+ -->\|read\| n_scope_0_renamed_/,
    );
    expect(out).not.toMatch(/n_scope_0_list_\d+ -->\|read\| n_scope_0_d_\d+/);
  });
});

describe("MermaidEmitter rendering: import label prefix rule", () => {
  const out = emit(
    [
      "import def from 'some-default';",
      "import { named, other as renamed } from 'some-named';",
      "import * as ns from 'some-namespace';",
      "void def; void named; void renamed; void ns;",
    ].join("\n"),
  );

  test("default imports get an 'import ' prefix on the local node", () => {
    expect(out).toMatch(/n_scope_0_def_\d+\["import def<br\/>L1"\]/);
  });

  test("named imports whose local name matches the imported name keep the 'import ' prefix", () => {
    expect(out).toMatch(/n_scope_0_named_\d+\["import named<br\/>L2"\]/);
  });

  test("renamed named imports drop the prefix on the local node (the original name lives on the intermediate)", () => {
    expect(out).toMatch(/n_scope_0_renamed_\d+\["renamed<br\/>L2"\]/);
    expect(out).not.toMatch(/n_scope_0_renamed_\d+\["import renamed<br\/>/);
    expect(out).toMatch(/import_some_named__other\["import other<br\/>L2"\]/);
  });

  test("namespace imports get an 'import ' prefix on the local node", () => {
    expect(out).toMatch(/n_scope_0_ns_\d+\["import ns<br\/>L3"\]/);
  });
});

describe("MermaidEmitter rendering: function parameters are not duplicated", () => {
  test("each parameter renders exactly one |read| edge into its return-use node", () => {
    const out = emit("function add(a, b) { return a + b; }\n");
    expect(countMatches(out, /n_scope_1_a_\d+ -->\|read\| ret_use_\w+/)).toBe(
      1,
    );
    expect(countMatches(out, /n_scope_1_b_\d+ -->\|read\| ret_use_\w+/)).toBe(
      1,
    );
  });
});
