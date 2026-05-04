import type { CommanderError } from "commander";

export function handleCommanderError(e: CommanderError): number {
  // commander already wrote help / version / error text to stdout/stderr
  // before throwing. Map exit codes: 0 for help/version, 2 for any other
  // CLI usage error (POSIX convention for misuse).
  if (e.code === "commander.helpDisplayed" || e.code === "commander.version") {
    return 0;
  }
  return 2;
}
