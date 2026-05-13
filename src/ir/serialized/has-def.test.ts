import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { assertHasDef, hasDef } from "./has-def.js";
import type { SerializedDefinition } from "./serialized-definition.js";
import type { SerializedVariable } from "./serialized-variable.js";

function variable(defs: readonly SerializedDefinition[]): SerializedVariable {
  return {
    id: "v",
    name: "x",
    scope: "s",
    identifiers: [],
    references: [],
    defs,
  };
}

const sampleDef = {
  type: DEFINITION_TYPE.Variable,
  name: { name: "x", span: { offset: 0, line: 1, column: 0 } },
  node: {
    type: AST_TYPE.Identifier,
    span: { offset: 0, line: 1, column: 0 },
  },
  parent: null,
  init: null,
  declarationKind: VARIABLE_DECLARATION_KIND.Let,
} as const satisfies SerializedDefinition;

describe("hasDef", () => {
  test("returns true when defs has at least one entry", () => {
    expect(hasDef(variable([sampleDef]))).toEqual(true);
  });

  test("returns false when defs is empty", () => {
    expect(hasDef(variable([]))).toEqual(false);
  });
});

describe("assertHasDef", () => {
  test("does not throw when defs has at least one entry", () => {
    expect(() => assertHasDef(variable([sampleDef]))).not.toThrow();
  });

  test("throws ValiError when defs is empty", () => {
    expect(() => assertHasDef(variable([]))).toThrow(
      /Variable\.defs must be non-empty/,
    );
  });
});
