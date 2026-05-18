import { analyze, type ScopeManager } from "eslint-scope";

import type { ParsedSource } from "../../src/pipeline/parse/parsed-source.js";
import type { ParityBaselineSourceType } from "./parity-baseline-source-type.js";

export function analyzeWithEslintScope(
  parsed: ParsedSource,
  sourceType: ParityBaselineSourceType,
): ScopeManager {
  return analyze(parsed.ast as Parameters<typeof analyze>[0], {
    sourceType,
    ecmaVersion: 2024,
  });
}
