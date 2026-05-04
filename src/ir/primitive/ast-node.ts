export type AstNode = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;
