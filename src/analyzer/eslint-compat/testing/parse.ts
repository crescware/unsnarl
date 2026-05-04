import { LANGUAGE, type Language } from "../../../language.js";
import { OxcParser } from "../../../parser/oxc-parser.js";
import type { NodeLike } from "../node-like.js";

export function parse(
  code: string,
  language: Language = LANGUAGE.Ts,
): NodeLike {
  const parser = new OxcParser();
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  return parsed.ast as unknown as NodeLike;
}
