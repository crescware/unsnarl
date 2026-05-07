import type { EmitterRegistry } from "../emit/emitter-registry.js";
import type { Parser } from "../parse/parser.js";
import type { IRSerializer } from "../serialize/ir-serializer.js";

export type PipelineConfig = Readonly<{
  parser: Parser;
  serializer: IRSerializer;
  emitters: EmitterRegistry;
}>;
