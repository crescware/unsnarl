#!/usr/bin/env node
import { runCli } from "./main/run-cli.js";

const isMain = import.meta.url === `file://${process.argv[1] ?? ""}`;
if (isMain) {
  runCli(process.argv.slice(2))
    .then((code) => {
      process.exit(code);
    })
    .catch((e: unknown) => {
      process.stderr.write(
        `fatal: ${e instanceof Error ? e.message : String(e)}\n`,
      );
      process.exit(1);
    });
}
