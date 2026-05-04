import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { EmitOptions } from "../../pipeline/emit/emit-options.js";
import type { Emitter } from "../../pipeline/emit/emitter.js";
import { buildVisualGraph } from "../../visual-graph/builder/build-visual-graph.js";

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
