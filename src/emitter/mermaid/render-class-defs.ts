import type { ColorTheme } from "./theme/color-theme.js";

type NestClassMap = ReadonlyMap<number, readonly string[]>;

export function renderClassDefs(
  stubIds: readonly string[],
  varIds: readonly string[],
  nestClassMap: NestClassMap,
  theme: ColorTheme,
  lines: /* mutable */ string[],
): void {
  if (stubIds.length > 0) {
    const c = theme.boundaryStub;
    lines.push(
      `  classDef boundaryStub fill:${c.fill},stroke:${c.stroke},stroke-dasharray:${c.strokeDasharray},color:${c.color};`,
    );
    for (const id of stubIds) {
      lines.push(`  class ${id} boundaryStub;`);
    }
  }
  if (varIds.length > 0) {
    // var-declared Variable nodes carry no edges in the visual graph
    // (their references are filtered out upstream). Render the border
    // dashed so the reader does not mistake them for ordinary nodes that
    // happen to be unconnected.
    lines.push(
      `  classDef varNode stroke-dasharray:${theme.varNode.strokeDasharray};`,
    );
    for (const id of varIds) {
      lines.push(`  class ${id} varNode;`);
    }
  }
  emitNestClassDefs(nestClassMap, theme, lines);
}

function emitNestClassDefs(
  nestClassMap: NestClassMap,
  theme: ColorTheme,
  lines: /* mutable */ string[],
): void {
  const paletteLength = theme.nestPalette.length;
  if (paletteLength === 0) {
    return;
  }
  // Iterate slots in ascending palette order so the output ordering is
  // deterministic regardless of insertion order into the map.
  for (let slot = 0; slot < paletteLength; slot++) {
    const ids = nestClassMap.get(slot);
    if (ids === undefined || ids.length === 0) {
      continue;
    }
    // The class name uses 1-based palette slots so the user-facing class
    // matches the "depth" the user reasons about (L1 = outermost subgraph).
    const level = slot + 1;
    const c = theme.nestPalette[slot];
    if (c === undefined) {
      continue;
    }
    lines.push(`  classDef nestL${level} fill:${c.fill},stroke:${c.stroke};`);
    for (const id of ids) {
      lines.push(`  class ${id} nestL${level};`);
    }
  }
}
