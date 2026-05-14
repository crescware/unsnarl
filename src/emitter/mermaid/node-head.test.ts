import { describe, expect, test } from "vitest";

import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeHead } from "./node-head.js";
import {
  baseImportBindingDefault,
  baseImportBindingNamed,
  baseImportBindingNamespace,
  baseVarBindingNode,
  baseLetBindingNode,
  baseNode,
  baseSimpleNode,
  baseWriteOpNode,
} from "./testing/make-node.js";

describe("nodeHead", () => {
  test("JSX element wraps the (escaped) name in &lt;...&gt;, ignoring kind-specific format", () => {
    expect(
      nodeHead({
        ...baseNode(),
        kind: NODE_KIND.FunctionDeclaration,
        name: "Foo",
        isJsxElement: true,
      }),
    ).toEqual("&lt;Foo&gt;");
  });

  test.each([
    { kind: NODE_KIND.FunctionDeclaration, name: "foo", expected: "foo()" },
    { kind: NODE_KIND.ClassDeclaration, name: "Foo", expected: "class Foo" },
    { kind: NODE_KIND.CatchParameter, name: "err", expected: "catch err" },
    {
      kind: NODE_KIND.SyntheticImplicitGlobal,
      name: "global1",
      expected: "global global1",
    },
    {
      kind: NODE_KIND.SyntheticModuleSource,
      name: "./mod",
      expected: "module ./mod",
    },
    {
      kind: NODE_KIND.SyntheticImportIntermediate,
      name: "named",
      expected: "import named",
    },
    {
      kind: NODE_KIND.SyntheticExpressionStatement,
      name: "console.log()",
      expected: "console.log()",
    },
  ] as const)("kind $kind formats as $expected", ({ kind, name, expected }) => {
    expect(nodeHead({ ...baseSimpleNode(kind), name })).toEqual(expected);
  });

  test.each([
    {
      name: "renamed named import keeps the local name only",
      node: { ...baseImportBindingNamed("original"), name: "renamed" },
      expected: "renamed",
    },
    {
      name: "non-renamed named import gets 'import' prefix",
      node: { ...baseImportBindingNamed("foo"), name: "foo" },
      expected: "import foo",
    },
    {
      name: "default import gets 'import' prefix",
      node: { ...baseImportBindingDefault(), name: "Foo" },
      expected: "import Foo",
    },
    {
      name: "namespace import gets 'import' prefix",
      node: { ...baseImportBindingNamespace(), name: "ns" },
      expected: "import ns",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toEqual(expected);
  });

  test.each([
    {
      name: "WriteOp with declarationKind=let prepends 'let'",
      node: {
        ...baseWriteOpNode(),
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Let,
      },
      expected: "let x",
    },
    {
      name: "WriteOp with declarationKind=const has no prefix",
      node: {
        ...baseWriteOpNode(),
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Const,
      },
      expected: "x",
    },
    {
      name: "WriteOp without declarationKind has no prefix",
      node: { ...baseWriteOpNode(), name: "x" },
      expected: "x",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toEqual(expected);
  });

  test.each([
    {
      name: "ConstBinding initialized with a function uses '<name>()' format",
      node: { ...baseNode(), name: "f", initIsFunction: true },
      expected: "f()",
    },
    {
      name: "LetBinding prepends 'let'",
      node: { ...baseLetBindingNode(), name: "x" },
      expected: "let x",
    },
    {
      name: "ConstBinding has no prefix",
      node: { ...baseNode(), name: "x" },
      expected: "x",
    },
    {
      name: "var-declared VarBinding prepends 'var'",
      node: { ...baseVarBindingNode(), name: "x" },
      expected: "var x",
    },
    {
      name: "initIsFunction wins over the var prefix",
      node: { ...baseVarBindingNode(), name: "f", initIsFunction: true },
      expected: "f()",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toEqual(expected);
  });

  test("ReturnUse falls through to the default formatting (uses name only)", () => {
    expect(
      nodeHead({
        ...baseNode(),
        kind: NODE_KIND.ReturnArgumentReference,
        name: "x",
      }),
    ).toEqual("x");
  });
});
