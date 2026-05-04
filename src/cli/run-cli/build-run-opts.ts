import type { PruningRunOptions } from "../../pipeline/prune/pruning-run-options.js";
import type { PipelineRunOptions } from "../../pipeline/runner/pipeline-run-options.js";
import { readSourceFile } from "../io.js";
import { detectLanguage } from "./detect-language.js";
import { resolveGenerations } from "./resolve-generations.js";
import type { ExecuteSource } from "./execute-source.js";
import type { NormalizedCliOptions } from "./normalized-cli-options.js";

type Return = Readonly<{
  text: string;
  runOpts: PipelineRunOptions;
}>;

export function buildRunOpts(
  src: ExecuteSource,
  opts: NormalizedCliOptions,
): Return {
  const text = src.stdin ? src.text : readSourceFile(src.path);
  const sourcePath = src.stdin ? `stdin.${src.stdinLang}` : src.path;
  const language = src.stdin ? src.stdinLang : detectLanguage(src.path);

  const pruning =
    0 < opts.roots.length
      ? ({
          roots: opts.roots,
          ...resolveGenerations({
            descendants: opts.descendants,
            ancestors: opts.ancestors,
            context: opts.context,
          }),
        } satisfies PruningRunOptions)
      : null;

  const runOpts = {
    format: opts.format,
    language,
    sourcePath,
    emit: { prettyJson: opts.prettyJson, prunedGraph: null, resolutions: null },
    pruning,
  } satisfies PipelineRunOptions;

  return { text, runOpts };
}
