import type { RootQueryResolution } from "../../visual-graph/prune/resolve-ambiguous-queries.js";
import type { VisualGraph } from "../../visual-graph/visual-graph.js";

export type EmitOptions = Readonly<{
  prettyJson: boolean;
  prunedGraph: VisualGraph | null;
  resolutions: readonly RootQueryResolution[] | null;
}>;
