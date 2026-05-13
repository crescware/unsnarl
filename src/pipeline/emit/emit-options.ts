import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import type { RootQueryResolution } from "../../visual-graph/prune/root-query-resolution.js";
import type { VisualGraph } from "../../visual-graph/visual-graph.js";
import type { HighlightRunOptions } from "../highlight/highlight-run-options.js";

export type EmitOptions = Readonly<{
  prettyJson: boolean;
  prunedGraph: VisualGraph | null;
  resolutions: readonly RootQueryResolution[] | null;
  // Set of VisualNode ids the renderer should paint as "highlighted".
  // null means "no highlight". Empty set means "highlight requested but
  // matched nothing"; the renderer treats that the same as null but the
  // distinction lets the pipeline produce a stderr warning upstream.
  highlightIds: ReadonlySet<string> | null;
  // The original highlight request, propagated so emitters that
  // surface the CLI form (currently markdown) can reconstruct `-H` /
  // `--highlight <queries>` in the rendered Query block. null means
  // the highlight flag was not given. Independent from `highlightIds`
  // because that one can be empty even when the request was non-empty
  // (the query missed every node in the pruned graph).
  highlight: HighlightRunOptions | null;
  debug: boolean;
  depths?: NestingDepths;
}>;
