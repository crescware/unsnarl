import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import type { EmitOptions } from "../emit/emit-options.js";
import type { ParseOptions } from "../parse/parse-options.js";
import type { PruningRunOptions } from "../prune/pruning-run-options.js";

export type PipelineRunOptions = ParseOptions &
  Readonly<{
    format: string;
    emit: EmitOptions;
    pruning: PruningRunOptions | null;
    depths?: NestingDepths;
  }>;
