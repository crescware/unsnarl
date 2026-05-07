import { describe, expect, test } from "vitest";

import { createEslintCompatAnalyzer } from "../../analyzer/create-eslint-compat-analyzer.js";
import {
  DEFINITION_TYPE,
  type DefinitionType,
} from "../../analyzer/definition-type.js";
import { DIAGNOSTIC_KIND } from "../../analyzer/diagnostic-kind.js";
import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import { LANGUAGE, type Language } from "../../language.js";
import { OxcParser } from "../../parser/oxc-parser.js";
import { defaultSourceTypeFor } from "../../pipeline/parse/default-source-type-for.js";
import {
  SOURCE_TYPE,
  type SourceType,
} from "../../pipeline/parse/source-type.js";

const parser = new OxcParser();
const analyzer = createEslintCompatAnalyzer();

function analyze(code: string, language: Language = LANGUAGE.Ts) {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
    sourceType: defaultSourceTypeFor(language),
  });
  return analyzer.analyze(parsed);
}

function analyzeAs(code: string, language: Language, sourceType: SourceType) {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
    sourceType,
  });
  return analyzer.analyze(parsed);
}

function variableNames(scope: Scope): /* mutable */ string[] {
  return scope.variables.map((v) => v.name);
}

function findVariable(scope: Scope, name: string): Variable | null {
  return scope.variables.find((v) => v.name === name) ?? null;
}

function defTypes(variable: Variable): readonly DefinitionType[] {
  return variable.defs.map((d) => d.type);
}

function collectScopes(root: Scope): readonly Scope[] {
  const out: /* mutable */ Scope[] = [];
  function visit(s: Scope) {
    out.push(s);
    for (const c of s.childScopes) {
      visit(c);
    }
  }
  visit(root);
  return out;
}

