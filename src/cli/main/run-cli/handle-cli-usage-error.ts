import type { CliUsageError } from "./cli-usage-error.js";

export function handleCliUsageError(e: CliUsageError): number {
  process.stderr.write(`error: ${e.message}\n`);
  if (e.help !== null) {
    process.stderr.write(e.help);
  }
  return 2;
}
