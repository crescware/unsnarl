import { AST_TYPE } from "../../ast-type.js";
import type { AstIdentifier, AstNode } from "../../ir/model.js";
import { isAstNode } from "./is-ast-node.js";

export function collectBindingIdentifiers(
  pattern: AstNode,
): readonly AstIdentifier[] {
  const out: /* mutable */ AstIdentifier[] = [];
  collect(pattern, out);
  return out;
}

function collect(node: AstNode, out: /* mutable */ AstIdentifier[]): void {
  switch (node.type) {
    case AST_TYPE.Identifier:
      out.push(node as AstIdentifier);
      return;
    case AST_TYPE.ObjectPattern: {
      const properties = node["properties"] as readonly AstNode[] | undefined;
      if (!properties) {
        return;
      }
      for (const p of properties) {
        if (p.type === AST_TYPE.Property) {
          const value = p["value"];
          if (isAstNode(value)) {
            collect(value, out);
          }
        } else if (p.type === AST_TYPE.RestElement) {
          const argument = p["argument"];
          if (isAstNode(argument)) {
            collect(argument, out);
          }
        }
      }
      return;
    }
    case AST_TYPE.ArrayPattern: {
      const elements = node["elements"] as
        | readonly (AstNode | null)[]
        | undefined;
      if (!elements) {
        return;
      }
      for (const el of elements) {
        if (el !== null) {
          collect(el, out);
        }
      }
      return;
    }
    case AST_TYPE.RestElement: {
      const argument = node["argument"];
      if (isAstNode(argument)) {
        collect(argument, out);
      }
      return;
    }
    case AST_TYPE.AssignmentPattern: {
      const left = node["left"];
      if (isAstNode(left)) {
        collect(left, out);
      }
      return;
    }
    default:
      return;
  }
}
