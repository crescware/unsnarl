import { describe, expect, test } from "vitest";

import type { Reference } from "../ir/reference/reference.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";
import { LANGUAGE, type Language } from "../language.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { runAnalysis } from "../pipeline/analyze/run-analysis.js";
import { defaultSourceTypeFor } from "../pipeline/parse/default-source-type-for.js";
import { DEFINITION_TYPE } from "./definition-type.js";
import { isUnused } from "./is-unused.js";
import { SCOPE_TYPE } from "./scope-type.js";

const parser = new OxcParser();

function analyze(code: string, language: Language = LANGUAGE.Ts) {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
    sourceType: defaultSourceTypeFor(language),
  });
  return runAnalysis(parsed);
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
    const { rootScope, annotations } = analyze(
      "const a = 1;\nconsole.log(a);\n",
    );
    const a = findVariable(rootScope, "a")!;
    expect(a.references.length).toEqual(2);
    // [0] is the init Write recorded at the declarator id; [1] is the read.
    const read = a.references[1]!;
    expect(read.isRead()).toEqual(true);
    expect(read.isWrite()).toEqual(false);
    expect(annotations.ofReference(read).flags.call).toEqual(false);
  });

  test("classifies init write on declarator, write on `=`, read+write on compound assignment, read+write on update", () => {
    const code = `
      let x = 0;
      x = 1;
      x += 1;
      x++;
    `;
    const { rootScope } = analyze(code);
    const refs = refsOf(rootScope, "x");
    expect(refs.length).toEqual(4);
    const [r0, r1, r2, r3] = refs;
    expect([r0?.isWrite(), r0?.isRead(), r0?.init]).toEqual([
      true,
      false,
      true,
    ]);
    expect([r1?.isWrite(), r1?.isRead()]).toEqual([true, false]);
    expect([r2?.isWrite(), r2?.isRead()]).toEqual([true, true]);
    expect([r3?.isWrite(), r3?.isRead()]).toEqual([true, true]);
    expect(r0?.isWriteOnly()).toEqual(true);
    expect(r1?.isWriteOnly()).toEqual(true);
    expect(r2?.isReadWrite()).toEqual(true);
    expect(r3?.isReadWrite()).toEqual(true);
  });

  test("classifies read+call on CallExpression callee and NewExpression callee", () => {
    const code = `
      function foo() { return 1; }
      class Bar {}
      foo();
      new Bar();
    `;
    const { rootScope, annotations } = analyze(code);
    const fooRefs = refsOf(rootScope, "foo");
    expect(fooRefs.length).toEqual(1);
    expect(annotations.ofReference(fooRefs[0]!).flags.call).toEqual(true);
    expect(fooRefs[0]?.isRead()).toEqual(true);

    const barRefs = refsOf(rootScope, "Bar");
    expect(barRefs.length).toEqual(1);
    expect(annotations.ofReference(barRefs[0]!).flags.call).toEqual(true);
    expect(barRefs[0]?.isRead()).toEqual(true);
  });

  test("captures member object as Read but skips member property name", () => {
    const code = `
      const obj = { a: 1 };
      const value = obj.a;
    `;
    const { rootScope } = analyze(code);
    const obj = findVariable(rootScope, "obj")!;
    expect(obj.references.length).toEqual(2);
    // [0] is the init Write at the declarator id; [1] is the member read.
    expect(obj.references[1]?.isRead()).toEqual(true);
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
        expect(r.identifier.name).not.toEqual("foo");
      }
    }
  });

  test("hoisted function reference is resolved before its declaration site", () => {
    const code = `
      const result = foo();
      function foo() { return 1; }
    `;
    const { rootScope, annotations } = analyze(code);
    const foo = findVariable(rootScope, "foo")!;
    expect(foo.references.length).toEqual(1);
    expect(annotations.ofReference(foo.references[0]!).flags.call).toEqual(
      true,
    );
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
    // outer x has only the init Write at its declarator; no other refs reach
    // it (the inner `return x;` resolves to the inner shadowing variable).
    expect(outerX.references.length).toEqual(1);
    expect(outerX.references[0]?.init).toEqual(true);
    expect(isUnused(outerX)).toEqual(true);
    expect(innerX.references.length).toEqual(2);
    // [0] is inner x's init Write; [1] is the `return x;` read.
    expect(innerX.references[1]?.isRead()).toEqual(true);
  });

  test("unresolved identifier resolves to an ImplicitGlobalVariable and is propagated through", () => {
    const code = `
      function foo() {
        return globalThing;
      }
    `;
    const { rootScope } = analyze(code);
    expect(rootScope.through.length).toEqual(1);
    expect(rootScope.through[0]?.identifier.name).toEqual("globalThing");
    const implicit = findVariable(rootScope, "globalThing");
    expect(implicit !== null && implicit !== undefined).toEqual(true);
    expect(implicit?.defs.map((v) => v.type)).toEqual([
      DEFINITION_TYPE.ImplicitGlobalVariable,
    ]);
    expect(rootScope.through[0]?.resolved).toEqual(implicit);
  });

  test("isUnused returns true for unused imports and consts", () => {
    const code = `
      import unused from "x";
      import { used } from "y";
      const a = used;
    `;
    const { rootScope } = analyze(code);
    const unused = findVariable(rootScope, "unused")!;
    const used = findVariable(rootScope, "used")!;
    const a = findVariable(rootScope, "a")!;
    expect(isUnused(unused)).toEqual(true);
    expect(isUnused(used)).toEqual(false);
    expect(isUnused(a)).toEqual(true);
  });

  test("isUnused returns true for a let that is re-assigned but never read (#45)", () => {
    const code = "let x = 1;\nx = 2;\n";
    const { rootScope } = analyze(code);
    const x = findVariable(rootScope, "x")!;
    expect(x.references.length).toEqual(2);
    // [0] init Write at the declarator id; [1] non-init Write at `x = 2`.
    expect(x.references[0]?.init).toEqual(true);
    expect(x.references[0]?.isWriteOnly()).toEqual(true);
    expect(x.references[1]?.init).toEqual(false);
    expect(x.references[1]?.isWriteOnly()).toEqual(true);
    expect(isUnused(x)).toEqual(true);
  });

  test("isUnused returns true for a recursive function with no external caller (#68)", () => {
    const code = "function foo() { foo(); }\n";
    const { rootScope } = analyze(code);
    const foo = findVariable(rootScope, "foo")!;
    expect(foo.references.length).toEqual(1);
    expect(foo.references[0]?.isRead()).toEqual(true);
    expect(foo.references[0]?.resolved).toEqual(foo);
    expect(isUnused(foo)).toEqual(true);
  });

  test("isUnused returns true for an arrow function whose only reference is the self-recursive body (#68)", () => {
    const code = "const a = () => a;\n";
    const { rootScope } = analyze(code);
    const a = findVariable(rootScope, "a")!;
    // [0] init Write at the declarator id; [1] body Read of `a`, resolving
    // back to the same variable.
    expect(a.references.length).toEqual(2);
    expect(a.references[0]?.init).toEqual(true);
    expect(a.references[1]?.isRead()).toEqual(true);
    expect(a.references[1]?.resolved).toEqual(a);
    expect(isUnused(a)).toEqual(true);
  });

  test("isUnused returns false for mutual recursion (each reference resolves to the other variable; #68)", () => {
    // Pinned to current behavior. `f`'s body reads `g` (not `f`), so the
    // Read does not self-resolve and `f` stays not-unused. eslint's
    // `no-unused-vars` defaults match this. Lifting this is out of scope.
    const code = "function f() { g(); }\nfunction g() { f(); }\n";
    const { rootScope } = analyze(code);
    const f = findVariable(rootScope, "f")!;
    const g = findVariable(rootScope, "g")!;
    expect(isUnused(f)).toEqual(false);
    expect(isUnused(g)).toEqual(false);
  });

  test("isUnused returns false for a recursive function that is also called externally", () => {
    // The outer `foo();` originates from the module scope, which is outside
    // `foo`'s body, so it counts as external use even though both refs
    // self-resolve to `foo`.
    const code = "function foo() { foo(); }\nfoo();\n";
    const { rootScope } = analyze(code);
    const foo = findVariable(rootScope, "foo")!;
    expect(foo.references.length).toEqual(2);
    expect(foo.references.every((v) => v.resolved === foo)).toEqual(true);
    expect(foo.references.every((v) => v.isRead())).toEqual(true);
    expect(isUnused(foo)).toEqual(false);
  });

  test("isUnused returns true for a class whose only reference is the recursive body (#71)", () => {
    // The inner `new C()` resolves to the inner ClassName declared in the
    // class scope itself; with #71 the body lookup includes
    // `variable.scope` for that case, so the self-reference is treated as
    // self-internal and the inner C is unused. The outer ClassName has
    // no references at all.
    const code = "class C { m() { new C(); } }\n";
    const { rootScope } = analyze(code);
    const outerC = findVariable(rootScope, "C")!;
    const classScope = rootScope.childScopes.find(
      (v) => v.type === SCOPE_TYPE.Class,
    )!;
    const innerC = findVariable(classScope, "C")!;
    expect(outerC.references.length).toEqual(0);
    expect(innerC.references.length).toEqual(1);
    expect(innerC.references[0]?.resolved).toEqual(innerC);
    expect(isUnused(outerC)).toEqual(true);
    expect(isUnused(innerC)).toEqual(true);
  });

  test("isUnused returns false for a class that is also instantiated externally (#71)", () => {
    // The trailing `new C()` originates from the module scope, which is
    // outside the class body, so the outer ClassName stays not-unused.
    // The inner ClassName still receives only the recursive reference and
    // is unused.
    const code = "class C { m() { new C(); } }\nnew C();\n";
    const { rootScope } = analyze(code);
    const outerC = findVariable(rootScope, "C")!;
    const classScope = rootScope.childScopes.find(
      (v) => v.type === SCOPE_TYPE.Class,
    )!;
    const innerC = findVariable(classScope, "C")!;
    expect(outerC.references.length).toEqual(1);
    expect(outerC.references[0]?.resolved).toEqual(outerC);
    expect(isUnused(outerC)).toEqual(false);
    expect(isUnused(innerC)).toEqual(true);
  });

  test("isUnused returns true for a self-extending class (#71)", () => {
    // `class C extends C {}` evaluates the extends expression inside the
    // class scope itself, which the inner ClassName owns, so the
    // self-reference counts as self-internal. Pinned to current behavior;
    // matches the issue's intent that self-extends stays unused.
    const code = "class C extends C {}\n";
    const { rootScope } = analyze(code);
    const outerC = findVariable(rootScope, "C")!;
    const classScope = rootScope.childScopes.find(
      (v) => v.type === SCOPE_TYPE.Class,
    )!;
    const innerC = findVariable(classScope, "C")!;
    expect(isUnused(outerC)).toEqual(true);
    expect(isUnused(innerC)).toEqual(true);
  });

  test("annotations.ofVariable mirrors isUnused for analyzed variables", () => {
    const code = `
      import unused from "x";
      import { used } from "y";
      const a = used;
    `;
    const { rootScope, annotations } = analyze(code);
    const unused = findVariable(rootScope, "unused")!;
    const used = findVariable(rootScope, "used")!;
    const a = findVariable(rootScope, "a")!;
    expect(annotations.ofVariable(unused).isUnused).toEqual(true);
    expect(annotations.ofVariable(used).isUnused).toEqual(false);
    expect(annotations.ofVariable(a).isUnused).toEqual(true);
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
    const aWrites = a.references.filter((v) => v.isWrite()).length;
    const aReads = a.references.filter((v) => v.isRead()).length;
    expect(aWrites > 0).toEqual(true);
    expect(aReads > 0).toEqual(true);
    const bWrites = b.references.filter((v) => v.isWrite()).length;
    const bReads = b.references.filter((v) => v.isRead()).length;
    expect(bWrites > 0).toEqual(true);
    expect(bReads > 0).toEqual(true);
  });

  test("init flag is true for VariableDeclarator init expressions", () => {
    const code = `
      const seed = 1;
      const x = seed;
    `;
    const { rootScope } = analyze(code);
    const seed = findVariable(rootScope, "seed")!;
    // [0] is the init Write at seed's declarator id; [1] is the read on the
    // RHS of `const x = seed;` which is also flagged init=true since it sits
    // in another VariableDeclarator's initializer.
    expect(seed.references.length).toEqual(2);
    expect(seed.references[0]?.init).toEqual(true);
    expect(seed.references[1]?.init).toEqual(true);
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
      (v) =>
        v.type === SCOPE_TYPE.Function &&
        v.upper?.type === SCOPE_TYPE.Function &&
        v.upper?.upper === rootScope,
    );
    expect(innerB !== null && innerB !== undefined).toEqual(true);
    expect(innerB?.through.length).toEqual(1);
    expect(rootScope.through.length).toEqual(1);
  });
});
