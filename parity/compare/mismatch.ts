import type { MismatchKind } from "./mismatch-kind.js";

export type Mismatch = Readonly<{
  kind: MismatchKind;
  scopePath: readonly number[];
  message: string;
}>;
