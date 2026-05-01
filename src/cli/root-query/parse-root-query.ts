import type { ParsedRootQuery } from "./parsed-root-query.js";

const ID_RE = /^[A-Za-z_$][A-Za-z0-9_$]*$/;
const LINE_RE = /^([0-9]+)$/;
const LINE_NAME_RE = /^([0-9]+):([A-Za-z_$][A-Za-z0-9_$]*)$/;
const RANGE_RE = /^([0-9]+)-([0-9]+)$/;
const RANGE_NAME_RE = /^([0-9]+)-([0-9]+):([A-Za-z_$][A-Za-z0-9_$]*)$/;

export function parseRootQuery(
  token: string,
): ParsedRootQuery | { error: string } {
  if (token === "") {
    return { error: "empty root query" };
  }

  const lineMatch = LINE_RE.exec(token);
  if (lineMatch !== null) {
    const line = Number.parseInt(lineMatch[1] ?? "", 10);
    if (line <= 0) {
      return { error: `invalid root query '${token}': line must be >= 1` };
    }
    return { kind: "line", line, raw: token };
  }

  const lineNameMatch = LINE_NAME_RE.exec(token);
  if (lineNameMatch !== null) {
    const line = Number.parseInt(lineNameMatch[1] ?? "", 10);
    const name = lineNameMatch[2] ?? "";
    if (line <= 0) {
      return { error: `invalid root query '${token}': line must be >= 1` };
    }
    return { kind: "line-name", line, name, raw: token };
  }

  const rangeNameMatch = RANGE_NAME_RE.exec(token);
  if (rangeNameMatch !== null) {
    const start = Number.parseInt(rangeNameMatch[1] ?? "", 10);
    const end = Number.parseInt(rangeNameMatch[2] ?? "", 10);
    const name = rangeNameMatch[3] ?? "";
    if (start <= 0 || end <= 0) {
      return { error: `invalid root query '${token}': line must be >= 1` };
    }
    if (start > end) {
      return {
        error: `invalid root query '${token}': range start must be <= end`,
      };
    }
    return { kind: "range-name", start, end, name, raw: token };
  }

  const rangeMatch = RANGE_RE.exec(token);
  if (rangeMatch !== null) {
    const start = Number.parseInt(rangeMatch[1] ?? "", 10);
    const end = Number.parseInt(rangeMatch[2] ?? "", 10);
    if (start <= 0 || end <= 0) {
      return { error: `invalid root query '${token}': line must be >= 1` };
    }
    if (start > end) {
      return {
        error: `invalid root query '${token}': range start must be <= end`,
      };
    }
    return { kind: "range", start, end, raw: token };
  }

  if (ID_RE.test(token)) {
    return { kind: "name", name: token, raw: token };
  }

  return { error: `invalid root query '${token}'` };
}
