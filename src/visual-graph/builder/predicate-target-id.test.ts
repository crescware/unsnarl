import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { predicateTargetId } from "./predicate-target-id.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeRef } from "./testing/make-ref.js";
import { makeScope } from "./testing/make-scope.js";
import { predicateContainer } from "./testing/predicate-container.js";
import { span } from "./testing/span.js";

function withSwitchAt(offset: number): {
  scopes: SerializedScope[];
  scopeMap: Map<string, SerializedScope>;
  refFrom: string;
} {
  const switchScope = makeScope({
    id: "switch1",
    type: "switch",
    upper: "outer",
    block: {
      type: "SwitchStatement",
      span: span(offset),
      endSpan: span(offset + 50),
    },
  });
  const inner = makeScope({ id: "inner", upper: "switch1" });
  const outer = makeScope({ id: "outer" });
  const scopes = [outer, switchScope, inner];
  return {
    scopes,
    scopeMap: new Map(scopes.map((s) => [s.id, s])),
    refFrom: "inner",
  };
}

describe("predicateTargetId", () => {
  test("no predicateContainer -> null", () => {
    const ref = makeRef({ predicateContainer: null });
    expect(predicateTargetId(ref, [], new Map())).toBeNull();
  });

  test("SwitchStatement matches an enclosing switch scope by offset", () => {
    const offset = 100;
    const { scopes, scopeMap, refFrom } = withSwitchAt(offset);
    const ref = makeRef({
      from: refFrom,
      predicateContainer: predicateContainer("SwitchStatement", offset),
    });
    expect(predicateTargetId(ref, scopes, scopeMap)).toBe("s_switch1");
  });

  test("SwitchStatement with no enclosing switch at that offset -> null", () => {
    const { scopes, scopeMap, refFrom } = withSwitchAt(100);
    const ref = makeRef({
      from: refFrom,
      predicateContainer: predicateContainer("SwitchStatement", 999),
    });
    expect(predicateTargetId(ref, scopes, scopeMap)).toBeNull();
  });

  test("IfStatement with two branches -> if-container subgraph id", () => {
    const consequent = makeScope({
      id: "c",
      upper: "outer",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const alternate = makeScope({
      id: "a",
      upper: "outer",
      blockContext: makeBlockContext("IfStatement", "alternate", 50),
    });
    const scopes = [consequent, alternate];
    const ref = makeRef({
      from: "outer",
      predicateContainer: predicateContainer("IfStatement", 50),
    });
    expect(predicateTargetId(ref, scopes, new Map())).toBe("cont_if_outer_50");
  });

  test("IfStatement with one branch -> that branch's subgraph id", () => {
    const consequent = makeScope({
      id: "lone",
      upper: "outer",
      blockContext: makeBlockContext("IfStatement", "consequent", 50),
    });
    const ref = makeRef({
      from: "outer",
      predicateContainer: predicateContainer("IfStatement", 50),
    });
    expect(predicateTargetId(ref, [consequent], new Map())).toBe("s_lone");
  });

  test("IfStatement with zero matching branches -> null", () => {
    const ref = makeRef({
      from: "outer",
      predicateContainer: predicateContainer("IfStatement", 50),
    });
    expect(predicateTargetId(ref, [], new Map())).toBeNull();
  });
});
