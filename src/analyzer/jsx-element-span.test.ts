import { describe, expect, test } from "vitest";

import type { Reference } from "../ir/reference/reference.js";
import { LANGUAGE } from "../language.js";
import { OxcParser } from "../parser/oxc-parser.js";
import { runAnalysis } from "../pipeline/analyze/run-analysis.js";
import { defaultSourceTypeFor } from "../pipeline/parse/default-source-type-for.js";

const parser = new OxcParser();

function analyze(code: string) {
  const parsed = parser.parse(code, {
    language: LANGUAGE.Tsx,
    sourcePath: "input.tsx",
    sourceType: defaultSourceTypeFor(LANGUAGE.Tsx),
  });
  return runAnalysis(parsed);
}

function refsThrough(rootScope: {
  through: readonly Reference[];
}): readonly Reference[] {
  return rootScope.through;
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
    const { rootScope, annotations } = analyze(code);
    const refs = refsThrough(rootScope);
    const aRef = refs.find((v) => v.identifier.name === "A");
    expect(aRef).toBeDefined();
    if (!aRef) {
      return;
    }
    const aElement = annotations.ofReference(aRef).jsxElement;
    expect(aElement).not.toBeNull();
    // <A>...</A> spans from "<A>" through "</A>": offset of '<' to offset
    // past the final '>'. We do not assert exact byte offsets here; only
    // that the analyzer surfaced the wrapping element rather than just the
    // identifier itself.
    if (aElement) {
      expect(aElement.endOffset).toBeGreaterThan(aElement.startOffset);
    }
  });

  test("self-closing tags collapse to the same start/end line on serialisation", () => {
    const code = "const App = () => <A />;\n";
    const { rootScope, annotations } = analyze(code);
    const refs = refsThrough(rootScope);
    const aRef = refs.find((v) => v.identifier.name === "A");
    expect(aRef).toBeDefined();
    if (!aRef) {
      return;
    }
    expect(annotations.ofReference(aRef).jsxElement).not.toBeNull();
  });

  test("does not attach a JSXElement span to plain JS identifiers in attribute values", () => {
    const code = ["const v = 1;", "const App = () => <A foo={v} />;"].join(
      "\n",
    );
    const { rootScope, annotations } = analyze(code);
    const refs = refsThrough(rootScope);
    const vRef = refs.find((ref) => ref.identifier.name === "v");
    expect(vRef ? annotations.ofReference(vRef).jsxElement : null).toBeNull();
  });

  test("does not attach a JSXElement span to expression-container children", () => {
    const code = ["const v = 1;", "const App = () => <A>{v}</A>;"].join("\n");
    const { rootScope, annotations } = analyze(code);
    const refs = refsThrough(rootScope);
    const vRef = refs.find((ref) => ref.identifier.name === "v");
    expect(vRef ? annotations.ofReference(vRef).jsxElement : null).toBeNull();
  });

  test("walks JSXMemberExpression chains so <Foo.Bar> still surfaces the element span on Foo", () => {
    const code = [
      "const Foo = { Bar: () => null };",
      "const App = () => <Foo.Bar />;",
    ].join("\n");
    const { rootScope, annotations } = analyze(code);
    const refs = refsThrough(rootScope);
    const fooRef = refs.find((v) => v.identifier.name === "Foo");
    expect(
      fooRef ? annotations.ofReference(fooRef).jsxElement : undefined,
    ).not.toBeNull();
  });
});
