import { parse } from "valibot";
import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { serializedVariable$ } from "../../ir/serialized/serialized-variable.js";
import { asVariableId } from "../../ir/serialized/variable-id.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { asFilledString } from "../../util/filled-string.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { makeVariableNode } from "./make-variable-node.js";
import { baseDef, baseSimpleDef } from "./testing/make-def.js";
import { baseVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";

describe("makeVariableNode", () => {
  test("let-declared Variable produces a LetBinding kind node", () => {
    const v = {
      ...baseVariable(),
      id: asVariableId("v1"),
      name: asFilledString("x"),
      identifiers: [span(0, 2)],
      defs: [baseDef(VARIABLE_DECLARATION_KIND.Let)] as const,
    };
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "n_v1",
      kind: NODE_KIND.LetBinding,
      name: asFilledString("x"),
      line: 2,
      isJsxElement: false,
    });
    if (node.kind !== NODE_KIND.LetBinding) {
      throw new Error("expected LetBinding kind");
    }
    expect(node.initIsFunction).toEqual(false);
  });

  test("falls back to def.name.span.line when identifiers is empty", () => {
    const v = {
      ...baseVariable(),
      id: asVariableId("v"),
      identifiers: [],
      defs: [
        {
          ...baseDef(VARIABLE_DECLARATION_KIND.Let),
          name: { name: asFilledString("x"), span: span(0, 7) },
        },
      ] as const,
    };
    expect(makeVariableNode(v).line).toEqual(7);
  });

  test("ImplicitGlobalVariable forces line=0 because the def is synthetic, not a real source location", () => {
    const v = {
      ...baseVariable(),
      id: asVariableId("v"),
      name: asFilledString("Math"),
      identifiers: [span(0, 4)],
      defs: [baseSimpleDef(DEFINITION_TYPE.ImplicitGlobalVariable)] as const,
    };
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.SyntheticImplicitGlobal);
    expect(node.line).toEqual(0);
  });

  test.each<{ initType: string; expected: boolean }>([
    { initType: AST_TYPE.ArrowFunctionExpression, expected: true },
    { initType: AST_TYPE.FunctionExpression, expected: true },
    { initType: AST_TYPE.Literal, expected: false },
  ])(
    "init.type=$initType yields initIsFunction=$expected",
    ({ initType, expected }) => {
      const v = parse(serializedVariable$, {
        ...baseVariable(),
        defs: [
          {
            ...baseDef(VARIABLE_DECLARATION_KIND.Let),
            init: { type: initType, span: span() },
          },
        ],
      });
      const node = makeVariableNode(v);
      if (node.kind !== NODE_KIND.LetBinding) {
        throw new Error("expected LetBinding kind");
      }
      expect(node.initIsFunction).toEqual(expected);
    },
  );

  test("var is emitted as a VarBinding node (no declarationKind field)", () => {
    const v = parse(serializedVariable$, {
      ...baseVariable(),
      defs: [baseDef("var")],
    });
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.VarBinding);
  });

  test("const is emitted as a ConstBinding node (no declarationKind field)", () => {
    const v = parse(serializedVariable$, {
      ...baseVariable(),
      defs: [baseDef("const")],
    });
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.ConstBinding);
  });

  test("let is emitted as a LetBinding node (no declarationKind field)", () => {
    const v = parse(serializedVariable$, {
      ...baseVariable(),
      defs: [baseDef("let")],
    });
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.LetBinding);
  });

  test("Named ImportBinding propagates importKind and importedName", () => {
    const v = parse(serializedVariable$, {
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
    });
    const node = makeVariableNode(v);
    expect(node).toMatchObject({
      kind: NODE_KIND.NamedImportBinding,
      importedName: "original",
    });
  });

  test("Default ImportBinding has no importedName field", () => {
    const v = parse(serializedVariable$, {
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
    });
    const node = makeVariableNode(v);
    expect(node.kind).toEqual(NODE_KIND.DefaultImportBinding);
  });
});
