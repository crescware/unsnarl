import type { CategoryDepths } from "../ir/annotations/scope-annotation.js";

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

export function makeDepths(value: number): CategoryDepths {
  return {
    [CATEGORY.Function]: value,
    [CATEGORY.If]: value,
    [CATEGORY.For]: value,
    [CATEGORY.While]: value,
    [CATEGORY.Switch]: value,
    [CATEGORY.TryCatchFinally]: value,
    [CATEGORY.Block]: value,
  };
}
