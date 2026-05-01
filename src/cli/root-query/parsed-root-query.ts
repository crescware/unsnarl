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
