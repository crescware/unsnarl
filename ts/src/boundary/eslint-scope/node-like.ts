// Raw AST node shape that flows through the eslint-scope-parity traversal.
// `type` is intentionally `string`, not `AstType`, because this is the
// unnormalized boundary input -- any string the parser produces must
// round-trip through this type before the first internal consumer maps
// it onto the enumerated `AstType` vocabulary via `asAstType()`. Tightening
// this field would force the boundary to normalize eagerly and would
// defeat the point of the sentinel.
export type NodeLike = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;
