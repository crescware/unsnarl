import { EslintCompatAnalyzer } from "../analyzer/eslint-compat/eslint-compat.js";
import { CLI_MERMAID_RENDERER } from "../cli/cli-mermaid-renderer.js";
import { IrEmitter } from "../emitter/ir/ir.js";
import { JsonEmitter } from "../emitter/json/json.js";
import { MarkdownEmitter } from "../emitter/markdown/markdown.js";
import {
  MermaidEmitter,
  type MermaidRenderer,
} from "../emitter/mermaid/mermaid.js";
import { dagreStrategy } from "../emitter/mermaid/strategy/dagre-strategy.js";
import { elkStrategy } from "../emitter/mermaid/strategy/elk-strategy.js";
import type { MermaidStrategy } from "../emitter/mermaid/strategy/strategy.js";
import { DefaultEmitterRegistry } from "../emitter/registry/registry.js";
import { StatsEmitter } from "../emitter/stats/stats.js";
import { OxcParser } from "../parser/oxc.js";
import { FlatSerializer } from "../serializer/flat/flat-serializer.js";
import { createPipeline } from "./pipeline.js";
import type { EmitterRegistry, Pipeline } from "./types.js";

export type DefaultRegistryOptions = Readonly<{
  mermaidRenderer: MermaidRenderer;
}>;

const STRATEGIES = {
  dagre: dagreStrategy,
  elk: elkStrategy,
} as const satisfies Record<MermaidRenderer, MermaidStrategy>;

export function createDefaultEmitterRegistry(
  options: DefaultRegistryOptions,
): EmitterRegistry {
  const reg = new DefaultEmitterRegistry();
  reg.register(new IrEmitter());
  reg.register(new JsonEmitter());
  const mermaid = new MermaidEmitter({
    strategy: STRATEGIES[options.mermaidRenderer],
  });
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
      emitters ??
      createDefaultEmitterRegistry({
        mermaidRenderer: CLI_MERMAID_RENDERER.Elk,
      }),
  });
}
