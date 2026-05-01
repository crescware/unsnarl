export function parseGenerationCount(value: string): number | null {
  if (!/^\d+$/.test(value)) {
    return null;
  }
  return Number.parseInt(value, 10);
}
