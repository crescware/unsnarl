import type { ParsedSource } from "../parse/parsed-source.js";
import type { AnalyzedSource } from "./analyzed-source.js";

export type ScopeAnalyzer = Readonly<{
  id: string;
  analyze(parsed: ParsedSource): AnalyzedSource;
}>;
