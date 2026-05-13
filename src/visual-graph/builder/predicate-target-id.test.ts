import { describe, expect, test } from "vitest";

import { PREDICATE_CONTAINER_TYPE } from "../../analyzer/predicate-container-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { BuildState } from "./build-state.js";
import { predicateTargetId } from "./predicate-target-id.js";
import { baseRef } from "./testing/make-ref.js";
import { predicateContainer } from "./testing/predicate-container.js";

function makeState(overrides: Partial<BuildState> = {}): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    ifTestAnchorByOffset: new Map(),
    switchDiscriminantAnchorByOffset: new Map(),
    whileTestAnchorByOffset: new Map(),
    doWhileTestAnchorByOffset: new Map(),
    forTestAnchorByOffset: new Map(),
    pendingLoopTestAnchors: [],
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: [],
    ...overrides,
  };
}

describe("predicateTargetId", () => {
  test("no predicateContainer -> null", () => {
    const ref = { ...baseRef(), predicateContainer: null };
    expect(predicateTargetId(ref, new Map(), makeState())).toEqual(null);
  });

  test("SwitchStatement resolves to the registered switch-discriminant anchor by offset", () => {
    const ref = {
      ...baseRef(),
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        100,
      ),
    };
    const state = makeState({
      switchDiscriminantAnchorByOffset: new Map([
        [100, "switch_discriminant_x"],
      ]),
    });
    expect(predicateTargetId(ref, new Map(), state)).toEqual(
      "switch_discriminant_x",
    );
  });

  test("SwitchStatement with no anchor registered at that offset -> null", () => {
    const ref = {
      ...baseRef(),
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        100,
      ),
    };
    expect(predicateTargetId(ref, new Map(), makeState())).toEqual(null);
  });

  test("IfStatement with no anchor registered at that offset -> null", () => {
    const ref = {
      ...baseRef(),
      from: asScopeId("outer"),
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.IfStatement,
        50,
      ),
    };
    expect(predicateTargetId(ref, new Map(), makeState())).toEqual(null);
  });

  test("WhileStatement resolves to the registered while-test anchor by offset", () => {
    const ref = {
      ...baseRef(),
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.WhileStatement,
        33,
      ),
    };
    const state = makeState({
      whileTestAnchorByOffset: new Map([[33, "while_test_x"]]),
    });
    expect(predicateTargetId(ref, new Map(), state)).toEqual("while_test_x");
  });

  test("DoWhileStatement resolves to the registered do-while-test anchor", () => {
    const ref = {
      ...baseRef(),
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.DoWhileStatement,
        33,
      ),
    };
    const state = makeState({
      doWhileTestAnchorByOffset: new Map([[33, "do_while_test_x"]]),
    });
    expect(predicateTargetId(ref, new Map(), state)).toEqual("do_while_test_x");
  });

  test("ForStatement / ForOfStatement / ForInStatement all resolve through forTestAnchorByOffset", () => {
    const state = makeState({
      forTestAnchorByOffset: new Map([[40, "for_test_x"]]),
    });
    for (const type of [
      PREDICATE_CONTAINER_TYPE.ForStatement,
      PREDICATE_CONTAINER_TYPE.ForOfStatement,
      PREDICATE_CONTAINER_TYPE.ForInStatement,
    ] as const) {
      const ref = {
        ...baseRef(),
        predicateContainer: predicateContainer(type, 40),
      };
      expect(predicateTargetId(ref, new Map(), state)).toEqual("for_test_x");
    }
  });
});
