import { basename, extname } from "node:path";

import type { ParsedRootQuery } from "../root-query/parsed-root-query.js";
import { radiusSuffix } from "./radius-suffix.js";
import { rootQueryToken } from "./root-query-token.js";

interface OutputNameInputs {
  readonly roots: readonly ParsedRootQuery[];
  readonly descendants: number | null;
  readonly ancestors: number | null;
  readonly context: number | null;
  readonly inputPath: string | null;
}

type OutputNameResult =
  | { readonly ok: true; readonly basename: string }
  | { readonly ok: false; readonly error: string };

export function deriveOutputBasename(
  inputs: OutputNameInputs,
): OutputNameResult {
  if (inputs.roots.length > 0) {
    const rootToken = inputs.roots.map(rootQueryToken).join("+");
    const suffix = radiusSuffix(inputs);
    return { ok: true, basename: rootToken + suffix };
  }
  if (inputs.inputPath === null) {
    return {
      ok: false,
      error: "--out-dir requires either -r/--roots or an input file path",
    };
  }
  const file = basename(inputs.inputPath);
  const ext = extname(file);
  return { ok: true, basename: ext === "" ? file : file.slice(0, -ext.length) };
}
