import { ScopeManager } from "../../analyzer/manager.js";
import { walk } from "../../analyzer/walk/walk.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import {
  SOURCE_TYPE,
  type SourceType,
} from "../../pipeline/parse/source-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import type { EslintScopeAnalysisResult } from "./analysis-result.js";
import { handleEnter } from "./handle-enter.js";
import { handleLeave } from "./handle-leave.js";
import { hoistInto } from "./hoist-into.js";
import type { NodeLike } from "./node-like.js";
import type { AnalysisVisitor } from "./visitor.js";

type AnalyzeOptions = Readonly<{
  sourceType: SourceType;
  raw: string;
}>;

export function analyze(
  ast: AstNode,
  options: AnalyzeOptions,
  visitor: AnalysisVisitor = {},
): EslintScopeAnalysisResult {
  const program = ast as unknown as NodeLike;
  const manager = new ScopeManager(
    options.sourceType === SOURCE_TYPE.Module ? "module" : "global",
    ast,
  );
  const diagnostics = new DiagnosticCollector();

  hoistInto(program, manager.current(), options.raw, diagnostics);

  walk(ast, {
    enter(node, parent, key, path) {
      return handleEnter(
        node as unknown as NodeLike,
        parent as unknown as NodeLike | null,
        key,
        path,
        manager,
        options.raw,
        diagnostics,
        visitor,
      );
    },
    leave(node, parent, key) {
      handleLeave(
        node as unknown as NodeLike,
        parent as unknown as NodeLike | null,
        key,
        manager,
      );
    },
  });

  for (const d of diagnostics.list()) {
    visitor.onDiagnostic?.(d);
  }

  return { globalScope: manager.globalScope };
}
