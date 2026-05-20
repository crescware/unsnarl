import { randomBytes } from "node:crypto";

// Returns a fresh name whose text carries no literal meaning. Use this
// when a test must hold for any binding name and the specific characters
// of the chosen name are not part of the assertion. The result is always
// a valid JavaScript identifier (letter prefix + hex tail).
export function freshName(): string {
  return `v${randomBytes(8).toString("hex")}`;
}
