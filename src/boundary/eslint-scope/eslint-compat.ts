import { ScopeManager } from "../../analyzer/manager.js";
import { walk } from "../../analyzer/walk/walk.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { AnalyzedSource } from "../../pipeline/analyze/analyzed-source.js";
import type { ScopeAnalyzer } from "../../pipeline/analyze/scope-analyzer.js";
import type { ParsedSource } from "../../pipeline/parse/parsed-source.js";
import { SOURCE_TYPE } from "../../pipeline/parse/source-type.js";
import { DiagnosticCollector } from "../../util/diagnostic.js";
import type { AnnotationBuilder } from "./annotation-builder.js";
import { handleEnter } from "./handle-enter.js";
import { handleLeave } from "./handle-leave.js";
import { hoistInto } from "./hoist-into.js";
import type { NodeLike } from "./node-like.js";

export class EslintCompatAnalyzer implements ScopeAnalyzer {
  readonly id = "eslint-compat";

  constructor(private readonly annotationBuilder: AnnotationBuilder) {}

  analyze(parsed: ParsedSource): AnalyzedSource {
    const program = parsed.ast as NodeLike;
    const manager = new ScopeManager(
      parsed.sourceType === SOURCE_TYPE.Module ? "module" : "global",
      program as unknown as AstNode,
    );
    const diagnostics = new DiagnosticCollector();
    const annotationBuilder = this.annotationBuilder;

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
          annotationBuilder,
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
      annotations: manager.annotations,
      diagnostics: diagnostics.list(),
      raw: parsed.raw,
    };
  }
}
