import { DEFINITION_TYPE, DIAGNOSTIC_KIND } from "../../constants.js";
import type { AstNode, Scope } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { spanFromOffset } from "../../util/span.js";
import { collectBindingIdentifiers } from "../declare/collect-binding-identifiers.js";
import { declareVariable } from "../declare/declare-variable.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function declareForLeft(
  node: NodeLike,
  scope: Scope,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const candidates = [node["init"], node["left"]] satisfies unknown[];
  for (const cand of candidates) {
    if (!isNodeLike(cand) || cand.type !== "VariableDeclaration") {
      continue;
    }
    const kind = cand["kind"];
    if (kind === "var") {
      diagnostics.add(
        DIAGNOSTIC_KIND.VarDetected,
        "var declaration is not supported and was skipped.",
        spanFromOffset(raw, cand.start ?? 0),
      );
      continue;
    }
    if (kind !== "const" && kind !== "let") {
      continue;
    }
    const declarations = cand["declarations"];
    if (!Array.isArray(declarations)) {
      continue;
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
          cand as unknown as AstNode,
        );
      }
    }
  }
}
