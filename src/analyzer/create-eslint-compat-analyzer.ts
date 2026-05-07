import { EslintCompatAnalyzer } from "../boundary/eslint-scope/eslint-compat.js";
import { DefaultAnnotationBuilder } from "./annotate/default-annotation-builder.js";

// Composition root: wires the eslint-compat algorithm with the
// unsnarl-specific annotation adapter. Lives outside eslint-compat so
// the algorithm layer never reaches into the annotation producers
// directly.
export function createEslintCompatAnalyzer(): EslintCompatAnalyzer {
  return new EslintCompatAnalyzer(new DefaultAnnotationBuilder());
}
