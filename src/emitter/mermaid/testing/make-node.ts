import { IMPORT_KIND } from "../../../serializer/import-kind.js";
import { VARIABLE_DECLARATION_KIND } from "../../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";
import type { VisualNode } from "../../../visual-graph/visual-node.js";

type SimpleKind =
  | typeof NODE_KIND.FunctionDeclaration
  | typeof NODE_KIND.ClassDeclaration
  | typeof NODE_KIND.FormalParameter
  | typeof NODE_KIND.CatchParameter
  | typeof NODE_KIND.SyntheticImplicitGlobal
  | typeof NODE_KIND.ReturnArgumentReference
  | typeof NODE_KIND.SyntheticIfStatementTest
  | typeof NODE_KIND.SyntheticSwitchStatementDiscriminant
  | typeof NODE_KIND.SyntheticWhileStatementTest
  | typeof NODE_KIND.SyntheticDoWhileStatementTest
  | typeof NODE_KIND.LegacyForTest
  | typeof NODE_KIND.SyntheticModuleSink
  | typeof NODE_KIND.SyntheticModuleSource
  | typeof NODE_KIND.SyntheticImportIntermediate
  | typeof NODE_KIND.SyntheticExpressionStatement
  | typeof NODE_KIND.SyntheticBeyondDepth;

const COMMON = {
  type: VISUAL_ELEMENT_TYPE.Node,
  id: "n_v",
  name: "x",
  line: 1,
  endLine: null,
  isJsxElement: false,
  unused: false,
} as const;

export function baseNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.ConstBinding }
> {
  return { ...COMMON, kind: NODE_KIND.ConstBinding, initIsFunction: false };
}

export function baseLegacyVariableNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.LegacyVariable }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.LegacyVariable,
    declarationKind: VARIABLE_DECLARATION_KIND.Var,
    initIsFunction: false,
  };
}

export function baseLetBindingNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.LetBinding }
> {
  return { ...COMMON, kind: NODE_KIND.LetBinding, initIsFunction: false };
}

export function baseWriteOpNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.WriteReference }
> {
  return { ...COMMON, kind: NODE_KIND.WriteReference, declarationKind: null };
}

export function baseSimpleNode(
  kind: SimpleKind,
): Extract<VisualNode, { kind: SimpleKind }> {
  return { ...COMMON, kind };
}

export function baseImportBindingNamed(importedName: string): Extract<
  VisualNode,
  {
    kind: typeof NODE_KIND.LegacyImportBinding;
    importKind: typeof IMPORT_KIND.Named;
  }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.LegacyImportBinding,
    importKind: IMPORT_KIND.Named,
    importedName,
  };
}

export function baseImportBindingDefault(): Extract<
  VisualNode,
  {
    kind: typeof NODE_KIND.LegacyImportBinding;
    importKind: typeof IMPORT_KIND.Default;
  }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.LegacyImportBinding,
    importKind: IMPORT_KIND.Default,
  };
}

export function baseImportBindingNamespace(): Extract<
  VisualNode,
  {
    kind: typeof NODE_KIND.LegacyImportBinding;
    importKind: typeof IMPORT_KIND.Namespace;
  }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.LegacyImportBinding,
    importKind: IMPORT_KIND.Namespace,
  };
}
