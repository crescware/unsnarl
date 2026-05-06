export function nodeTypeOf(node: { type?: string } | null | undefined): string {
  return node?.type ?? "";
}