describe("EslintCompatAnalyzer / declarations", () => {
  test("collects const/let at module scope as Variable defs", () => {
    const code = "const a = 1;\nlet b = 2;\n";
    const { rootScope } = analyze(code);

    expect(rootScope.type).toBe<ScopeType>("module");
    expect(rootScope.isStrict).toBe(true);
    expect(variableNames(rootScope).sort()).toEqual(["a", "b"]);

    const a = findVariable(rootScope, "a");
    const b = findVariable(rootScope, "b");
    expect(a && defTypes(a)).toEqual([DEFINITION_TYPE.Variable]);
    expect(b && defTypes(b)).toEqual([DEFINITION_TYPE.Variable]);
  });

  test("collects function declarations as FunctionName and class as ClassName", () => {
    const code = "function foo() {}\nclass Bar {}\n";
    const { rootScope } = analyze(code);

    expect(variableNames(rootScope).sort()).toEqual(["Bar", "foo"]);

    const foo = findVariable(rootScope, "foo");
    const bar = findVariable(rootScope, "Bar");
    expect(foo && defTypes(foo)).toEqual([DEFINITION_TYPE.FunctionName]);
    expect(bar && defTypes(bar)).toEqual([DEFINITION_TYPE.ClassName]);
  });

  test("expands destructuring patterns into individual Variables", () => {
    const code = `
      const { a, b: c } = obj;
      const [x, y, ...rest] = arr;
      const { nested: { deep } } = obj;
    `;
    const { rootScope } = analyze(code);
    const declared = rootScope.variables
      .filter((v) => v.defs.some((d) => d.type === DEFINITION_TYPE.Variable))
      .map((v) => v.name)
      .sort();
    expect(declared).toEqual(["a", "c", "deep", "rest", "x", "y"]);
  });

  test("declares import bindings (default / named / namespace) as ImportBinding", () => {
    const code = `
      import def from "x";
      import { a, b as c } from "y";
      import * as ns from "z";
    `;
    const { rootScope } = analyze(code);
    expect(variableNames(rootScope).sort()).toEqual(["a", "c", "def", "ns"]);
    for (const v of rootScope.variables) {
      expect(defTypes(v)).toEqual([DEFINITION_TYPE.ImportBinding]);
    }
  });

  test("creates function scope with parameters as Parameter defs", () => {
    const code = "function foo(a, { b }, [c], ...rest) { const inner = 1; }\n";
    const { rootScope } = analyze(code);

    expect(variableNames(rootScope)).toEqual(["foo"]);

    const fnScope = rootScope.childScopes[0];
    expect(fnScope?.type).toBe("function");
    expect(fnScope && variableNames(fnScope).sort()).toEqual([
      "a",
      "arguments",
      "b",
      "c",
      "inner",
      "rest",
    ]);
    const a = fnScope && findVariable(fnScope, "a");
    expect(a && defTypes(a)).toEqual([DEFINITION_TYPE.Parameter]);
    const inner = fnScope && findVariable(fnScope, "inner");
    expect(inner && defTypes(inner)).toEqual([DEFINITION_TYPE.Variable]);
  });

  test("creates block scope only for non-function blocks", () => {
    const code = `
      function foo() {
        const a = 1;
        {
          const b = 2;
        }
      }
    `;
    const { rootScope } = analyze(code);
    const fn = rootScope.childScopes[0];
    expect(fn?.type).toBe("function");
    // function scope の直下に block scope (内側 {}) のみ
    expect(fn?.childScopes.length).toBe(1);
    const inner = fn?.childScopes[0];
    expect(inner?.type).toBe("block");
    expect(inner && variableNames(inner)).toEqual(["b"]);
    // a は function scope に。`arguments` は FunctionDeclarationInstantiation
    // による暗黙束縛として function scope に並ぶ。
    expect(fn && variableNames(fn).sort()).toEqual(["a", "arguments"]);
  });

  test("creates for scope and binds let inside the for-init", () => {
    const code = "for (let i = 0; i < 10; i++) { const x = i; }\n";
    const { rootScope } = analyze(code);
    const forScope = rootScope.childScopes[0];
    expect(forScope?.type).toBe("for");
    expect(forScope && variableNames(forScope)).toEqual(["i"]);
    const block = forScope?.childScopes[0];
    expect(block?.type).toBe("block");
    expect(block && variableNames(block)).toEqual(["x"]);
  });

  test("creates catch scope with CatchClause Definition", () => {
    const code = "try { } catch (e) { const x = 1; }\n";
    const { rootScope } = analyze(code);
    // try block (block scope) と catch scope (catch)
    const allScopes = collectScopes(rootScope);
    const catchScope = allScopes.find((s) => s.type === SCOPE_TYPE.Catch);
    expect(catchScope).toBeDefined();
    expect(catchScope && variableNames(catchScope!).sort()).toEqual(["e", "x"]);
    const e = catchScope && findVariable(catchScope!, "e");
    expect(e && defTypes(e)).toEqual([DEFINITION_TYPE.CatchClause]);
  });

  test("declares var bindings while still emitting a var-detected diagnostic", () => {
    const code = "var legacy = 1;\nconst modern = 2;\n";
    const { rootScope, diagnostics } = analyze(code);
    expect(variableNames(rootScope).sort()).toEqual(["legacy", "modern"]);
    expect(diagnostics.length).toBe(1);
    expect(diagnostics[0]?.kind).toBe(DIAGNOSTIC_KIND.VarDetected);
    expect(diagnostics[0]?.span?.line).toBe(1);
  });

  test("ignores TS interface / type / enum at the top level", () => {
    const code = `
      interface I { x: number }
      type T = string;
      enum E { A, B }
      const value = 1;
    `;
    const { rootScope } = analyze(code);
    // interface / type / enum は宣言収集対象外
    expect(variableNames(rootScope)).toEqual(["value"]);
  });

  test("treats top-level body as the analyzer's primary scope (module for ts)", () => {
    const code = "const x = 1;\n";
    const { rootScope } = analyze(code, "ts");
    expect(rootScope.type).toBe("module");
    expect(rootScope.isStrict).toBe(true);
  });

  test("hoists function declarations across the module scope", () => {
    const code = `
      const result = foo();
      function foo() { return 1; }
    `;
    const { rootScope } = analyze(code);
    expect(variableNames(rootScope).sort()).toEqual(["foo", "result"]);
  });

  test("treats a JS source parsed as module as a module scope (strict)", () => {
    const code = "const x = 1;\n";
    const { rootScope } = analyzeAs(code, LANGUAGE.Js, SOURCE_TYPE.Module);
    expect(rootScope.type).toBe<ScopeType>("module");
    expect(rootScope.isStrict).toBe(true);
  });

  test("treats a TS source parsed as script as a global scope", () => {
    const code = "const x = 1;\n";
    const { rootScope } = analyzeAs(code, LANGUAGE.Ts, SOURCE_TYPE.Script);
    expect(rootScope.type).toBe<ScopeType>("global");
  });

  test("creates separate Variables for shadowing inside nested function", () => {
    const code = `
      const x = 1;
      function inner() {
        const x = 2;
      }
    `;
    const { rootScope } = analyze(code);
    expect(variableNames(rootScope).sort()).toEqual(["inner", "x"]);
    const inner = rootScope.childScopes[0];
    expect(inner && variableNames(inner).sort()).toEqual(["arguments", "x"]);
    const outerX = findVariable(rootScope, "x");
    const innerX = inner && findVariable(inner, "x");
    expect(outerX).not.toBe(innerX);
  });
});
