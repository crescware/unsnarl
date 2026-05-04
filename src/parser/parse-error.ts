type ParseErrorDetail = Readonly<{
  message: string;
  start: number;
  end: number;
}>;

export class ParseError extends Error {
  override readonly name = "ParseError";
  readonly errors: readonly ParseErrorDetail[];

  constructor(message: string, errors: readonly ParseErrorDetail[]) {
    super(message);
    this.errors = errors;
  }
}
