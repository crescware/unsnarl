import type { Language } from "../../language.js";
import { defaultSourceTypeFor } from "./default-source-type-for.js";
import { SOURCE_TYPE, type SourceType } from "./source-type.js";

// `.mjs` / `.cjs` are spec-pinned: ESM and CommonJS respectively. For
// every other extension we fall back to the language-level default
// (`defaultSourceTypeFor`), which mirrors Node.js's behavior of treating
// bare `.js` as Script. Callers that have richer information (e.g.
// package.json `"type"`, an explicit CLI flag) should override the
// returned value before handing it to the parser.
export function sourceTypeFromPath(
  path: string,
  language: Language,
): SourceType {
  if (path.endsWith(".mjs")) {
    return SOURCE_TYPE.Module;
  }
  if (path.endsWith(".cjs")) {
    return SOURCE_TYPE.Script;
  }
  return defaultSourceTypeFor(language);
}
