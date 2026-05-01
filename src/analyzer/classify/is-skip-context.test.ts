import { describe, expect, test } from "vitest";

import type { AstNode } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { isSkipContext } from "./is-skip-context.js";

const nodeOf = (overrides: Record<string, unknown>): AstNode =>
  ({ type: overrides["type"] as string, ...overrides }) as unknown as AstNode;

describe("isSkipContext", () => {
  test("ImportSpecifier#imported is skipped", () => {
    expect(
      isSkipContext(
        AST_TYPE.ImportSpecifier,
        "imported",
        nodeOf({ type: AST_TYPE.ImportSpecifier }),
      ),
    ).toBe(true);
    expect(
      isSkipContext(
        AST_TYPE.ImportSpecifier,
        "local",
        nodeOf({ type: AST_TYPE.ImportSpecifier }),
      ),
    ).toBe(false);
  });

  test("ExportSpecifier#exported is skipped", () => {
    expect(
      isSkipContext(
        AST_TYPE.ExportSpecifier,
        "exported",
        nodeOf({ type: AST_TYPE.ExportSpecifier }),
      ),
    ).toBe(true);
  });

  test("MemberExpression#property: skipped only when not computed", () => {
    expect(
      isSkipContext(
        AST_TYPE.MemberExpression,
        "property",
        nodeOf({ type: AST_TYPE.MemberExpression, computed: false }),
      ),
    ).toBe(true);
    expect(
      isSkipContext(
        AST_TYPE.MemberExpression,
        "property",
        nodeOf({ type: AST_TYPE.MemberExpression, computed: true }),
      ),
    ).toBe(false);
  });

  test("Property/MethodDefinition/PropertyDefinition/AccessorProperty key is skipped only when not computed", () => {
    for (const t of [
      AST_TYPE.Property,
      AST_TYPE.MethodDefinition,
      AST_TYPE.PropertyDefinition,
      AST_TYPE.AccessorProperty,
    ]) {
      expect(
        isSkipContext(t, "key", nodeOf({ type: t, computed: false })),
      ).toBe(true);
      expect(isSkipContext(t, "key", nodeOf({ type: t, computed: true }))).toBe(
        false,
      );
    }
  });

  test("JSXAttribute#name is skipped", () => {
    expect(
      isSkipContext(
        AST_TYPE.JSXAttribute,
        "name",
        nodeOf({ type: AST_TYPE.JSXAttribute }),
      ),
    ).toBe(true);
  });

  test("JSXMemberExpression#property is skipped", () => {
    expect(
      isSkipContext(
        AST_TYPE.JSXMemberExpression,
        "property",
        nodeOf({ type: AST_TYPE.JSXMemberExpression }),
      ),
    ).toBe(true);
  });

  test("JSXClosingElement is skipped regardless of key", () => {
    expect(
      isSkipContext(
        AST_TYPE.JSXClosingElement,
        null,
        nodeOf({ type: AST_TYPE.JSXClosingElement }),
      ),
    ).toBe(true);
    expect(
      isSkipContext(
        AST_TYPE.JSXClosingElement,
        "name",
        nodeOf({ type: AST_TYPE.JSXClosingElement }),
      ),
    ).toBe(true);
  });

  test("LabeledStatement/Continue/Break label is skipped", () => {
    for (const t of [
      AST_TYPE.LabeledStatement,
      AST_TYPE.ContinueStatement,
      AST_TYPE.BreakStatement,
    ]) {
      expect(isSkipContext(t, "label", nodeOf({ type: t }))).toBe(true);
      expect(isSkipContext(t, "body", nodeOf({ type: t }))).toBe(false);
    }
  });

  test("unrelated context → false", () => {
    expect(
      isSkipContext(
        AST_TYPE.CallExpression,
        "callee",
        nodeOf({ type: AST_TYPE.CallExpression }),
      ),
    ).toBe(false);
  });
});
