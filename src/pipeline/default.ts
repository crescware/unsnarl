import { EslintCompatAnalyzer } from "../analyzer/eslint-compat.js";
import { IrEmitter } from "../emitter/ir.js";
import { JsonEmitter } from "../emitter/json.js";
import { MarkdownEmitter } from "../emitter/markdown.js";
import { MermaidEmitter, type MermaidRenderer } from "../emitter/mermaid.js";
import { DefaultEmitterRegistry } from "../emitter/registry.js";
import { StatsEmitter } from "../emitter/stats.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat.js";
import { createPipeline } from "./pipeline.js";
import type { EmitterRegistry, Pipeline } from "./types.js";

export interface DefaultRegistryOptions {
  mermaidRenderer: MermaidRenderer;
}

export function createDefaultEmitterRegistry(
  options: DefaultRegistryOptions,
): EmitterRegistry {
  const reg = new DefaultEmitterRegistry();
  reg.register(new IrEmitter());
  reg.register(new JsonEmitter());
  const mermaid = new MermaidEmitter({ renderer: options.mermaidRenderer });
  reg.register(mermaid);
  reg.register(new MarkdownEmitter(mermaid));
  reg.register(new StatsEmitter());
  return reg;
}

export function createDefaultPipeline(emitters?: EmitterRegistry): Pipeline {
  return createPipeline({
    parser: new OxcParser(),
    analyzer: new EslintCompatAnalyzer(),
    serializer: new FlatSerializer(),
    emitters:
      emitters ?? createDefaultEmitterRegistry({ mermaidRenderer: "elk" }),
  });
}
