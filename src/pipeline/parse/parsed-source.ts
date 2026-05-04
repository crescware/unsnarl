import type { Language } from "../../cli/language.js";

export type ParsedSource = Readonly<{
  ast: unknown;
  language: Language;
  sourcePath: string;
  raw: string;
}>;
