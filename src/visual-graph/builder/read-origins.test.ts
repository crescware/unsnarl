import { describe, expect, test } from "vitest";

import type {
  SerializedIR,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import type { BuilderContext } from "./context.js";
import { readOrigins } from "./read-origins.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";
import { makeWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

function makeCtx(opts: {
  scopes: SerializedScope[];
  writeOpsByVariable?: Map<string, WriteOp[]>;
  sortedCasesByContainer?: Map<string, SerializedScope[]>;
}): BuilderContext {
  const ir: SerializedIR = {
    version: 1,
    source: { path: "x.ts", language: "ts" },
    raw: "",
    scopes: opts.scopes,
    variables: [],
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  };
  return {
    ir,
    variableMap: new Map<string, SerializedVariable>(),
    scopeMap: new Map(opts.scopes.map((s) => [s.id, s])),
    subgraphOwnerVar: new Map(),
    hiddenVariables: new Set(),
    writeOpsByVariable: opts.writeOpsByVariable ?? new Map(),
    writeOpsByScope: new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: opts.sortedCasesByContainer ?? new Map(),
  };
}

describe("readOrigins", () => {
  test("no prior writes returns nodeId of the variable", () => {
    const ctx = makeCtx({ scopes: [makeScope({ id: "s" })] });
    expect(readOrigins("v", 100, "s", ctx)).toEqual(["n_v"]);
  });

  test("prior write in ancestor scope returns its writeOpNodeId", () => {
    const root = makeScope({ id: "root" });
    const child = makeScope({ id: "child", upper: "root" });
    const op = makeWriteOp({ refId: "rRoot", offset: 5, scopeId: "root" });
    const ctx = makeCtx({
      scopes: [root, child],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    expect(readOrigins("v", 50, "child", ctx)).toEqual(["wr_rRoot"]);
  });

  test("prior write in non-ancestor non-branch scope returns its writeOpNodeId", () => {
    const root = makeScope({ id: "root" });
    const sibA = makeScope({ id: "a", upper: "root" });
    const sibB = makeScope({ id: "b", upper: "root" });
    const op = makeWriteOp({ refId: "rA", offset: 5, scopeId: "a" });
    const ctx = makeCtx({
      scopes: [root, sibA, sibB],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    expect(readOrigins("v", 50, "b", ctx)).toEqual(["wr_rA"]);
  });

  test("if without alternate adds the pre-if origin (variable id when no prior write)", () => {
    const root = makeScope({ id: "root" });
    const cons = makeScope({
      id: "cons",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const op = makeWriteOp({ refId: "rCons", offset: 60, scopeId: "cons" });
    const ctx = makeCtx({
      scopes: [root, cons],
      writeOpsByVariable: new Map([["v", [op]]]),
    });
    const got = readOrigins("v", 100, "root", ctx);
    expect(new Set(got)).toEqual(new Set(["wr_rCons", "n_v"]));
  });

  test("if without alternate uses the last pre-if write as the second origin", () => {
    const root = makeScope({ id: "root" });
    const cons = makeScope({
      id: "cons",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const preIf = makeWriteOp({ refId: "rPre", offset: 10, scopeId: "root" });
    const inIf = makeWriteOp({ refId: "rCons", offset: 60, scopeId: "cons" });
    const ctx = makeCtx({
      scopes: [root, cons],
      writeOpsByVariable: new Map([["v", [preIf, inIf]]]),
    });
    const got = readOrigins("v", 100, "root", ctx);
    expect(new Set(got)).toEqual(new Set(["wr_rCons", "wr_rPre"]));
  });

  test("if-else with writes in both branches yields one origin per branch", () => {
    const root = makeScope({ id: "root" });
    const cons = makeScope({
      id: "cons",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const alt = makeScope({
      id: "alt",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "alternate", 50),
    });
    const opCons = makeWriteOp({ refId: "rCons", offset: 60, scopeId: "cons" });
    const opAlt = makeWriteOp({ refId: "rAlt", offset: 70, scopeId: "alt" });
    const ctx = makeCtx({
      scopes: [root, cons, alt],
      writeOpsByVariable: new Map([["v", [opCons, opAlt]]]),
    });
    expect(new Set(readOrigins("v", 100, "root", ctx))).toEqual(
      new Set(["wr_rCons", "wr_rAlt"]),
    );
  });

  test("switch case with exitsFunction is excluded", () => {
    const root = makeScope({ id: "root" });
    const switchScope = makeScope({
      id: "switch",
      type: "switch",
      upper: "root",
    });
    const c1 = makeScope({
      id: "c1",
      upper: "switch",
      blockContext: makeBlockContext("SwitchStatement", "cases", 100),
      exitsFunction: true,
    });
    const c2 = makeScope({
      id: "c2",
      upper: "switch",
      blockContext: makeBlockContext("SwitchStatement", "cases", 100),
    });
    const opC1 = makeWriteOp({ refId: "rC1", offset: 110, scopeId: "c1" });
    const opC2 = makeWriteOp({ refId: "rC2", offset: 120, scopeId: "c2" });
    const containerKey = "switch:switch:100";
    const ctx = makeCtx({
      scopes: [root, switchScope, c1, c2],
      writeOpsByVariable: new Map([["v", [opC1, opC2]]]),
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(readOrigins("v", 200, "root", ctx)).toEqual(["wr_rC2"]);
  });

  test("switch case that falls through to a later case is excluded (only its non-fallthrough successor counts)", () => {
    const root = makeScope({ id: "root" });
    const switchScope = makeScope({
      id: "switch",
      type: "switch",
      upper: "root",
    });
    const c1 = makeScope({
      id: "c1",
      upper: "switch",
      blockContext: makeBlockContext("SwitchStatement", "cases", 100),
      fallsThrough: true,
    });
    const c2 = makeScope({
      id: "c2",
      upper: "switch",
      blockContext: makeBlockContext("SwitchStatement", "cases", 100),
    });
    const opC1 = makeWriteOp({ refId: "rC1", offset: 110, scopeId: "c1" });
    const opC2 = makeWriteOp({ refId: "rC2", offset: 120, scopeId: "c2" });
    const containerKey = "switch:switch:100";
    const ctx = makeCtx({
      scopes: [root, switchScope, c1, c2],
      writeOpsByVariable: new Map([["v", [opC1, opC2]]]),
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(readOrigins("v", 200, "root", ctx)).toEqual(["wr_rC2"]);
  });

  test("duplicate origins are deduplicated", () => {
    const root = makeScope({ id: "root" });
    const cons = makeScope({
      id: "cons",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const alt = makeScope({
      id: "alt",
      upper: "root",
      blockContext: makeBlockContext("IfStatement", "alternate", 50),
    });
    const opCons = makeWriteOp({
      refId: "shared",
      offset: 60,
      scopeId: "cons",
    });
    const opAlt = makeWriteOp({ refId: "shared", offset: 70, scopeId: "alt" });
    const ctx = makeCtx({
      scopes: [root, cons, alt],
      writeOpsByVariable: new Map([["v", [opCons, opAlt]]]),
    });
    expect(readOrigins("v", 100, "root", ctx)).toEqual(["wr_shared"]);
  });
});
