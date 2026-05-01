export type NodeLike = {
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
};
