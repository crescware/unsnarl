import { buildVisualGraph } from "../visual-graph/builder/build-visual-graph.js";
import { pruneVisualGraph } from "../visual-graph/prune/prune-visual-graph.js";
import { resolveAmbiguousQueries } from "../visual-graph/prune/resolve-ambiguous-queries.js";
import type { EmitOptions } from "./emit/emit-options.js";
import type { PipelineConfig } from "./runner/pipeline-config.js";
import type { PipelineRunDetails } from "./runner/pipeline-run-details.js";
import type { PipelineRunOptions } from "./runner/pipeline-run-options.js";
import type { Pipeline } from "./runner/pipeline.js";

export function createPipeline(config: PipelineConfig): Pipeline {
  function runDetailed(
    code: string,
    opts: PipelineRunOptions,
  ): PipelineRunDetails {
    const parsed = config.parser.parse(code, {
      language: opts.language,
      sourcePath: opts.sourcePath,
    });
    const analyzed = config.analyzer.analyze(parsed);
    const ir = config.serializer.serialize({
      rootScope: analyzed.rootScope,
      diagnostics: analyzed.diagnostics,
      raw: analyzed.raw,
      source: { path: opts.sourcePath, language: opts.language },
    });
    const emitter = config.emitters.get(opts.format);
    if (!emitter) {
      const available = config.emitters.list().join(", ");
      throw new Error(
        `Unknown emitter format: ${opts.format}. Available: ${available}`,
      );
    }

    let emitOpts: EmitOptions = opts.emit;
    let perQuery: PipelineRunDetails["pruning"] = null;
    let resolutions: PipelineRunDetails["resolutions"] = null;

    if (opts.pruning !== null && emitter.format !== "ir") {
      const built = buildVisualGraph(ir);
      const resolution = resolveAmbiguousQueries(built, opts.pruning.roots);
      const pr = pruneVisualGraph(built, {
        ...opts.pruning,
        roots: resolution.resolved,
      });
      emitOpts = {
        ...opts.emit,
        prunedGraph: pr.graph,
        resolutions: resolution.resolutions,
      };
      perQuery = pr.perQuery.map(({ query, matched }) => ({
        query: query.raw,
        matched,
      }));
      resolutions = resolution.resolutions;
    }

    const text = emitter.emit(ir, emitOpts);
    return { text, pruning: perQuery, resolutions };
  }

  return { runDetailed };
}
