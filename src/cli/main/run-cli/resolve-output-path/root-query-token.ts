import type { ParsedRootQuery } from "../../../../root-query/parsed-root-query.js";

export function rootQueryToken(q: ParsedRootQuery): string {
  switch (q.kind) {
    case "name":
      return q.name;
    case "line":
      return `l${q.line}`;
    case "line-name":
      return `l${q.line}-${q.name}`;
    case "range":
      return `l${q.start}-${q.end}`;
    case "range-name":
      return `l${q.start}-${q.end}-${q.name}`;
    case "line-or-name":
      // Filename is derived from CLI args before the resolver runs, so
      // we only know the line number here. Normalize to the same lowercase
      // shape as a plain Line query to keep `-r L12` and `-r 12` filenames
      // aligned (the L-prefix is a typing-convenience syntax).
      return `l${q.line}`;
  }
}
