import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";

export function edgeLabelOfRef(r: SerializedReference): string {
  const parts: /* mutable */ string[] = [];
  if (r.flags.read) {
    parts.push("read");
  }
  if (r.flags.write) {
    parts.push("write");
  }
  if (r.flags.call) {
    parts.push("call");
  }
  return parts.length > 0 ? parts.join(",") : "ref";
}
