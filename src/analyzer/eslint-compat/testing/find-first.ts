import { isNodeLike } from "../is-node-like.js";
import type { NodeLike } from "../node-like.js";

export function findFirst(root: NodeLike, type: string): NodeLike {
  const found = search(root, type);
  if (!found) {
    throw new Error(`No node with type ${type} found in subtree`);
  }
  return found;
}

function search(node: NodeLike, type: string): NodeLike | null {
  if (node.type === type) {
    return node;
  }
  for (const key of Object.keys(node)) {
    const value = node[key];
    if (Array.isArray(value)) {
      for (const child of value) {
        if (isNodeLike(child)) {
          const hit = search(child, type);
          if (hit) {
            return hit;
          }
        }
      }
    } else if (isNodeLike(value)) {
      const hit = search(value, type);
      if (hit) {
        return hit;
      }
    }
  }
  return null;
}
