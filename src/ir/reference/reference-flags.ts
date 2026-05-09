export const ReferenceFlags = {
  None: 0,
  Read: 1 << 0,
  Write: 1 << 1,
} as const;

export type ReferenceFlagBits = number;
