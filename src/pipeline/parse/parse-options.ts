import type { Language } from "../../cli/language.js";

export type ParseOptions = Readonly<{
  language: Language;
  sourcePath: string;
}>;
