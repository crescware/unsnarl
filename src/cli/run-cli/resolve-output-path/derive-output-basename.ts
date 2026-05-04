import { basename, extname } from "node:path";

import type { ParsedRootQuery } from "../../../root-query/parsed-root-query.js";
import { radiusSuffix } from "./radius-suffix.js";
import { rootQueryToken } from "./root-query-token.js";

type OutputNameInputs = Readonly<{
  roots: readonly ParsedRootQuery[];
  descendants: number | null;
  ancestors: number | null;
  context: number | null;
  inputPath: string;
}>;

export function deriveOutputBasename(inputs: OutputNameInputs): string {
  if (0 < inputs.roots.length) {
    const rootToken = inputs.roots.map(rootQueryToken).join("+");
    const suffix = radiusSuffix(inputs);
    return `${rootToken}${suffix}`;
  }

  const file = basename(inputs.inputPath);
  const ext = extname(file);
  return ext === "" ? file : file.slice(0, -ext.length);
}
