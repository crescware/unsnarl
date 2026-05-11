import type { RenderState } from "./render-state.js";
import { nestPaletteIndex } from "./theme/nest-palette-index.js";

/**
 * Append a subgraph id to its 1-based depth's palette slot in
 * RenderState.nestClassMap. Idempotent on empty palettes (no-op when
 * the theme defines an empty nest palette, which is invalid but
 * defended against to keep the renderer robust). Used by both the
 * function-wrapper and the plain-subgraph paths so they share the
 * same depth bookkeeping.
 */
export function recordNestSlot(
  state: RenderState,
  subgraphId: string,
  depth: number,
): void {
  const paletteLength = state.theme.nestPalette.length;
  if (paletteLength === 0) {
    return;
  }
  const slot = nestPaletteIndex(depth, paletteLength);
  const existing = state.nestClassMap.get(slot);
  if (existing === undefined) {
    state.nestClassMap.set(slot, [subgraphId]);
    return;
  }
  existing.push(subgraphId);
}
