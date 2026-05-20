import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { DIAGNOSTIC_KIND } from "../../analyzer/diagnostic-kind.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { Scope } from "../../ir/scope/scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { spanFromOffset } from "../../util/span.js";
import { collectBindingIdentifiers } from "./declare/collect-binding-identifiers.js";
import { declareVariable } from "./declare/declare-variable.js";
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
    if (!isNodeLike(cand) || cand.type !== AST_TYPE.VariableDeclaration) {
      continue;
    }
    const kind = cand["kind"];
    if (
      kind !== VARIABLE_DECLARATION_KIND.Var &&
      kind !== VARIABLE_DECLARATION_KIND.Let &&
      kind !== VARIABLE_DECLARATION_KIND.Const
    ) {
      continue;
    }
    if (kind === VARIABLE_DECLARATION_KIND.Var) {
      diagnostics.add(
        DIAGNOSTIC_KIND.VarDetected,
        "var declaration detected; rendered as node only (no edges).",
        spanFromOffset(raw, cand.start ?? 0),
      );
    }
    const declarations = cand["declarations"];
    if (!Array.isArray(declarations)) {
      continue;
    }
    // var bindings hoist out of the for-statement scope into the
    // enclosing function / module / global scope.
    const target =
      kind === VARIABLE_DECLARATION_KIND.Var ? scope.variableScope : scope;
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
          target,
          ident,
          DEFINITION_TYPE.Variable,
          dec as unknown as AstNode,
          cand as unknown as AstNode,
        );
      }
    }
  }
}
