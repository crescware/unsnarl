import type { ParsedRootQuery } from "../../../root-query/parsed-root-query.js";

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
  }
}
