export type VisualGraphPruning = Readonly<{
  roots: readonly Readonly<{ query: string; matched: number }>[];
  descendants: number;
  ancestors: number;
}>;
