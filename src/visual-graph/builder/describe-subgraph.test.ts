import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { ScopeType, SerializedVariable } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { DIRECTION } from "../direction.js";
import type { VisualSubgraph } from "../model.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { describeSubgraph } from "./describe-subgraph.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";
import { makeVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";

describe("describeSubgraph", () => {
  test("function subgraph returns kind=function with owner metadata", () => {
    const fnScope = makeScope({
      id: "fnScope",
      type: SCOPE_TYPE.Function,
      block: {
        type: AST_TYPE.FunctionDeclaration,
        span: span(0, 5),
        endSpan: span(50, 10),
      },
    });
    const owner: SerializedVariable = makeVariable({
      id: "ownerVar",
      name: "myFn",
      identifiers: [span(0, 5)],
    });
    const owners = new Map([["fnScope", "ownerVar"]]);
    const variables = new Map([["ownerVar", owner]]);

    const sg = describeSubgraph(fnScope, owners, variables);

    expect(sg).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: "s_fnScope",
      kind: SUBGRAPH_KIND.Function,
      direction: DIRECTION.RL,
      ownerNodeId: "n_ownerVar",
      ownerName: "myFn",
      line: 5,
      endLine: 10,
      elements: [],
    });
  });

  test("function subgraph falls back to scope.block.span.line when owner has no identifiers", () => {
    const fnScope = makeScope({
      id: "fn",
      type: SCOPE_TYPE.Function,
      block: {
        type: AST_TYPE.FunctionDeclaration,
        span: span(0, 7),
        endSpan: span(20, 9),
      },
    });
    const owner = makeVariable({ id: "o", name: "f", identifiers: [] });
    const sg = describeSubgraph(
      fnScope,
      new Map([["fn", "o"]]),
      new Map([["o", owner]]),
    );
    expect(sg.line).toBe(7);
  });

  test("function subgraph throws if owner var id is missing from owner map", () => {
    const scope = makeScope({ id: "fn", type: SCOPE_TYPE.Function });
    expect(() => describeSubgraph(scope, new Map(), new Map())).toThrow();
  });

  test.each<{
    name: string;
    type: ScopeType;
    expectedKind: VisualSubgraph["kind"];
  }>([
    { name: "for", type: SCOPE_TYPE.For, expectedKind: "for" },
    { name: "catch", type: SCOPE_TYPE.Catch, expectedKind: "catch" },
    { name: "switch", type: SCOPE_TYPE.Switch, expectedKind: "switch" },
  ])(
    "control subgraph for scope type $name -> kind=$expectedKind",
    ({ type, expectedKind }) => {
      const scope = makeScope({
        id: "ctrl",
        type,
        block: { type: "Block", span: span(0, 1), endSpan: span(10, 3) },
      });
      const sg = describeSubgraph(scope, new Map(), new Map());
      expect(sg.kind).toBe(expectedKind);
      expect(sg.id).toBe("s_ctrl");
      expect(sg.line).toBe(1);
      expect(sg.endLine).toBe(3);
    },
  );

  test("case subgraph captures caseTest from blockContext", () => {
    const scope = makeScope({
      id: "case1",
      type: SCOPE_TYPE.Block,
      block: { type: "Block", span: span(0, 1), endSpan: span(10, 2) },
      blockContext: makeBlockContext(
        AST_TYPE.SwitchStatement,
        "cases",
        0,
        "x === 1",
      ),
    });
    const sg = describeSubgraph(scope, new Map(), new Map());
    expect(sg.kind).toBe("case");
    expect(sg.caseTest).toBe("x === 1");
  });

  test("case subgraph defaults caseTest to null when blockContext lacks it", () => {
    const scope = makeScope({
      id: "case-default",
      type: SCOPE_TYPE.Block,
      blockContext: makeBlockContext(AST_TYPE.SwitchStatement, "cases", 0),
    });
    const sg = describeSubgraph(scope, new Map(), new Map());
    expect(sg.caseTest).toBeNull();
  });

  test("throws when scope is neither a function subgraph nor a control kind", () => {
    const scope = makeScope({ id: "plain", type: SCOPE_TYPE.Block });
    expect(() => describeSubgraph(scope, new Map(), new Map())).toThrow();
  });
});
