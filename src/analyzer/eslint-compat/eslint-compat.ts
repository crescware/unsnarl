import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { AnalyzedSource } from "../../pipeline/analyze/analyzed-source.js";
import type { ScopeAnalyzer } from "../../pipeline/analyze/scope-analyzer.js";
import type { ParsedSource } from "../../pipeline/parse/parsed-source.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import { ScopeManager } from "../manager.js";
import { walk } from "../walk/walk.js";
import { handleEnter } from "./handle-enter.js";
import { handleLeave } from "./handle-leave.js";
import { hoistInto } from "./hoist-into.js";
import type { NodeLike } from "./node-like.js";

export class EslintCompatAnalyzer implements ScopeAnalyzer {
  readonly id = "eslint-compat";

  analyze(parsed: ParsedSource): AnalyzedSource {
    const program = parsed.ast as NodeLike;
    const isModule = parsed.language !== "js";
    const manager = new ScopeManager(
      isModule ? "module" : "global",
      program as unknown as AstNode,
    );
    const diagnostics = new DiagnosticCollector();

    hoistInto(program, manager.current(), parsed.raw, diagnostics);

    walk(program as unknown as AstNode, {
      enter(node, parent, key, path) {
        return handleEnter(
          node as unknown as NodeLike,
          parent as unknown as NodeLike | null,
          key,
          path,
          manager,
          parsed.raw,
          diagnostics,
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

    return {
      rootScope: manager.globalScope,
      diagnostics: diagnostics.list(),
      raw: parsed.raw,
    };
  }
}
