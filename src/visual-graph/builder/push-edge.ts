import type { BuildState } from "./build-state.js";

export function pushEdge(
  state: BuildState,
  from: string,
  label: string,
  to: string,
): void {
  const key = `${from} -->|${label}| ${to}`;
  if (state.emittedEdges.has(key)) {
    return;
  }
  state.emittedEdges.add(key);
  state.edges.push({ from, to, label });
}
