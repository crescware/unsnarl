import { InvalidArgumentError, Option } from "commander";

import {
  type CliMermaidRenderer,
  MERMAID_RENDERERS,
} from "./cli-mermaid-renderer.js";

function coerceMermaidRenderer(value: string): CliMermaidRenderer {
  if (!MERMAID_RENDERERS.has(value)) {
    throw new InvalidArgumentError(`Invalid mermaid renderer: ${value}`);
  }
  return value as CliMermaidRenderer;
}

export function mermaidRendererOptions(): readonly Option[] {
  return [
    new Option(
      "--mermaid-renderer <renderer>",
      "Layout engine for Mermaid output",
    ).argParser(coerceMermaidRenderer),
  ];
}
