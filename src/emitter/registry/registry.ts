import type { EmitterRegistry } from "../../pipeline/emit/emitter-registry.js";
import type { Emitter } from "../../pipeline/emit/emitter.js";

export class DefaultEmitterRegistry implements EmitterRegistry {
  private readonly map = new Map<string, Emitter>();

  register(emitter: Emitter): void {
    if (this.map.has(emitter.format)) {
      throw new Error(`Duplicate emitter format: ${emitter.format}`);
    }
    this.map.set(emitter.format, emitter);
  }

  get(format: string): Emitter | undefined {
    return this.map.get(format);
  }

  list(): readonly string[] {
    return [...this.map.keys()];
  }
}
