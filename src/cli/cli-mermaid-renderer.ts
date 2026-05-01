export const CLI_MERMAID_RENDERER = {
  Dagre: "dagre",
  Elk: "elk",
} as const;
export type CliMermaidRenderer =
  (typeof CLI_MERMAID_RENDERER)[keyof typeof CLI_MERMAID_RENDERER];
