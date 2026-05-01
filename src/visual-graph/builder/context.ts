import type {
  SerializedIR,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import type { WriteOp } from "./write-op.js";

export type BuilderContext = Readonly<{
  ir: SerializedIR;
  variableMap: Map<string, SerializedVariable>;
  scopeMap: Map<string, SerializedScope>;
  subgraphOwnerVar: Map<string, string>;
  hiddenVariables: Set<string>;
  writeOpsByVariable: Map<string, /* mutable */ WriteOp[]>;
  writeOpsByScope: Map<string, /* mutable */ WriteOp[]>;
  writeOpByRef: Map<string, WriteOp>;
  sortedCasesByContainer: Map<string, /* mutable */ SerializedScope[]>;
}>;
