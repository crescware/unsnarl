import { parseSync } from "oxc-parser";

import type { ParseOptions } from "../pipeline/parse/parse-options.js";
import type { ParsedSource } from "../pipeline/parse/parsed-source.js";
import type { Parser } from "../pipeline/parse/parser.js";
import { ParseError } from "./parse-error.js";

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
      const detail = fatal.map((e) => {
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
