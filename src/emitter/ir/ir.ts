import type { SerializedIR } from "../../ir/model.js";
import type { EmitOptions, Emitter } from "../../pipeline/types.js";

export class IrEmitter implements Emitter {
  readonly format = "ir";
  readonly contentType = "application/json";
  readonly extension = "json";

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const text = opts.pretty ? JSON.stringify(ir, null, 2) : JSON.stringify(ir);
    return `${text}\n`;
  }
}
