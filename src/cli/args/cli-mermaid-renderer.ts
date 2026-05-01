export type CliMermaidRenderer = "dagre" | "elk";

export const MERMAID_RENDERERS: ReadonlySet<string> = new Set([
  "dagre",
  "elk",
] satisfies CliMermaidRenderer[]);
