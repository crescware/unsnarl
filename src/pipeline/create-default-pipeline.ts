import { OxcParser } from "../parser/oxc-parser.js";
import { FlatSerializer } from "../serializer/flat/flat-serializer.js";
import { createDefaultEmitterRegistry } from "./create-default-emitter-registry.js";
import type { EmitterRegistry } from "./emit/emitter-registry.js";
import { createPipeline } from "./pipeline.js";
import type { Pipeline } from "./runner/pipeline.js";

export function createDefaultPipeline(emitters?: EmitterRegistry): Pipeline {
  return createPipeline({
    parser: new OxcParser(),
    serializer: new FlatSerializer(),
    emitters: emitters ?? createDefaultEmitterRegistry(),
  });
}
