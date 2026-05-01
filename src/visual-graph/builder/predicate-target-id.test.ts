import { describe, expect, test } from "vitest";

import { PREDICATE_CONTAINER_TYPE } from "../../analyzer/predicate-container-type.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { predicateTargetId } from "./predicate-target-id.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseRef } from "./testing/make-ref.js";
import { baseScope } from "./testing/make-scope.js";
import { predicateContainer } from "./testing/predicate-container.js";
import { span } from "./testing/span.js";

function withSwitchAt(offset: number): {
  scopes: readonly SerializedScope[];
  scopeMap: Map<string, SerializedScope>;
  refFrom: string;
} {
  const switchScope = {
    ...baseScope(),
    id: "switch1",
    type: SCOPE_TYPE.Switch,
    upper: "outer",
    block: {
      type: AST_TYPE.SwitchStatement,
      span: span(offset),
      endSpan: span(offset + 50),
    },
  };
  const inner = { ...baseScope(), id: "inner", upper: "switch1" };
  const outer = { ...baseScope(), id: "outer" };
  const scopes = [outer, switchScope, inner] satisfies SerializedScope[];
  return {
    scopes,
    scopeMap: new Map(scopes.map((s) => [s.id, s])),
    refFrom: "inner",
  };
}

describe("predicateTargetId", () => {
  test("no predicateContainer -> null", () => {
    const ref = { ...baseRef(), predicateContainer: null };
    expect(predicateTargetId(ref, [], new Map())).toBeNull();
  });

  test("SwitchStatement matches an enclosing switch scope by offset", () => {
    const offset = 100;
    const { scopes, scopeMap, refFrom } = withSwitchAt(offset);
    const ref = {
      ...baseRef(),
      from: refFrom,
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        offset,
      ),
    };
    expect(predicateTargetId(ref, scopes, scopeMap)).toBe("s_switch1");
  });

  test("SwitchStatement with no enclosing switch at that offset -> null", () => {
    const { scopes, scopeMap, refFrom } = withSwitchAt(100);
    const ref = {
      ...baseRef(),
      from: refFrom,
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        999,
      ),
    };
    expect(predicateTargetId(ref, scopes, scopeMap)).toBeNull();
  });

  test("IfStatement with two branches -> if-container subgraph id", () => {
    const consequent = {
      ...baseScope(),
      id: "c",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 50,
      },
    };
    const alternate = {
      ...baseScope(),
      id: "a",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 50,
      },
    };
    const scopes = [consequent, alternate] satisfies SerializedScope[];
    const ref = {
      ...baseRef(),
      from: "outer",
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.IfStatement,
        50,
      ),
    };
    expect(predicateTargetId(ref, scopes, new Map())).toBe("cont_if_outer_50");
  });

  test("IfStatement with one branch -> that branch's subgraph id", () => {
    const consequent = {
      ...baseScope(),
      id: "lone",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 50,
      },
    };
    const ref = {
      ...baseRef(),
      from: "outer",
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.IfStatement,
        50,
      ),
    };
    expect(predicateTargetId(ref, [consequent], new Map())).toBe("s_lone");
  });

  test("IfStatement with zero matching branches -> null", () => {
    const ref = {
      ...baseRef(),
      from: "outer",
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.IfStatement,
        50,
      ),
    };
    expect(predicateTargetId(ref, [], new Map())).toBeNull();
  });
});
