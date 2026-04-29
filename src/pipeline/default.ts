import { EslintCompatAnalyzer } from "../analyzer/eslint-compat.js";
import { IrEmitter } from "../emitter/ir.js";
import { JsonEmitter } from "../emitter/json.js";
import { MarkdownEmitter } from "../emitter/markdown.js";
import { MermaidEmitter } from "../emitter/mermaid.js";
import { DefaultEmitterRegistry } from "../emitter/registry.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat.js";
import { createPipeline } from "./pipeline.js";
import type { EmitterRegistry, Pipeline } from "./types.js";

export function createDefaultEmitterRegistry(): EmitterRegistry {
  const reg = new DefaultEmitterRegistry();
  reg.register(new IrEmitter());
  reg.register(new JsonEmitter());
  reg.register(new MermaidEmitter());
  reg.register(new MarkdownEmitter());
  return reg;
}

export function createDefaultPipeline(emitters?: EmitterRegistry): Pipeline {
  return createPipeline({
    parser: new OxcParser(),
    analyzer: new EslintCompatAnalyzer(),
    serializer: new FlatSerializer(),
    emitters: emitters ?? createDefaultEmitterRegistry(),
  });
}
