import {
  CLI_MERMAID_RENDERER,
  type CliMermaidRenderer,
} from "../cli/cli-mermaid-renderer.js";
import { IrEmitter } from "../emitter/ir/ir.js";
import { JsonEmitter } from "../emitter/json/json.js";
import { MarkdownEmitter } from "../emitter/markdown/markdown.js";
import { MermaidEmitter } from "../emitter/mermaid/mermaid.js";
import { dagreStrategy } from "../emitter/mermaid/strategy/dagre-strategy.js";
import { elkStrategy } from "../emitter/mermaid/strategy/elk-strategy.js";
import type { MermaidStrategy } from "../emitter/mermaid/strategy/strategy.js";
import { DefaultEmitterRegistry } from "../emitter/registry/registry.js";
import { StatsEmitter } from "../emitter/stats/stats.js";
import type { EmitterRegistry } from "./emit/emitter-registry.js";

type DefaultRegistryOptions = Readonly<{
  mermaidRenderer: CliMermaidRenderer;
}>;

const STRATEGIES = {
  dagre: dagreStrategy,
  elk: elkStrategy,
} as const satisfies Record<CliMermaidRenderer, MermaidStrategy>;

function createConfiguredEmitterRegistry(
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

export function createDefaultEmitterRegistry(): EmitterRegistry {
  return createConfiguredEmitterRegistry({
    mermaidRenderer: CLI_MERMAID_RENDERER.Elk,
  });
}
