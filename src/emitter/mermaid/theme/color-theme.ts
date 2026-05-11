type NestPaletteEntry = Readonly<{
  fill: string;
  stroke: string;
}>;

export type ColorTheme = Readonly<{
  // BoundaryStub deliberately omits `fill`: the stub should pick up
  // the same background that regular Mermaid nodes use so the dashed
  // circle does not "float" against its parent subgraph fill.
  boundaryStub: Readonly<{
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
  // Function wrappers (`wrap_*`) and their body subgraphs share the
  // same depth slot, so each function reads as a single solid color
  // rather than two stacked colors.
  nestPalette: readonly NestPaletteEntry[];
}>;
