import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { asVariableId } from "../../ir/serialized/variable-id.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { DIRECTION } from "../direction.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { describeSubgraph } from "./describe-subgraph.js";
import { baseCaseClauseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";
import { baseVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";

describe("describeSubgraph", () => {
  test("function subgraph returns kind=function with owner metadata", () => {
    const fnScope = {
      ...baseScope(),
      id: asScopeId("fnScope"),
      type: SCOPE_TYPE.Function,
      block: {
        type: AST_TYPE.FunctionDeclaration,
        span: span(0, 5),
        endSpan: span(50, 10),
      },
    } as const satisfies SerializedScope;
    const owner = {
      ...baseVariable(),
      id: asVariableId("ownerVar"),
      name: asFilledString("myFn"),
      identifiers: [span(0, 5)],
    } as const satisfies SerializedVariable;
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
    const fnScope = {
      ...baseScope(),
      id: asScopeId("fn"),
      type: SCOPE_TYPE.Function,
      block: {
        type: AST_TYPE.FunctionDeclaration,
        span: span(0, 7),
        endSpan: span(20, 9),
      },
    } as const satisfies SerializedScope;
    const owner = {
      ...baseVariable(),
      id: asVariableId("o"),
      name: asFilledString("f"),
      identifiers: [],
    } as const satisfies SerializedVariable;
    const sg = describeSubgraph(
      fnScope,
      new Map([["fn", "o"]]),
      new Map([["o", owner]]),
    );
    expect(sg.line).toEqual(7);
  });

  test("function subgraph without an owner var renders as anonymous (ownerNodeId null, ownerName empty)", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("fn"),
      type: SCOPE_TYPE.Function,
    } as const satisfies SerializedScope;
    const sg = describeSubgraph(scope, new Map(), new Map());
    expect(sg.kind).toEqual(SUBGRAPH_KIND.Function);
    if (sg.kind === SUBGRAPH_KIND.Function) {
      expect(sg.ownerNodeId).toEqual(null);
      expect(sg.ownerName).toEqual("");
    }
  });

  test.each<{
    name: string;
    type: ScopeType;
    expectedKind: VisualSubgraph["kind"];
  }>([
    {
      name: asFilledString("for"),
      type: SCOPE_TYPE.For,
      expectedKind: SUBGRAPH_KIND.For,
    },
    {
      name: asFilledString("catch"),
      type: SCOPE_TYPE.Catch,
      expectedKind: SUBGRAPH_KIND.Catch,
    },
    {
      name: asFilledString("switch"),
      type: SCOPE_TYPE.Switch,
      expectedKind: SUBGRAPH_KIND.Switch,
    },
  ])(
    "control subgraph for scope type $name -> kind=$expectedKind",
    ({ type, expectedKind }) => {
      const scope = {
        ...baseScope(),
        id: asScopeId("ctrl"),
        type,
        block: {
          type: AST_TYPE.BlockStatement,
          span: span(0, 1),
          endSpan: span(10, 3),
        },
      } as const satisfies SerializedScope;
      const sg = describeSubgraph(scope, new Map(), new Map());
      expect(sg.kind).toEqual(expectedKind);
      expect(sg.id).toEqual("s_ctrl");
      expect(sg.line).toEqual(1);
      expect(sg.endLine).toEqual(3);
    },
  );

  test("case subgraph captures caseTest from blockContext", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("case1"),
      type: SCOPE_TYPE.Block,
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(0, 1),
        endSpan: span(10, 2),
      },
      blockContext: {
        ...baseCaseClauseBlockContext(),
        caseTest: asFilledString("x === 1"),
      },
    } as const satisfies SerializedScope;
    const sg = describeSubgraph(scope, new Map(), new Map());
    expect(sg.kind).toEqual(SUBGRAPH_KIND.Case);
    if (sg.kind !== SUBGRAPH_KIND.Case) {
      throw new Error("expected case");
    }
    expect(sg.caseTest).toEqual("x === 1");
  });

  test("case subgraph keeps caseTest null when the case clause has no test (default)", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("case-default"),
      type: SCOPE_TYPE.Block,
      blockContext: baseCaseClauseBlockContext(),
    } as const satisfies SerializedScope;
    const sg = describeSubgraph(scope, new Map(), new Map());
    if (sg.kind !== SUBGRAPH_KIND.Case) {
      throw new Error("expected case");
    }
    expect(sg.caseTest).toEqual(null);
  });

  test("plain block scope renders as the generic 'block' subgraph", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("plain"),
      type: SCOPE_TYPE.Block,
    } as const satisfies SerializedScope;
    const sg = describeSubgraph(scope, new Map(), new Map());
    expect(sg.kind).toEqual(SUBGRAPH_KIND.Block);
  });

  test("class scope with a ClassName binding picks up the inner identifier as className", () => {
    const classScope = {
      ...baseScope(),
      id: asScopeId("clsScope"),
      type: SCOPE_TYPE.Class,
      block: {
        type: AST_TYPE.ClassExpression,
        span: span(0, 2),
        endSpan: span(30, 4),
      },
      variables: [asVariableId("innerNameVar")],
    } as const satisfies SerializedScope;
    const inner = {
      ...baseVariable(),
      id: asVariableId("innerNameVar"),
      name: asFilledString("Foo"),
      identifiers: [span(0, 2)],
    } as const satisfies SerializedVariable;
    const sg = describeSubgraph(
      classScope,
      new Map(),
      new Map([["innerNameVar", inner]]),
    );
    expect(sg).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: "s_clsScope",
      kind: SUBGRAPH_KIND.Class,
      direction: DIRECTION.RL,
      className: "Foo",
      line: 2,
      endLine: 4,
      elements: [],
    });
  });

  test("class scope with no variables (anonymous ClassExpression) yields className=null", () => {
    const classScope = {
      ...baseScope(),
      id: asScopeId("anon"),
      type: SCOPE_TYPE.Class,
      block: {
        type: AST_TYPE.ClassExpression,
        span: span(0, 1),
        endSpan: span(10, 1),
      },
      variables: [],
    } as const satisfies SerializedScope;
    const sg = describeSubgraph(classScope, new Map(), new Map());
    expect(sg.kind).toEqual(SUBGRAPH_KIND.Class);
    if (sg.kind !== SUBGRAPH_KIND.Class) {
      throw new Error("expected class");
    }
    expect(sg.className).toEqual(null);
  });

  test("throws when scope is neither a function subgraph nor a control kind (e.g. module / global)", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("mod"),
      type: SCOPE_TYPE.Module,
    } as const satisfies SerializedScope;
    expect(() => describeSubgraph(scope, new Map(), new Map())).toThrow();
  });
});
