export type ParsedRootQuery =
  | { readonly kind: "line"; readonly line: number; readonly raw: string }
  | {
      readonly kind: "line-name";
      readonly line: number;
      readonly name: string;
      readonly raw: string;
    }
  | {
      readonly kind: "range";
      readonly start: number;
      readonly end: number;
      readonly raw: string;
    }
  | {
      readonly kind: "range-name";
      readonly start: number;
      readonly end: number;
      readonly name: string;
      readonly raw: string;
    }
  | { readonly kind: "name"; readonly name: string; readonly raw: string };

export interface RootQueryParseSuccess {
  readonly ok: true;
  readonly queries: readonly ParsedRootQuery[];
}

export interface RootQueryParseFailure {
  readonly ok: false;
  readonly error: string;
}

export type RootQueryParseResult =
  | RootQueryParseSuccess
  | RootQueryParseFailure;

export function parseRootQuery(
  token: string,
): ParsedRootQuery | { error: string } {
  void token;
  return { error: "not implemented" };
}

export function parseRootQueries(value: string): RootQueryParseResult {
  void value;
  return { ok: false, error: "not implemented" };
}
