import { CLI_MERMAID_RENDERER } from "../cli-mermaid-renderer.js";
import { createConfiguredEmitterRegistry } from "./create-configured-emitter-registry.js";
import type { EmitterRegistry } from "./emit/emitter-registry.js";

export function createDefaultEmitterRegistry(): EmitterRegistry {
  return createConfiguredEmitterRegistry({
    mermaidRenderer: CLI_MERMAID_RENDERER.Elk,
  });
}
