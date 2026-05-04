import type { Span } from "../../../ir/primitive/span.js";
import { span } from "./span.js";

export function returnContainer(
  startOffset: number,
  endOffset: number,
  startLine = 1,
  endLine = startLine,
): { startSpan: Span; endSpan: Span } {
  return {
    startSpan: span(startOffset, startLine),
    endSpan: span(endOffset, endLine),
  };
}
