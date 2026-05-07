import type { Language } from "../../language.js";
import type { SourceType } from "./source-type.js";

export type ParsedSource = Readonly<{
  ast: unknown;
  language: Language;
  sourcePath: string;
  sourceType: SourceType;
  raw: string;
}>;
