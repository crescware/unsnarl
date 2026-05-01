import { describe, expect, test } from "vitest";

import { makeVariableNode } from "./make-variable-node.js";
import { makeDef } from "./testing/make-def.js";
import { makeVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";

describe("makeVariableNode", () => {
  test("plain Variable definition produces a Variable kind node", () => {
    const v = makeVariable({ id: "v1", name: "x", identifiers: [span(0, 2)] });
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      type: "node",
      id: "n_v1",
      kind: "Variable",
      name: "x",
      line: 2,
      isJsxElement: false,
    });
    expect(node.initIsFunction).toBeUndefined();
    expect(node.declarationKind).toBeUndefined();
  });

  test("falls back to def.name.span.line when identifiers is empty", () => {
    const v = makeVariable({
      id: "v",
      identifiers: [],
      defs: [makeDef({ name: { name: "x", span: span(0, 7) } })],
    });
    expect(makeVariableNode(v).line).toBe(7);
  });

  test("falls back to 0 when both identifiers and def are absent", () => {
    const v = makeVariable({ id: "v", identifiers: [], defs: [] });
    expect(makeVariableNode(v).line).toBe(0);
  });

  test.each<{ initType: string; expected: boolean }>([
    { initType: "ArrowFunctionExpression", expected: true },
    { initType: "FunctionExpression", expected: true },
    { initType: "Literal", expected: false },
  ])(
    "initType=$initType yields initIsFunction=$expected",
    ({ initType, expected }) => {
      const v = makeVariable({
        defs: [makeDef({ initType })],
      });
      const node = makeVariableNode(v);
      if (expected) {
        expect(node.initIsFunction).toBe(true);
      } else {
        expect(node.initIsFunction).toBeUndefined();
      }
    },
  );

  test.each<{ kind: "var" | "let" | "const" }>([
    { kind: "var" },
    { kind: "let" },
    { kind: "const" },
  ])("preserves declarationKind=$kind", ({ kind }) => {
    const v = makeVariable({
      defs: [makeDef({ declarationKind: kind })],
    });
    expect(makeVariableNode(v).declarationKind).toBe(kind);
  });

  test("ImportBinding propagates importKind, importedName, and importSource", () => {
    const v = makeVariable({
      name: "renamed",
      defs: [
        makeDef({
          type: "ImportBinding",
          importKind: "named",
          importedName: "original",
          importSource: "./mod.js",
        }),
      ],
    });
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      kind: "ImportBinding",
      importKind: "named",
      importedName: "original",
      importSource: "./mod.js",
    });
  });

  test("ImportBinding with null importedName still sets the field to null", () => {
    const v = makeVariable({
      defs: [
        makeDef({
          type: "ImportBinding",
          importKind: "default",
          importedName: null,
          importSource: "./mod.js",
        }),
      ],
    });
    const node = makeVariableNode(v);
    expect(node.importedName).toBeNull();
  });

  test("non-import definitions do not set importedName/importSource", () => {
    const v = makeVariable({
      defs: [makeDef({ type: "Variable", declarationKind: "let" })],
    });
    const node = makeVariableNode(v);
    expect(node.importedName).toBeUndefined();
    expect(node.importSource).toBeUndefined();
  });

  test("falls back to kind=Variable when defs is empty", () => {
    const v = makeVariable({ id: "v", defs: [] });
    expect(makeVariableNode(v).kind).toBe("Variable");
  });

  test.each<{
    defType: "FunctionName" | "ClassName" | "Parameter" | "CatchClause";
  }>([
    { defType: "FunctionName" },
    { defType: "ClassName" },
    { defType: "Parameter" },
    { defType: "CatchClause" },
  ])("kind reflects definition type $defType", ({ defType }) => {
    const v = makeVariable({ defs: [makeDef({ type: defType })] });
    expect(makeVariableNode(v).kind).toBe(defType);
  });
});
