import { describe, expect, test } from "vitest";

import { renderHighlight } from "./render-highlight.js";
import { darkTheme } from "./theme/dark-theme.js";

describe("renderHighlight", () => {
  test("writes nothing when no ids are highlighted", () => {
    const lines: /* mutable */ string[] = [];
    renderHighlight(new Set<string>(), [], darkTheme, lines);
    expect(lines).toEqual([]);
  });

  test("emits one `style` line per highlighted node", () => {
    const lines: /* mutable */ string[] = [];
    renderHighlight(new Set(["n_a"]), [], darkTheme, lines);
    expect(lines).toEqual([
      `  style n_a fill:${darkTheme.highlight.fill},stroke:${darkTheme.highlight.stroke},color:${darkTheme.highlight.color};`,
    ]);
  });

  test("emits a single linkStyle line covering every highlighted edge index", () => {
    const lines: /* mutable */ string[] = [];
    renderHighlight(new Set(["n_a"]), [0, 2, 3], darkTheme, lines);
    expect(lines.at(-1)).toEqual(
      `  linkStyle 0,2,3 stroke:${darkTheme.highlight.edgeStroke},stroke-width:${darkTheme.highlight.edgeStrokeWidth};`,
    );
  });

  test("skips the linkStyle line when no edge indices are supplied", () => {
    const lines: /* mutable */ string[] = [];
    renderHighlight(new Set(["n_a"]), [], darkTheme, lines);
    expect(lines.some((v) => v.startsWith("  linkStyle"))).toEqual(false);
  });
});
