import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { rendersInSyntheticBlock } from "./renders-in-synthetic-block.js";
import {
  baseImportBindingDefault,
  baseImportBindingNamed,
  baseImportBindingNamespace,
  baseLetBindingNode,
  baseNode,
  baseSimpleNode,
  baseVarBindingNode,
  baseWriteOpNode,
} from "./testing/make-node.js";

function nodeOfKind(kind: VisualNode["kind"]): VisualNode {
  switch (kind) {
    case NODE_KIND.ConstBinding:
      return baseNode();
    case NODE_KIND.LetBinding:
      return baseLetBindingNode();
    case NODE_KIND.VarBinding:
      return baseVarBindingNode();
    case NODE_KIND.WriteReference:
      return baseWriteOpNode();
    case NODE_KIND.DefaultImportBinding:
      return baseImportBindingDefault();
    case NODE_KIND.NamedImportBinding:
      return baseImportBindingNamed("imported");
    case NODE_KIND.NamespaceImportBinding:
      return baseImportBindingNamespace();
    default:
      return baseSimpleNode(kind);
  }
}

describe("rendersInSyntheticBlock", () => {
  test.each<{ kind: VisualNode["kind"]; expected: boolean }>([
    { kind: NODE_KIND.SyntheticModuleSink, expected: true },
    { kind: NODE_KIND.SyntheticModuleSource, expected: true },
    { kind: NODE_KIND.SyntheticImportIntermediate, expected: true },
    { kind: NODE_KIND.SyntheticExpressionStatement, expected: true },
    { kind: NODE_KIND.VarBinding, expected: false },
    { kind: NODE_KIND.ConstBinding, expected: false },
    { kind: NODE_KIND.LetBinding, expected: false },
    { kind: NODE_KIND.FunctionDeclaration, expected: false },
    { kind: NODE_KIND.ClassDeclaration, expected: false },
    { kind: NODE_KIND.FormalParameter, expected: false },
    { kind: NODE_KIND.CatchParameter, expected: false },
    { kind: NODE_KIND.NamedImportBinding, expected: false },
    { kind: NODE_KIND.DefaultImportBinding, expected: false },
    { kind: NODE_KIND.NamespaceImportBinding, expected: false },
    { kind: NODE_KIND.SyntheticImplicitGlobal, expected: false },
    { kind: NODE_KIND.WriteReference, expected: false },
    { kind: NODE_KIND.ReturnArgumentReference, expected: false },
  ])("kind=$kind -> $expected", ({ kind, expected }) => {
    expect(rendersInSyntheticBlock(nodeOfKind(kind))).toEqual(expected);
  });
});
