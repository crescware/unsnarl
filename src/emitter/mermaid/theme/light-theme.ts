import type { ColorTheme } from "./color-theme.js";

export const lightTheme: ColorTheme = {
  fnWrap: {
    fill: "#e0e8f0",
    stroke: "#5a7d99",
  },
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
  nestPalette: [
    { fill: "#f5f7fa", stroke: "#c8d2e0" },
    { fill: "#edf2f8", stroke: "#b8c5d6" },
    { fill: "#e5edf6", stroke: "#a8b8cc" },
    { fill: "#dde8f4", stroke: "#98abc2" },
  ],
};
