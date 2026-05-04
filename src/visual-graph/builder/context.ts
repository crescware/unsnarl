import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import type { WriteOp } from "./write-op.js";

export type BuilderContext = Readonly<{
  ir: SerializedIR;
  variableMap: Map<string, SerializedVariable>;
  scopeMap: Map<string, SerializedScope>;
  subgraphOwnerVar: Map<string, string>;
  writeOpsByVariable: Map<string, /* mutable */ WriteOp[]>;
  writeOpsByScope: Map<string, /* mutable */ WriteOp[]>;
  writeOpByRef: Map<string, WriteOp>;
  sortedCasesByContainer: Map<string, /* mutable */ SerializedScope[]>;
}>;
