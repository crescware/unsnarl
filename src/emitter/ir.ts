import type { SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";

export class IrEmitter implements Emitter {
  readonly format = "ir";
  readonly contentType = "application/json";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const pretty = opts.pretty !== false;
    const text = pretty ? JSON.stringify(ir, null, 2) : JSON.stringify(ir);
    return `${text}\n`;
  }
}
