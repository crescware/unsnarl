import { AST_TYPE } from "../../ast-type.js";
import { DEFINITION_TYPE } from "../../definition-type.js";
import { IMPORT_KIND } from "../../import-kind.js";
import type {
  AstNode,
  Definition,
  SerializedDefinition,
  Span,
} from "../../ir/model.js";
import { VARIABLE_DECLARATION_KIND } from "../../variable-declaration-kind.js";
import { spanOf } from "./span-of.js";

export function serializeDefinition(
  d: Definition,
  raw: string,
): SerializedDefinition {
  let initType: string | null = null;
  let initSpan: Span | null = null;
  if (d.node.type === AST_TYPE.VariableDeclarator) {
    const init = d.node["init"];
    if (
      init !== null &&
      typeof init === "object" &&
      "type" in init &&
      typeof (init as { type: unknown }).type === "string"
    ) {
      const initNode = init as AstNode;
      initType = initNode.type;
      initSpan = spanOf(initNode, raw);
    }
  }
  let importKind: SerializedDefinition["importKind"] = null;
  let importSource: string | null = null;
  let importedName: string | null = null;
  if (d.type === DEFINITION_TYPE.ImportBinding) {
    if (d.node.type === AST_TYPE.ImportDefaultSpecifier) {
      importKind = IMPORT_KIND.Default;
    } else if (d.node.type === AST_TYPE.ImportNamespaceSpecifier) {
      importKind = IMPORT_KIND.Namespace;
    } else if (d.node.type === AST_TYPE.ImportSpecifier) {
      importKind = IMPORT_KIND.Named;
      const imported = d.node["imported"];
      if (imported !== null && typeof imported === "object") {
        const head = imported as { name?: unknown; value?: unknown };
        if (typeof head.name === "string") {
          importedName = head.name;
        } else if (typeof head.value === "string") {
          importedName = head.value;
        }
      }
    }
    const parent = d.parent;
    if (parent && parent.type === AST_TYPE.ImportDeclaration) {
      const source = parent["source"];
      if (source !== null && typeof source === "object") {
        const value = (source as { value?: unknown }).value;
        if (typeof value === "string") {
          importSource = value;
        }
      }
    }
  }
  let declarationKind: SerializedDefinition["declarationKind"] = null;
  if (
    d.type === DEFINITION_TYPE.Variable &&
    d.parent !== null &&
    d.parent.type === AST_TYPE.VariableDeclaration
  ) {
    const kind = d.parent["kind"];
    if (
      kind === VARIABLE_DECLARATION_KIND.Var ||
      kind === VARIABLE_DECLARATION_KIND.Let ||
      kind === VARIABLE_DECLARATION_KIND.Const
    ) {
      declarationKind = kind;
    }
  }
  return {
    type: d.type,
    name: { name: d.name.name, span: spanOf(d.name, raw) },
    node: { type: d.node.type, span: spanOf(d.node, raw) },
    parent:
      d.parent === null
        ? null
        : { type: d.parent.type, span: spanOf(d.parent, raw) },
    initType,
    initSpan,
    importKind,
    importSource,
    importedName,
    declarationKind,
  };
}
