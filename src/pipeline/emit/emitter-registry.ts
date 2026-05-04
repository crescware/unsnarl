import type { Emitter } from "./emitter.js";

export type EmitterRegistry = Readonly<{
  register(emitter: Emitter): void;
  get(format: string): Emitter | undefined;
  list(): readonly string[];
}>;
