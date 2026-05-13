import { buildVisualGraph } from "../visual-graph/builder/build-visual-graph.js";
import { collectHighlightIds } from "../visual-graph/highlight/collect-highlight-ids.js";
import { pruneVisualGraph } from "../visual-graph/prune/prune-visual-graph.js";
import { resolveAmbiguousQueries } from "../visual-graph/prune/resolve-ambiguous-queries.js";
import { runAnalysis } from "./analyze/run-analysis.js";
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
      sourceType: opts.sourceType,
    });
    const analyzed = runAnalysis(parsed);

    const ir = config.plugins.reduce(
      (acc, plugin) => plugin.transform(acc),
      config.serializer.serialize({
        rootScope: analyzed.rootScope,
        annotations: analyzed.annotations,
        diagnostics: analyzed.diagnostics,
        raw: analyzed.raw,
        source: { path: opts.sourcePath, language: opts.language },
      }),
    );

    const emitter = config.emitters.get(opts.format);
    if (!emitter) {
      const available = config.emitters.list().join(", ");
      throw new Error(
        `Unknown emitter format: ${opts.format}. Available: ${available}`,
      );
    }

    let emitOpts: EmitOptions = opts.depths
      ? { ...opts.emit, depths: opts.depths }
      : opts.emit;
    let perQuery: PipelineRunDetails["pruning"] = null;
    let resolutions: PipelineRunDetails["resolutions"] = null;

    if (
      (opts.pruning !== null || opts.highlight !== null) &&
      emitter.format !== "ir"
    ) {
      const built = buildVisualGraph(
        ir,
        opts.depths ? { depths: opts.depths } : undefined,
      );
      let workingGraph = built;
      // Captured from the prune walk so `-H` in roots mode can paint
      // the exact same id set that pruning treated as roots, instead
      // of re-matching with the looser highlight matcher.
      let pruneRootIds: ReadonlySet<string> | null = null;
      if (opts.pruning !== null) {
        const resolution = resolveAmbiguousQueries(built, opts.pruning.roots);
        const pr = pruneVisualGraph(built, {
          ...opts.pruning,
          roots: resolution.resolved,
        });
        workingGraph = pr.graph;
        pruneRootIds = pr.rootIds;
        emitOpts = {
          ...opts.emit,
          ...(opts.depths ? { depths: opts.depths } : {}),
          prunedGraph: pr.graph,
          resolutions: resolution.resolutions,
        };
        perQuery = pr.perQuery.map(({ query, matched }) => ({
          query: query.raw,
          matched,
        }));
        resolutions = resolution.resolutions;
      }
      if (opts.highlight !== null) {
        // Roots mode mirrors `-r`'s match set verbatim, so it
        // inherits `NAME_QUERY_EXCLUDED` (a bare name query like
        // `-r counter` excludes `WriteOp` / `ReturnUse`, and so does
        // `-r counter -H`). Queries mode (`-H <raw>`) uses the looser
        // highlight matcher so explicit highlight queries paint every
        // occurrence of the identifier.
        let highlightIds: ReadonlySet<string>;
        if (opts.highlight.kind === "roots") {
          highlightIds = pruneRootIds ?? new Set<string>();
        } else {
          const highlightResolution = resolveAmbiguousQueries(
            workingGraph,
            opts.highlight.queries,
          );
          highlightIds = collectHighlightIds(
            workingGraph,
            highlightResolution.resolved,
          );
        }
        emitOpts = { ...emitOpts, highlightIds, highlight: opts.highlight };
      }
    }

    const text = emitter.emit(ir, emitOpts);
    return {
      text,
      pruning: perQuery,
      resolutions,
      diagnostics: ir.diagnostics,
    };
  }

  return { runDetailed };
}
