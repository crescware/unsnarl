import type { SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";
import { buildVisualGraph } from "../visual-graph/builder.js";

export class JsonEmitter implements Emitter {
  readonly format = "json";
  readonly contentType = "application/json";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const graph = opts.prunedGraph ?? buildVisualGraph(ir);
    const pretty = opts.pretty !== false;
    const text = pretty
      ? JSON.stringify(graph, null, 2)
      : JSON.stringify(graph);
    return `${text}\n`;
  }
}
