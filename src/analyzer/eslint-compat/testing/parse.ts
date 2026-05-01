import { OxcParser } from "../../../parser/oxc.js";
import type { NodeLike } from "../node-like.js";

export function parse(
  code: string,
  language: "ts" | "tsx" | "js" | "jsx" = "ts",
): NodeLike {
  const parser = new OxcParser();
  const parsed = parser.parse(code, {
    language,
    sourcePath: `input.${language}`,
  });
  return parsed.ast as unknown as NodeLike;
}
