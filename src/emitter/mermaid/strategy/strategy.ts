interface EmptySubgraphContext {
  /** Subgraph id about to close with no body content. */
  readonly subgraphId: string;
  /** Indent prefix to use for any line emitted inside the subgraph. */
  readonly indent: string;
  /** True if any edge in the graph references this subgraph as src or dst. */
  readonly referencedByEdge: boolean;
}

export interface MermaidStrategy {
  /** Lines emitted before `flowchart` (e.g. an `%%{init: ...}%%` directive). */
  readonly preambleLines: readonly string[];

  /**
   * Called when a subgraph is about to close with zero emitted children.
   * Return `{ line, placeholderId }` to insert a single line inside the
   * subgraph and register a placeholder id; return `null` to leave it empty.
   */
  emptySubgraphPlaceholder(
    ctx: EmptySubgraphContext,
  ): { line: string; placeholderId: string } | null;

  /**
   * Lines appended at the end of the diagram, after every node, edge and
   * other classDef. Receives every placeholder id produced during the run
   * so the strategy can attach a `classDef` / `class` styling block.
   */
  trailerLines(placeholderIds: readonly string[]): readonly string[];
}
