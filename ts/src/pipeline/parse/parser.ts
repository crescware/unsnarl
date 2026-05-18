import type { ParseOptions } from "./parse-options.js";
import type { ParsedSource } from "./parsed-source.js";

export type Parser = Readonly<{
  id: string;
  parse(code: string, opts: ParseOptions): ParsedSource;
}>;
