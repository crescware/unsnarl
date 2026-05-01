import { LANGUAGE } from "../../constants.js";

export type { CliLanguage } from "../../constants.js";

export const LANGUAGES: ReadonlySet<string> = new Set(Object.values(LANGUAGE));
