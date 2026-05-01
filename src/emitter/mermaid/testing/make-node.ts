import { IMPORT_KIND } from "../../../serializer/import-kind.js";
import type { VisualNode } from "../../../visual-graph/model.js";
import { NODE_KIND } from "../../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";

type SimpleKind =
  | typeof NODE_KIND.FunctionName
  | typeof NODE_KIND.ClassName
  | typeof NODE_KIND.Parameter
  | typeof NODE_KIND.CatchClause
  | typeof NODE_KIND.ImplicitGlobalVariable
  | typeof NODE_KIND.ReturnUse
  | typeof NODE_KIND.ModuleSink
  | typeof NODE_KIND.ModuleSource
  | typeof NODE_KIND.ImportIntermediate;

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
  { kind: typeof NODE_KIND.Variable }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.Variable,
    declarationKind: null,
    initIsFunction: false,
  };
}

export function baseWriteOpNode(): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.WriteOp }
> {
  return { ...COMMON, kind: NODE_KIND.WriteOp, declarationKind: null };
}

export function baseSimpleNode(
  kind: SimpleKind,
): Extract<VisualNode, { kind: SimpleKind }> {
  return { ...COMMON, kind };
}

export function baseImportBindingNamed(
  importedName: string,
): Extract<
  VisualNode,
  { kind: typeof NODE_KIND.ImportBinding; importKind: typeof IMPORT_KIND.Named }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.ImportBinding,
    importKind: IMPORT_KIND.Named,
    importedName,
  };
}

export function baseImportBindingDefault(): Extract<
  VisualNode,
  {
    kind: typeof NODE_KIND.ImportBinding;
    importKind: typeof IMPORT_KIND.Default;
  }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.ImportBinding,
    importKind: IMPORT_KIND.Default,
  };
}

export function baseImportBindingNamespace(): Extract<
  VisualNode,
  {
    kind: typeof NODE_KIND.ImportBinding;
    importKind: typeof IMPORT_KIND.Namespace;
  }
> {
  return {
    ...COMMON,
    kind: NODE_KIND.ImportBinding,
    importKind: IMPORT_KIND.Namespace,
  };
}
