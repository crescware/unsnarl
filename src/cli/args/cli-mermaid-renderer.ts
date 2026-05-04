import { CLI_MERMAID_RENDERER } from "../cli-mermaid-renderer.js";

export const MERMAID_RENDERERS: ReadonlySet<string> = new Set(
  Object.values(CLI_MERMAID_RENDERER),
);
