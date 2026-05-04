import type { Span } from "../ir/primitive/span.js";

export function spanFromOffset(raw: string, offset: number): Span {
  let line = 1;
  let lastNewline = -1;
  const limit = Math.min(offset, raw.length);
  for (let i = 0; i < limit; i++) {
    if (raw.charCodeAt(i) === 10 /* \n */) {
      line += 1;
      lastNewline = i;
    }
  }
  const column = offset - lastNewline - 1;
  return { line, column, offset };
}
