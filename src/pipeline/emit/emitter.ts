import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { EmitOptions } from "./emit-options.js";

export type Emitter = Readonly<{
  format: string;
  contentType: string;
  // File extension without leading dot, used by `--out-dir` to derive
  // output filenames. Multiple emitters may share an extension (e.g. ir
  // and json both write JSON).
  extension: string;
  emit(ir: SerializedIR, opts: EmitOptions): string;
}>;
