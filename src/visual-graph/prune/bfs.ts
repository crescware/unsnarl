export function bfs(
  starts: ReadonlySet<string>,
  adj: ReadonlyMap<string, readonly string[]>,
  maxDepth: number,
): Set<string> {
  const reached = new Set<string>(starts);
  if (maxDepth <= 0) {
    return reached;
  }
  let frontier = new Set<string>(starts);
  for (let depth = 0; depth < maxDepth && frontier.size > 0; depth++) {
    const next = new Set<string>();
    for (const id of frontier) {
      const neighbors = adj.get(id);
      if (neighbors === undefined) {
        continue;
      }
      for (const n of neighbors) {
        if (!reached.has(n)) {
          reached.add(n);
          next.add(n);
        }
      }
    }
    frontier = next;
  }
  return reached;
}
