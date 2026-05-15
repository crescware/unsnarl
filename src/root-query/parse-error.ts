type ParseError = Readonly<{
  message: string;
}>;

export type ParseResult<T> =
  | Readonly<{ ok: true; value: T }>
  | Readonly<{ ok: false; errors: readonly ParseError[] }>;
