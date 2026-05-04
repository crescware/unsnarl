import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

export function writeOutput(outputPath: string | null, text: string): void {
  if (outputPath !== null) {
    mkdirSync(dirname(outputPath), { recursive: true });
    writeFileSync(outputPath, text);
    return;
  }

  process.stdout.write(text);
}
