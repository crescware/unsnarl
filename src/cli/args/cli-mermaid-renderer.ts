import { CLI_MERMAID_RENDERER } from "../../constants.js";

export type { CliMermaidRenderer } from "../../constants.js";

export const MERMAID_RENDERERS: ReadonlySet<string> = new Set(
  Object.values(CLI_MERMAID_RENDERER),
);
