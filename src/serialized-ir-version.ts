// SerializedIR.version is a numeric discriminator: bump it every time the
// on-disk shape changes and consumers can switch on it.
export const SERIALIZED_IR_VERSION = 1;
export type SerializedIRVersion = typeof SERIALIZED_IR_VERSION;
