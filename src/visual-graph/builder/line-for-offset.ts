export function lineForOffset(raw: string, offset: number): number {
  let line = 1;
  const limit = Math.min(offset, raw.length);
  for (let i = 0; i < limit; i++) {
    if (raw.charCodeAt(i) === 10) {
      line += 1;
    }
  }
  return line;
}
