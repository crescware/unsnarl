import { describe, expect, test } from "vitest";

import { LANGUAGE, type Language } from "../constants.js";
import type { Scope, Variable } from "../ir/model.js";
import { OxcParser } from "../parser/oxc.js";
import { EslintCompatAnalyzer } from "./eslint-compat/eslint-compat.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();

function analyze(code: string, language: Language = LANGUAGE.Ts) {
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  return analyzer.analyze(parsed);
}

function findVariable(scope: Scope, name: string): Variable | undefined {
  return scope.variables.find((v) => v.name === name);
}

describe("EslintCompatAnalyzer / type annotation skipping", () => {
  test("does not generate references for identifiers used purely as TS types", () => {
    const code = `
      type SomeType = string;
      function take(x: SomeType): SomeType { return x; }
    `;
    const { rootScope } = analyze(code);
    // SomeType is a type, not a Variable
    expect(findVariable(rootScope, "SomeType")).toBeUndefined();
    // No ImplicitGlobalVariable for SomeType either
    expect(findVariable(rootScope, "SomeType")).toBeUndefined();
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
      rootScope.through.find((r) => r.identifier.name === "I"),
    ).toBeUndefined();
    expect(
      rootScope.through.find((r) => r.identifier.name === "foo"),
    ).toBeUndefined();
    expect(
      rootScope.through.find((r) => r.identifier.name === "bar"),
    ).toBeUndefined();
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
    expect(findVariable(rootScope, "Color")).toBeUndefined();
    expect(findVariable(rootScope, "Red")).toBeUndefined();
    expect(rootScope.variables.map((v) => v.name)).toEqual(["c"]);
  });

  test("preserves the value-side of `as` casts while skipping the type annotation", () => {
    const code = `
      const value = 1;
      const widened = value as number;
    `;
    const { rootScope } = analyze(code);
    const value = findVariable(rootScope, "value")!;
    expect(value.references.length).toBe(1);
    // `number` は型なので参照にもならない
    expect(findVariable(rootScope, "number")).toBeUndefined();
  });

  test("classifies JSX tag names as read references", () => {
    const code = `
      const Comp = () => null;
      const App = () => <Comp />;
    `;
    const { rootScope } = analyze(code, "tsx");
    const Comp = findVariable(rootScope, "Comp")!;
    expect(Comp.references.length).toBe(1);
    expect(Comp.references[0]?.isRead()).toBe(true);
  });

  test("does not register JSX attribute names as references", () => {
    const code = `
      const App = () => <div className="x" />;
    `;
    const { rootScope } = analyze(code, "tsx");
    expect(
      rootScope.through.find((r) => r.identifier.name === "className"),
    ).toBeUndefined();
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
      "ImplicitGlobalVariable",
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
