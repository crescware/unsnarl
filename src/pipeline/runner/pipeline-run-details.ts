import type { RootQueryResolution } from "../../visual-graph/prune/resolve-ambiguous-queries.js";

export type PipelineRunDetails = Readonly<{
  text: string;
  pruning:
    | readonly Readonly<{
        query: string;
        matched: number;
      }>[]
    | null;
  resolutions: readonly RootQueryResolution[] | null;
}>;
