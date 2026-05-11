type NestPaletteEntry = Readonly<{
  fill: string;
  stroke: string;
}>;

export type ColorTheme = Readonly<{
  fnWrap: Readonly<{
    fill: string;
    stroke: string;
  }>;
  boundaryStub: Readonly<{
    fill: string;
    stroke: string;
    strokeDasharray: string;
    color: string;
  }>;
  varNode: Readonly<{
    strokeDasharray: string;
  }>;
  elkEmptyPlaceholder: Readonly<{
    fill: string;
    stroke: string;
    color: string;
  }>;
  // Cycled per subgraph depth (1-based). Empty is invalid: every theme
  // must provide at least one palette entry so cycling has a target.
  nestPalette: readonly NestPaletteEntry[];
}>;
