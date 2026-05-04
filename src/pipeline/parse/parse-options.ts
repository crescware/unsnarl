import type { Language } from "../../language.js";

export type ParseOptions = Readonly<{
  language: Language;
  sourcePath: string;
}>;
