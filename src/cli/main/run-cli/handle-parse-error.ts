import type { ParseError } from "../../../parser/parse-error.js";

export function handleParseError(e: ParseError): number {
  process.stderr.write(`parse error: ${e.message}\n`);
  return 1;
}
