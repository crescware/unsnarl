import { CLI_COLOR_THEME } from "../cli-color-theme.js";
import { CLI_MERMAID_RENDERER } from "../cli-mermaid-renderer.js";
import { createConfiguredEmitterRegistry } from "./create-configured-emitter-registry.js";
import type { EmitterRegistry } from "./emit/emitter-registry.js";

export function createDefaultEmitterRegistry(): EmitterRegistry {
  return createConfiguredEmitterRegistry({
    mermaidRenderer: CLI_MERMAID_RENDERER.Elk,
    colorTheme: CLI_COLOR_THEME.Dark,
  });
}
