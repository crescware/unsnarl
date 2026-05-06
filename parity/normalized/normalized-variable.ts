import type { NormalizedDefinition } from "./normalized-definition.js";

export type NormalizedVariable = Readonly<{
  name: string;
  defs: readonly NormalizedDefinition[];
  referenceCount: number;
}>;
