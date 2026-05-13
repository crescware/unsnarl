import {
  array,
  looseObject,
  nonEmpty,
  parse,
  pipe,
  safeParse,
  unknown,
} from "valibot";

import type { SerializedDefinition } from "./serialized-definition.js";
import type { SerializedVariable } from "./serialized-variable.js";

// The boundary maintains `defs.length === 0 ⟺ identifiers.length === 0`
// (the only producer of empty defs is declareImplicitArguments). Code paths
// that render a Variable as a graph node must narrow through hasDef /
// assertHasDef first; the implicit-arguments case never satisfies them.
type SerializedVariableWithDef = SerializedVariable & {
  defs: readonly [SerializedDefinition, ...SerializedDefinition[]];
};

const VariableWithDefSchema = looseObject({
  defs: pipe(array(unknown()), nonEmpty("Variable.defs must be non-empty")),
});

export function hasDef(
  value: SerializedVariable,
): value is SerializedVariableWithDef {
  return safeParse(VariableWithDefSchema, value).success;
}

export function assertHasDef(
  value: SerializedVariable,
): asserts value is SerializedVariableWithDef {
  parse(VariableWithDefSchema, value);
}
