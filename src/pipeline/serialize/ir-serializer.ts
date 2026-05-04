import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializeContext } from "./serialize-context.js";

export type IRSerializer = Readonly<{
  id: string;
  serialize(ctx: SerializeContext): SerializedIR;
}>;
