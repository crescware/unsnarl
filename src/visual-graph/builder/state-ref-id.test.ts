import { describe, expect, test } from "vitest";

import { LANGUAGE } from "../../cli/language.js";
import type {
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import type { BuilderContext } from "./context.js";
import { stateRefId } from "./state-ref-id.js";
import { baseRef } from "./testing/make-ref.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import { span } from "./testing/span.js";
import type { WriteOp } from "./write-op.js";

function makeCtx(overrides: Partial<BuilderContext>): BuilderContext {
  const ir = {
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    raw: "",
    scopes: [],
    variables: [],
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  } as const satisfies SerializedIR;
  return {
    ir,
    variableMap: new Map<string, SerializedVariable>(),
    scopeMap: new Map<string, SerializedScope>(),
    subgraphOwnerVar: new Map<string, string>(),
    hiddenVariables: new Set<string>(),
    writeOpsByVariable: new Map<string, WriteOp[]>(),
    writeOpsByScope: new Map<string, WriteOp[]>(),
    writeOpByRef: new Map<string, WriteOp>(),
    sortedCasesByContainer: new Map<string, SerializedScope[]>(),
    ...overrides,
  };
}

const writeOp = { ...baseWriteOp(), refId: "wRef", varId: "v", offset: 10 };
const earlierOp = {
  ...baseWriteOp(),
  refId: "wEarlier",
  varId: "v",
  offset: 5,
};

describe("stateRefId", () => {
  test("refId that names a writeOp returns the writeOp's node id", () => {
    const ctx = makeCtx({
      writeOpByRef: new Map([["wRef", writeOp]]),
    });
    expect(stateRefId("wRef", "v", ctx)).toBe("wr_wRef");
  });

  test.each<{
    name: string;
    refs: readonly SerializedReference[];
    ops: WriteOp[];
    refId: string;
    varId: string;
    expected: string;
  }>([
    {
      name: "reference not found in ir.references -> nodeId(varId)",
      refs: [],
      ops: [],
      refId: "missing",
      varId: "v",
      expected: "n_v",
    },
    {
      name: "reference exists but no prior writes -> nodeId(varId)",
      refs: [
        {
          ...baseRef(),
          id: "readRef",
          identifier: { name: "x", span: span(20) },
        },
      ],
      ops: [],
      refId: "readRef",
      varId: "v",
      expected: "n_v",
    },
    {
      name: "reference exists with prior write -> writeOpNodeId of the prior write",
      refs: [
        {
          ...baseRef(),
          id: "readRef",
          identifier: { name: "x", span: span(20) },
        },
      ],
      ops: [earlierOp],
      refId: "readRef",
      varId: "v",
      expected: "wr_wEarlier",
    },
  ])("$name", ({ refs, ops, refId, varId, expected }) => {
    const ctx = makeCtx({
      ir: {
        version: SERIALIZED_IR_VERSION,
        source: { path: "x.ts", language: LANGUAGE.Ts },
        raw: "",
        scopes: [],
        variables: [],
        references: refs,
        unusedVariableIds: [],
        diagnostics: [],
      },
      writeOpsByVariable: new Map([["v", ops]]),
    });
    expect(stateRefId(refId, varId, ctx)).toBe(expected);
  });
});
