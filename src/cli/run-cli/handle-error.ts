export function handleError(e: Error): number {
  process.stderr.write(`error: ${e.message}\n`);
  return 1;
}
