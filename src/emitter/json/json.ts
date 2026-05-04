import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { EmitOptions, Emitter } from "../../pipeline/types.js";
import { buildVisualGraph } from "../../visual-graph/builder.js";

export class JsonEmitter implements Emitter {
  readonly format = "json";
  readonly contentType = "application/json";
  readonly extension = "json";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const graph = opts.prunedGraph ?? buildVisualGraph(ir);
    const text = opts.prettyJson
      ? JSON.stringify(graph, null, 2)
      : JSON.stringify(graph);
    return `${text}\n`;
  }
}
