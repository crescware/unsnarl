import type { Range } from "./range.js";

export type NormalizedDefinition = Readonly<{
  type: string;
  nameRange: Range;
  nodeType: string;
  nodeRange: Range;
}>;
