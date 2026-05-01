import type { AstNode, BlockContext } from "../../ir/model.js";
import type { DiagnosticCollector } from "../../util/diagnostic.js";
import { hoistDeclarations } from "../hoisting/hoist-declarations.js";
import type { ScopeManager } from "../manager.js";
import { caseExitsFunction } from "./case-exits-function.js";
import { caseFallsThrough } from "./case-falls-through.js";
import { formatCaseTest } from "./format-case-test.js";
import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function enterSwitchCase(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
  raw: string,
  diagnostics: DiagnosticCollector,
): void {
  const test = node["test"];
  const caseTest = isNodeLike(test) ? formatCaseTest(test, raw) : null;
  const ctx: BlockContext | null =
    parent && key !== null
      ? {
          kind: "case-clause",
          parentType: parent.type,
          key,
          parentSpanOffset: parent.start ?? 0,
          caseTest,
        }
      : null;
  const scope = manager.push("block", node as unknown as AstNode, ctx);
  const consequent = node["consequent"];
  if (Array.isArray(consequent)) {
    (
      scope as unknown as {
        unsnarlFallsThrough: boolean;
        unsnarlExitsFunction: boolean;
      }
    ).unsnarlFallsThrough = caseFallsThrough(consequent);
    (
      scope as unknown as {
        unsnarlFallsThrough: boolean;
        unsnarlExitsFunction: boolean;
      }
    ).unsnarlExitsFunction = caseExitsFunction(consequent);
    hoistDeclarations(consequent, scope, raw, diagnostics);
  } else {
    (scope as unknown as { unsnarlFallsThrough: boolean }).unsnarlFallsThrough =
      true;
  }
}
