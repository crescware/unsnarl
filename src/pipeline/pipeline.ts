import { buildVisualGraph } from "../visual-graph/builder.js";
import { pruneVisualGraph } from "../visual-graph/prune/prune-visual-graph.js";
import type {
  EmitOptions,
  Pipeline,
  PipelineConfig,
  PipelineRunDetails,
  PipelineRunOptions,
} from "./types.js";

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

    if (opts.pruning !== null && emitter.format !== "ir") {
      const built = buildVisualGraph(ir);
      const pr = pruneVisualGraph(built, opts.pruning);
      emitOpts = { ...opts.emit, prunedGraph: pr.graph };
      perQuery = pr.perQuery.map(({ query, matched }) => ({
        query: query.raw,
        matched,
      }));
    }

    const text = emitter.emit(ir, emitOpts);
    return { text, pruning: perQuery };
  }

  return { runDetailed };
}
