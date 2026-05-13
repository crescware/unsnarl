import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { makeVariableNode } from "./make-variable-node.js";
import { baseDef, baseSimpleDef } from "./testing/make-def.js";
import { baseVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";

describe("makeVariableNode", () => {
  test("plain Variable definition produces a Variable kind node", () => {
    const v = {
      ...baseVariable(),
      id: "v1",
      name: "x",
      identifiers: [span(0, 2)],
      defs: [baseDef(VARIABLE_DECLARATION_KIND.Let)],
    };
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "n_v1",
      kind: NODE_KIND.LegacyVariable,
      name: "x",
      line: 2,
      isJsxElement: false,
    });
    if (node.kind !== NODE_KIND.LegacyVariable) {
      throw new Error("expected Variable kind");
    }
    expect(node.initIsFunction).toEqual(false);
    expect(node.declarationKind).toEqual("let");
  });

  test("falls back to def.name.span.line when identifiers is empty", () => {
    const v = {
      ...baseVariable(),
      id: "v",
      identifiers: [],
      defs: [
        {
          ...baseDef(VARIABLE_DECLARATION_KIND.Let),
          name: { name: "x", span: span(0, 7) },
        },
      ],
    };
    expect(makeVariableNode(v).line).toEqual(7);
  });

  test("ImplicitGlobalVariable forces line=0 because the def is synthetic, not a real source location", () => {
    const v = {
      ...baseVariable(),
      id: "v",
      name: "Math",
      identifiers: [span(0, 4)],
      defs: [baseSimpleDef(DEFINITION_TYPE.ImplicitGlobalVariable)],
    };
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.LegacyImplicitGlobalVariable);
    expect(node.line).toEqual(0);
  });

  test.each<{ initType: string; expected: boolean }>([
    { initType: AST_TYPE.ArrowFunctionExpression, expected: true },
    { initType: AST_TYPE.FunctionExpression, expected: true },
    { initType: AST_TYPE.Literal, expected: false },
  ])(
    "init.type=$initType yields initIsFunction=$expected",
    ({ initType, expected }) => {
      const v = {
        ...baseVariable(),
        defs: [
          {
            ...baseDef(VARIABLE_DECLARATION_KIND.Let),
            init: { type: initType, span: span() },
          },
        ],
      };
      const node = makeVariableNode(v);
      if (node.kind !== NODE_KIND.LegacyVariable) {
        throw new Error("expected Variable kind");
      }
      expect(node.initIsFunction).toEqual(expected);
    },
  );

  test.each<{ kind: "var" | "let" | "const" }>([
    { kind: "var" },
    { kind: "let" },
    { kind: "const" },
  ])("preserves declarationKind=$kind", ({ kind }) => {
    const v = {
      ...baseVariable(),
      defs: [baseDef(kind)],
    };
    const node = makeVariableNode(v);
    if (node.kind !== NODE_KIND.LegacyVariable) {
      throw new Error("expected Variable kind");
    }
    expect(node.declarationKind).toEqual(kind);
  });

  test("Named ImportBinding propagates importKind and importedName", () => {
    const v = {
      ...baseVariable(),
      name: "renamed",
      defs: [
        {
          ...baseDef(VARIABLE_DECLARATION_KIND.Let),
          type: DEFINITION_TYPE.ImportBinding,
          importKind: IMPORT_KIND.Named,
          importedName: "original",
          importSource: "./mod.js",
        },
      ],
    };
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      kind: NODE_KIND.LegacyImportBinding,
      importKind: IMPORT_KIND.Named,
      importedName: "original",
    });
  });

  test("Default ImportBinding has no importedName field", () => {
    const v = {
      ...baseVariable(),
      defs: [
        {
          ...baseDef(VARIABLE_DECLARATION_KIND.Let),
          type: DEFINITION_TYPE.ImportBinding,
          importKind: IMPORT_KIND.Default,
          importedName: null,
          importSource: "./mod.js",
        },
      ],
    };
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.LegacyImportBinding);
    if (node.kind === NODE_KIND.LegacyImportBinding) {
      expect(node.importKind).toEqual(IMPORT_KIND.Default);
    }
  });
});
