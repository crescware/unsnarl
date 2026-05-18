import type { ColorTheme } from "./theme/color-theme.js";

// Emit per-node inline styles and the global `linkStyle` so the
// renderer paints highlighted nodes yellow and any edges touching them
// in the matching colour. Inline `style` wins against `classDef`
// declarations applied via `class`, which is why we don't try to define
// a "highlightNode" class -- a class would lose to anything mermaid
// applied earlier in the diagram source.
export function renderHighlight(
  highlightIds: ReadonlySet<string>,
  highlightEdgeIndices: readonly number[],
  theme: ColorTheme,
  lines: /* mutable */ string[],
): void {
  if (highlightIds.size === 0) {
    return;
  }
  const h = theme.highlight;
  for (const id of highlightIds) {
    lines.push(
      `  style ${id} fill:${h.fill},stroke:${h.stroke},color:${h.color};`,
    );
  }
  if (highlightEdgeIndices.length > 0) {
    const list = highlightEdgeIndices.join(",");
    lines.push(
      `  linkStyle ${list} stroke:${h.edgeStroke},stroke-width:${h.edgeStrokeWidth};`,
    );
  }
}
