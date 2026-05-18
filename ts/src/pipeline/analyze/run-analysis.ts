import { computeNestingDepths } from "../../analyzer/compute-nesting-depths.js";
import { analyze } from "../../boundary/eslint-scope/analyze.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { ParsedSource } from "../parse/parsed-source.js";
import type { AnalyzedSource } from "./analyzed-source.js";
import { buildAnalysisVisitor } from "./build-analysis-visitor.js";

export function runAnalysis(parsed: ParsedSource): AnalyzedSource {
  const ast = parsed.ast as AstNode;
  const nestingDepthsByOffset = computeNestingDepths(ast);
  const { visitor, capture } = buildAnalysisVisitor(
    parsed.raw,
    nestingDepthsByOffset,
  );
  const { globalScope } = analyze(
    ast,
    { sourceType: parsed.sourceType, raw: parsed.raw },
    visitor,
  );
  const { annotations, diagnostics } = capture(globalScope);
  return {
    rootScope: globalScope,
    annotations,
    diagnostics,
    raw: parsed.raw,
  };
}
