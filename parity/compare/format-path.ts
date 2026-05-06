export function formatPath(path: readonly number[]): string {
  return path.length === 0 ? "root" : `root.${path.join(".")}`;
}
