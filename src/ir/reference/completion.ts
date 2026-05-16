import type { InferOutput } from "valibot";

import type { normal$, return$, throw$ } from "./completion-kind.js";

/**
 * Reference の値がどの ECMAScript Completion 種別の [[Value]] として
 * 産出されるかを表す annotation-side (offset-based) 型。
 *
 * ECMA §6.2.4 では Completion Record の [[Type]] は
 *   normal | break | continue | return | throw
 * の 5 種と定義され、abrupt completion はこのうち normal 以外の 4 種を
 * 指す（"A Completion Record whose [[Type]] is `normal` is called a
 * normal completion. Every Completion Record other than a normal
 * completion is also known as an abrupt completion."）。
 *
 * unsnarl は Reference 単位で「この read の値はどの completion に
 * 運ばれるか」を追跡するため、本型は normal / abrupt を直接の union
 * として表現する。null は使用しない (total)。
 *
 * 旧設計 (`returnContainer | null` と `throwContainer | null` の独立
 * 2 フィールド) は 4 通りの状態を表現可能だったが、AST 文法上
 * 「両方 non-null」は到達不能であり、型に不変条件が欠けていた。
 * 本型はその不変条件を構造として持つ。
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type Completion =
  | AbruptCompletion
  | Readonly<{ kind: InferOutput<typeof normal$> }>;

/**
 * ECMA §6.2.4 が定義する abrupt completion (break | continue | return |
 * throw の 4 種) のうち、Reference の値が運ばれ得るものだけをモデル化
 * する。
 *
 * break / continue は構文上 argument に Expression を取らない
 * （optional label の Identifier のみ）。eslint-scope では label は
 * Label 扱いで Reference ではないため、Reference の値が break /
 * continue の completion に運ばれることは構文上発生し得ない。よって
 * 本型は return / throw の 2 variant に限定する。
 *
 * 将来 JS 文法に新たな value-carrying abrupt completion が追加された
 * 場合のみ、この union を拡張する。
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export type AbruptCompletion =
  | Readonly<{
      kind: InferOutput<typeof return$>;
      startOffset: number;
      endOffset: number;
    }>
  | Readonly<{
      kind: InferOutput<typeof throw$>;
      startOffset: number;
      endOffset: number;
    }>;
