import { picklist, type InferOutput } from "valibot";

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

const nestingKind$ = picklist([
  NESTING_KIND.Function,
  NESTING_KIND.If,
  NESTING_KIND.For,
  NESTING_KIND.While,
  NESTING_KIND.Switch,
  NESTING_KIND.TryCatchFinally,
  NESTING_KIND.Block,
]);

export type NestingKind = InferOutput<typeof nestingKind$>;

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
