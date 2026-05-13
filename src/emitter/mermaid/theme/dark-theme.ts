import type { ColorTheme } from "./color-theme.js";

export const darkTheme: ColorTheme = {
  boundaryStub: {
    stroke: "#888",
    strokeDasharray: "3 3",
    color: "#888",
  },
  varNode: {
    strokeDasharray: "5 5",
  },
  elkEmptyPlaceholder: {
    fill: "transparent",
    stroke: "transparent",
    color: "transparent",
  },
  // Six-entry gradient kept in the dark half of the spectrum so the
  // diagram stays comfortable on a dark page and white label text
  // retains a WCAG AA contrast ratio against every fill. The top
  // entry sits at #3f5175 (luminance ~0.084, ~7.8:1 against white)
  // rather than reaching into the light range. Stroke is transparent
  // in the built-in themes: the fill gradient alone communicates the
  // nesting, and stroke rings would add visual noise.
  nestPalette: [
    { fill: "#11192a", stroke: "transparent" },
    { fill: "#1a2538", stroke: "transparent" },
    { fill: "#243047", stroke: "transparent" },
    { fill: "#2d3b57", stroke: "transparent" },
    { fill: "#364666", stroke: "transparent" },
    { fill: "#3f5175", stroke: "transparent" },
  ],
  // Pure yellow fill keeps the highlight unmistakable on a dark page;
  // black text re-establishes contrast against that fill. Matching
  // edgeStroke keeps the connecting arrows visually tied to the node.
  highlight: {
    fill: "#facc15",
    stroke: "#facc15",
    color: "#0a0a0a",
    edgeStroke: "#facc15",
    edgeStrokeWidth: "2px",
  },
};
