import type { DefinitionType } from "../../analyzer/definition-type.js";
import type { AstIdentifier } from "../primitive/ast-identifier.js";
import type { AstNode } from "../primitive/ast-node.js";

export type Definition = Readonly<{
  type: DefinitionType;
  name: AstIdentifier;
  node: AstNode;
  parent: AstNode | null;
}>;
