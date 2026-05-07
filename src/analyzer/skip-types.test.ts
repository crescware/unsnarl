import { describe, expect, test } from "vitest";

import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";
import { LANGUAGE, type Language } from "../language.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { defaultSourceTypeFor } from "../pipeline/parse/default-source-type-for.js";
import { createEslintCompatAnalyzer } from "./create-eslint-compat-analyzer.js";
import { DEFINITION_TYPE } from "./definition-type.js";

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

function findVariable(scope: Scope, name: string): Variable | null {
  return scope.variables.find((v) => v.name === name) ?? null;
}

describe("EslintCompatAnalyzer / type annotation skipping", () => {
  test("does not generate references for identifiers used purely as TS types", () => {
    const code = `
      type SomeType = string;
      function take(x: SomeType): SomeType { return x; }
    `;
    const { rootScope } = analyze(code);
    // SomeType is a type, not a Variable
    expect(findVariable(rootScope, "SomeType")).toBeNull();
    // No ImplicitGlobalVariable for SomeType either
    expect(findVariable(rootScope, "SomeType")).toBeNull();
    // take は Variable として登録される
    const take = findVariable(rootScope, "take");
    expect(take).toBeDefined();
  });

  test("does not enter interface bodies (no spurious references)", () => {
    const code = `
      interface I { foo: number; bar: string; }
      const x = 1;
    `;
    const { rootScope } = analyze(code);
    expect(
      rootScope.through.find((r) => r.identifier.name === "I") ?? null,
    ).toBeNull();
    expect(
      rootScope.through.find((r) => r.identifier.name === "foo") ?? null,
    ).toBeNull();
    expect(
      rootScope.through.find((r) => r.identifier.name === "bar") ?? null,
    ).toBeNull();
    // 唯一の Variable は x
    expect(rootScope.variables.map((v) => v.name)).toEqual(["x"]);
  });

  test("does not register enum members as Variables", () => {
    const code = `
      enum Color { Red, Green, Blue }
      const c = 1;
    `;
    const { rootScope } = analyze(code);
    // enum 自体も無視
    expect(findVariable(rootScope, "Color")).toBeNull();
    expect(findVariable(rootScope, "Red")).toBeNull();
    expect(rootScope.variables.map((v) => v.name)).toEqual(["c"]);
  });

  test("preserves the value-side of `as` casts while skipping the type annotation", () => {
    const code = `
      const value = 1;
      const widened = value as number;
    `;
    const { rootScope } = analyze(code);
    const value = findVariable(rootScope, "value")!;
    // [0] is the init Write at value's declarator id; [1] is the read in
    // `value as number` whose type annotation is skipped.
    expect(value.references.length).toBe(2);
    expect(findVariable(rootScope, "number")).toBeNull();
  });

  test("classifies JSX tag names as read references", () => {
    const code = `
      const Comp = () => null;
      const App = () => <Comp />;
    `;
    const { rootScope } = analyze(code, "tsx");
    const Comp = findVariable(rootScope, "Comp")!;
    // [0] is the init Write at Comp's declarator id; [1] is the JSX tag read.
    expect(Comp.references.length).toBe(2);
    expect(Comp.references[1]?.isRead()).toBe(true);
  });

  test("does not register JSX attribute names as references", () => {
    const code = `
      const App = () => <div className="x" />;
    `;
    const { rootScope } = analyze(code, "tsx");
    expect(
      rootScope.through.find((r) => r.identifier.name === "className") ?? null,
    ).toBeNull();
  });
});

describe("EslintCompatAnalyzer / ImplicitGlobalVariable", () => {
  test("creates ImplicitGlobalVariable on the global scope for free identifiers", () => {
    const code = `
      function f() {
        return console.log("hi");
      }
    `;
    const { rootScope } = analyze(code);
    const console_ = findVariable(rootScope, "console");
    expect(console_).toBeDefined();
    expect(console_?.defs.map((d) => d.type)).toEqual([
      DEFINITION_TYPE.ImplicitGlobalVariable,
    ]);
    expect(console_?.references.length).toBe(1);
  });

  test("merges multiple uses of the same global into one Variable", () => {
    const code = `
      console.log("a");
      console.log("b");
    `;
    const { rootScope } = analyze(code);
    const console_ = findVariable(rootScope, "console")!;
    expect(console_.references.length).toBe(2);
  });
});
