import type { ParseResult } from "./parse-error.js";

const DIR_TOKEN_RE = /^\+([abc])([0-9]*)$/;

type ParsedDirectionToken = Readonly<{
  dir: "a" | "b" | "c";
  level: number | null;
}>;

export function parseDirectionToken(
  text: string,
): ParseResult<ParsedDirectionToken> {
  const m = DIR_TOKEN_RE.exec(text);
  if (m === null) {
    return {
      ok: false,
      errors: [
        {
          message: `unexpected direction token '${text}' (expected one of '+a', '+b', '+c')`,
        },
      ],
    };
  }
  const dir = m[1] as "a" | "b" | "c";
  const levelStr = m[2] ?? "";
  const level = levelStr === "" ? null : Number.parseInt(levelStr, 10);
  return { ok: true, value: { dir, level } };
}
