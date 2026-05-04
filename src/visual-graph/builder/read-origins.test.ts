import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { LANGUAGE } from "../../language.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import type { BuilderContext } from "./context.js";
import { readOrigins } from "./read-origins.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

function makeCtx(opts: {
  scopes: readonly SerializedScope[];
  writeOpsByVariable?: Map<string, WriteOp[]>;
  sortedCasesByContainer?: Map<string, readonly SerializedScope[]>;
}): BuilderContext {
  const ir = {
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    raw: "",
    scopes: opts.scopes,
    variables: [],
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  } as const satisfies SerializedIR;
  return {
    ir,
    variableMap: new Map<string, SerializedVariable>(),
    scopeMap: new Map(opts.scopes.map((s) => [s.id, s])),
    subgraphOwnerVar: new Map(),
    writeOpsByVariable: opts.writeOpsByVariable ?? new Map(),
    writeOpsByScope: new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: opts.sortedCasesByContainer ?? new Map(),
  };
}

describe("readOrigins", () => {
  test("no prior writes returns nodeId of the variable", () => {
    const ctx = makeCtx({ scopes: [{ ...baseScope(), id: "s" }] });
    expect(readOrigins("v", 100, "s", ctx)).toEqual(["n_v"]);
  });

  test("prior write in ancestor scope returns its writeOpNodeId", () => {
    const root = { ...baseScope(), id: "root" };
    const child = { ...baseScope(), id: "child", upper: "root" };
    const op = { ...baseWriteOp(), refId: "rRoot", offset: 5, scopeId: "root" };
    const ctx = makeCtx({
      scopes: [root, child],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    expect(readOrigins("v", 50, "child", ctx)).toEqual(["wr_rRoot"]);
  });

  test("prior write in non-ancestor non-branch scope returns its writeOpNodeId", () => {
    const root = { ...baseScope(), id: "root" };
    const sibA = { ...baseScope(), id: "a", upper: "root" };
    const sibB = { ...baseScope(), id: "b", upper: "root" };
    const op = { ...baseWriteOp(), refId: "rA", offset: 5, scopeId: "a" };
    const ctx = makeCtx({
      scopes: [root, sibA, sibB],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    expect(readOrigins("v", 50, "b", ctx)).toEqual(["wr_rA"]);
  });

  test("if without alternate adds the pre-if origin (variable id when no prior write)", () => {
    const root = { ...baseScope(), id: "root" };
    const cons = {
      ...baseScope(),
      id: "cons",
      upper: "root",
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const op = {
      ...baseWriteOp(),
      refId: "rCons",
      offset: 60,
      scopeId: "cons",
    };
    const ctx = makeCtx({
      scopes: [root, cons],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    const got = readOrigins("v", 100, "root", ctx);
    expect(new Set(got)).toEqual(new Set(["wr_rCons", "n_v"]));
  });

  test("if without alternate uses the last pre-if write as the second origin", () => {
    const root = { ...baseScope(), id: "root" };
    const cons = {
      ...baseScope(),
      id: "cons",
      upper: "root",
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const preIf = {
      ...baseWriteOp(),
      refId: "rPre",
      offset: 10,
      scopeId: "root",
    };
    const inIf = {
      ...baseWriteOp(),
      refId: "rCons",
      offset: 60,
      scopeId: "cons",
    };
    const ctx = makeCtx({
      scopes: [root, cons],
      writeOpsByVariable: new Map([["v", [preIf, inIf]]]),
    });
    const got = readOrigins("v", 100, "root", ctx);
    expect(new Set(got)).toEqual(new Set(["wr_rCons", "wr_rPre"]));
  });

  test("if-else with writes in both branches yields one origin per branch", () => {
    const root = { ...baseScope(), id: "root" };
    const cons = {
      ...baseScope(),
      id: "cons",
      upper: "root",
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const alt = {
      ...baseScope(),
      id: "alt",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        key: "alternate",
        parentSpanOffset: 50,
      },
    };
    const opCons = {
      ...baseWriteOp(),
      refId: "rCons",
      offset: 60,
      scopeId: "cons",
    };
    const opAlt = {
      ...baseWriteOp(),
      refId: "rAlt",
      offset: 70,
      scopeId: "alt",
    };
    const ctx = makeCtx({
      scopes: [root, cons, alt],
      writeOpsByVariable: new Map([["v", [opCons, opAlt]]]),
    });
    expect(new Set(readOrigins("v", 100, "root", ctx))).toEqual(
      new Set(["wr_rCons", "wr_rAlt"]),
    );
  });

  test("switch case with exitsFunction is excluded", () => {
    const root = { ...baseScope(), id: "root" };
    const switchScope = {
      ...baseScope(),
      id: "switch",
      type: SCOPE_TYPE.Switch,
      upper: "root",
    };
    const c1 = {
      ...baseScope(),
      id: "c1",
      upper: "switch",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 100,
      },
      exitsFunction: true,
    };
    const c2 = {
      ...baseScope(),
      id: "c2",
      upper: "switch",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 100,
      },
    };
    const opC1 = { ...baseWriteOp(), refId: "rC1", offset: 110, scopeId: "c1" };
    const opC2 = { ...baseWriteOp(), refId: "rC2", offset: 120, scopeId: "c2" };
    const containerKey = "switch:switch:100";
    const ctx = makeCtx({
      scopes: [root, switchScope, c1, c2],
      writeOpsByVariable: new Map([["v", [opC1, opC2]]]),
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(readOrigins("v", 200, "root", ctx)).toEqual(["wr_rC2"]);
  });

  test("switch case that falls through to a later case is excluded (only its non-fallthrough successor counts)", () => {
    const root = { ...baseScope(), id: "root" };
    const switchScope = {
      ...baseScope(),
      id: "switch",
      type: SCOPE_TYPE.Switch,
      upper: "root",
    };
    const c1 = {
      ...baseScope(),
      id: "c1",
      upper: "switch",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 100,
      },
      fallsThrough: true,
    };
    const c2 = {
      ...baseScope(),
      id: "c2",
      upper: "switch",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 100,
      },
    };
    const opC1 = { ...baseWriteOp(), refId: "rC1", offset: 110, scopeId: "c1" };
    const opC2 = { ...baseWriteOp(), refId: "rC2", offset: 120, scopeId: "c2" };
    const containerKey = "switch:switch:100";
    const ctx = makeCtx({
      scopes: [root, switchScope, c1, c2],
      writeOpsByVariable: new Map([["v", [opC1, opC2]]]),
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(readOrigins("v", 200, "root", ctx)).toEqual(["wr_rC2"]);
  });

  test("try/catch with writes in both branches yields one origin per branch", () => {
    const root = { ...baseScope(), id: "root" };
    const tryBlock = {
      ...baseScope(),
      id: "tryBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 50,
      },
    };
    const catchBlock = {
      ...baseScope(),
      id: "catchBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "handler",
        parentSpanOffset: 50,
      },
    };
    const opTry = {
      ...baseWriteOp(),
      refId: "rTry",
      offset: 60,
      scopeId: "tryBlock",
    };
    const opCatch = {
      ...baseWriteOp(),
      refId: "rCatch",
      offset: 70,
      scopeId: "catchBlock",
    };
    const ctx = makeCtx({
      scopes: [root, tryBlock, catchBlock],
      writeOpsByVariable: new Map([["v", [opTry, opCatch]]]),
    });
    expect(new Set(readOrigins("v", 100, "root", ctx))).toEqual(
      new Set(["wr_rTry", "wr_rCatch"]),
    );
  });

  test("try without catch handler adds the pre-try origin (variable id when no prior write)", () => {
    const root = { ...baseScope(), id: "root" };
    const tryBlock = {
      ...baseScope(),
      id: "tryBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 50,
      },
    };
    const opTry = {
      ...baseWriteOp(),
      refId: "rTry",
      offset: 60,
      scopeId: "tryBlock",
    };
    const ctx = makeCtx({
      scopes: [root, tryBlock],
      writeOpsByVariable: new Map([["v", [opTry]]]),
    });
    expect(new Set(readOrigins("v", 100, "root", ctx))).toEqual(
      new Set(["wr_rTry", "n_v"]),
    );
  });

  test("read inside finally sees writes from try and catch siblings", () => {
    const root = { ...baseScope(), id: "root" };
    const tryBlock = {
      ...baseScope(),
      id: "tryBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 50,
      },
    };
    const catchBlock = {
      ...baseScope(),
      id: "catchBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "handler",
        parentSpanOffset: 50,
      },
    };
    const finallyBlock = {
      ...baseScope(),
      id: "finallyBlock",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "finalizer",
        parentSpanOffset: 50,
      },
    };
    const opTry = {
      ...baseWriteOp(),
      refId: "rTry",
      offset: 60,
      scopeId: "tryBlock",
    };
    const opCatch = {
      ...baseWriteOp(),
      refId: "rCatch",
      offset: 70,
      scopeId: "catchBlock",
    };
    const ctx = makeCtx({
      scopes: [root, tryBlock, catchBlock, finallyBlock],
      writeOpsByVariable: new Map([["v", [opTry, opCatch]]]),
    });
    expect(new Set(readOrigins("v", 90, "finallyBlock", ctx))).toEqual(
      new Set(["wr_rTry", "wr_rCatch"]),
    );
  });

  test("duplicate origins are deduplicated", () => {
    const root = { ...baseScope(), id: "root" };
    const cons = {
      ...baseScope(),
      id: "cons",
      upper: "root",
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const alt = {
      ...baseScope(),
      id: "alt",
      upper: "root",
      blockContext: {
        ...baseBlockContext(),
        key: "alternate",
        parentSpanOffset: 50,
      },
    };
    const opCons = {
      ...baseWriteOp(),
      refId: "shared",
      offset: 60,
      scopeId: "cons",
    };
    const opAlt = {
      ...baseWriteOp(),
      refId: "shared",
      offset: 70,
      scopeId: "alt",
    };
    const ctx = makeCtx({
      scopes: [root, cons, alt],
      writeOpsByVariable: new Map([["v", [opCons, opAlt]]]),
    });
    expect(readOrigins("v", 100, "root", ctx)).toEqual(["wr_shared"]);
  });
});
