export type NodeLike = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;
