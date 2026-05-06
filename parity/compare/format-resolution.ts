import type { NormalizedResolution } from "../normalized/normalized-resolution.js";
import { formatPath } from "./format-path.js";

export function formatResolution(r: NormalizedResolution | null): string {
  return r === null ? "unresolved" : `${formatPath(r.scopePath)}#${r.varName}`;
}
