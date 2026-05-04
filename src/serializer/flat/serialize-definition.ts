import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type {
  AstNode,
  Definition,
  SerializedDefinition,
  Span,
} from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { IMPORT_KIND } from "../import-kind.js";
import {
  VARIABLE_DECLARATION_KIND,
  type VariableDeclarationKind,
} from "../variable-declaration-kind.js";
import { spanOf } from "./span-of.js";

export function serializeDefinition(
  d: Definition,
  raw: string,
): SerializedDefinition {
  const common = {
    name: { name: d.name.name, span: spanOf(d.name, raw) },
    node: { type: d.node.type, span: spanOf(d.node, raw) },
    parent:
      d.parent === null
        ? null
        : { type: d.parent.type, span: spanOf(d.parent, raw) },
  } as const;

  if (d.type === DEFINITION_TYPE.ImportBinding) {
    const parent = d.parent;
    if (!parent || parent.type !== AST_TYPE.ImportDeclaration) {
      throw new Error(
        `expected ImportDeclaration parent for ImportBinding ${d.name.name}`,
      );
    }
    const source = parent["source"];
    if (
      source === null ||
      typeof source !== "object" ||
      typeof (source as { value?: unknown }).value !== "string"
    ) {
      throw new Error(
        `expected string source for ImportDeclaration parent of ${d.name.name}`,
      );
    }
    const importSource = (source as { value: string }).value;
    if (d.node.type === AST_TYPE.ImportDefaultSpecifier) {
      return {
        ...common,
        type: DEFINITION_TYPE.ImportBinding,
        importKind: IMPORT_KIND.Default,
        importSource,
      };
    }
    if (d.node.type === AST_TYPE.ImportNamespaceSpecifier) {
      return {
        ...common,
        type: DEFINITION_TYPE.ImportBinding,
        importKind: IMPORT_KIND.Namespace,
        importSource,
      };
    }
    if (d.node.type === AST_TYPE.ImportSpecifier) {
      const imported = d.node["imported"];
      if (imported === null || typeof imported !== "object") {
        throw new Error(
          `expected imported object on ImportSpecifier for ${d.name.name}`,
        );
      }
      const head = imported as { name?: unknown; value?: unknown };
      const importedName =
        typeof head.name === "string"
          ? head.name
          : typeof head.value === "string"
            ? head.value
            : null;
      if (importedName === null) {
        throw new Error(
          `expected imported.name or imported.value on ImportSpecifier for ${d.name.name}`,
        );
      }
      return {
        ...common,
        type: DEFINITION_TYPE.ImportBinding,
        importKind: IMPORT_KIND.Named,
        importedName,
        importSource,
      };
    }
    throw new Error(
      `unexpected ImportBinding node type ${d.node.type} for ${d.name.name}`,
    );
  }

  if (d.type === DEFINITION_TYPE.Variable) {
    let init: Readonly<{ type: string; span: Span }> | null = null;
    if (d.node.type === AST_TYPE.VariableDeclarator) {
      const initNode = d.node["init"];
      if (
        initNode !== null &&
        typeof initNode === "object" &&
        "type" in initNode &&
        typeof (initNode as { type: unknown }).type === "string"
      ) {
        const node = initNode as AstNode;
        init = { type: node.type, span: spanOf(node, raw) };
      }
    }
    let declarationKind: VariableDeclarationKind | null = null;
    if (d.parent !== null && d.parent.type === AST_TYPE.VariableDeclaration) {
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
      ...common,
      type: DEFINITION_TYPE.Variable,
      init,
      declarationKind,
    };
  }

  return { ...common, type: d.type };
}
