import type {
  AstIdentifier,
  AstNode,
  Definition,
  DefinitionType,
  Scope,
  Variable,
} from "../ir/model.js";
import { VariableImpl } from "./scope.js";

export function collectBindingIdentifiers(pattern: AstNode): AstIdentifier[] {
  const out: AstIdentifier[] = [];
  collect(pattern, out);
  return out;
}

function collect(node: AstNode, out: AstIdentifier[]): void {
  switch (node.type) {
    case "Identifier":
      out.push(node as AstIdentifier);
      return;
    case "ObjectPattern": {
      const properties = node["properties"] as
        | ReadonlyArray<AstNode>
        | undefined;
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
        | ReadonlyArray<AstNode | null>
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

function isAstNode(value: unknown): value is AstNode {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}

export function declareVariable(
  scope: Scope,
  identifier: AstIdentifier,
  defType: DefinitionType,
  defNode: AstNode,
  parent: AstNode | null,
): Variable {
  let variable = scope.set.get(identifier.name);
  if (!variable) {
    variable = new VariableImpl(identifier.name, scope);
    scope.set.set(identifier.name, variable);
    scope.variables.push(variable);
  }
  variable.identifiers.push(identifier);
  const def: Definition = {
    type: defType,
    name: identifier,
    node: defNode,
    parent,
  };
  variable.defs.push(def);
  return variable;
}
