import type { SerializedScope } from "../../ir/model.js";
import { sanitize } from "./sanitize.js";

export function subgraphScopeId(scope: SerializedScope): string {
  return `s_${sanitize(scope.id)}`;
}
