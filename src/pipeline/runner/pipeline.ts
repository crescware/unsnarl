import type { PipelineRunDetails } from "./pipeline-run-details.js";
import type { PipelineRunOptions } from "./pipeline-run-options.js";

export type Pipeline = Readonly<{
  runDetailed(code: string, opts: PipelineRunOptions): PipelineRunDetails;
}>;
