export const CATEGORY = {
  Function: "function",
  If: "if",
  For: "for",
  While: "while",
  Switch: "switch",
  TryCatchFinally: "try-catch-finally",
  Block: "block",
} as const;
export type Category = (typeof CATEGORY)[keyof typeof CATEGORY];
