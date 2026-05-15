import type { ParseResult } from "./parse-error.js";
import type { ParsedRootQuery } from "./parsed-root-query.js";
import { ROOT_QUERY_KIND } from "./root-query-kind.js";

const ID_RE = /^[A-Za-z_$][A-Za-z0-9_$]*$/;
const LINE_RE = /^([0-9]+)$/;
const LINE_NAME_RE = /^([0-9]+):([A-Za-z_$][A-Za-z0-9_$]*)$/;
const RANGE_RE = /^([0-9]+)-([0-9]+)$/;
const RANGE_NAME_RE = /^([0-9]+)-([0-9]+):([A-Za-z_$][A-Za-z0-9_$]*)$/;
const L_RANGE_RE = /^[Ll]([0-9]+)-([0-9]+)$/;
const L_LINE_OR_NAME_RE = /^[Ll]([0-9]+)$/;

const EMPTY_AFTER_COLON_RE = /^(?:[0-9]+(?:-[0-9]+)?|[Ll][0-9]+):$/;
const EMPTY_RANGE_END_RE = /^(?:[0-9]+|[Ll][0-9]+)-$/;
const IDENTIFIER_LIKE_HEAD_RE = /^[A-Za-z_$]/;
const NON_ID_CHAR_RE = /[^A-Za-z0-9_$]/;

export function parseEndpointQuery(text: string): ParseResult<ParsedRootQuery> {
  const lineMatch = LINE_RE.exec(text);
  if (lineMatch !== null) {
    const line = Number.parseInt(lineMatch[1] ?? "", 10);
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Line, line, raw: text },
    };
  }

  const lineNameMatch = LINE_NAME_RE.exec(text);
  if (lineNameMatch !== null) {
    const line = Number.parseInt(lineNameMatch[1] ?? "", 10);
    const name = lineNameMatch[2] ?? "";
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.LineName, line, name, raw: text },
    };
  }

  const rangeNameMatch = RANGE_NAME_RE.exec(text);
  if (rangeNameMatch !== null) {
    const start = Number.parseInt(rangeNameMatch[1] ?? "", 10);
    const end = Number.parseInt(rangeNameMatch[2] ?? "", 10);
    const name = rangeNameMatch[3] ?? "";
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.RangeName, start, end, name, raw: text },
    };
  }

  const rangeMatch = RANGE_RE.exec(text);
  if (rangeMatch !== null) {
    const start = Number.parseInt(rangeMatch[1] ?? "", 10);
    const end = Number.parseInt(rangeMatch[2] ?? "", 10);
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Range, start, end, raw: text },
    };
  }

  const lRangeMatch = L_RANGE_RE.exec(text);
  if (lRangeMatch !== null) {
    const start = Number.parseInt(lRangeMatch[1] ?? "", 10);
    const end = Number.parseInt(lRangeMatch[2] ?? "", 10);
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Range, start, end, raw: text },
    };
  }

  const lLineOrNameMatch = L_LINE_OR_NAME_RE.exec(text);
  if (lLineOrNameMatch !== null) {
    const line = Number.parseInt(lLineOrNameMatch[1] ?? "", 10);
    return {
      ok: true,
      value: {
        kind: ROOT_QUERY_KIND.LineOrName,
        line,
        name: text,
        raw: text,
      },
    };
  }

  if (ID_RE.test(text)) {
    return {
      ok: true,
      value: { kind: ROOT_QUERY_KIND.Name, name: text, raw: text },
    };
  }

  return { ok: false, errors: [{ message: diagnose(text) }] };
}

function diagnose(text: string): string {
  if (EMPTY_AFTER_COLON_RE.test(text)) {
    return `unexpected empty identifier after ':' in '${text}'`;
  }
  if (EMPTY_RANGE_END_RE.test(text)) {
    return `unexpected empty range end in '${text}' (expected 'n-m')`;
  }
  if (IDENTIFIER_LIKE_HEAD_RE.test(text) && NON_ID_CHAR_RE.test(text)) {
    return `unexpected character in identifier '${text}'`;
  }
  return `unrecognized token '${text}'`;
}
