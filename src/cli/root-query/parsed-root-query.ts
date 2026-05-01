export type ParsedRootQuery =
  | Readonly<{ kind: "line"; line: number; raw: string }>
  | Readonly<{
      kind: "line-name";
      line: number;
      name: string;
      raw: string;
    }>
  | Readonly<{
      kind: "range";
      start: number;
      end: number;
      raw: string;
    }>
  | Readonly<{
      kind: "range-name";
      start: number;
      end: number;
      name: string;
      raw: string;
    }>
  | Readonly<{ kind: "name"; name: string; raw: string }>;
