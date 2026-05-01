import type { PredicateContainer } from "../ir/model.js";
import type { PathEntry } from "./walk/walk.js";

export function findPredicateContainer(
  parent: { type: string; start?: number } | null,
  key: string | null,
  path: ReadonlyArray<PathEntry>,
): PredicateContainer | null {
  let curKey: string | null = key;
  for (let i = path.length - 1; i >= 0; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const type = entry.node.type;
    if (type === "IfStatement" && curKey === "test") {
      return { type: "IfStatement", offset: entry.node.start ?? 0 };
    }
    if (type === "SwitchStatement" && curKey === "discriminant") {
      return { type: "SwitchStatement", offset: entry.node.start ?? 0 };
    }
    curKey = entry.key;
  }
  if (parent && key === "test" && parent.type === "IfStatement") {
    return { type: "IfStatement", offset: parent.start ?? 0 };
  }
  if (parent && key === "discriminant" && parent.type === "SwitchStatement") {
    return { type: "SwitchStatement", offset: parent.start ?? 0 };
  }
  return null;
}
