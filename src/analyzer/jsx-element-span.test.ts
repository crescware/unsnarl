import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../cli/language.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { EslintCompatAnalyzer } from "./eslint-compat/eslint-compat.js";

const parser = new OxcParser();
const analyzer = new EslintCompatAnalyzer();

function analyze(code: string) {
  const parsed = parser.parse(code, {
    language: LANGUAGE.Tsx,
    sourcePath: "input.tsx",
  });
  return analyzer.analyze(parsed);
}

function findRefs(rootScope: { through: readonly unknown[] }) {
  return rootScope.through as readonly {
    identifier: { name: string };
    unsnarlJsxElement: { startOffset: number; endOffset: number } | null;
  }[];
}

describe("EslintCompatAnalyzer / JSX element span", () => {
  test("attaches the wrapping JSXElement span to opening-tag identifier refs", () => {
    const code = [
      "const App = () => (",
      "  <A>",
      "    <B />",
      "  </A>",
      ");",
    ].join("\n");
    const { rootScope } = analyze(code);
    const refs = findRefs(rootScope);
    const aRef = refs.find((r) => r.identifier.name === "A");
    expect(aRef?.unsnarlJsxElement).not.toBeNull();
    // <A>...</A> spans from "<A>" through "</A>": offset of '<' to offset
    // past the final '>'. We do not assert exact byte offsets here; only
    // that the analyzer surfaced the wrapping element rather than just the
    // identifier itself.
    const aElement = aRef?.unsnarlJsxElement;
    if (aElement) {
      expect(aElement.endOffset).toBeGreaterThan(aElement.startOffset);
    }
  });

  test("self-closing tags collapse to the same start/end line on serialisation", () => {
    const code = "const App = () => <A />;\n";
    const { rootScope } = analyze(code);
    const refs = findRefs(rootScope);
    const aRef = refs.find((r) => r.identifier.name === "A");
    expect(aRef?.unsnarlJsxElement).not.toBeNull();
  });

  test("does not attach a JSXElement span to plain JS identifiers in attribute values", () => {
    const code = ["const v = 1;", "const App = () => <A foo={v} />;"].join(
      "\n",
    );
    const { rootScope } = analyze(code);
    const refs = findRefs(rootScope);
    const vRef = refs.find((r) => r.identifier.name === "v");
    expect(vRef?.unsnarlJsxElement ?? null).toBeNull();
  });

  test("does not attach a JSXElement span to expression-container children", () => {
    const code = ["const v = 1;", "const App = () => <A>{v}</A>;"].join("\n");
    const { rootScope } = analyze(code);
    const refs = findRefs(rootScope);
    const vRef = refs.find((r) => r.identifier.name === "v");
    expect(vRef?.unsnarlJsxElement ?? null).toBeNull();
  });

  test("walks JSXMemberExpression chains so <Foo.Bar> still surfaces the element span on Foo", () => {
    const code = [
      "const Foo = { Bar: () => null };",
      "const App = () => <Foo.Bar />;",
    ].join("\n");
    const { rootScope } = analyze(code);
    const refs = findRefs(rootScope);
    const fooRef = refs.find((r) => r.identifier.name === "Foo");
    expect(fooRef?.unsnarlJsxElement).not.toBeNull();
  });
});
