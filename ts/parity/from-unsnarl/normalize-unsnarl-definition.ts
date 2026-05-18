import type { Definition } from "../../src/ir/scope/definition.js";
import type { NormalizedDefinition } from "../normalized/normalized-definition.js";
import { nodeTypeOf } from "../util/node-type-of.js";
import { rangeOf } from "../util/range-of.js";

export function normalizeUnsnarlDefinition(
  def: Definition,
): NormalizedDefinition {
  return {
    type: def.type,
    nameRange: rangeOf(def.name),
    nodeType: nodeTypeOf(def.node),
    nodeRange: rangeOf(def.node),
  };
}
