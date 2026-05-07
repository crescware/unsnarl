import type { NestingDepths } from "../ir/annotations/scope-annotation.js";

export const NESTING_KIND = {
  Function: "function",
  If: "if",
  For: "for",
  While: "while",
  Switch: "switch",
  TryCatchFinally: "try-catch-finally",
  Block: "block",
} as const;
export type NestingKind = (typeof NESTING_KIND)[keyof typeof NESTING_KIND];

export function uniformNestingDepths(value: number): NestingDepths {
  return {
    [NESTING_KIND.Function]: value,
    [NESTING_KIND.If]: value,
    [NESTING_KIND.For]: value,
    [NESTING_KIND.While]: value,
    [NESTING_KIND.Switch]: value,
    [NESTING_KIND.TryCatchFinally]: value,
    [NESTING_KIND.Block]: value,
  };
}
