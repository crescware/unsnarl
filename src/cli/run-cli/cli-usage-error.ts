export class CliUsageError extends Error {
  override readonly name = "CliUsageError";
  readonly help: string | null;

  constructor(message: string, help: string | null) {
    super(message);
    this.help = help;
  }
}
