import type { Span } from "../../../ir/primitive/span.js";
import { span } from "./span.js";

/**
 * Serialized completion shape (with spans) を生成するテストヘルパー。
 * ECMA §6.2.4 の Completion Record の Reference annotation 投影に対応。
 *
 * 命名は仕様の各 Completion 種別（NormalCompletion / ReturnCompletion /
 * ThrowCompletion）に揃える。
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */

export type SerializedReturnCompletion = Readonly<{
  kind: "return";
  startSpan: Span;
  endSpan: Span;
}>;

export type SerializedThrowCompletion = Readonly<{
  kind: "throw";
  startSpan: Span;
  endSpan: Span;
}>;

export type SerializedNormalCompletion = Readonly<{ kind: "normal" }>;

export function normalCompletion(): SerializedNormalCompletion {
  return { kind: "normal" };
}

export function returnCompletion(
  startOffset: number,
  endOffset: number,
  startLine = 1,
  endLine = startLine,
): SerializedReturnCompletion {
  return {
    kind: "return",
    startSpan: span(startOffset, startLine),
    endSpan: span(endOffset, endLine),
  };
}

export function throwCompletion(
  startOffset: number,
  endOffset: number,
  startLine = 1,
  endLine = startLine,
): SerializedThrowCompletion {
  return {
    kind: "throw",
    startSpan: span(startOffset, startLine),
    endSpan: span(endOffset, endLine),
  };
}
