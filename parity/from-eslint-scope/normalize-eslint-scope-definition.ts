import type { Definition } from "eslint-scope";

import type { NormalizedDefinition } from "../normalized/normalized-definition.js";
import { nodeTypeOf } from "../util/node-type-of.js";
import { rangeOf } from "../util/range-of.js";

export function normalizeEslintScopeDefinition(
  def: Definition,
): NormalizedDefinition {
  return {
    type: def.type,
    nameRange: rangeOf(def.name as { start?: number; end?: number }),
    nodeType: nodeTypeOf(def.node as { type?: string }),
    nodeRange: rangeOf(def.node as { start?: number; end?: number }),
  };
}
