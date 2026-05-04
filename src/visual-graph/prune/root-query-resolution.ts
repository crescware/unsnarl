export type RootQueryResolution = Readonly<{
  raw: string;
  line: number;
  name: string;
  resolvedAs: "name" | "line";
}>;
