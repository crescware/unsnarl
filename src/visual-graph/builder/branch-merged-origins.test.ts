import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { LANGUAGE } from "../../language.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { branchMergedOrigins } from "./branch-merged-origins.js";
import type { BuilderContext } from "./context.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import type { WriteOp } from "./write-op.js";

function makeCtx(opts: {
  scopes: readonly SerializedScope[];
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
    scopeMap: new Map(opts.scopes.map((v) => [v.id, v])),
    subgraphOwnerVar: new Map(),
    writeOpsByVariable: new Map(),
    writeOpsByScope: new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: opts.sortedCasesByContainer ?? new Map(),
  };
}

describe("branchMergedOrigins", () => {
  test("returns empty when the branch has no writes", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const branch = {
      ...baseScope(),
      id: asScopeId("br"),
      upper: asScopeId("root"),
    };
    const ctx = makeCtx({ scopes: [root, branch] });
    expect(branchMergedOrigins("br", [], ctx)).toEqual([]);
  });

  test("returns the linearly last write when it sits directly in the branch scope", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const branch = {
      ...baseScope(),
      id: asScopeId("br"),
      upper: asScopeId("root"),
    };
    const op = { ...baseWriteOp(), refId: "rA", offset: 10, scopeId: "br" };
    const ctx = makeCtx({ scopes: [root, branch] });
    expect(branchMergedOrigins("br", [op], ctx)).toEqual(["wr_rA"]);
  });

  test("recurses into nested if/else under the outer branch and collects both arms", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      upper: asScopeId("root"),
    };
    const cons = {
      ...baseScope(),
      id: asScopeId("cons"),
      upper: asScopeId("outer"),
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const alt = {
      ...baseScope(),
      id: asScopeId("alt"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        key: "alternate",
        parentSpanOffset: 50,
      },
    };
    const opCons: WriteOp = {
      ...baseWriteOp(),
      refId: "rCons",
      offset: 60,
      scopeId: "cons",
    };
    const opAlt: WriteOp = {
      ...baseWriteOp(),
      refId: "rAlt",
      offset: 70,
      scopeId: "alt",
    };
    const ctx = makeCtx({ scopes: [root, outer, cons, alt] });
    expect(new Set(branchMergedOrigins("outer", [opCons, opAlt], ctx))).toEqual(
      new Set(["wr_rCons", "wr_rAlt"]),
    );
  });

  test("recurses into a nested switch and collects all reachable cases", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      upper: asScopeId("root"),
    };
    const switchScope = {
      ...baseScope(),
      id: asScopeId("switch"),
      type: SCOPE_TYPE.Switch,
      upper: asScopeId("outer"),
    };
    const c1 = {
      ...baseScope(),
      id: asScopeId("c1"),
      upper: asScopeId("switch"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 100,
      },
    };
    const c2 = {
      ...baseScope(),
      id: asScopeId("c2"),
      upper: asScopeId("switch"),
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
      scopes: [root, outer, switchScope, c1, c2],
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(new Set(branchMergedOrigins("outer", [opC1, opC2], ctx))).toEqual(
      new Set(["wr_rC1", "wr_rC2"]),
    );
  });

  test("excludes a switch case that exits the function", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      upper: asScopeId("root"),
    };
    const switchScope = {
      ...baseScope(),
      id: asScopeId("switch"),
      type: SCOPE_TYPE.Switch,
      upper: asScopeId("outer"),
    };
    const c1 = {
      ...baseScope(),
      id: asScopeId("c1"),
      upper: asScopeId("switch"),
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
      id: asScopeId("c2"),
      upper: asScopeId("switch"),
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
      scopes: [root, outer, switchScope, c1, c2],
      sortedCasesByContainer: new Map([[containerKey, [c1, c2]]]),
    });
    expect(branchMergedOrigins("outer", [opC1, opC2], ctx)).toEqual(["wr_rC2"]);
  });

  test("descends through 3-level nesting (writes in middle.cons and leaf.cons/alt)", () => {
    // outer
    // ├── midCons writes opMid                  (consequent of outer-of-outer if)
    // └── midAlt
    //     └── inner if/else
    //         ├── leafCons writes opLeafC
    //         └── leafAlt  writes opLeafA
    //
    // After outer's case runs, the reachable last writers are:
    // - midCons path: opMid
    // - midAlt path: leafCons OR leafAlt (inner if has alternate, so always
    //   one of them runs; midAlt itself never writes the variable directly)
    // -> merged origins: {opMid, opLeafC, opLeafA}.
    const root = { ...baseScope(), id: asScopeId("root") };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      upper: asScopeId("root"),
    };
    const middleCons = {
      ...baseScope(),
      id: asScopeId("midCons"),
      upper: asScopeId("outer"),
      blockContext: { ...baseBlockContext(), parentSpanOffset: 50 },
    };
    const middleAlt = {
      ...baseScope(),
      id: asScopeId("midAlt"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        key: "alternate",
        parentSpanOffset: 50,
      },
    };
    const leafCons = {
      ...baseScope(),
      id: asScopeId("leafCons"),
      upper: asScopeId("midAlt"),
      blockContext: { ...baseBlockContext(), parentSpanOffset: 70 },
    };
    const leafAlt = {
      ...baseScope(),
      id: asScopeId("leafAlt"),
      upper: asScopeId("midAlt"),
      blockContext: {
        ...baseBlockContext(),
        key: "alternate",
        parentSpanOffset: 70,
      },
    };
    const opMidCons = {
      ...baseWriteOp(),
      refId: "rMid",
      offset: 60,
      scopeId: "midCons",
    };
    const opLeafCons = {
      ...baseWriteOp(),
      refId: "rLeafC",
      offset: 80,
      scopeId: "leafCons",
    };
    const opLeafAlt = {
      ...baseWriteOp(),
      refId: "rLeafA",
      offset: 90,
      scopeId: "leafAlt",
    };
    const ctx = makeCtx({
      scopes: [root, outer, middleCons, middleAlt, leafCons, leafAlt],
    });
    expect(
      new Set(
        branchMergedOrigins("outer", [opMidCons, opLeafCons, opLeafAlt], ctx),
      ),
    ).toEqual(new Set(["wr_rMid", "wr_rLeafC", "wr_rLeafA"]));
  });

  test("inner if without alternate keeps the pre-inner write reachable", () => {
    // outer (case-like) writes opPre directly, then runs an `if (cond)`
    // that has no else clause.
    //   case 0:
    //     x = pre        // opPre, scopeId = outer
    //     if (cond) {
    //       x = inner    // opInner, scopeId = innerCons
    //     }
    //     break;
    // Without the fallback, only opInner would be returned, but since the
    // inner-if can be skipped at runtime, opPre is still a possible last
    // writer and must remain in the merged origins.
    const root = { ...baseScope(), id: asScopeId("root") };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      upper: asScopeId("root"),
    };
    const innerCons = {
      ...baseScope(),
      id: asScopeId("innerCons"),
      upper: asScopeId("outer"),
      blockContext: { ...baseBlockContext(), parentSpanOffset: 100 },
    };
    const opPre = {
      ...baseWriteOp(),
      refId: "rPre",
      offset: 60,
      scopeId: "outer",
    };
    const opInner = {
      ...baseWriteOp(),
      refId: "rInner",
      offset: 110,
      scopeId: "innerCons",
    };
    const ctx = makeCtx({ scopes: [root, outer, innerCons] });
    expect(
      new Set(branchMergedOrigins("outer", [opPre, opInner], ctx)),
    ).toEqual(new Set(["wr_rPre", "wr_rInner"]));
  });
});
