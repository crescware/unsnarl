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
  While: "while",
  DoWhile: "do-while",
  Return: "return",
  Block: "block",
} as const;
export type SubgraphKind = (typeof SUBGRAPH_KIND)[keyof typeof SUBGRAPH_KIND];
