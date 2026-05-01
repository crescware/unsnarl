export function intermediateKey(source: string, originalName: string): string {
  return `${source}::${originalName}`;
}
