export const ReferenceFlags = {
  None: 0,
  Read: 1 << 0,
  Write: 1 << 1,
  Call: 1 << 2,
  Receiver: 1 << 3,
} as const;

export type ReferenceFlagBits = number;
