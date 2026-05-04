import type { RootQueryResolution } from "../../visual-graph/prune/root-query-resolution.js";

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
