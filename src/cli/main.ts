import { runCli } from "./main/run-cli/run-cli.js";

export async function main(argv: readonly string[]): Promise<number> {
  try {
    return await runCli(argv);
  } catch (e) {
    process.stderr.write(
      `fatal: ${e instanceof Error ? e.message : String(e)}\n`,
    );
    return 1;
  }
}
