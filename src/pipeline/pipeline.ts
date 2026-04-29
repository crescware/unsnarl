import type { Pipeline, PipelineConfig, PipelineRunOptions } from "./types.js";

export function createPipeline(config: PipelineConfig): Pipeline {
  return {
    run(code: string, opts: PipelineRunOptions): string {
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
      return emitter.emit(ir, opts.emit ?? {});
    },
  };
}
