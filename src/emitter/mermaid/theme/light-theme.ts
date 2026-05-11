import type { ColorTheme } from "./color-theme.js";

export const lightTheme: ColorTheme = {
  boundaryStub: {
    fill: "transparent",
    stroke: "#555",
    strokeDasharray: "3 3",
    color: "#555",
  },
  varNode: {
    strokeDasharray: "5 5",
  },
  elkEmptyPlaceholder: {
    fill: "transparent",
    stroke: "transparent",
    color: "transparent",
  },
  // Six-entry gradient mirroring the dark theme's compression: kept
  // in the light half of the spectrum so dark label text retains a
  // strong contrast ratio against every fill. Stroke is transparent
  // in the built-in themes.
  nestPalette: [
    { fill: "#f4f7fb", stroke: "transparent" },
    { fill: "#e8eff7", stroke: "transparent" },
    { fill: "#dce6f3", stroke: "transparent" },
    { fill: "#d1ddef", stroke: "transparent" },
    { fill: "#c5d4eb", stroke: "transparent" },
    { fill: "#b9cbe7", stroke: "transparent" },
  ],
};
