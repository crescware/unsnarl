import { describe, expect, test } from "vitest";

import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { nodeHead } from "./node-head.js";
import { makeNode } from "./testing/make-node.js";

describe("nodeHead", () => {
  test("JSX element wraps the (escaped) name in &lt;...&gt;, ignoring kind-specific format", () => {
    expect(
      nodeHead(
        makeNode({
          kind: NODE_KIND.FunctionName,
          name: "Foo",
          isJsxElement: true,
        }),
      ),
    ).toBe("&lt;Foo&gt;");
  });

  test.each<{
    kind: Parameters<typeof makeNode>[0] extends undefined
      ? never
      : Parameters<typeof makeNode>[0];
    name: string;
    expected: string;
  }>([
    { kind: { kind: NODE_KIND.FunctionName }, name: "foo", expected: "foo()" },
    { kind: { kind: NODE_KIND.ClassName }, name: "Foo", expected: "class Foo" },
    {
      kind: { kind: NODE_KIND.CatchClause },
      name: "err",
      expected: "catch err",
    },
    {
      kind: { kind: NODE_KIND.ImplicitGlobalVariable },
      name: "global1",
      expected: "global global1",
    },
    {
      kind: { kind: NODE_KIND.ModuleSource },
      name: "./mod",
      expected: "module ./mod",
    },
    {
      kind: { kind: NODE_KIND.ImportIntermediate },
      name: "named",
      expected: "import named",
    },
  ])("kind $kind.kind formats as $expected", ({ kind, name, expected }) => {
    expect(nodeHead(makeNode({ ...kind, name }))).toBe(expected);
  });

  test.each([
    {
      name: "renamed named import keeps the local name only",
      node: makeNode({
        kind: NODE_KIND.ImportBinding,
        name: "renamed",
        importKind: IMPORT_KIND.Named,
        importedName: "original",
      }),
      expected: "renamed",
    },
    {
      name: "non-renamed named import gets 'import' prefix",
      node: makeNode({
        kind: NODE_KIND.ImportBinding,
        name: "foo",
        importKind: IMPORT_KIND.Named,
        importedName: "foo",
      }),
      expected: "import foo",
    },
    {
      name: "default import gets 'import' prefix",
      node: makeNode({
        kind: NODE_KIND.ImportBinding,
        name: "Foo",
        importKind: IMPORT_KIND.Default,
        importedName: null,
      }),
      expected: "import Foo",
    },
    {
      name: "namespace import gets 'import' prefix",
      node: makeNode({
        kind: NODE_KIND.ImportBinding,
        name: "ns",
        importKind: IMPORT_KIND.Namespace,
        importedName: null,
      }),
      expected: "import ns",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toBe(expected);
  });

  test.each([
    {
      name: "WriteOp with declarationKind=let prepends 'let'",
      node: makeNode({
        kind: NODE_KIND.WriteOp,
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Let,
      }),
      expected: "let x",
    },
    {
      name: "WriteOp with declarationKind=const has no prefix",
      node: makeNode({
        kind: NODE_KIND.WriteOp,
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Const,
      }),
      expected: "x",
    },
    {
      name: "WriteOp without declarationKind has no prefix",
      node: makeNode({ kind: NODE_KIND.WriteOp, name: "x" }),
      expected: "x",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toBe(expected);
  });

  test.each([
    {
      name: "Variable initialized with a function uses '<name>()' format",
      node: makeNode({ name: "f", initIsFunction: true }),
      expected: "f()",
    },
    {
      name: "let-declared Variable prepends 'let'",
      node: makeNode({
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Let,
      }),
      expected: "let x",
    },
    {
      name: "const-declared Variable has no prefix",
      node: makeNode({
        name: "x",
        declarationKind: VARIABLE_DECLARATION_KIND.Const,
      }),
      expected: "x",
    },
    {
      name: "Variable without declarationKind has no prefix",
      node: makeNode({ name: "x" }),
      expected: "x",
    },
    {
      name: "initIsFunction wins over declarationKind",
      node: makeNode({
        name: "f",
        initIsFunction: true,
        declarationKind: VARIABLE_DECLARATION_KIND.Let,
      }),
      expected: "f()",
    },
  ])("$name", ({ node, expected }) => {
    expect(nodeHead(node)).toBe(expected);
  });

  test("ReturnUse falls through to the default formatting (uses name only)", () => {
    expect(nodeHead(makeNode({ kind: NODE_KIND.ReturnUse, name: "x" }))).toBe(
      "x",
    );
  });
});
