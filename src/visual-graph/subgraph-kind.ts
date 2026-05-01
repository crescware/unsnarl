export const SUBGRAPH_KIND = {
  Function: "function",
  Switch: "switch",
  Case: "case",
  If: "if",
  Else: "else",
  IfElseContainer: "if-else-container",
  Try: "try",
  Catch: "catch",
  Finally: "finally",
  For: "for",
  Return: "return",
} as const;
export type SubgraphKind = (typeof SUBGRAPH_KIND)[keyof typeof SUBGRAPH_KIND];
