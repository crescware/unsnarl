import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";
import type { VisualNode } from "../visual-node.js";

// Highlight reuses the `-r/--roots` query grammar but intentionally
// diverges on matching semantics: pruning applies `NAME_QUERY_EXCLUDED`
// on bare name queries (so `-r counter` does not drag every assignment /
// JSX read into the root set), whereas highlight is about painting
// "every place this identifier appears" and benefits from matching those
// use-sites. The grammar (line / line-name / range / range-name / name)
// stays the same -- only the name-query exclusion is dropped.
export function nodeMatchesHighlightQuery(
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
      return node.name === q.name;
    case "line-or-name":
      // resolveAmbiguousQueries rewrites every line-or-name into Line
      // or Name before highlight runs; this arm stays only for switch
      // exhaustiveness.
      return false;
  }
}
