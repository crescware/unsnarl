import { AST_TYPE } from "../ast-type.js";
import type { PathEntry } from "./walk/walk.js";

type JsxElementSpan = Readonly<{
  startOffset: number;
  endOffset: number;
}>;

// A JSX opening-tag identifier (and identifiers reached through a
// JSXMemberExpression chain on the tag name) logically extends from the
// opening tag to the matching closing tag. This helper returns the wrapping
// JSXElement's span when the reference lives in that position; everything
// else (attribute values, embedded expressions, plain JS identifiers) yields
// null so callers fall back to the identifier's own line.
export function findJsxElementSpan(
  path: readonly PathEntry[],
): JsxElementSpan | null {
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      return null;
    }
    if (entry.node.type === AST_TYPE.JSXOpeningElement) {
      const elementEntry = path[i - 1];
      if (!elementEntry || elementEntry.node.type !== AST_TYPE.JSXElement) {
        return null;
      }
      const start = (elementEntry.node as unknown as { start?: number }).start;
      const end = (elementEntry.node as unknown as { end?: number }).end;
      if (typeof start !== "number" || typeof end !== "number") {
        return null;
      }
      return { startOffset: start, endOffset: end };
    }
    if (entry.node.type === AST_TYPE.JSXMemberExpression) {
      continue;
    }
    return null;
  }
  return null;
}
