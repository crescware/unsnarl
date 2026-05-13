import { IMPORT_KIND } from "../../../serializer/import-kind.js";
import { NODE_KIND } from "../../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";
import type { VisualNode } from "../../../visual-graph/visual-node.js";

type SimpleKind =
  | typeof NODE_KIND.LegacyFunctionName
  | typeof NODE_KIND.LegacyClassName
  | typeof NODE_KIND.LegacyParameter
  | typeof NODE_KIND.LegacyCatchClause
  | typeof NODE_KIND.LegacyImplicitGlobalVariable
  | typeof NODE_KIND.LegacyReturnUse
  | typeof NODE_KIND.LegacyIfTest
  | typeof NODE_KIND.LegacySwitchDiscriminant
  | typeof NODE_KIND.LegacyWhileTest
  | typeof NODE_KIND.LegacyDoWhileTest
  | typeof NODE_KIND.LegacyForTest
  | typeof NODE_KIND.LegacyModuleSink
  | typeof NODE_KIND.LegacyModuleSource
  | typeof NODE_KIND.LegacyImportIntermediate
  | typeof NODE_KIND.LegacyExpressionStatement
  | typeof NODE_KIND.LegacyBeyondDepth;

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
  { kind: typeof NODE_KIND.LegacyVariable }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.LegacyVariable,
    declarationKind: null,
    initIsFunction: false,
  };
}

export function baseWriteOpNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.LegacyWriteOp }
> {
  return { ...COMMON, kind: NODE_KIND.LegacyWriteOp, declarationKind: null };
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
