import type { InferOutput } from "valibot";

import type { Span } from "../../../ir/primitive/span.js";
import {
  normal$,
  return$,
  throw$,
} from "../../../ir/reference/completion-kind.js";
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
  kind: InferOutput<typeof return$>;
  startSpan: Span;
  endSpan: Span;
}>;

export type SerializedThrowCompletion = Readonly<{
  kind: InferOutput<typeof throw$>;
  startSpan: Span;
  endSpan: Span;
}>;

export type SerializedNormalCompletion = Readonly<{
  kind: InferOutput<typeof normal$>;
}>;

export function normalCompletion(): SerializedNormalCompletion {
  return { kind: normal$.literal };
}

export function returnCompletion(
  startOffset: number,
  endOffset: number,
  startLine = 1,
  endLine = startLine,
): SerializedReturnCompletion {
  return {
    kind: return$.literal,
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
    kind: throw$.literal,
    startSpan: span(startOffset, startLine),
    endSpan: span(endOffset, endLine),
  };
}
