export type NodeLike = Readonly<{
  type: string;
  start?: number;
  end?: number;
  [key: string]: unknown;
}>;

export function isNodeLike(value: unknown): value is NodeLike {
  return (
    value !== null &&
    typeof value === "object" &&
    "type" in value &&
    typeof (value as { type: unknown }).type === "string"
  );
}
