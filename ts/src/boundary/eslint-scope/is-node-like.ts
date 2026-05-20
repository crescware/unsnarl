import type { NodeLike } from "./node-like.js";

export function isNodeLike(value: unknown): value is NodeLike {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
