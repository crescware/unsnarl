import type { ParsedRootQuery } from "../../cli/root-query/parsed-root-query.js";
import { ROOT_QUERY_KIND } from "../../cli/root-query/root-query-kind.js";
import type { VisualGraph } from "../model.js";
import { iterateVisualNodes } from "./iterate-visual-nodes.js";
import { NAME_QUERY_EXCLUDED } from "./name-query-excluded.js";

export type RootQueryResolution = Readonly<{
  raw: string;
  line: number;
  name: string;
  resolvedAs: "name" | "line";
}>;

const L_PREFIX_RE = /^[Ll][0-9]+$/;

export function resolveAmbiguousQueries(
  graph: VisualGraph,
  queries: readonly ParsedRootQuery[],
): Readonly<{
  resolved: readonly ParsedRootQuery[];
  resolutions: readonly RootQueryResolution[];
}> {
  const hasAmbiguous = queries.some(
    (q) => q.kind === ROOT_QUERY_KIND.LineOrName,
  );
  if (!hasAmbiguous) {
    return { resolved: queries, resolutions: [] };
  }

  // Collect names that a `Name` query could actually match: candidate
  // nodes (per ROOT_CANDIDATE_KINDS via iterateVisualNodes) minus the
  // use-site kinds excluded from name matching. Anything else is invisible
  // to `-r <id>`, so it must not influence the ambiguity decision either.
  const matchableNames = new Set<string>();
  for (const node of iterateVisualNodes(graph.elements)) {
    if (NAME_QUERY_EXCLUDED.has(node.kind)) {
      continue;
    }
    matchableNames.add(node.name);
  }

  let anyLPrefixedMatchable = false;
  for (const n of matchableNames) {
    if (L_PREFIX_RE.test(n)) {
      anyLPrefixedMatchable = true;
      break;
    }
  }

  const resolved: ParsedRootQuery[] = [];
  const resolutions: RootQueryResolution[] = [];

  for (const q of queries) {
    if (q.kind !== ROOT_QUERY_KIND.LineOrName) {
      resolved.push(q);
      continue;
    }
    if (!anyLPrefixedMatchable) {
      resolved.push({ kind: ROOT_QUERY_KIND.Line, line: q.line, raw: q.raw });
      continue;
    }
    if (matchableNames.has(q.name)) {
      resolved.push({ kind: ROOT_QUERY_KIND.Name, name: q.name, raw: q.raw });
      resolutions.push({
        raw: q.raw,
        line: q.line,
        name: q.name,
        resolvedAs: "name",
      });
    } else {
      resolved.push({ kind: ROOT_QUERY_KIND.Line, line: q.line, raw: q.raw });
      resolutions.push({
        raw: q.raw,
        line: q.line,
        name: q.name,
        resolvedAs: "line",
      });
    }
  }

  return { resolved, resolutions };
}
