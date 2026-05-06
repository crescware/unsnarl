import type { ParseOptions } from "../src/pipeline/parse/parse-options.js";
import type { ParityBaselineSourceType } from "./baseline/parity-baseline-source-type.js";

export type ParityInput = Readonly<{
  fixtureId: string;
  code: string;
  language: ParseOptions["language"];
  sourceType: ParityBaselineSourceType;
}>;
