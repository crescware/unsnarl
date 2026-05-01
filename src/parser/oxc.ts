import { parseSync } from "oxc-parser";

import type { ParseOptions, ParsedSource, Parser } from "../pipeline/types.js";

export type ParseErrorDetail = Readonly<{
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

export class OxcParser implements Parser {
  readonly id = "oxc";

  parse(code: string, opts: ParseOptions): ParsedSource {
    const filename = synthesizeFilename(opts.sourcePath, opts.language);
    const result = parseSync(filename, code, {
      lang: opts.language,
      sourceType: "module",
      range: true,
    });

    const fatal = result.errors.filter((e) => e.severity === "Error");
    if (fatal.length > 0) {
      const first = fatal[0];
      const head = first?.message ?? "Unknown parse error";
      const detail = fatal.map<ParseErrorDetail>((e) => {
        const label = e.labels[0];
        return {
          message: e.message,
          start: label?.start ?? 0,
          end: label?.end ?? 0,
        };
      });
      throw new ParseError(
        `Parse error in ${opts.sourcePath}: ${head}`,
        detail,
      );
    }

    return {
      ast: result.program,
      language: opts.language,
      sourcePath: opts.sourcePath,
      raw: code,
    };
  }
}

function synthesizeFilename(
  sourcePath: string,
  language: ParseOptions["language"],
): string {
  const ext = `.${language}`;
  if (sourcePath.endsWith(ext)) {
    return sourcePath;
  }
  return sourcePath || `source${ext}`;
}
