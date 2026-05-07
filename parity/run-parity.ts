import { analyze } from "../src/boundary/eslint-scope/analyze.js";
import type { AstNode } from "../src/ir/primitive/ast-node.js";
import { OxcParser } from "../src/parser/oxc-parser.js";
import { analyzeWithEslintScope } from "./baseline/analyze-with-eslint-scope.js";
import { compareNormalized } from "./compare/compare-normalized.js";
import type { Mismatch } from "./compare/mismatch.js";
import { filterKnownDivergences } from "./filter-known-divergences.js";
import { normalizeFromEslintScope } from "./from-eslint-scope/normalize-from-eslint-scope.js";
import { normalizeFromUnsnarl } from "./from-unsnarl/normalize-from-unsnarl.js";
import type { ParityInput } from "./parity-input.js";

export function runParity(input: ParityInput): readonly Mismatch[] {
  const parser = new OxcParser();
  const parsed = parser.parse(input.code, {
    language: input.language,
    sourcePath: `parity.${input.language}`,
    sourceType: input.sourceType,
  });

  const { globalScope: unsnarlGlobalScope } = analyze(parsed.ast as AstNode, {
    sourceType: parsed.sourceType,
    raw: parsed.raw,
  });
  const unsnarlNormalized = normalizeFromUnsnarl(unsnarlGlobalScope);

  const baselineManager = analyzeWithEslintScope(parsed, input.sourceType);
  const globalScope = baselineManager.globalScope;
  if (globalScope === null) {
    throw new Error(
      `eslint-scope baseline returned null globalScope for ${input.fixtureId}`,
    );
  }
  const baselineRoot =
    input.sourceType === "module"
      ? (globalScope.childScopes[0] ?? globalScope)
      : globalScope;
  const baselineNormalized = normalizeFromEslintScope(baselineRoot);

  const all = compareNormalized(unsnarlNormalized, baselineNormalized);
  return filterKnownDivergences(input.fixtureId, all);
}
