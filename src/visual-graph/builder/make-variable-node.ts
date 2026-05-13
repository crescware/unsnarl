import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { assertHasDef } from "../../ir/serialized/has-def.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import { nodeId } from "./node-id.js";

export function makeVariableNode(v: SerializedVariable): VisualNode {
  assertHasDef(v);
  const def = v.defs[0];
  // ImplicitGlobalVariable has no source-level definition; the analyzer
  // pins its synthetic def to the first reference, so any line we read
  // from it would lie about where the global "lives". Treat it as
  // location-less (line 0), mirroring ModuleSink.
  const line =
    def.type === DEFINITION_TYPE.ImplicitGlobalVariable
      ? 0
      : (v.identifiers[0]?.line ?? def.name.span.line);
  const common = {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: nodeId(v.id),
    name: v.name,
    line,
    endLine: null,
    isJsxElement: false,
    unused: false,
  } as const;

  if (def.type === DEFINITION_TYPE.ImportBinding) {
    if (def.importKind === IMPORT_KIND.Named) {
      if (def.importedName === null) {
        throw new Error(
          `expected importedName for Named ImportBinding ${nodeId(v.id)}`,
        );
      }
      return {
        ...common,
        kind: NODE_KIND.LegacyImportBinding,
        importKind: IMPORT_KIND.Named,
        importedName: def.importedName,
      };
    }
    if (def.importKind === IMPORT_KIND.Default) {
      return {
        ...common,
        kind: NODE_KIND.LegacyImportBinding,
        importKind: IMPORT_KIND.Default,
      };
    }
    if (def.importKind === IMPORT_KIND.Namespace) {
      return {
        ...common,
        kind: NODE_KIND.LegacyImportBinding,
        importKind: IMPORT_KIND.Namespace,
      };
    }
    throw new Error(`expected importKind for ImportBinding ${nodeId(v.id)}`);
  }

  if (def.type === DEFINITION_TYPE.Variable) {
    const initType = def.init?.type ?? null;
    const initIsFunction =
      initType === AST_TYPE.ArrowFunctionExpression ||
      initType === AST_TYPE.FunctionExpression;
    return {
      ...common,
      kind: NODE_KIND.LegacyVariable,
      declarationKind: def.declarationKind,
      initIsFunction,
    };
  }

  switch (def.type) {
    case DEFINITION_TYPE.FunctionName:
      return { ...common, kind: NODE_KIND.LegacyFunctionName };
    case DEFINITION_TYPE.ClassName:
      return { ...common, kind: NODE_KIND.LegacyClassName };
    case DEFINITION_TYPE.Parameter:
      return { ...common, kind: NODE_KIND.LegacyParameter };
    case DEFINITION_TYPE.CatchClause:
      return { ...common, kind: NODE_KIND.LegacyCatchClause };
    case DEFINITION_TYPE.ImplicitGlobalVariable:
      return { ...common, kind: NODE_KIND.LegacyImplicitGlobalVariable };
  }
}
