import { describe, expect, test } from "vitest";

import { EslintCompatAnalyzer } from "../../analyzer/eslint-compat/eslint-compat.js";
import type { SerializedIR } from "../../ir/model.js";
import { OxcParser } from "../../parser/oxc.js";
import { FlatSerializer } from "./flat-serializer.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();
const serializer = new FlatSerializer();

function pipe(
  code: string,
  language: "ts" | "tsx" | "js" = "ts",
): SerializedIR {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  const analyzed = analyzer.analyze(parsed);
  return serializer.serialize({
    rootScope: analyzed.rootScope,
    diagnostics: analyzed.diagnostics,
    raw: analyzed.raw,
    source: { path: `input.${language}`, language },
  });
}

describe("FlatSerializer", () => {
  test("emits version 1 IR with the source metadata", () => {
    const ir = pipe("const a = 1;\n");
    expect(ir.version).toBe(1);
    expect(ir.source).toEqual({ path: "input.ts", language: "ts" });
  });

  test("assigns deterministic scope and variable ids", () => {
    const ir1 = pipe("const a = 1;\nconst b = a;\n");
    const ir2 = pipe("const a = 1;\nconst b = a;\n");
    expect(ir1).toEqual(ir2);
    expect(ir1.scopes[0]?.id).toBe("scope#0");
    expect(ir1.variables.map((v) => v.id)).toEqual([
      "scope#0:a@6",
      "scope#0:b@19",
    ]);
  });

  test("orders references by source offset and assigns ref#N ids", () => {
    const code = `
      const a = 1;
      const b = a;
      a;
    `;
    const ir = pipe(code);
    expect(ir.references.map((r) => r.id)).toEqual(["ref#0", "ref#1"]);
    expect(ir.references[0]?.identifier.name).toBe("a");
    expect(ir.references[1]?.identifier.name).toBe("a");
    expect(ir.references[0]?.identifier.span.offset).toBeLessThan(
      ir.references[1]?.identifier.span.offset ?? Infinity,
    );
  });

  test("breaks circular references by linking ids only", () => {
    const ir = pipe("function f() { return f; }\n");
    const fVar = ir.variables.find((v) => v.name === "f");
    expect(fVar).toBeDefined();
    // Variable.scope は string (ScopeId)
    expect(typeof fVar?.scope).toBe("string");
    // Reference.resolved は VariableId 文字列
    const resolvedRef = ir.references.find((r) => r.identifier.name === "f");
    expect(resolvedRef?.resolved).toBe(fVar?.id);
    // Scope.upper も id 参照
    const fnScope = ir.scopes.find((s) => s.type === "function");
    expect(fnScope?.upper).toBe("scope#0");
  });

  test("populates flags correctly for read/write/call", () => {
    const code = `
      let x = 0;
      function add() { return x; }
      x = 1;
      add();
    `;
    const ir = pipe(code);
    const refs = ir.references;
    const xReads = refs.filter(
      (r) => r.identifier.name === "x" && r.flags.read,
    );
    const xWrites = refs.filter(
      (r) => r.identifier.name === "x" && r.flags.write,
    );
    expect(xReads.length).toBeGreaterThan(0);
    expect(xWrites.length).toBe(1);
    const addRef = refs.find((r) => r.identifier.name === "add");
    expect(addRef?.flags.call).toBe(true);
    expect(addRef?.flags.read).toBe(true);
  });

  test("collects unused variables but excludes ImplicitGlobalVariable", () => {
    const code = `
      const used = 1;
      const unused = 2;
      console.log(used);
    `;
    const ir = pipe(code);
    const unusedNames = ir.unusedVariableIds.map((id) => {
      const v = ir.variables.find((vv) => vv.id === id);
      return v?.name;
    });
    expect(unusedNames).toEqual(["unused"]);
    expect(unusedNames).not.toContain("console");
  });

  test("preserves diagnostics including var-detected entries", () => {
    const code = "var legacy = 1;\nconst x = 2;\n";
    const ir = pipe(code);
    expect(ir.diagnostics.length).toBe(1);
    expect(ir.diagnostics[0]?.kind).toBe("var-detected");
  });

  test("computes line/column for spans", () => {
    const code = "const a = 1;\nconst b = a;\n";
    const ir = pipe(code);
    const a = ir.variables.find((v) => v.name === "a");
    const b = ir.variables.find((v) => v.name === "b");
    expect(a?.identifiers[0]?.line).toBe(1);
    expect(b?.identifiers[0]?.line).toBe(2);
  });
});
