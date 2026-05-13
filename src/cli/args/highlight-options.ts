import { InvalidArgumentError, Option } from "commander";

import { parseRootQueries } from "../../root-query/parse-root-queries.js";
import type { ParsedRootQuery } from "../../root-query/parsed-root-query.js";

// Raw value commander hands the action callback for `--highlight`:
// - `false` when the flag is omitted (the default)
// - `true` when `-h` / `--highlight` is given with no inline value
// - `readonly ParsedRootQuery[]` when an inline value is provided and
//   `parseRootQueries` has accepted it
export type RawHighlight = false | true | readonly ParsedRootQuery[];

function coerceHighlight(value: string): readonly ParsedRootQuery[] {
  const r = parseRootQueries(value);
  if (!r.ok) {
    throw new InvalidArgumentError(r.error);
  }
  return r.queries;
}

export function highlightOptions(): readonly Option[] {
  return [
    new Option(
      "-H, --highlight [queries]",
      "Highlight matching nodes and adjacent edges (defaults to the -r/--roots queries)",
    )
      .argParser(coerceHighlight)
      .default(false as RawHighlight),
  ];
}
