import { InvalidArgumentError, Option } from "commander";

import { createDefaultEmitterRegistry } from "../../pipeline/create-default-emitter-registry.js";
import type { CliMermaidRenderer } from "../cli-mermaid-renderer.js";
import { MERMAID_RENDERERS } from "./cli-mermaid-renderer.js";

function coerceMermaidRenderer(value: string): CliMermaidRenderer {
  if (!MERMAID_RENDERERS.has(value)) {
    throw new InvalidArgumentError(`Invalid mermaid renderer: ${value}`);
  }
  return value as CliMermaidRenderer;
}

export function formatOptions(): readonly Option[] {
  const availableFormats = createDefaultEmitterRegistry().list();
  return [
    new Option(
      "-f, --format <id>",
      `Emitter format (${availableFormats.join(", ")})`,
    ).default("mermaid"),
    new Option("--no-pretty-json", "Disable pretty-printed JSON output"),
    new Option(
      "--mermaid-renderer <renderer>",
      "Layout engine for Mermaid output",
    )
      .argParser(coerceMermaidRenderer)
      .default(null, "auto"),
  ];
}
