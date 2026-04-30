import { basename, extname } from "node:path";

import type { ParsedRootQuery } from "./root-query.js";

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

function rootQueryToken(q: ParsedRootQuery): string {
  switch (q.kind) {
    case "name":
      return q.name;
    case "line":
      return `l${q.line}`;
    case "line-name":
      return `l${q.line}-${q.name}`;
    case "range":
      return `l${q.start}-${q.end}`;
    case "range-name":
      return `l${q.start}-${q.end}-${q.name}`;
  }
}

function radiusSuffix(inputs: {
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
}): string {
  // Stable a-b-c ordering keeps the filename deterministic regardless of
  // CLI argument order. Flags the user did not type are omitted, so an
  // implicit -C 10 default produces no suffix. -C is shorthand for setting
  // both -A and -B; if both are typed explicitly, -C has no remaining
  // effect on the run, so we drop it from the filename too.
  const parts: string[] = [];
  if (inputs.descendants !== null) {
    parts.push(`a${inputs.descendants}`);
  }
  if (inputs.ancestors !== null) {
    parts.push(`b${inputs.ancestors}`);
  }
  const bothExplicit = inputs.descendants !== null && inputs.ancestors !== null;
  if (inputs.context !== null && !bothExplicit) {
    parts.push(`c${inputs.context}`);
  }
  return parts.length === 0 ? "" : `-${parts.join("-")}`;
}
