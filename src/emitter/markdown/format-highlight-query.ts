import type { HighlightRunOptions } from "../../pipeline/highlight/highlight-run-options.js";

// Mirrors the CLI form so the rendered Query block round-trips back to
// the invocation the user typed: `-H` for the no-value (roots-tracking)
// mode, `-H <raw>` when the user supplied a query list. The raw string
// is reconstructed from the parsed queries' `.raw` so multi-token
// `-H "a,L7"` keeps its comma form rather than getting normalized.
export function formatHighlightQuery(h: HighlightRunOptions): string {
  if (h.kind === "roots") {
    return "-H";
  }
  return `-H ${h.queries.map((v) => v.raw).join(",")}`;
}
