export type NormalizedReferenceFlags = Readonly<{
  read: boolean;
  write: boolean;
  call: boolean;
  receiver: boolean;
}>;
