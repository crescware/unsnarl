export const IMPORT_KIND = {
  Default: "default",
  Named: "named",
  Namespace: "namespace",
} as const;
export type ImportKind = (typeof IMPORT_KIND)[keyof typeof IMPORT_KIND];
