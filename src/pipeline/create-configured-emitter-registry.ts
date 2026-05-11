import type { CliColorTheme } from "../cli-color-theme.js";
import type { CliMermaidRenderer } from "../cli-mermaid-renderer.js";
import { IrEmitter } from "../emitter/ir/ir.js";
import { JsonEmitter } from "../emitter/json/json.js";
import { MarkdownEmitter } from "../emitter/markdown/markdown.js";
import { MermaidEmitter } from "../emitter/mermaid/mermaid.js";
import { dagreStrategy } from "../emitter/mermaid/strategy/dagre-strategy.js";
import { elkStrategy } from "../emitter/mermaid/strategy/elk-strategy.js";
import type { MermaidStrategy } from "../emitter/mermaid/strategy/strategy.js";
import { COLOR_THEMES } from "../emitter/mermaid/theme/color-themes.js";
import { DefaultEmitterRegistry } from "../emitter/registry/registry.js";
import { StatsEmitter } from "../emitter/stats/stats.js";
import type { EmitterRegistry } from "./emit/emitter-registry.js";

type ConfiguredRegistryOptions = Readonly<{
  mermaidRenderer: CliMermaidRenderer;
  colorTheme: CliColorTheme;
}>;

const STRATEGIES = {
  dagre: dagreStrategy,
  elk: elkStrategy,
} as const satisfies Record<CliMermaidRenderer, MermaidStrategy>;

export function createConfiguredEmitterRegistry(
  options: ConfiguredRegistryOptions,
): EmitterRegistry {
  const reg = new DefaultEmitterRegistry();
  reg.register(new IrEmitter());
  reg.register(new JsonEmitter());
  const mermaid = new MermaidEmitter({
    strategy: STRATEGIES[options.mermaidRenderer],
    theme: COLOR_THEMES[options.colorTheme],
  });
  reg.register(mermaid);
  reg.register(new MarkdownEmitter(mermaid));
  reg.register(new StatsEmitter());
  return reg;
}
