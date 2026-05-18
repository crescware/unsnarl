import type { Language } from "../../language.js";
import type { SourceType } from "./source-type.js";

export type ParseOptions = Readonly<{
  language: Language;
  sourcePath: string;
  sourceType: SourceType;
}>;
