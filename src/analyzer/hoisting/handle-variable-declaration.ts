import { DEFINITION_TYPE, DIAGNOSTIC_KIND } from "../../constants.js";
import type { AstNode, Scope } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { spanFromOffset } from "../../util/span.js";
import { collectBindingIdentifiers } from "../declare/collect-binding-identifiers.js";
import { declareVariable } from "../declare/declare-variable.js";
import { isNodeLike, type NodeLike } from "./node-like.js";

export function handleVariableDeclaration(
  node: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const kind = node["kind"];
  if (kind === "var") {
    const start = node.start ?? 0;
    diagnostics.add(
      DIAGNOSTIC_KIND.VarDetected,
      "var declaration is not supported and was skipped.",
      spanFromOffset(raw, start),
    );
    return;
  }
  if (kind !== "const" && kind !== "let") {
    return;
  }
  const declarations = node["declarations"];
  if (!Array.isArray(declarations)) {
    return;
  }
  for (const dec of declarations) {
    if (!isNodeLike(dec)) {
      continue;
    }
    const id = dec["id"];
    if (!isNodeLike(id)) {
      continue;
    }
    const idents = collectBindingIdentifiers(id as unknown as AstNode);
    for (const ident of idents) {
      declareVariable(
        scope,
        ident,
        DEFINITION_TYPE.Variable,
        dec as unknown as AstNode,
        node as unknown as AstNode,
      );
    }
  }
}
