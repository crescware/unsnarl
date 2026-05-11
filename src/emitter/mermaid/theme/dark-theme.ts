import type { ColorTheme } from "./color-theme.js";

export const darkTheme: ColorTheme = {
  fnWrap: {
    fill: "#1a2030",
    stroke: "#5a7d99",
  },
  boundaryStub: {
    fill: "transparent",
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
  nestPalette: [
    { fill: "#1e2738", stroke: "#3d4a63" },
    { fill: "#233045", stroke: "#475670" },
    { fill: "#283952", stroke: "#51637d" },
    { fill: "#2d425f", stroke: "#5b708a" },
  ],
};
