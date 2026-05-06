export function renderClassDefs(
  wrapperIds: readonly string[],
  stubIds: readonly string[],
  varIds: readonly string[],
  lines: /* mutable */ string[],
): void {
  if (wrapperIds.length > 0) {
    // Distinct background so the function wrapper is visually separable
    // from the inner function body subgraph (otherwise both inherit the
    // same Mermaid cluster fill and the nesting becomes invisible).
    lines.push("  classDef fnWrap fill:#1a2030,stroke:#5a7d99;");
    for (const id of wrapperIds) {
      lines.push(`  class ${id} fnWrap;`);
    }
  }
  if (stubIds.length > 0) {
    lines.push(
      "  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;",
    );
    for (const id of stubIds) {
      lines.push(`  class ${id} boundaryStub;`);
    }
  }
  if (varIds.length > 0) {
    // var-declared Variable nodes carry no edges in the visual graph
    // (their references are filtered out upstream). Render the border
    // dashed so the reader does not mistake them for ordinary nodes that
    // happen to be unconnected.
    lines.push("  classDef varNode stroke-dasharray:5 5;");
    for (const id of varIds) {
      lines.push(`  class ${id} varNode;`);
    }
  }
}
