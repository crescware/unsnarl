import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import type { VisualNode } from "../model.js";
import { NAME_QUERY_EXCLUDED } from "./name-query-excluded.js";

export function nodeMatchesQuery(
  node: VisualNode,
  q: ParsedRootQuery,
): boolean {
  const startLine = node.line;
  const endLine = node.endLine ?? node.line;
  switch (q.kind) {
    case "line":
      return q.line >= startLine && q.line <= endLine;
    case "line-name":
      return q.line >= startLine && q.line <= endLine && node.name === q.name;
    case "range":
      return startLine <= q.end && endLine >= q.start;
    case "range-name":
      return startLine <= q.end && endLine >= q.start && node.name === q.name;
    case "name":
      return !NAME_QUERY_EXCLUDED.has(node.kind) && node.name === q.name;
    case "line-or-name":
      // resolveAmbiguousQueries rewrites every line-or-name into Line or
      // Name before pruning runs, so this branch is unreachable. The arm
      // exists to keep the switch exhaustive.
      return false;
  }
}
