import type { CategoryDepths } from "../../ir/annotations/scope-annotation.js";
import type { RootQueryResolution } from "../../visual-graph/prune/root-query-resolution.js";
import type { VisualGraph } from "../../visual-graph/visual-graph.js";

export type EmitOptions = Readonly<{
  prettyJson: boolean;
  prunedGraph: VisualGraph | null;
  resolutions: readonly RootQueryResolution[] | null;
  debug: boolean;
  depths?: CategoryDepths;
}>;
