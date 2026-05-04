import type { Span } from "../../../ir/primitive/span.js";

export function span(offset = 0, line = 1, column = offset): Span {
  return { line, column, offset };
}
