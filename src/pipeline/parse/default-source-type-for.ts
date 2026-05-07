import { LANGUAGE, type Language } from "../../language.js";
import { SOURCE_TYPE, type SourceType } from "./source-type.js";

// Heuristic that mirrors Node.js's default for `.js` files (Script unless a
// loader / `package.json` says otherwise). `.ts` / `.tsx` / `.jsx` are
// treated as Module because the project that consumes them virtually
// always emits modules. Callers with better information (e.g. CLI flag,
// `.mjs` / `.cjs` extension, package.json `type`) should pass an explicit
// `sourceType` instead.
export function defaultSourceTypeFor(language: Language): SourceType {
  return language === LANGUAGE.Js ? SOURCE_TYPE.Script : SOURCE_TYPE.Module;
}
