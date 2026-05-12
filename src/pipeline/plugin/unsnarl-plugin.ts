import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";

export type UnsnarlPlugin = Readonly<{
  meta: Readonly<{ name: string }>;
  transform(ir: SerializedIR): SerializedIR;
}>;
