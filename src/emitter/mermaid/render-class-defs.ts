export function renderClassDefs(
  wrapperIds: readonly string[],
  stubIds: readonly string[],
  lines: string[],
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
}
