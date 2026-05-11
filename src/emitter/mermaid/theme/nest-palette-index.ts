/**
 * Maps a 1-based subgraph depth to a 0-based palette index by cycling. A
 * palette of length N covers depths 1..N as palette[0]..palette[N-1], then
 * depth N+1 wraps back to palette[0]. Depth must be >= 1; palette must be
 * non-empty.
 */
export function nestPaletteIndex(depth: number, paletteLength: number): number {
  if (paletteLength <= 0) {
    throw new Error("paletteLength must be > 0");
  }
  if (depth < 1) {
    throw new Error("depth must be >= 1");
  }
  return (depth - 1) % paletteLength;
}
