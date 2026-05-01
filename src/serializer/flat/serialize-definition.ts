import type {
  AstNode,
  Definition,
  SerializedDefinition,
  Span,
} from "../../ir/model.js";
import { spanOf } from "./span-of.js";

export function serializeDefinition(
  d: Definition,
  raw: string,
): SerializedDefinition {
  let initType: string | null = null;
  let initSpan: Span | null = null;
  if (d.node.type === "VariableDeclarator") {
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
  if (d.type === "ImportBinding") {
    if (d.node.type === "ImportDefaultSpecifier") {
      importKind = "default";
    } else if (d.node.type === "ImportNamespaceSpecifier") {
      importKind = "namespace";
    } else if (d.node.type === "ImportSpecifier") {
      importKind = "named";
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
    if (parent && parent.type === "ImportDeclaration") {
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
    d.type === "Variable" &&
    d.parent !== null &&
    d.parent.type === "VariableDeclaration"
  ) {
    const kind = d.parent["kind"];
    if (kind === "var" || kind === "let" || kind === "const") {
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
