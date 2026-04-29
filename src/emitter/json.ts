import type { SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";

export class JsonEmitter implements Emitter {
  readonly format = "json";
  readonly contentType = "application/json";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const pretty = opts.pretty !== false;
    const text = pretty ? JSON.stringify(ir, null, 2) : JSON.stringify(ir);
    return `${text}\n`;
  }
}
