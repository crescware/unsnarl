import type { VisualGraph } from "../../visual-graph/model.js";
import type { RootQueryResolution } from "../../visual-graph/prune/resolve-ambiguous-queries.js";

export type EmitOptions = Readonly<{
  prettyJson: boolean;
  prunedGraph: VisualGraph | null;
  resolutions: readonly RootQueryResolution[] | null;
}>;
