import { describe, expect, test } from "vitest";

import { LANGUAGE, type Language } from "../language.js";
import type { Reference } from "../ir/reference/reference.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { DEFINITION_TYPE } from "./definition-type.js";
import { EslintCompatAnalyzer } from "./eslint-compat/eslint-compat.js";
import { SCOPE_TYPE } from "./scope-type.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();

function analyze(code: string, language: Language = LANGUAGE.Ts) {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  return analyzer.analyze(parsed);
}

function findVariable(scope: Scope, name: string): Variable | null {
  return scope.variables.find((v) => v.name === name) ?? null;
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

function refsOf(scope: Scope, name: string): readonly Reference[] {
  const v = findVariable(scope, name);
  return v ? [...v.references] : [];
}

describe("EslintCompatAnalyzer / references", () => {
  test("classifies a simple read reference", () => {
    const { rootScope } = analyze("const a = 1;\nconsole.log(a);\n");
    const a = findVariable(rootScope, "a")!;
    expect(a.references.length).toBe(1);
    expect(a.references[0]?.isRead()).toBe(true);
    expect(a.references[0]?.isWrite()).toBe(false);
    expect(a.references[0]?.isCall?.()).toBe(false);
  });

  test("classifies write on `=`, read+write on compound assignment, read+write on update", () => {
    const code = `
      let x = 0;
      x = 1;
      x += 1;
      x++;
    `;
    const { rootScope } = analyze(code);
    const refs = refsOf(rootScope, "x");
    expect(refs.length).toBe(3);
    const [r1, r2, r3] = refs;
    expect([r1?.isWrite(), r1?.isRead()]).toEqual([true, false]);
    expect([r2?.isWrite(), r2?.isRead()]).toEqual([true, true]);
    expect([r3?.isWrite(), r3?.isRead()]).toEqual([true, true]);
    expect(r1?.isWriteOnly()).toBe(true);
    expect(r2?.isReadWrite()).toBe(true);
    expect(r3?.isReadWrite()).toBe(true);
  });

  test("classifies read+call on CallExpression callee and NewExpression callee", () => {
    const code = `
      function foo() { return 1; }
      class Bar {}
      foo();
      new Bar();
    `;
    const { rootScope } = analyze(code);
    const fooRefs = refsOf(rootScope, "foo");
    expect(fooRefs.length).toBe(1);
    expect(fooRefs[0]?.isCall?.()).toBe(true);
    expect(fooRefs[0]?.isRead()).toBe(true);

    const barRefs = refsOf(rootScope, "Bar");
    expect(barRefs.length).toBe(1);
    expect(barRefs[0]?.isCall?.()).toBe(true);
    expect(barRefs[0]?.isRead()).toBe(true);
  });

  test("captures member object as Read but skips member property name", () => {
    const code = `
      const obj = { a: 1 };
      const value = obj.a;
    `;
    const { rootScope } = analyze(code);
    const obj = findVariable(rootScope, "obj")!;
    expect(obj.references.length).toBe(1);
    expect(obj.references[0]?.isRead()).toBe(true);
  });

  test("does not generate references for object literal property keys (computed=false)", () => {
    const code = `
      const obj = { foo: 1 };
    `;
    const { rootScope } = analyze(code);
    // foo は宣言されていない名前なので、参照が誤って作られると through に積まれる
    const allScopes = collectScopes(rootScope);
    for (const s of allScopes) {
      for (const r of s.through) {
        expect(r.identifier.name).not.toBe("foo");
      }
    }
  });

  test("hoisted function reference is resolved before its declaration site", () => {
    const code = `
      const result = foo();
      function foo() { return 1; }
    `;
    const { rootScope } = analyze(code);
    const foo = findVariable(rootScope, "foo")!;
    expect(foo.references.length).toBe(1);
    expect(foo.references[0]?.isCall?.()).toBe(true);
  });

  test("inner reference resolves to inner shadowing variable, outer remains unused", () => {
    const code = `
      const x = 1;
      function inner() {
        const x = 2;
        return x;
      }
    `;
    const { rootScope } = analyze(code);
    const outerX = findVariable(rootScope, "x")!;
    const innerScope = rootScope.childScopes[0]!;
    const innerX = findVariable(innerScope, "x")!;
    expect(outerX.references.length).toBe(0);
    expect(outerX.unsnarlIsUnused?.()).toBe(true);
    expect(innerX.references.length).toBe(1);
    expect(innerX.references[0]?.isRead()).toBe(true);
  });

  test("unresolved identifier resolves to an ImplicitGlobalVariable and is propagated through", () => {
    const code = `
      function foo() {
        return globalThing;
      }
    `;
    const { rootScope } = analyze(code);
    expect(rootScope.through.length).toBe(1);
    expect(rootScope.through[0]?.identifier.name).toBe("globalThing");
    const implicit = findVariable(rootScope, "globalThing");
    expect(implicit).toBeDefined();
    expect(implicit?.defs.map((d) => d.type)).toEqual([
      DEFINITION_TYPE.ImplicitGlobalVariable,
    ]);
    expect(rootScope.through[0]?.resolved).toBe(implicit);
  });

  test("unsnarlIsUnused returns true for unused imports and consts", () => {
    const code = `
      import unused from "x";
      import { used } from "y";
      const a = used;
    `;
    const { rootScope } = analyze(code);
    const unused = findVariable(rootScope, "unused")!;
    const used = findVariable(rootScope, "used")!;
    const a = findVariable(rootScope, "a")!;
    expect(unused.unsnarlIsUnused?.()).toBe(true);
    expect(used.unsnarlIsUnused?.()).toBe(false);
    expect(a.unsnarlIsUnused?.()).toBe(true);
  });

  test("destructuring assignment to existing names is recorded as write references", () => {
    const code = `
      let a = 1;
      let b = 2;
      [a, b] = [b, a];
    `;
    const { rootScope } = analyze(code);
    const a = findVariable(rootScope, "a")!;
    const b = findVariable(rootScope, "b")!;
    // a, b それぞれ: 右辺の読みと、左辺の書き込み
    const aWrites = a.references.filter((r) => r.isWrite()).length;
    const aReads = a.references.filter((r) => r.isRead()).length;
    expect(aWrites).toBeGreaterThan(0);
    expect(aReads).toBeGreaterThan(0);
    const bWrites = b.references.filter((r) => r.isWrite()).length;
    const bReads = b.references.filter((r) => r.isRead()).length;
    expect(bWrites).toBeGreaterThan(0);
    expect(bReads).toBeGreaterThan(0);
  });

  test("init flag is true for VariableDeclarator init expressions", () => {
    const code = `
      const seed = 1;
      const x = seed;
    `;
    const { rootScope } = analyze(code);
    const seed = findVariable(rootScope, "seed")!;
    expect(seed.references.length).toBe(1);
    expect(seed.references[0]?.init).toBe(true);
  });

  test("through propagates upward across nested scopes", () => {
    const code = `
      function a() {
        function b() {
          return globalThing;
        }
      }
    `;
    const { rootScope } = analyze(code);
    const all = collectScopes(rootScope);
    const innerB = all.find(
      (s) =>
        s.type === SCOPE_TYPE.Function &&
        s.upper?.type === SCOPE_TYPE.Function &&
        s.upper?.upper === rootScope,
    );
    expect(innerB).toBeDefined();
    expect(innerB?.through.length).toBe(1);
    expect(rootScope.through.length).toBe(1);
  });
});
