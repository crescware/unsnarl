import type { NormalizedReference } from "./normalized-reference.js";
import type { NormalizedVariable } from "./normalized-variable.js";
import type { Range } from "./range.js";

export type NormalizedScope = Readonly<{
  type: string;
  isStrict: boolean;
  path: readonly number[];
  blockType: string;
  blockRange: Range;
  variables: readonly NormalizedVariable[];
  references: readonly NormalizedReference[];
  childScopes: readonly NormalizedScope[];
}>;
