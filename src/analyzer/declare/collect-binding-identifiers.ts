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
    case "Identifier":
      out.push(node as AstIdentifier);
      return;
    case "ObjectPattern": {
      const properties = node["properties"] as readonly AstNode[] | undefined;
      if (!properties) {
        return;
      }
      for (const p of properties) {
        if (p.type === "Property") {
          const value = p["value"];
          if (isAstNode(value)) {
            collect(value, out);
          }
        } else if (p.type === "RestElement") {
          const argument = p["argument"];
          if (isAstNode(argument)) {
            collect(argument, out);
          }
        }
      }
      return;
    }
    case "ArrayPattern": {
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
    case "RestElement": {
      const argument = node["argument"];
      if (isAstNode(argument)) {
        collect(argument, out);
      }
      return;
    }
    case "AssignmentPattern": {
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
